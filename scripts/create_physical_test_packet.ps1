<#
.SYNOPSIS
    Creates a blank physical-Windows test record for the exact sealed preview.

.DESCRIPTION
    Verifies the ZIP, manifest, checksum sidecar, clean source identities, and packaged executable,
    then writes candidate-stamped JSON and Markdown under target. It records no test pass and grants
    no release approval; all observations remain for a human tester and release owner.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$ChecksumPath = "dist\hatchspire_windows.sha256",
    [string]$OutputRoot = "target\physical-windows-test-packet"
)

$ErrorActionPreference = "Stop"

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
$targetDir = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
$archive = Resolve-ProjectPath $projectDir $ArchivePath
$manifestFile = Resolve-ProjectPath $projectDir $ManifestPath
$checksumFile = Resolve-ProjectPath $projectDir $ChecksumPath
$outputRootDir = Resolve-ProjectPath $projectDir $OutputRoot
Assert-ChildPath $targetDir $outputRootDir

foreach ($required in @($archive, $manifestFile, $checksumFile)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Required package evidence is missing: $required"
    }
}

$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json
$archiveInfo = Get-Item -LiteralPath $archive
$archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
$builtUtc = if ($manifest.built_utc -is [DateTime]) {
    $manifest.built_utc.ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
} else {
    [string]$manifest.built_utc
}
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
    throw "Physical-test packets require clean Hatchspire and toolkit package identities."
}
if ([string]$manifest.git_commit -notmatch '^[0-9a-f]{40}$' -or
    [string]$manifest.toolkit_git_commit -notmatch '^[0-9a-f]{40}$' -or
    [string]::IsNullOrWhiteSpace([string]$manifest.build_id)) {
    throw "Package source identities are missing or invalid."
}

$candidateDir = Join-Path $outputRootDir ([string]$manifest.git_commit)
$runDir = Join-Path $targetDir ("physical-windows-test-run-" + [Guid]::NewGuid().ToString("N"))
$executable = Join-Path $runDir "hatchspire.exe"
Assert-ChildPath $targetDir $candidateDir
Assert-ChildPath $targetDir $runDir

