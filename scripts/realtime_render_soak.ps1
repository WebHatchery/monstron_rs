<#
.SYNOPSIS
    Runs a wall-clock render/device soak against the exact packaged Windows executable.

.DESCRIPTION
    The default is a visible two-hour run. Shorter or headless runs validate the harness but do not
    satisfy the release gate. The script verifies the archive identity, extracts its exact EXE into
    an isolated target directory, cycles all release scenes in one process, enforces a minimum frame
    duration, and records CPU, elapsed-time, memory-distribution, and capture evidence locally.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$ChecksumPath = "dist\hatchspire_windows.sha256",
    [int]$DurationSeconds = 7200,
    [double]$MinFrameMilliseconds = 16.667,
    [double]$MaxP95CpuMs = 16.667,
    [int]$WindowWidth = 1280,
    [int]$WindowHeight = 720,
    [switch]$Fullscreen,
    [switch]$Headless,
    [switch]$AllowDirty
)

$ErrorActionPreference = "Stop"

if ($DurationSeconds -lt 5 -or $DurationSeconds -gt 14400) {
    throw "DurationSeconds must be between 5 seconds and the four-hour release maximum (14400)."
}
if ($MinFrameMilliseconds -lt 5 -or $MinFrameMilliseconds -gt 1000) {
    throw "MinFrameMilliseconds must be between 5 and 1000."
}
if ($MaxP95CpuMs -le 0) { throw "MaxP95CpuMs must be greater than zero." }
if ($WindowWidth -lt 640 -or $WindowHeight -lt 360) {
    throw "The soak surface must be at least 640x360."
}

function Resolve-ProjectPath {
    param([string]$ProjectDir, [string]$Path)

    if ([IO.Path]::IsPathRooted($Path)) {
        [IO.Path]::GetFullPath($Path)
    } else {
        [IO.Path]::GetFullPath((Join-Path $ProjectDir $Path))
    }
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
$archive = Resolve-ProjectPath $projectDir $ArchivePath
$manifestFile = Resolve-ProjectPath $projectDir $ManifestPath
$checksumFile = Resolve-ProjectPath $projectDir $ChecksumPath
foreach ($required in @($archive, $manifestFile, $checksumFile)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Required package evidence is missing: $required"
    }
}

$dirtyLines = @(& git -C $projectDir status --porcelain)
if ($LASTEXITCODE -ne 0) { throw "Could not inspect the Hatchspire working tree." }
$isDirty = $dirtyLines.Count -gt 0
if ($isDirty -and -not $AllowDirty) {
    throw "The working tree is dirty. Commit the candidate or pass -AllowDirty for harness validation."
}
$commit = (& git -C $projectDir rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw "Could not resolve the Hatchspire commit." }

$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json
$archiveInfo = Get-Item -LiteralPath $archive
$archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ([string]$manifest.archive -ne $archiveInfo.Name -or
    [long]$manifest.archive_bytes -ne $archiveInfo.Length -or
    [string]$manifest.archive_sha256 -ne $archiveHash) {
    throw "External manifest does not identify the exact archive."
}
$checksumText = (Get-Content -LiteralPath $checksumFile -Raw).Trim()
if ($checksumText -notmatch '^([0-9A-Fa-f]{64})\s+(.+)$' -or
    $Matches[1].ToLowerInvariant() -ne $archiveHash -or $Matches[2] -ne $archiveInfo.Name) {
    throw "Checksum sidecar does not identify the exact archive."
}
if ([string]$manifest.git_commit -ne $commit -or [bool]$manifest.working_tree_dirty -ne $isDirty) {
    throw "Archive identity does not match the current Git candidate."
}

$scenes = @(
    "mainmenu", "new_game_warning", "save_recovery", "autosave_notice",
    "save_migration_notice", "save_reset_warning", "help", "settings",
    "town", "hatchery", "stable", "breeding", "workshop", "shop",
    "tower", "combat", "combat_status", "combat_victory", "combat_defeat"
)
$framesPerScene = [Math]::Ceiling(
    ($DurationSeconds * 1000.0) / ($MinFrameMilliseconds * $scenes.Count)
)
if ($framesPerScene -gt [uint32]::MaxValue) { throw "Requested soak frame count is too large." }
$scheduledMilliseconds = $framesPerScene * $MinFrameMilliseconds * $scenes.Count
$timeoutSeconds = [int][Math]::Ceiling(($scheduledMilliseconds / 1000.0) * 1.25 + 120)

