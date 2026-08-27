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
    [string]$ManifestPath = "",
    [string]$ChecksumPath = "",
    [double]$MaxP95CpuMs = 16.667,
    [double]$MaxSampledWorkingSetMb = 0,
    [int]$RelocatedRestartCount = 5,
    [switch]$AllowDirty
)

$ErrorActionPreference = "Stop"

if ($MaxP95CpuMs -le 0) {
    throw "The p95 performance limit must be greater than zero."
}
if ($MaxSampledWorkingSetMb -lt 0) {
    throw "The sampled working-set limit cannot be negative."
}
if ($RelocatedRestartCount -lt 2 -or $RelocatedRestartCount -gt 20) {
    throw "The relocated restart count must be between 2 and 20."
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

function Resolve-ProjectPath {
    param([string]$ProjectDir, [string]$Path)

    if ([IO.Path]::IsPathRooted($Path)) {
        [IO.Path]::GetFullPath($Path)
    } else {
        [IO.Path]::GetFullPath((Join-Path $ProjectDir $Path))
    }
}

function Assert-FileRecords {
    param([object[]]$Actual, [object[]]$Expected, [string]$Label)

    if ($Actual.Count -ne $Expected.Count) {
        throw "$Label records $($Expected.Count) files; archive contains $($Actual.Count)."
    }
    $expectedByPath = @{}
    foreach ($record in $Expected) {
        $path = [string]$record.path
        if ([string]::IsNullOrWhiteSpace($path) -or $expectedByPath.ContainsKey($path)) {
            throw "$Label contains an empty or duplicate file path: $path"
        }
        $expectedByPath[$path] = $record
    }
    foreach ($record in $Actual) {
        $path = [string]$record.path
        if (-not $expectedByPath.ContainsKey($path)) {
            throw "$Label omits archive file: $path"
        }
        $expected = $expectedByPath[$path]
        if ([long]$expected.bytes -ne [long]$record.bytes -or
            [string]$expected.sha256 -ne [string]$record.sha256) {
            throw "$Label has the wrong size or SHA-256 for $path."
        }
    }
}

function Get-IconPixelHash {
    param([Drawing.Icon]$Icon)

    $bitmap = $Icon.ToBitmap()
    $hasher = [Security.Cryptography.SHA256]::Create()
    try {
        if ($bitmap.Width -ne 32 -or $bitmap.Height -ne 32) {
            throw "Expected a 32x32 Windows icon; found $($bitmap.Width)x$($bitmap.Height)."
        }
        $pixels = [byte[]]::new($bitmap.Width * $bitmap.Height * 4)
        $offset = 0
        for ($y = 0; $y -lt $bitmap.Height; $y++) {
            for ($x = 0; $x -lt $bitmap.Width; $x++) {
                $pixel = $bitmap.GetPixel($x, $y)
                $pixels[$offset] = $pixel.A
                $pixels[$offset + 1] = $pixel.R
                $pixels[$offset + 2] = $pixel.G
                $pixels[$offset + 3] = $pixel.B
                $offset += 4
            }
        }
        [Convert]::ToHexString($hasher.ComputeHash($pixels)).ToLowerInvariant()
    } finally {
        $hasher.Dispose()
        $bitmap.Dispose()
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
$archiveDir = Split-Path -Parent $archive
$manifestFile = if ([string]::IsNullOrWhiteSpace($ManifestPath)) {
    Join-Path $archiveDir "hatchspire_windows_manifest.json"
} else {
    Resolve-ProjectPath $projectDir $ManifestPath
}
$checksumFile = if ([string]::IsNullOrWhiteSpace($ChecksumPath)) {
    Join-Path $archiveDir "hatchspire_windows.sha256"
} else {
    Resolve-ProjectPath $projectDir $ChecksumPath
}
if (-not (Test-Path -LiteralPath $manifestFile -PathType Leaf) -or
    -not (Test-Path -LiteralPath $checksumFile -PathType Leaf)) {
    throw "External package manifest or checksum sidecar is missing."
}
$externalManifestRaw = Get-Content -LiteralPath $manifestFile -Raw
if ($externalManifestRaw -notmatch '"built_utc"\s*:\s*"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z"') {
    throw "External package manifest has an invalid UTC build date."
}
$externalManifest = $externalManifestRaw | ConvertFrom-Json
$archiveInfo = Get-Item -LiteralPath $archive
$archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
$sidecarParts = (Get-Content -LiteralPath $checksumFile -Raw).Trim() -split '\s+'
if ($sidecarParts.Count -lt 2 -or $sidecarParts[0].ToLowerInvariant() -ne $archiveHash -or
    $sidecarParts[-1] -ne $archiveInfo.Name) {
    throw "Checksum sidecar does not identify the exact archive."
}
if ($externalManifest.schema_version -ne 1 -or
    $externalManifest.archive -ne $archiveInfo.Name -or
    [long]$externalManifest.archive_bytes -ne $archiveInfo.Length -or
    [string]$externalManifest.archive_sha256 -ne $archiveHash) {
    throw "External package manifest does not identify the exact archive."
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
        $archiveRecords = @($zip.Entries |
            Where-Object { -not [string]::IsNullOrEmpty($_.Name) } |
            ForEach-Object {
                [PSCustomObject]@{
                    path = $_.FullName.Replace('\', '/')
                    bytes = $_.Length
                    sha256 = Get-ZipEntryHash $zip $_.FullName
                }
            })
    } finally {
        $zip.Dispose()
    }

    Assert-FileRecords $archiveRecords @($externalManifest.included_files) "External manifest"
    $payloadRecords = @($archiveRecords | Where-Object { $_.path -ne "BUILD_INFO.json" })
    Assert-FileRecords $payloadRecords @($buildInfo.payload_files) "Embedded build manifest"
    $requiredPackageFiles = @(
        "hatchspire.exe", "BUILD_INFO.json", "docs/README.md", "docs/SUPPORT.md",
        "docs/KNOWN_ISSUES.md", "docs/CREDITS.md", "docs/THIRD_PARTY_NOTICES.md",
        "docs/PRIVACY.md", "docs/THIRD_PARTY_COMPONENTS.txt"
    )
    foreach ($requiredFile in $requiredPackageFiles) {
        if (-not ($archiveRecords.path -contains $requiredFile)) {
            throw "Required package file is missing: $requiredFile"
        }
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
    foreach ($field in @("package_status", "version", "build_id", "git_commit", "working_tree_dirty", "built_utc")) {
        if ([string]$externalManifest.$field -ne [string]$buildInfo.$field) {
            throw "External and embedded manifests disagree on $field."
        }
    }
    $releaseExeHash = (Get-FileHash -LiteralPath $releaseExe -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($packagedExeHash -ne $releaseExeHash) {
        throw "Packaged executable differs from the release executable."
    }
    $versionInfo = (Get-Item -LiteralPath $releaseExe).VersionInfo
    $expectedVersionFields = [ordered]@{
        FileDescription = "Hatchspire"
        ProductName = "Hatchspire"
        OriginalFilename = "hatchspire.exe"
        InternalName = "hatchspire"
        FileVersion = [string]$package.version
        ProductVersion = [string]$package.version
    }
    foreach ($field in $expectedVersionFields.Keys) {
        if ([string]$versionInfo.$field -ne $expectedVersionFields[$field]) {
            throw "Windows version field $field is '$($versionInfo.$field)'; expected '$($expectedVersionFields[$field])'."
        }
    }
    Add-Type -AssemblyName System.Drawing
    $sourceIconPath = Join-Path $projectDir "assets\branding\hatchspire.ico"
    if (-not (Test-Path -LiteralPath $sourceIconPath -PathType Leaf)) {
        throw "Windows icon source is missing: $sourceIconPath"
    }
    $expectedIcon = [Drawing.Icon]::new($sourceIconPath, [Drawing.Size]::new(32, 32))
    $embeddedIcon = [Drawing.Icon]::ExtractAssociatedIcon($releaseExe)
    try {
        if ($null -eq $embeddedIcon) {
            throw "Packaged executable has no extractable Windows icon."
        }
        $expectedIconHash = Get-IconPixelHash -Icon $expectedIcon
        $embeddedIconHash = Get-IconPixelHash -Icon $embeddedIcon
        if ($embeddedIconHash -ne $expectedIconHash) {
            throw "Packaged executable icon does not match assets/branding/hatchspire.ico."
        }
    } finally {
        $expectedIcon.Dispose()
        if ($null -ne $embeddedIcon) { $embeddedIcon.Dispose() }
    }

    $scenes = @(
        "mainmenu", "new_game_warning", "save_recovery", "autosave_notice",
        "save_migration_notice", "save_reset_warning", "help", "settings",
        "town", "hatchery", "stable", "breeding", "workshop", "shop",
        "tower", "combat", "combat_status", "combat_victory", "combat_defeat"
    )
    $outputDir = "target\release-smoke"
    $shared = Join-Path (Split-Path -Parent $projectDir) "macroquad-toolkit\scripts\capture_ui.ps1"
    $captureDir = Join-Path $projectDir $outputDir
    New-Item -ItemType Directory -Path $captureDir -Force | Out-Null
    $performanceReport = Join-Path $captureDir "performance.jsonl"
    $memoryReports = @()
    $memoryReport = Join-Path $captureDir "memory_1280x720.json"
    if (Test-Path -LiteralPath $performanceReport) {
        Remove-Item -LiteralPath $performanceReport -Force
    }
    Set-Item Env:HATCHSPIRE_PERF_REPORT $performanceReport
    try {
        & $shared -GameDir $projectDir -Scenes $scenes -Frames 30 -WindowWidth 1280 -WindowHeight 720 -OutputDir $outputDir -MinBytes 20000 -ProcessReportPath $memoryReport -SkipBuild -Release
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
    $memoryReports += Get-Content -LiteralPath $memoryReport -Raw | ConvertFrom-Json

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
        $matrixMemoryReport = Join-Path $captureDir ("memory_{0}.json" -f $resolution.Label)
        & $shared -GameDir $projectDir -Scenes $scenes -Frames 30 `
            -WindowWidth $resolution.Width -WindowHeight $resolution.Height `
            -OutputDir $matrixOutputDir -MinBytes 20000 -SkipBuild -Release `
            -ProcessReportPath $matrixMemoryReport `
            -Fullscreen:$resolution.Fullscreen
        if (-not $?) { throw "Release capture failed at $($resolution.Label)." }
        $memoryReports += Get-Content -LiteralPath $matrixMemoryReport -Raw | ConvertFrom-Json

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

    $sampledWorkingSetLimitBytes = [Math]::Round($MaxSampledWorkingSetMb * 1MB)
    foreach ($memorySample in $memoryReports) {
        if ($memorySample.scenes -ne $scenes.Count -or
            $memorySample.frames_per_scene -ne 30 -or
            $memorySample.max_sampled_working_set_bytes -le 0 -or
            $memorySample.os_peak_working_set_bytes -le 0) {
            throw "Invalid capture-process memory report."
        }
        if ($MaxSampledWorkingSetMb -gt 0 -and
            $memorySample.max_sampled_working_set_bytes -gt $sampledWorkingSetLimitBytes) {
            $sampledMb = [Math]::Round($memorySample.max_sampled_working_set_bytes / 1MB, 1)
            throw "Capture process sampled working set is $sampledMb MB; limit is $MaxSampledWorkingSetMb MB."
        }
    }
    $worstSampledMemory = $memoryReports | Sort-Object max_sampled_working_set_bytes -Descending | Select-Object -First 1
    $worstOsPeakMemory = $memoryReports | Sort-Object os_peak_working_set_bytes -Descending | Select-Object -First 1
    $worstSampledMemoryMb = [Math]::Round($worstSampledMemory.max_sampled_working_set_bytes / 1MB, 1)
    $worstOsPeakMemoryMb = [Math]::Round($worstOsPeakMemory.os_peak_working_set_bytes / 1MB, 1)

    $targetRoot = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
    $relocatedDir = Join-Path $targetRoot ("release smoke Ω path " + [Guid]::NewGuid().ToString("N"))
    Assert-ChildPath $targetRoot $relocatedDir
    $relocatedExe = Join-Path $relocatedDir "hatchspire.exe"
    $relocatedManifest = Join-Path $captureDir ".relocated_manifest_$PID.tsv"
    $relocatedStdout = Join-Path $captureDir ".relocated_stdout_$PID.log"
    $relocatedStderr = Join-Path $captureDir ".relocated_stderr_$PID.log"
    try {
        New-Item -ItemType Directory -Path $relocatedDir | Out-Null
        Copy-Item -LiteralPath $releaseExe -Destination $relocatedExe
        (Get-Item -LiteralPath $relocatedExe).IsReadOnly = $true
        Set-Item Env:HATCHSPIRE_CAPTURE_MANIFEST $relocatedManifest
        Set-Item Env:HATCHSPIRE_CAPTURE_FRAMES "30"
        Set-Item Env:HATCHSPIRE_WINDOW_WIDTH "1280"
        Set-Item Env:HATCHSPIRE_WINDOW_HEIGHT "720"
        Set-Item Env:HATCHSPIRE_HEADLESS "1"

        for ($attempt = 1; $attempt -le $RelocatedRestartCount; $attempt++) {
            $relocatedCapture = Join-Path $captureDir "ui_relocated_mainmenu_$attempt.png"
            Remove-Item -LiteralPath $relocatedCapture, $relocatedStdout, $relocatedStderr `
                -Force -ErrorAction SilentlyContinue
            Set-Content -LiteralPath $relocatedManifest -Value "mainmenu`t$relocatedCapture" -Encoding utf8

            $relocatedProcess = Start-Process -FilePath $relocatedExe -WorkingDirectory $relocatedDir `
                -PassThru -WindowStyle Hidden -RedirectStandardOutput $relocatedStdout `
                -RedirectStandardError $relocatedStderr
            if (-not $relocatedProcess.WaitForExit(60000)) {
                $relocatedProcess.Kill()
                throw "Relocated release executable attempt $attempt did not exit within 60 seconds."
            }
            if ($relocatedProcess.ExitCode -ne 0) {
                $details = @(
                    if (Test-Path -LiteralPath $relocatedStdout) { Get-Content $relocatedStdout -Tail 40 }
                    if (Test-Path -LiteralPath $relocatedStderr) { Get-Content $relocatedStderr -Tail 40 }
                ) -join [Environment]::NewLine
                throw "Relocated release executable attempt $attempt exited with code $($relocatedProcess.ExitCode). $details"
            }
            $launchFiles = @(Get-ChildItem -LiteralPath $relocatedDir -File)
            if ($launchFiles.Count -ne 1 -or $launchFiles[0].Name -ne "hatchspire.exe") {
                throw "Relocated launch attempt $attempt wrote unexpected files beside the executable."
            }
            if ((Get-Item -LiteralPath $relocatedCapture).Length -lt 20000 -or
                (Get-PngDimension $relocatedCapture 16) -ne 1280 -or
                (Get-PngDimension $relocatedCapture 20) -ne 720) {
                throw "Relocated release capture attempt $attempt is missing, blank, or the wrong size."
            }
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
    Write-Host "  Windows metadata: Hatchspire $($package.version), identity fields and custom icon verified"
    Write-Host "  Package contract: $($archiveRecords.Count) entry hashes, required documents, UTC manifest, and sidecar verified"
    Write-Host "  Scenes: $($scenes.Count) at 1280x720, 1366x768, 1920x1080, and 960x540"
    Write-Host "  Worst p95 update+draw: $($worstP95.scene) $($worstP95.p95_cpu_micros) us"
    Write-Host "  Worst single update+draw: $($worstSingle.scene) $($worstSingle.max_cpu_micros) us"
    Write-Host "  Enforced CPU limit: p95 $p95LimitMicros us"
    Write-Host "  Max sampled working set: $worstSampledMemoryMb MB across four capture processes"
    Write-Host "  Diagnostic OS peak working set: $worstOsPeakMemoryMb MB"
    if ($MaxSampledWorkingSetMb -gt 0) {
        Write-Host "  Enforced memory limit: sampled $MaxSampledWorkingSetMb MB per capture process"
    } else {
        Write-Host "  Memory limit: diagnostic only; no stable capture-process ceiling established"
    }
    Write-Host "  Relocated restarts: $RelocatedRestartCount clean starts/exits from a spaces + Unicode path, read-only EXE, no sidecar writes"
    Write-Host "  Package status: internal preview; public approval still required" -ForegroundColor Yellow
} finally {
    Pop-Location
}
