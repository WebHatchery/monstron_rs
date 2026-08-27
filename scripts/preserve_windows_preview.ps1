<#
.SYNOPSIS
    Preserves one verified internal Windows preview for rollback.

.DESCRIPTION
    Copies the exact archive, external manifest, and checksum sidecar into
    dist/history/<git-commit>. Existing preserved files are never overwritten
    unless they already match byte-for-byte.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$ChecksumPath = "dist\hatchspire_windows.sha256"
)

$ErrorActionPreference = "Stop"

function Assert-ChildPath {
    param([string]$Parent, [string]$Child)

    $parentFull = [IO.Path]::GetFullPath($Parent).TrimEnd('\', '/') + [IO.Path]::DirectorySeparatorChar
    $childFull = [IO.Path]::GetFullPath($Child)
    if (-not $childFull.StartsWith($parentFull, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Path escapes the expected directory: $childFull"
    }
}

function Resolve-DistFile {
    param([string]$ProjectDir, [string]$DistDir, [string]$Path)

    $resolved = if ([IO.Path]::IsPathRooted($Path)) {
        [IO.Path]::GetFullPath($Path)
    } else {
        [IO.Path]::GetFullPath((Join-Path $ProjectDir $Path))
    }
    Assert-ChildPath $DistDir $resolved
    if (-not (Test-Path -LiteralPath $resolved -PathType Leaf)) {
        throw "Required package evidence is missing: $resolved"
    }
    $resolved
}

function Copy-WithoutReplacingDifferentFile {
    param([string]$Source, [string]$Destination)

    if (Test-Path -LiteralPath $Destination) {
        $sourceHash = (Get-FileHash -LiteralPath $Source -Algorithm SHA256).Hash
        $destinationHash = (Get-FileHash -LiteralPath $Destination -Algorithm SHA256).Hash
        if ($sourceHash -ne $destinationHash) {
            throw "Refusing to replace different preserved evidence: $Destination"
        }
        return
    }
    Copy-Item -LiteralPath $Source -Destination $Destination
}

$projectDir = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$distDir = [IO.Path]::GetFullPath((Join-Path $projectDir "dist"))
$archive = Resolve-DistFile $projectDir $distDir $ArchivePath
$manifestFile = Resolve-DistFile $projectDir $distDir $ManifestPath
$checksumFile = Resolve-DistFile $projectDir $distDir $ChecksumPath
$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json

if ($manifest.working_tree_dirty -ne $false) {
    throw "Only a clean-build package can become rollback evidence."
}
if ([string]$manifest.git_commit -notmatch '^[0-9a-f]{40}$') {
    throw "Package manifest has an invalid Git commit."
}
$actualHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actualHash -ne [string]$manifest.archive_sha256) {
    throw "Archive hash does not match its external manifest."
}
$sidecarHash = ((Get-Content -LiteralPath $checksumFile -Raw).Trim() -split '\s+')[0].ToLowerInvariant()
if ($sidecarHash -ne $actualHash) {
    throw "Archive hash does not match its checksum sidecar."
}

$historyRoot = Join-Path $distDir "history"
$preservedDir = Join-Path $historyRoot ([string]$manifest.git_commit)
Assert-ChildPath $distDir $historyRoot
Assert-ChildPath $historyRoot $preservedDir
New-Item -ItemType Directory -Path $preservedDir -Force | Out-Null

Copy-WithoutReplacingDifferentFile $archive (Join-Path $preservedDir "hatchspire_windows.zip")
Copy-WithoutReplacingDifferentFile $manifestFile (Join-Path $preservedDir "hatchspire_windows_manifest.json")
Copy-WithoutReplacingDifferentFile $checksumFile (Join-Path $preservedDir "hatchspire_windows.sha256")

Write-Host "Rollback candidate preserved:" -ForegroundColor Green
Write-Host "  Build ID: $($manifest.build_id)"
Write-Host "  Commit: $($manifest.git_commit)"
Write-Host "  SHA-256: $actualHash"
Write-Host "  Directory: $preservedDir"