$targetRoot = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
$evidenceDir = Join-Path $targetRoot "realtime-render-soak"
$runDir = Join-Path $targetRoot ("realtime-render-soak-run-" + [Guid]::NewGuid().ToString("N"))
$packageDir = Join-Path $runDir "package"
Assert-ChildPath $targetRoot $evidenceDir
Assert-ChildPath $targetRoot $runDir
$processReport = Join-Path $evidenceDir "process.json"
$performanceReport = Join-Path $evidenceDir "performance.jsonl"
$summaryReport = Join-Path $evidenceDir "summary.json"
$captureOutput = "target\realtime-render-soak\captures"
$sharedCapture = Join-Path (Split-Path -Parent $projectDir) "macroquad-toolkit\scripts\capture_ui.ps1"

try {
    New-Item -ItemType Directory -Path $packageDir -Force | Out-Null
    New-Item -ItemType Directory -Path $evidenceDir -Force | Out-Null
    Remove-Item -LiteralPath $processReport, $performanceReport, $summaryReport `
        -Force -ErrorAction SilentlyContinue

    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead($archive)
    try {
        foreach ($entry in $zip.Entries) {
            $segments = @($entry.FullName.Replace('\', '/').Split('/'))
            if ([IO.Path]::IsPathRooted($entry.FullName) -or $segments -contains "..") {
                throw "Unsafe path in archive: $($entry.FullName)"
            }
        }
    } finally {
        $zip.Dispose()
    }
    [IO.Compression.ZipFile]::ExtractToDirectory($archive, $packageDir)

    $executable = Join-Path $packageDir "hatchspire.exe"
    $exeRecord = @($manifest.included_files | Where-Object { $_.path -eq "hatchspire.exe" })
    if ($exeRecord.Count -ne 1 -or -not (Test-Path -LiteralPath $executable -PathType Leaf)) {
        throw "Archive or manifest does not contain exactly one hatchspire.exe."
    }
    $exeInfo = Get-Item -LiteralPath $executable
    $exeHash = (Get-FileHash -LiteralPath $executable -Algorithm SHA256).Hash.ToLowerInvariant()
    if ([long]$exeRecord[0].bytes -ne $exeInfo.Length -or
        [string]$exeRecord[0].sha256 -ne $exeHash) {
        throw "Extracted executable does not match the external manifest."
    }

    Set-Item Env:HATCHSPIRE_PERF_REPORT $performanceReport
    $captureArgs = @{
        GameDir = $projectDir
        Prefix = "HATCHSPIRE"
        ExecutablePath = $executable
        Scenes = $scenes
        Frames = [int]$framesPerScene
        MinFrameMilliseconds = $MinFrameMilliseconds
        WindowWidth = $WindowWidth
        WindowHeight = $WindowHeight
        OutputDir = $captureOutput
        ProcessReportPath = $processReport
        MinBytes = 20000
        TimeoutSeconds = $timeoutSeconds
        SkipBuild = $true
        Release = $true
    }
    if ($Fullscreen) { $captureArgs.Fullscreen = $true }
    if (-not $Headless) { $captureArgs.Visible = $true }
    & $sharedCapture @captureArgs
    if (-not $?) { throw "Sustained capture harness failed." }

    $process = Get-Content -LiteralPath $processReport -Raw | ConvertFrom-Json
    if ([long]$process.elapsed_wall_milliseconds -lt ($DurationSeconds * 1000) -or
        [int]$process.scenes -ne $scenes.Count -or
        [int]$process.frames_per_scene -ne $framesPerScene -or
        [int]$process.sample_count -le 0 -or
        [long]$process.median_sampled_working_set_bytes -gt [long]$process.p95_sampled_working_set_bytes -or
        [long]$process.p95_sampled_working_set_bytes -gt [long]$process.max_sampled_working_set_bytes) {
        throw "Sustained process evidence is incomplete or internally inconsistent."
    }

    $timings = @(Get-Content -LiteralPath $performanceReport | ForEach-Object { $_ | ConvertFrom-Json })
    if ($timings.Count -ne $scenes.Count) { throw "Expected one timing record per release scene." }
    $p95LimitMicros = [Math]::Round($MaxP95CpuMs * 1000)
    foreach ($timing in $timings) {
        if ([int]$timing.frames -ne $framesPerScene -or [long]$timing.p95_cpu_micros -gt $p95LimitMicros) {
            throw "Sustained CPU evidence failed for scene $($timing.scene)."
        }
    }
    $worstP95 = $timings | Sort-Object p95_cpu_micros -Descending | Select-Object -First 1
    $worstSingle = $timings | Sort-Object max_cpu_micros -Descending | Select-Object -First 1
    $releaseDurationMet = $DurationSeconds -ge 7200 -and $DurationSeconds -le 14400
    $releaseEvidence = $releaseDurationMet -and -not $Headless
    $status = if ($releaseEvidence) { "release_evidence" } else { "validation_only" }

    [pscustomobject]@{
        schema_version = 1
        status = $status
        package_status = $manifest.package_status
        version = $manifest.version
        build_id = $manifest.build_id
        git_commit = $manifest.git_commit
        archive_sha256 = $archiveHash
        executable_sha256 = $exeHash
        requested_duration_seconds = $DurationSeconds
        scheduled_duration_milliseconds = [long][Math]::Ceiling($scheduledMilliseconds)
        elapsed_wall_milliseconds = [long]$process.elapsed_wall_milliseconds
        visible = -not [bool]$Headless
        fullscreen = [bool]$Fullscreen
        width = $WindowWidth
        height = $WindowHeight
        scenes = $scenes.Count
        frames_per_scene = [int]$framesPerScene
        total_frames = [long]$framesPerScene * $scenes.Count
        worst_p95_cpu_scene = $worstP95.scene
        worst_p95_cpu_micros = [long]$worstP95.p95_cpu_micros
        worst_single_cpu_scene = $worstSingle.scene
        worst_single_cpu_micros = [long]$worstSingle.max_cpu_micros
        memory_sample_count = [int]$process.sample_count
        median_working_set_bytes = [long]$process.median_sampled_working_set_bytes
        p95_working_set_bytes = [long]$process.p95_sampled_working_set_bytes
        final_working_set_bytes = [long]$process.final_sampled_working_set_bytes
        max_working_set_bytes = [long]$process.max_sampled_working_set_bytes
        os_peak_working_set_bytes = [long]$process.os_peak_working_set_bytes
    } | ConvertTo-Json | Set-Content -LiteralPath $summaryReport -Encoding utf8

    $elapsedMinutes = [Math]::Round($process.elapsed_wall_milliseconds / 60000.0, 2)
    $p95MemoryMb = [Math]::Round($process.p95_sampled_working_set_bytes / 1MB, 1)
    $finalMemoryMb = [Math]::Round($process.final_sampled_working_set_bytes / 1MB, 1)
    Write-Host "Sustained exact-package render probe passed:" -ForegroundColor Green
    Write-Host "  Status: $status"
    Write-Host "  Build ID: $($manifest.build_id)"
    Write-Host "  Archive SHA-256: $archiveHash"
    Write-Host "  Elapsed: $elapsedMinutes minutes across $($scenes.Count) scenes / $([long]$framesPerScene * $scenes.Count) frames"
    Write-Host "  Worst p95 update+draw: $($worstP95.scene) $($worstP95.p95_cpu_micros) us"
    Write-Host "  Working set: p95 $p95MemoryMb MB; final $finalMemoryMb MB"
    Write-Host "  Evidence: $summaryReport"
    if (-not $releaseEvidence) {
        Write-Host "  This short or headless run validates the harness but does not satisfy the visible 2-4 hour release gate." -ForegroundColor Yellow
    }
} finally {
    Remove-Item Env:HATCHSPIRE_PERF_REPORT -ErrorAction SilentlyContinue
    if (Test-Path -LiteralPath $runDir) {
        Assert-ChildPath $targetRoot $runDir
        Remove-Item -LiteralPath $runDir -Recurse -Force
    }
}
