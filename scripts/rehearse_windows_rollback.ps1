<#
.SYNOPSIS
    Rehearses patch deployment, rollback, and patch restoration locally.

.DESCRIPTION
    Uses a preserved clean package as the previous build and the current clean
    package as the patch. Copies both through an isolated target directory and
    verifies the archive hash and embedded build identity at every switch.
#>
param(
    [Parameter(Mandatory = $true)]
    [string]$PreservedDir,
    [string]$CurrentArchivePath = "dist\hatchspire_windows.zip",
    [string]$CurrentManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$CurrentChecksumPath = "dist\hatchspire_windows.sha256"
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

function Resolve-ProjectPath {
    param([string]$ProjectDir, [string]$Path)

    if ([IO.Path]::IsPathRooted($Path)) {
        [IO.Path]::GetFullPath($Path)
    } else {
        [IO.Path]::GetFullPath((Join-Path $ProjectDir $Path))
    }
}

function Read-Candidate {
    param([string]$Archive, [string]$ManifestPath, [string]$ChecksumPath)

    if (-not (Test-Path -LiteralPath $Archive -PathType Leaf) -or
        -not (Test-Path -LiteralPath $ManifestPath -PathType Leaf) -or
        -not (Test-Path -LiteralPath $ChecksumPath -PathType Leaf)) {
        throw "Rollback candidate archive, manifest, or checksum is missing."
    }
    $manifest = Get-Content -LiteralPath $ManifestPath -Raw | ConvertFrom-Json
    if ($manifest.working_tree_dirty -ne $false) {
        throw "Rollback rehearsal requires clean-build candidates."
    }
    $hash = (Get-FileHash -LiteralPath $Archive -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($hash -ne [string]$manifest.archive_sha256) {
        throw "Candidate archive does not match its manifest: $Archive"
    }
    if ((Get-Item -LiteralPath $Archive).Length -ne [long]$manifest.archive_bytes) {
        throw "Candidate archive size does not match its manifest: $Archive"
    }
    $sidecarHash = ((Get-Content -LiteralPath $ChecksumPath -Raw).Trim() -split '\s+')[0].ToLowerInvariant()
    if ($sidecarHash -ne $hash) {
        throw "Candidate archive does not match its checksum sidecar: $Archive"
    }
    [PSCustomObject]@{
        Archive = $Archive
        Commit = [string]$manifest.git_commit
        BuildId = [string]$manifest.build_id
        Hash = $hash
    }
}

function Assert-DeployedCandidate {
    param([string]$Archive, [PSCustomObject]$Candidate)

    $hash = (Get-FileHash -LiteralPath $Archive -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($hash -ne $Candidate.Hash) {
        throw "Deployed rehearsal archive hash mismatch."
    }

    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead($Archive)
    try {
        $entry = $zip.GetEntry("BUILD_INFO.json")
        if ($null -eq $entry) { throw "Rehearsal archive lacks BUILD_INFO.json." }
        $reader = [IO.StreamReader]::new($entry.Open())
        try { $buildInfo = $reader.ReadToEnd() | ConvertFrom-Json } finally { $reader.Dispose() }
    } finally {
        $zip.Dispose()
    }
    if ([string]$buildInfo.git_commit -ne $Candidate.Commit -or
        [string]$buildInfo.build_id -ne $Candidate.BuildId) {
        throw "Embedded build identity does not match the external manifest."
    }
}

$projectDir = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$distDir = [IO.Path]::GetFullPath((Join-Path $projectDir "dist"))
$historyRoot = Join-Path $distDir "history"
$preserved = Resolve-ProjectPath $projectDir $PreservedDir
Assert-ChildPath $historyRoot $preserved
$previous = Read-Candidate `
    (Join-Path $preserved "hatchspire_windows.zip") `
    (Join-Path $preserved "hatchspire_windows_manifest.json") `
    (Join-Path $preserved "hatchspire_windows.sha256")
$current = Read-Candidate `
    (Resolve-ProjectPath $projectDir $CurrentArchivePath) `
    (Resolve-ProjectPath $projectDir $CurrentManifestPath) `
    (Resolve-ProjectPath $projectDir $CurrentChecksumPath)
if ($previous.Commit -eq $current.Commit) {
    throw "Previous and patch candidates must identify different commits."
}

$targetRoot = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
$rehearsalDir = Join-Path $targetRoot ("rollback-rehearsal-" + [Guid]::NewGuid().ToString("N"))
$deployedArchive = Join-Path $rehearsalDir "hatchspire_windows.zip"
$evidencePath = Join-Path $targetRoot "release-rollback-rehearsal.json"
Assert-ChildPath $targetRoot $rehearsalDir

try {
    New-Item -ItemType Directory -Path $rehearsalDir | Out-Null

    Copy-Item -LiteralPath $current.Archive -Destination $deployedArchive
    Assert-DeployedCandidate $deployedArchive $current

    Copy-Item -LiteralPath $previous.Archive -Destination $deployedArchive -Force
    Assert-DeployedCandidate $deployedArchive $previous

    Copy-Item -LiteralPath $current.Archive -Destination $deployedArchive -Force
    Assert-DeployedCandidate $deployedArchive $current

    [ordered]@{
        schema_version = 1
        previous_commit = $previous.Commit
        previous_build_id = $previous.BuildId
        previous_sha256 = $previous.Hash
        patch_commit = $current.Commit
        patch_build_id = $current.BuildId
        patch_sha256 = $current.Hash
        sequence = @("patch", "rollback", "patch_restored")
        final_sha256 = $current.Hash
    } | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath $evidencePath -Encoding utf8

    Write-Host "Local rollback rehearsal passed:" -ForegroundColor Green
    Write-Host "  Patch: $($current.BuildId) $($current.Hash)"
    Write-Host "  Rollback: $($previous.BuildId) $($previous.Hash)"
    Write-Host "  Final state: patch restored and hash verified"
    Write-Host "  Evidence: $evidencePath"
} finally {
    if (Test-Path -LiteralPath $rehearsalDir) {
        Assert-ChildPath $targetRoot $rehearsalDir
        Remove-Item -LiteralPath $rehearsalDir -Recurse -Force
    }
}
