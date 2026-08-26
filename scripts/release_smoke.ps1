<#
.SYNOPSIS
    Boots the exact packaged Windows executable through every deterministic UI scene.

.DESCRIPTION
    Run .\publish.ps1 and .\scripts\package_windows_preview.ps1 first. The script
    verifies BUILD_INFO.json against Git/Cargo, proves the packaged executable is
    byte-identical to the release build, captures every registered scene with the
    optimized executable at 1280x720, enforces the update+draw CPU budget, and
    validates every PNG header and size.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [double]$MaxP95CpuMs = 16.667,
    [switch]$AllowDirty
)

$ErrorActionPreference = "Stop"

if ($MaxP95CpuMs -le 0) {
    throw "The p95 performance limit must be greater than zero."
}

function Get-ZipEntryText {
    param([IO.Compression.ZipArchive]$Zip, [string]$Name)

    $entry = $Zip.GetEntry($Name)
    if ($null -eq $entry) { throw "Package entry is missing: $Name" }
    $reader = [IO.StreamReader]::new($entry.Open())
    try {
        $reader.ReadToEnd()
    } finally {
        $reader.Dispose()
    }
}

function Get-ZipEntryHash {
    param([IO.Compression.ZipArchive]$Zip, [string]$Name)

    $entry = $Zip.GetEntry($Name)
    if ($null -eq $entry) { throw "Package entry is missing: $Name" }
    $stream = $entry.Open()
    $hasher = [Security.Cryptography.SHA256]::Create()
    try {
        [Convert]::ToHexString($hasher.ComputeHash($stream)).ToLowerInvariant()
    } finally {
        $hasher.Dispose()
        $stream.Dispose()
    }
}

function Get-PngDimension {
    param([string]$Path, [int]$Offset)

    $bytes = [IO.File]::ReadAllBytes($Path)
    if ($bytes.Length -lt 24 -or
        $bytes[0] -ne 137 -or $bytes[1] -ne 80 -or $bytes[2] -ne 78 -or $bytes[3] -ne 71) {
        throw "Capture is not a PNG: $Path"
    }
    ([int]$bytes[$Offset] -shl 24) -bor
        ([int]$bytes[$Offset + 1] -shl 16) -bor
        ([int]$bytes[$Offset + 2] -shl 8) -bor
        [int]$bytes[$Offset + 3]
}

