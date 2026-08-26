<#
.SYNOPSIS
    Turns the publisher's Windows ZIP into a self-describing internal preview.

.DESCRIPTION
    Run .\publish.ps1 first. This script preserves the built executable, adds
    the player/support/legal drafts, records payload hashes and build identity,
    replaces dist\hatchspire_windows.zip, and emits an external manifest plus
    SHA-256 sidecar for the exact archive.

    This does not approve the build for public release. The packaged documents
    deliberately retain the unresolved human rights and support gates.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [switch]$AllowDirty
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

function Get-FileRecord {
    param([string]$Root, [IO.FileInfo]$File)

    $relative = [IO.Path]::GetRelativePath($Root, $File.FullName).Replace('\', '/')
    [ordered]@{
        path = $relative
        bytes = $File.Length
        sha256 = (Get-FileHash -LiteralPath $File.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    }
}

$projectDir = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$distDir = [IO.Path]::GetFullPath((Join-Path $projectDir "dist"))
$archive = if ([IO.Path]::IsPathRooted($ArchivePath)) {
    [IO.Path]::GetFullPath($ArchivePath)
} else {
    [IO.Path]::GetFullPath((Join-Path $projectDir $ArchivePath))
}
Assert-ChildPath $distDir $archive

if (-not (Test-Path -LiteralPath $archive -PathType Leaf)) {
    throw "Windows archive not found. Run .\publish.ps1 first: $archive"
}

$dirtyLines = @(& git -C $projectDir status --porcelain)
if ($LASTEXITCODE -ne 0) { throw "Could not inspect the Hatchspire working tree." }
$isDirty = $dirtyLines.Count -gt 0
if ($isDirty -and -not $AllowDirty) {
    throw "The working tree is dirty. Commit the candidate inputs or pass -AllowDirty for an internal test."
}

$commit = (& git -C $projectDir rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw "Could not resolve the Hatchspire commit." }

Push-Location $projectDir
try {
    $metadata = (& cargo metadata --manifest-path Cargo.toml --format-version 1) |
        ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) { throw "Could not read Cargo package metadata." }
    $manifest = (Resolve-Path Cargo.toml).Path
    $package = $metadata.packages |
        Where-Object { [IO.Path]::GetFullPath($_.manifest_path) -eq $manifest } |
        Select-Object -First 1
    if ($null -eq $package) { throw "Could not find Hatchspire in Cargo metadata." }
    $version = [string]$package.version

    $treeLines = @(& cargo tree -p hatchspire --target x86_64-pc-windows-msvc -e normal --prefix none --format "{p}|{l}")
    if ($LASTEXITCODE -ne 0) { throw "Could not inventory Windows dependencies." }
} finally {
    Pop-Location
}

$stamp = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
$shortCommit = $commit.Substring(0, [Math]::Min(12, $commit.Length))
$dirtySuffix = if ($isDirty) { "-dirty" } else { "" }
$displayBuildId = "$version+g$shortCommit$dirtySuffix"
$workDir = Join-Path $distDir (".windows-preview-" + [Guid]::NewGuid().ToString("N"))
$candidateArchive = Join-Path $distDir (".hatchspire-windows-" + [Guid]::NewGuid().ToString("N") + ".zip")
Assert-ChildPath $distDir $workDir
Assert-ChildPath $distDir $candidateArchive

$documentMap = [ordered]@{
    "PLAYER_README.md" = "README.md"
    "SUPPORT.md" = "SUPPORT.md"
    "KNOWN_ISSUES.md" = "KNOWN_ISSUES.md"
    "PRIVACY.md" = "PRIVACY.md"
    "CREDITS.md" = "CREDITS.md"
    "THIRD_PARTY_NOTICES.md" = "THIRD_PARTY_NOTICES.md"
}

try {
    New-Item -ItemType Directory -Path $workDir | Out-Null

    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $sourceZip = [IO.Compression.ZipFile]::OpenRead($archive)
    try {
        foreach ($entry in $sourceZip.Entries) {
            $segments = @($entry.FullName.Replace('\', '/').Split('/'))
            if ([IO.Path]::IsPathRooted($entry.FullName) -or $segments -contains "..") {
                throw "Unsafe path in source archive: $($entry.FullName)"
            }
        }
    } finally {
        $sourceZip.Dispose()
    }
    [IO.Compression.ZipFile]::ExtractToDirectory($archive, $workDir)

    $executable = Join-Path $workDir "hatchspire.exe"
    if (-not (Test-Path -LiteralPath $executable -PathType Leaf)) {
        throw "The source archive does not contain hatchspire.exe at its root."
    }

    $buildInfoPath = Join-Path $workDir "BUILD_INFO.json"
    if (Test-Path -LiteralPath $buildInfoPath) {
        Remove-Item -LiteralPath $buildInfoPath -Force
    }
    $docsDir = Join-Path $workDir "docs"
    if (Test-Path -LiteralPath $docsDir) {
        Assert-ChildPath $workDir $docsDir
        Remove-Item -LiteralPath $docsDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $docsDir -Force | Out-Null
    foreach ($sourceName in $documentMap.Keys) {
        $source = Join-Path $projectDir $sourceName
        if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
            throw "Required package document is missing: $sourceName"
        }
        Copy-Item -LiteralPath $source -Destination (Join-Path $docsDir $documentMap[$sourceName]) -Force
    }

    $dependencies = @{}
    foreach ($line in $treeLines) {
        if ($line -notmatch '^(?<name>[A-Za-z0-9_.-]+) v(?<version>[^ ]+)(?: \([^|]*\))?\|(?<license>.*?)(?: \(\*\))?$') {
            continue
        }
        $key = "$($Matches.name) $($Matches.version)"
        $license = $Matches.license.Trim()
        if ([string]::IsNullOrWhiteSpace($license)) { $license = "NOT DECLARED IN CARGO METADATA" }
        $dependencies[$key] = $license
    }
    $licensesDir = Join-Path $docsDir "licenses"
    New-Item -ItemType Directory -Path $licensesDir -Force | Out-Null
    $licensePaths = @{}
    $licenseFileGaps = [Collections.Generic.List[string]]::new()
    foreach ($dependency in $dependencies.GetEnumerator()) {
        $parts = $dependency.Name.Split(' ', 2)
        $dependencyPackage = $metadata.packages |
            Where-Object { $_.name -eq $parts[0] -and $_.version -eq $parts[1] } |
            Select-Object -First 1
        if ($null -eq $dependencyPackage -or $null -eq $dependencyPackage.source) {
            $licensePaths[$dependency.Name] = @()
            continue
        }

        $sourceDir = Split-Path $dependencyPackage.manifest_path -Parent
        $licenseFiles = @(Get-ChildItem -LiteralPath $sourceDir -File |
            Where-Object { $_.Name -match '^(LICENSE|LICENCE|COPYING|NOTICE|UNLICENSE|COPYRIGHT)($|[._-])' } |
            Sort-Object Name)
        if ($licenseFiles.Count -eq 0) {
            $licenseFileGaps.Add($dependency.Name)
            $licensePaths[$dependency.Name] = @()
            continue
        }

        $safePackageName = ($dependency.Name -replace '[^A-Za-z0-9._-]', '_')
        $destination = Join-Path $licensesDir $safePackageName
        New-Item -ItemType Directory -Path $destination -Force | Out-Null
        $copiedPaths = foreach ($licenseFile in $licenseFiles) {
            $copiedFile = Join-Path $destination $licenseFile.Name
            Copy-Item -LiteralPath $licenseFile.FullName -Destination $copiedFile -Force
            (Get-Item -LiteralPath $copiedFile).LastWriteTimeUtc = [DateTime]::UtcNow
            "licenses/$safePackageName/$($licenseFile.Name)"
        }
        $licensePaths[$dependency.Name] = @($copiedPaths)
    }

    $dependencyLines = @(
        "Hatchspire Windows dependency inventory"
        "Generated: $stamp"
        "Target: x86_64-pc-windows-msvc"
        ""
        "These are Cargo license expressions, not a substitute for required license texts or legal review."
        ""
    )
    $dependencyLines += $dependencies.GetEnumerator() | Sort-Object Name | ForEach-Object {
        $texts = @($licensePaths[$_.Name])
        $textLabel = if ($texts.Count -gt 0) { $texts -join ", " } else { "NO BUNDLED LICENSE FILE" }
        "$($_.Name) | $($_.Value) | $textLabel"
    }
    Set-Content -LiteralPath (Join-Path $docsDir "THIRD_PARTY_COMPONENTS.txt") -Value $dependencyLines -Encoding utf8

    $payloadFiles = @(Get-ChildItem -LiteralPath $workDir -Recurse -File |
        Where-Object { $_.Name -ne "BUILD_INFO.json" } |
        Sort-Object FullName |
        ForEach-Object { Get-FileRecord $workDir $_ })
    $buildInfo = [ordered]@{
        schema_version = 1
        title = "Hatchspire"
        package_status = "internal_preview_not_publicly_approved"
        version = $version
        build_id = $displayBuildId
        git_commit = $commit
        working_tree_dirty = $isDirty
        built_utc = $stamp
        platform = "Windows 10/11 x64"
        dependency_license_file_gaps = @($licenseFileGaps)
        payload_files = $payloadFiles
    }
    $buildInfo | ConvertTo-Json -Depth 6 |
        Set-Content -LiteralPath $buildInfoPath -Encoding utf8

    Compress-Archive -Path (Join-Path $workDir "*") -DestinationPath $candidateArchive -CompressionLevel Optimal -ProgressAction SilentlyContinue
    Move-Item -LiteralPath $candidateArchive -Destination $archive -Force

    $archiveInfo = Get-Item -LiteralPath $archive
    $archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
    $allFiles = @(Get-ChildItem -LiteralPath $workDir -Recurse -File |
        Sort-Object FullName |
        ForEach-Object { Get-FileRecord $workDir $_ })
    $releaseManifest = [ordered]@{
        schema_version = 1
        package_status = "internal_preview_not_publicly_approved"
        archive = $archiveInfo.Name
        archive_bytes = $archiveInfo.Length
        archive_sha256 = $archiveHash
        version = $version
        build_id = $displayBuildId
        git_commit = $commit
        working_tree_dirty = $isDirty
        built_utc = $stamp
        dependency_license_file_gaps = @($licenseFileGaps)
        included_files = $allFiles
    }
    $manifestPath = Join-Path $distDir "hatchspire_windows_manifest.json"
    $releaseManifest | ConvertTo-Json -Depth 6 |
        Set-Content -LiteralPath $manifestPath -Encoding utf8
    Set-Content -LiteralPath (Join-Path $distDir "hatchspire_windows.sha256") -Value "$archiveHash  $($archiveInfo.Name)" -Encoding ascii

    $verificationZip = [IO.Compression.ZipFile]::OpenRead($archive)
    try {
        $actualNames = @($verificationZip.Entries |
            Where-Object { -not [string]::IsNullOrEmpty($_.Name) } |
            ForEach-Object { $_.FullName.Replace('\', '/') } |
            Sort-Object)
    } finally {
        $verificationZip.Dispose()
    }
    $expectedNames = @($allFiles.path | Sort-Object)
    $lineSeparator = [Environment]::NewLine
    if (($actualNames -join $lineSeparator) -ne ($expectedNames -join $lineSeparator)) {
        throw "Archive verification failed: packaged file list differs from the manifest."
    }

    Write-Host "Windows preview package verified:" -ForegroundColor Green
    Write-Host "  Archive: $archive"
    Write-Host "  Version: $version"
    Write-Host "  Commit: $commit"
    Write-Host "  Files: $($allFiles.Count)"
    Write-Host "  Bytes: $($archiveInfo.Length)"
    Write-Host "  SHA-256: $archiveHash"
    Write-Host "  Status: internal preview; human release and rights approval still required" -ForegroundColor Yellow
} finally {
    if (Test-Path -LiteralPath $candidateArchive) {
        Assert-ChildPath $distDir $candidateArchive
        Remove-Item -LiteralPath $candidateArchive -Force
    }
    if (Test-Path -LiteralPath $workDir) {
        Assert-ChildPath $distDir $workDir
        Remove-Item -LiteralPath $workDir -Recurse -Force
    }
}
