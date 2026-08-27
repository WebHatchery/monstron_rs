<#
.SYNOPSIS
    Scans the exact sealed Windows preview with this host's Microsoft Defender engine.

.DESCRIPTION
    Verifies the archive, checksum, clean manifest, and extracted executable before running
    non-remediating custom scans. Evidence is local and diagnostic: it does not establish
    SmartScreen reputation or results on any other machine or antivirus product.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$ChecksumPath = "dist\hatchspire_windows.sha256",
    [double]$MaxSignatureAgeDays = 3
)

$ErrorActionPreference = "Stop"

if ($MaxSignatureAgeDays -le 0 -or $MaxSignatureAgeDays -gt 30) {
    throw "MaxSignatureAgeDays must be greater than zero and no more than 30."
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

function Invoke-DefenderFileScan {
    param([string]$Scanner, [string]$Path)

    $startedUtc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
    $output = @(& $Scanner -Scan -ScanType 3 -File $Path -DisableRemediation 2>&1 |
        ForEach-Object { $_.ToString().TrimEnd("`r", "`n") })
    $exitCode = $LASTEXITCODE
    $finishedUtc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
    if ($exitCode -ne 0) {
        throw "Microsoft Defender scan failed or found a threat in $Path (exit $exitCode): $($output -join ' ')"
    }
    [pscustomobject]@{
        path = $Path
        started_utc = $startedUtc
        finished_utc = $finishedUtc
        exit_code = $exitCode
        output = $output
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

$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json
$archiveInfo = Get-Item -LiteralPath $archive
$archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($manifest.schema_version -ne 1 -or
    [string]$manifest.archive -ne $archiveInfo.Name -or
    [long]$manifest.archive_bytes -ne $archiveInfo.Length -or
    [string]$manifest.archive_sha256 -ne $archiveHash) {
    throw "External manifest does not identify the exact archive."
}
$checksumText = (Get-Content -LiteralPath $checksumFile -Raw).Trim()
if ($checksumText -notmatch '^([0-9A-Fa-f]{64})\s+(.+)$' -or
    $Matches[1].ToLowerInvariant() -ne $archiveHash -or $Matches[2] -ne $archiveInfo.Name) {
    throw "Checksum sidecar does not identify the exact archive."
}
if ([bool]$manifest.working_tree_dirty -or [bool]$manifest.toolkit_working_tree_dirty) {
    throw "Microsoft Defender release evidence requires a clean Hatchspire and toolkit package."
}
if ([string]$manifest.git_commit -notmatch '^[0-9a-f]{40}$' -or
    [string]$manifest.toolkit_git_commit -notmatch '^[0-9a-f]{40}$') {
    throw "Package source identities are missing or invalid."
}

$defenderCommand = Get-Command Get-MpComputerStatus -ErrorAction SilentlyContinue
$scanner = Join-Path $env:ProgramFiles "Windows Defender\MpCmdRun.exe"
if ($null -eq $defenderCommand -or -not (Test-Path -LiteralPath $scanner -PathType Leaf)) {
    throw "Microsoft Defender command-line scanning is unavailable on this host."
}
$defender = Get-MpComputerStatus
if (-not $defender.AMServiceEnabled -or -not $defender.AntivirusEnabled -or
    -not $defender.RealTimeProtectionEnabled) {
    throw "Microsoft Defender antivirus and real-time protection must be active."
}
$signatureUpdatedUtc = $defender.AntivirusSignatureLastUpdated.ToUniversalTime()
$signatureAgeDays = ([DateTime]::UtcNow - $signatureUpdatedUtc).TotalDays
if ($signatureAgeDays -lt -0.05 -or $signatureAgeDays -gt $MaxSignatureAgeDays) {
    throw "Microsoft Defender signatures are outside the allowed age window."
}

$targetRoot = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
$evidenceDir = Join-Path $targetRoot "defender-scan"
$runDir = Join-Path $targetRoot ("defender-scan-run-" + [Guid]::NewGuid().ToString("N"))
$executable = Join-Path $runDir "hatchspire.exe"
$summaryPath = Join-Path $evidenceDir "summary.json"
Assert-ChildPath $targetRoot $evidenceDir
Assert-ChildPath $targetRoot $runDir

try {
    New-Item -ItemType Directory -Path $runDir -Force | Out-Null
    New-Item -ItemType Directory -Path $evidenceDir -Force | Out-Null

    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead($archive)
    try {
        $entries = @($zip.Entries | Where-Object { $_.FullName -eq "hatchspire.exe" })
        if ($entries.Count -ne 1) { throw "Archive does not contain exactly one hatchspire.exe." }
        [IO.Compression.ZipFileExtensions]::ExtractToFile($entries[0], $executable, $true)
    } finally {
        $zip.Dispose()
    }

    $exeRecord = @($manifest.included_files | Where-Object { $_.path -eq "hatchspire.exe" })
    $exeHash = (Get-FileHash -LiteralPath $executable -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($exeRecord.Count -ne 1 -or [long]$exeRecord[0].bytes -ne (Get-Item $executable).Length -or
        [string]$exeRecord[0].sha256 -ne $exeHash) {
        throw "Extracted executable does not match the external manifest."
    }

    $archiveScan = Invoke-DefenderFileScan $scanner $archive
    $executableScan = Invoke-DefenderFileScan $scanner $executable
    [ordered]@{
        schema_version = 1
        status = "host_diagnostic_only"
        product = "Microsoft Defender Antivirus"
        antimalware_engine_version = [string]$defender.AMEngineVersion
        antivirus_signature_version = [string]$defender.AntivirusSignatureVersion
        antivirus_signature_updated_utc = $signatureUpdatedUtc.ToString("yyyy-MM-ddTHH:mm:ssZ")
        signature_age_days = [Math]::Round($signatureAgeDays, 3)
        real_time_protection_enabled = [bool]$defender.RealTimeProtectionEnabled
        scanner_file_version = [string](Get-Item $scanner).VersionInfo.FileVersion
        build_id = [string]$manifest.build_id
        git_commit = [string]$manifest.git_commit
        toolkit_build_id = [string]$manifest.toolkit_build_id
        toolkit_git_commit = [string]$manifest.toolkit_git_commit
        archive_bytes = [long]$archiveInfo.Length
        archive_sha256 = $archiveHash
        executable_sha256 = $exeHash
        archive_scan = $archiveScan
        executable_scan = $executableScan
    } | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $summaryPath -Encoding utf8

    Write-Host "Microsoft Defender exact-package scan passed:" -ForegroundColor Green
    Write-Host "  Build ID: $($manifest.build_id)"
    Write-Host "  Toolkit: $($manifest.toolkit_build_id)"
    Write-Host "  Archive SHA-256: $archiveHash"
    Write-Host "  Executable SHA-256: $exeHash"
    Write-Host "  Signatures: $($defender.AntivirusSignatureVersion), updated $($signatureUpdatedUtc.ToString('u'))"
    Write-Host "  Evidence: $summaryPath"
    Write-Host "  Host diagnostic only; SmartScreen and other-machine antivirus gates remain open." -ForegroundColor Yellow
} finally {
    if (Test-Path -LiteralPath $runDir) {
        Assert-ChildPath $targetRoot $runDir
        Remove-Item -LiteralPath $runDir -Recurse -Force
    }
}