function Assert-ChildPath {
    param([string]$Parent, [string]$Child)

    $parentFull = [IO.Path]::GetFullPath($Parent).TrimEnd('\', '/') + [IO.Path]::DirectorySeparatorChar
    $childFull = [IO.Path]::GetFullPath($Child)
    if (-not $childFull.StartsWith($parentFull, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Path escapes the expected directory: $childFull"
    }
}

$projectDir = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$archive = if ([IO.Path]::IsPathRooted($ArchivePath)) {
    [IO.Path]::GetFullPath($ArchivePath)
} else {
    [IO.Path]::GetFullPath((Join-Path $projectDir $ArchivePath))
}
if (-not (Test-Path -LiteralPath $archive -PathType Leaf)) {
    throw "Windows package not found: $archive"
}

$dirtyLines = @(& git -C $projectDir status --porcelain)
if ($LASTEXITCODE -ne 0) { throw "Could not inspect the Hatchspire working tree." }
$isDirty = $dirtyLines.Count -gt 0
if ($isDirty -and -not $AllowDirty) {
    throw "The working tree is dirty. Commit the smoke-test inputs or pass -AllowDirty for an internal test."
}
$commit = (& git -C $projectDir rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw "Could not resolve the Hatchspire commit." }

Push-Location $projectDir
try {
    $metadata = (& cargo metadata --manifest-path Cargo.toml --no-deps --format-version 1) |
        ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) { throw "Could not read Cargo metadata." }
    $manifest = (Resolve-Path Cargo.toml).Path
    $package = $metadata.packages |
        Where-Object { [IO.Path]::GetFullPath($_.manifest_path) -eq $manifest } |
        Select-Object -First 1
    if ($null -eq $package) { throw "Could not find Hatchspire in Cargo metadata." }
    $releaseExe = Join-Path $metadata.target_directory "release\hatchspire.exe"
    if (-not (Test-Path -LiteralPath $releaseExe -PathType Leaf)) {
        throw "Release executable is missing. Run .\publish.ps1 first: $releaseExe"
    }

    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead($archive)
    try {
        $buildInfo = (Get-ZipEntryText $zip "BUILD_INFO.json") | ConvertFrom-Json
        $packagedExeHash = Get-ZipEntryHash $zip "hatchspire.exe"
    } finally {
        $zip.Dispose()
    }

    if ($buildInfo.package_status -ne "internal_preview_not_publicly_approved") {
        throw "Unexpected package status: $($buildInfo.package_status)"
    }
    if ($buildInfo.git_commit -ne $commit) {
        throw "Package commit $($buildInfo.git_commit) does not match HEAD $commit."
    }
    if ($buildInfo.version -ne $package.version) {
        throw "Package version $($buildInfo.version) does not match Cargo $($package.version)."
    }
    $dirtySuffix = if ($isDirty) { "-dirty" } else { "" }
    $expectedBuildId = "$($package.version)+g$($commit.Substring(0, [Math]::Min(12, $commit.Length)))$dirtySuffix"
    if ($buildInfo.build_id -ne $expectedBuildId) {
        throw "Package build ID $($buildInfo.build_id) does not match $expectedBuildId."
    }
    $releaseExeHash = (Get-FileHash -LiteralPath $releaseExe -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($packagedExeHash -ne $releaseExeHash) {
        throw "Packaged executable differs from the release executable."
    }

    $scenes = @(
        "mainmenu", "new_game_warning", "save_recovery", "autosave_notice",
        "save_migration_notice", "save_reset_warning", "help", "settings",
        "town", "hatchery", "stable", "breeding", "workshop", "shop",
        "tower", "combat", "combat_victory", "combat_defeat"
    )
    $outputDir = "target\release-smoke"
    $shared = Join-Path (Split-Path -Parent $projectDir) "macroquad-toolkit\scripts\capture_ui.ps1"
    $captureDir = Join-Path $projectDir $outputDir
    New-Item -ItemType Directory -Path $captureDir -Force | Out-Null
    $performanceReport = Join-Path $captureDir "performance.jsonl"
    if (Test-Path -LiteralPath $performanceReport) {
        Remove-Item -LiteralPath $performanceReport -Force
    }
    Set-Item Env:HATCHSPIRE_PERF_REPORT $performanceReport
    try {
        & $shared -GameDir $projectDir -Scenes $scenes -Frames 30 -WindowWidth 1280 -WindowHeight 720 -OutputDir $outputDir -MinBytes 20000 -SkipBuild -Release
        if (-not $?) { throw "Release capture harness failed." }
    } finally {
        Remove-Item Env:HATCHSPIRE_PERF_REPORT -ErrorAction SilentlyContinue
    }

    if (-not (Test-Path -LiteralPath $performanceReport -PathType Leaf)) {
        throw "Release capture did not write its performance report."
    }
    $performanceSamples = @(Get-Content -LiteralPath $performanceReport |
        Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
        ForEach-Object { $_ | ConvertFrom-Json })
    if ($performanceSamples.Count -ne $scenes.Count) {
        throw "Performance report contains $($performanceSamples.Count) scenes; expected $($scenes.Count)."
    }
    $p95LimitMicros = [Math]::Round($MaxP95CpuMs * 1000.0)
    foreach ($sample in $performanceSamples) {
        if ($sample.frames -ne 30 -or $sample.width -ne 1280 -or $sample.height -ne 720) {
            throw "Invalid performance sample shape for $($sample.scene)."
        }
        if ($sample.p95_cpu_micros -gt $p95LimitMicros) {
            throw "$($sample.scene) p95 CPU time is $($sample.p95_cpu_micros) us; limit is $p95LimitMicros us."
        }
    }
    $worstP95 = $performanceSamples | Sort-Object p95_cpu_micros -Descending | Select-Object -First 1
    $worstSingle = $performanceSamples | Sort-Object max_cpu_micros -Descending | Select-Object -First 1

    foreach ($scene in $scenes) {
        $path = Join-Path $captureDir "ui_$scene.png"
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Release smoke capture is missing: $path"
        }
        $width = Get-PngDimension $path 16
        $height = Get-PngDimension $path 20
        if ($width -ne 1280 -or $height -ne 720) {
            throw ("Release smoke capture has wrong dimensions: {0} is {1}x{2}" -f $path, $width, $height)
        }
    }

    $additionalResolutions = @(
        [pscustomobject]@{ Width = 1366; Height = 768; Label = "1366x768"; Fullscreen = $false },
        [pscustomobject]@{ Width = 1920; Height = 1080; Label = "1920x1080"; Fullscreen = $true },
        [pscustomobject]@{ Width = 960; Height = 540; Label = "960x540"; Fullscreen = $false }
    )
    foreach ($resolution in $additionalResolutions) {
        $matrixOutputDir = Join-Path $outputDir $resolution.Label
        if ($resolution.Fullscreen) {
            Set-Item Env:HATCHSPIRE_CAPTURE_FULLSCREEN "1"
        }
        try {
            & $shared -GameDir $projectDir -Scenes $scenes -Frames 30 `
                -WindowWidth $resolution.Width -WindowHeight $resolution.Height `
                -OutputDir $matrixOutputDir -MinBytes 20000 -SkipBuild -Release
            if (-not $?) { throw "Release capture failed at $($resolution.Label)." }
        } finally {
            Remove-Item Env:HATCHSPIRE_CAPTURE_FULLSCREEN -ErrorAction SilentlyContinue
        }

        $matrixCaptureDir = Join-Path $projectDir $matrixOutputDir
        foreach ($scene in $scenes) {
            $path = Join-Path $matrixCaptureDir "ui_$scene.png"
            if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
                throw "Release matrix capture is missing: $path"
            }
            $width = Get-PngDimension $path 16
            $height = Get-PngDimension $path 20
            if ($width -ne $resolution.Width -or $height -ne $resolution.Height) {
                throw ("Release matrix capture has wrong dimensions: {0} is {1}x{2}" -f $path, $width, $height)
            }
        }
    }

    $targetRoot = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
    $relocatedDir = Join-Path $targetRoot ("release smoke Ω path " + [Guid]::NewGuid().ToString("N"))
    Assert-ChildPath $targetRoot $relocatedDir
    $relocatedExe = Join-Path $relocatedDir "hatchspire.exe"
    $relocatedCapture = Join-Path $captureDir "ui_relocated_mainmenu.png"
    $relocatedManifest = Join-Path $captureDir ".relocated_manifest_$PID.tsv"
    $relocatedStdout = Join-Path $captureDir ".relocated_stdout_$PID.log"
    $relocatedStderr = Join-Path $captureDir ".relocated_stderr_$PID.log"
    try {
        New-Item -ItemType Directory -Path $relocatedDir | Out-Null
        Copy-Item -LiteralPath $releaseExe -Destination $relocatedExe
        (Get-Item -LiteralPath $relocatedExe).IsReadOnly = $true
        if (Test-Path -LiteralPath $relocatedCapture) {
            Remove-Item -LiteralPath $relocatedCapture -Force
        }
        Set-Content -LiteralPath $relocatedManifest -Value "mainmenu`t$relocatedCapture" -Encoding utf8
        Set-Item Env:HATCHSPIRE_CAPTURE_MANIFEST $relocatedManifest
        Set-Item Env:HATCHSPIRE_CAPTURE_FRAMES "30"
        Set-Item Env:HATCHSPIRE_WINDOW_WIDTH "1280"
        Set-Item Env:HATCHSPIRE_WINDOW_HEIGHT "720"
        Set-Item Env:HATCHSPIRE_HEADLESS "1"

        $relocatedProcess = Start-Process -FilePath $relocatedExe -WorkingDirectory $relocatedDir `
            -PassThru -WindowStyle Hidden -RedirectStandardOutput $relocatedStdout `
            -RedirectStandardError $relocatedStderr
        if (-not $relocatedProcess.WaitForExit(60000)) {
            $relocatedProcess.Kill()
            throw "Relocated release executable did not exit within 60 seconds."
        }
        if ($relocatedProcess.ExitCode -ne 0) {
            $details = @(
                if (Test-Path -LiteralPath $relocatedStdout) { Get-Content $relocatedStdout -Tail 40 }
                if (Test-Path -LiteralPath $relocatedStderr) { Get-Content $relocatedStderr -Tail 40 }
            ) -join [Environment]::NewLine
            throw "Relocated release executable exited with code $($relocatedProcess.ExitCode). $details"
        }
        $launchFiles = @(Get-ChildItem -LiteralPath $relocatedDir -File)
        if ($launchFiles.Count -ne 1 -or $launchFiles[0].Name -ne "hatchspire.exe") {
            throw "Relocated launch wrote unexpected files beside the executable."
        }
        if ((Get-Item -LiteralPath $relocatedCapture).Length -lt 20000 -or
            (Get-PngDimension $relocatedCapture 16) -ne 1280 -or
            (Get-PngDimension $relocatedCapture 20) -ne 720) {
            throw "Relocated release capture is missing, blank, or the wrong size."
        }
    } finally {
        Remove-Item Env:HATCHSPIRE_CAPTURE_MANIFEST, Env:HATCHSPIRE_CAPTURE_FRAMES, `
            Env:HATCHSPIRE_WINDOW_WIDTH, Env:HATCHSPIRE_WINDOW_HEIGHT, `
            Env:HATCHSPIRE_HEADLESS -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $relocatedManifest, $relocatedStdout, $relocatedStderr `
            -Force -ErrorAction SilentlyContinue
        if (Test-Path -LiteralPath $relocatedExe) {
            (Get-Item -LiteralPath $relocatedExe).IsReadOnly = $false
        }
        if (Test-Path -LiteralPath $relocatedDir) {
            Assert-ChildPath $targetRoot $relocatedDir
            Remove-Item -LiteralPath $relocatedDir -Recurse -Force
        }
    }

    Write-Host "Release-profile smoke passed:" -ForegroundColor Green
    Write-Host "  Commit: $commit"
    Write-Host "  Build ID: $expectedBuildId"
    Write-Host "  Executable SHA-256: $releaseExeHash"
    Write-Host "  Scenes: $($scenes.Count) at 1280x720, 1366x768, 1920x1080, and 960x540"
    Write-Host "  Worst p95 update+draw: $($worstP95.scene) $($worstP95.p95_cpu_micros) us"
    Write-Host "  Worst single update+draw: $($worstSingle.scene) $($worstSingle.max_cpu_micros) us"
    Write-Host "  Enforced CPU limit: p95 $p95LimitMicros us"
    Write-Host "  Relocated launch: spaces + Unicode path, read-only EXE, no sidecar writes"
    Write-Host "  Package status: internal preview; public approval still required" -ForegroundColor Yellow
} finally {
    Pop-Location
}
