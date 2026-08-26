<#
.SYNOPSIS
    Boots the exact packaged Windows executable through every deterministic UI scene.

.DESCRIPTION
    Run .\publish.ps1 and .\scripts\package_windows_preview.ps1 first. The script
    verifies BUILD_INFO.json against Git/Cargo, proves the packaged executable is
    byte-identical to the release build, captures every registered scene with the
    optimized executable at 1280x720, and validates every PNG header and size.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [switch]$AllowDirty
)

$ErrorActionPreference = "Stop"

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
if ($dirtyLines.Count -gt 0 -and -not $AllowDirty) {
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
    $releaseExeHash = (Get-FileHash -LiteralPath $releaseExe -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($packagedExeHash -ne $releaseExeHash) {
        throw "Packaged executable differs from the release executable."
    }

    $scenes = @(
        "mainmenu", "new_game_warning", "save_recovery", "autosave_notice",
        "save_migration_notice", "save_reset_warning", "help", "settings",
        "town", "hatchery", "stable", "breeding", "workshop", "shop",
        "tower", "combat"
    )
    $outputDir = "target\release-smoke"
    $shared = Join-Path (Split-Path -Parent $projectDir) "macroquad-toolkit\scripts\capture_ui.ps1"
    & $shared -GameDir $projectDir -Scenes $scenes -Frames 30 -WindowWidth 1280 -WindowHeight 720 -OutputDir $outputDir -MinBytes 20000 -SkipBuild -Release
    if (-not $?) { throw "Release capture harness failed." }

    $captureDir = Join-Path $projectDir $outputDir
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

    Write-Host "Release-profile smoke passed:" -ForegroundColor Green
    Write-Host "  Commit: $commit"
    Write-Host "  Version: $($package.version)"
    Write-Host "  Executable SHA-256: $releaseExeHash"
    Write-Host "  Scenes: $($scenes.Count) at 1280x720"
    Write-Host "  Package status: internal preview; public approval still required" -ForegroundColor Yellow
} finally {
    Pop-Location
}