try {
    New-Item -ItemType Directory -Path $runDir -Force | Out-Null
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead($archive)
    try {
        $entries = @($zip.Entries | Where-Object { $_.FullName -eq "hatchspire.exe" })
        if ($entries.Count -ne 1) {
            throw "Archive does not contain exactly one hatchspire.exe."
        }
        [IO.Compression.ZipFileExtensions]::ExtractToFile($entries[0], $executable, $true)
    } finally {
        $zip.Dispose()
    }

    $exeRecord = @($manifest.included_files | Where-Object { $_.path -eq "hatchspire.exe" })
    $exeInfo = Get-Item -LiteralPath $executable
    $exeHash = (Get-FileHash -LiteralPath $executable -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($exeRecord.Count -ne 1 -or [long]$exeRecord[0].bytes -ne $exeInfo.Length -or
        [string]$exeRecord[0].sha256 -ne $exeHash) {
        throw "Packaged executable does not match the external manifest."
    }

    $signature = Get-AuthenticodeSignature -LiteralPath $executable
    $signerSubject = if ($null -eq $signature.SignerCertificate) {
        $null
    } else {
        [string]$signature.SignerCertificate.Subject
    }
    $signerThumbprint = if ($null -eq $signature.SignerCertificate) {
        $null
    } else {
        [string]$signature.SignerCertificate.Thumbprint
    }

    New-Item -ItemType Directory -Path $candidateDir -Force | Out-Null
    $generatedUtc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
    $candidate = [ordered]@{
        schema_version = 1
        status = "awaiting_human_evidence"
        generated_utc = $generatedUtc
        package_status = [string]$manifest.package_status
        archive = $archiveInfo.Name
        archive_bytes = [long]$archiveInfo.Length
        archive_sha256 = $archiveHash
        executable_sha256 = $exeHash
        version = [string]$manifest.version
        build_id = [string]$manifest.build_id
        built_utc = $builtUtc
        git_commit = [string]$manifest.git_commit
        toolkit_build_id = [string]$manifest.toolkit_build_id
        toolkit_git_commit = [string]$manifest.toolkit_git_commit
        authenticode = [ordered]@{
            status = [string]$signature.Status
            status_message = [string]$signature.StatusMessage
            signer_subject = $signerSubject
            signer_thumbprint = $signerThumbprint
        }
        human_results_recorded = $false
        release_approval_granted = $false
    }
    $candidatePath = Join-Path $candidateDir "CANDIDATE.json"
    $candidate | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath $candidatePath -Encoding utf8

    $record = @"
# Hatchspire physical Windows test record

Status: **AWAITING HUMAN EVIDENCE — NOT RELEASE APPROVAL**  
Generated UTC: $generatedUtc

## Exact candidate

| Field | Expected value |
| --- | --- |
| Build ID | $($manifest.build_id) |
| Hatchspire commit | $($manifest.git_commit) |
| Toolkit build | $($manifest.toolkit_build_id) |
| Toolkit commit | $($manifest.toolkit_git_commit) |
| Package build time | $builtUtc |
| ZIP filename | $($archiveInfo.Name) |
| ZIP bytes | $($archiveInfo.Length) |
| ZIP SHA-256 | $archiveHash |
| EXE SHA-256 | $exeHash |
| Authenticode status | $($signature.Status) |

This packet only verifies local package identity. It does not prove the restricted-channel download,
SmartScreen or antivirus behavior elsewhere, a physical-machine pass, or approval of an unsigned build.

## Distribution receipt

- Source page/channel: ____________________
- Access mode (Restricted / unlisted Public): ____________________
- Downloaded UTC: ____________________
- Downloaded filename: ____________________
- Observed ZIP SHA-256: ____________________
- [ ] Observed hash exactly equals the expected ZIP SHA-256 above.
- [ ] Build was downloaded through the player-facing channel, not copied from a developer machine.

## Tester and machine

Keep this completed record outside Git unless every person named has explicitly consented.

| Field | Human entry |
| --- | --- |
| Test session ID |  |
| Tester (name or consented alias) |  |
| Tester consent recorded | Yes / No |
| Test date and local time |  |
| Machine owner differs from other required machine | Yes / No |
| Device manufacturer/model |  |
| Windows edition/version/build |  |
| CPU |  |
| GPU and driver |  |
| RAM |  |
| Display resolution(s) |  |
| Display scale(s) |  |
| Multi-monitor layout |  |
| Audio device(s) |  |
| Input device(s) |  |
| Modest integrated-GPU machine | Yes / No |
| Rust/developer tools/repository absent | Yes / No |
| Install/extract path (note spaces/non-ASCII) |  |

## Launch, operating-system, and security checks

Record Pass, Fail, or Not tested plus evidence/issue ID for every row.

| Check | Result | Evidence / issue ID |
| --- | --- | --- |
| ZIP extracts with ordinary Windows tools |  |  |
| First launch from a path containing spaces |  |  |
| Launch from a path containing non-ASCII characters |  |  |
| SmartScreen behavior and exact wording recorded |  |  |
| Normal antivirus ZIP scan |  |  |
| Normal antivirus executable scan |  |  |
| Five consecutive launch and clean-quit cycles |  |  |
| Window resize at supported sizes |  |  |
| Fullscreen enter and return to windowed mode |  |  |
| Alt+Tab away and back during play |  |  |
| Minimize and restore |  |  |
| Sleep/wake and resume play |  |  |
| Required display scales, including laptop scaling |  |  |
| Multi-monitor move/use when available |  |  |
| Mouse completes every required action |  |  |
| Keyboard supplements but is not required |  |  |
| Clean quit leaves no unexpected files beside the EXE |  |  |

## Save, recovery, audio, and full-demo checks

| Check | Result | Evidence / issue ID |
| --- | --- | --- |
| Fresh New Game starts cleanly |  |  |
| Save is created in the documented location |  |  |
| Restart and Continue restore progress |  |  |
| Autosave survives a completed action and restart |  |  |
| New Game overwrite warning is clear |  |  |
| Backup restore works and preserves the displaced file |  |  |
| Corrupt-save recovery/help is understandable |  |  |
| Master, music, and SFX controls behave as labeled |  |  |
| Mute leaves the game understandable |  |  |
| Speakers and headphones checked |  |  |
| Entire approved demo completed from a fresh save |  |  |
| No crash, hang, progress loss, required clipping, or missing asset |  |  |

## Visible two-to-four-hour play/device soak

- Observer: ____________________
- Start time: ____________________
- End time: ____________________
- Duration: ____________________
- Build remained visible and interactive: Yes / No
- Town/facilities/tower/combat/save/load/quit-restart exercised: ____________________
- Frame-pacing or responsiveness degradation observed: ____________________
- Memory/resource concern observed: ____________________
- OS/device events exercised during the interval: ____________________
- Result and evidence/issue IDs: ____________________

## Session outcome

- Overall result: PASS / FAIL / INCOMPLETE
- Blocker issue IDs: ____________________
- Important issue IDs: ____________________
- Tester notes: ____________________
- Tester/observer attestation and date: ____________________

A PASS here is one machine/session result only. It does not satisfy the two-machine gate by itself and
does not authorize publishing.

## Release-owner review

- Required machine records attached: ____________________
- Exact restricted-channel hashes match: Yes / No
- Unsigned-build decision: ACCEPT / REJECT / UNDECIDED
- Accepted important issues and rationale: ____________________
- Physical Windows gate: PASS / FAIL / INCOMPLETE
- Release owner and decision time: ____________________
- Public release authorized by this record: **NO — use the separate final GO record**
"@
    $recordPath = Join-Path $candidateDir "PHYSICAL_WINDOWS_TEST_RECORD.md"
    Set-Content -LiteralPath $recordPath -Value $record -Encoding utf8

    Write-Host "Physical Windows test packet created:" -ForegroundColor Green
    Write-Host "  Build ID: $($manifest.build_id)"
    Write-Host "  ZIP SHA-256: $archiveHash"
    Write-Host "  Authenticode: $($signature.Status)"
    Write-Host "  Record: $recordPath"
    Write-Host "  Status: awaiting human evidence; no physical test or release approval recorded." -ForegroundColor Yellow
} finally {
    if (Test-Path -LiteralPath $runDir) {
        Assert-ChildPath $targetDir $runDir
        Remove-Item -LiteralPath $runDir -Recurse -Force
    }
}
