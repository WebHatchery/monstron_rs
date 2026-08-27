<#
.SYNOPSIS
    Creates an exact-candidate storefront-copy review packet.

.DESCRIPTION
    Verifies a clean sealed Windows package and substitutes only technical manifest facts into the
    tracked itch page and release-message drafts. Human decisions remain explicit placeholders; no
    page, message, upload, or approval is created.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$OutputRoot = "target\storefront-review-packet"
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
$outputRootDir = Resolve-ProjectPath $projectDir $OutputRoot
Assert-ChildPath $targetDir $outputRootDir
foreach ($required in @($archive, $manifestFile)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Required storefront evidence is missing: $required"
    }
}

$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json
$archiveInfo = Get-Item -LiteralPath $archive
$archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($manifest.schema_version -ne 1 -or
    [string]$manifest.archive -ne $archiveInfo.Name -or
    [long]$manifest.archive_bytes -ne $archiveInfo.Length -or
    [string]$manifest.archive_sha256 -ne $archiveHash -or
    [bool]$manifest.working_tree_dirty -or [bool]$manifest.toolkit_working_tree_dirty) {
    throw "External manifest does not identify a clean exact archive."
}
if ([string]$manifest.git_commit -notmatch '^[0-9a-f]{40}$' -or
    [string]$manifest.toolkit_git_commit -notmatch '^[0-9a-f]{40}$') {
    throw "Package source identities are missing or invalid."
}
$headCommit = (& git -C $projectDir rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0 -or $headCommit -notmatch '^[0-9a-f]{40}$') {
    throw "Could not resolve the storefront-copy source commit."
}
$dirtyLines = @(& git -C $projectDir status --porcelain)
if ($LASTEXITCODE -ne 0) {
    throw "Could not inspect the storefront-copy working tree."
}
if ($dirtyLines.Count -gt 0) {
    throw "Storefront review packets require committed copy in a clean working tree."
}
if ($headCommit -ne [string]$manifest.git_commit) {
    throw "Storefront copy HEAD does not match the sealed package commit."
}
$exeRecord = @($manifest.included_files | Where-Object { $_.path -eq "hatchspire.exe" })
if ($exeRecord.Count -ne 1) {
    throw "External manifest must identify exactly one hatchspire.exe."
}

$builtUtc = if ($manifest.built_utc -is [DateTime]) {
    $manifest.built_utc.ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
} else {
    [string]$manifest.built_utc
}
$zipMib = [Math]::Round($archiveInfo.Length / 1MB, 1).ToString("0.0", [Globalization.CultureInfo]::InvariantCulture)
$tokens = [ordered]@{
    "{{BUILD_ID}}" = [string]$manifest.build_id
    "{{GIT_COMMIT}}" = [string]$manifest.git_commit
    "{{TOOLKIT_BUILD_ID}}" = [string]$manifest.toolkit_build_id
    "{{TOOLKIT_GIT_COMMIT}}" = [string]$manifest.toolkit_git_commit
    "{{BUILT_UTC}}" = $builtUtc
    "{{ZIP_FILENAME}}" = $archiveInfo.Name
    "{{ZIP_BYTES}}" = [string]$archiveInfo.Length
    "{{ZIP_MIB}}" = $zipMib
    "{{ZIP_SHA256}}" = $archiveHash
    "{{EXE_SHA256}}" = [string]$exeRecord[0].sha256
}

$candidateDir = Join-Path $outputRootDir ([string]$manifest.git_commit)
Assert-ChildPath $targetDir $candidateDir
if (Test-Path -LiteralPath $candidateDir) {
    throw "Storefront packet already exists; refusing to overwrite possible human evidence: $candidateDir"
}
New-Item -ItemType Directory -Path $candidateDir -Force | Out-Null

$drafts = [ordered]@{
    "docs\itch_store_page_draft.md" = "ITCH_STORE_PAGE_REVIEW.md"
    "docs\demo_release_messages_draft.md" = "RELEASE_MESSAGES_REVIEW.md"
}
$outputRecords = [Collections.Generic.List[object]]::new()
foreach ($sourceRelative in $drafts.Keys) {
    $source = Join-Path $projectDir $sourceRelative
    if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
        throw "Tracked storefront draft is missing: $sourceRelative"
    }
    $rendered = Get-Content -LiteralPath $source -Raw
    foreach ($token in $tokens.Keys) {
        $rendered = $rendered.Replace($token, $tokens[$token])
    }
    if ($rendered -match '\{\{[A-Z0-9_]+\}\}') {
        throw "Unresolved technical token remains in ${sourceRelative}: $($Matches[0])"
    }
    if ($rendered -notmatch '\[\[HUMAN (APPROVAL|TEST EVIDENCE) REQUIRED') {
        throw "Rendered storefront draft unexpectedly contains no human-decision gates: $sourceRelative"
    }
    $destination = Join-Path $candidateDir $drafts[$sourceRelative]
    Set-Content -LiteralPath $destination -Value $rendered -Encoding utf8
    $destinationInfo = Get-Item -LiteralPath $destination
    $outputRecords.Add([ordered]@{
        path = $destinationInfo.Name
        bytes = $destinationInfo.Length
        sha256 = (Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash.ToLowerInvariant()
        unresolved_human_gate_count = [regex]::Matches(
            $rendered,
            '\[\[HUMAN (APPROVAL|TEST EVIDENCE) REQUIRED'
        ).Count
    })
}

$generatedUtc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
$candidate = [ordered]@{
    schema_version = 1
    status = "awaiting_human_storefront_approval"
    generated_utc = $generatedUtc
    build_id = [string]$manifest.build_id
    git_commit = [string]$manifest.git_commit
    copy_source_git_commit = $headCommit
    toolkit_build_id = [string]$manifest.toolkit_build_id
    toolkit_git_commit = [string]$manifest.toolkit_git_commit
    archive = $archiveInfo.Name
    archive_bytes = [long]$archiveInfo.Length
    archive_sha256 = $archiveHash
    executable_sha256 = [string]$exeRecord[0].sha256
    rendered_documents = @($outputRecords)
    human_storefront_review_recorded = $false
    upload_authorized = $false
    release_approval_granted = $false
}
$candidatePath = Join-Path $candidateDir "CANDIDATE.json"
$candidate | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath $candidatePath -Encoding utf8

$approval = @"
# Hatchspire storefront approval record

Status: **AWAITING HUMAN STOREFRONT APPROVAL — NO UPLOAD OR RELEASE AUTHORITY**  
Generated UTC: $generatedUtc

| Field | Exact candidate |
| --- | --- |
| Build ID | $($manifest.build_id) |
| Git commit | $($manifest.git_commit) |
| Toolkit build | $($manifest.toolkit_build_id) |
| ZIP | $($archiveInfo.Name) |
| ZIP bytes | $($archiveInfo.Length) |
| ZIP SHA-256 | $archiveHash |
| EXE SHA-256 | $($exeRecord[0].sha256) |

Resolve every human marker in both review documents before approval.

- [ ] Audience, purpose, scope, endpoint, playtime, and save promise match the approved definition.
- [ ] Price, payment mode, access mode, release status, and page slug are approved.
- [ ] Windows is the only selected platform and the downloadable ZIP is marked correctly.
- [ ] Minimum/recommended requirements come from physical-machine evidence.
- [ ] Support route and response expectation are sustainable and public-ready.
- [ ] Privacy, content warnings, generated-material metadata, rights, credits, and notices are approved.
- [ ] Three to five reviewed exact-candidate screenshots and the approved cover are selected.
- [ ] Known issues are truthful, accepted, and do not conceal a release blocker.
- [ ] FAQ, release notes, invitation, launch post, and patch-post wording are accurate.
- [ ] The processed restricted download's SHA-256 is recorded and matches the approved candidate.

Decision: APPROVE COPY / REJECT / INCOMPLETE  
Reviewer and authority: ____________________  
Decision time: ____________________  
Required changes and issue IDs: ____________________  
Upload authorized by this record: **NO — use explicit upload authorization**  
Public release authorized by this record: **NO — use the separate final GO record**
"@
$approvalPath = Join-Path $candidateDir "STOREFRONT_APPROVAL_RECORD.md"
Set-Content -LiteralPath $approvalPath -Value $approval -Encoding utf8

Write-Host "Candidate storefront-review packet created:" -ForegroundColor Green
Write-Host "  Build ID: $($manifest.build_id)"
Write-Host "  ZIP SHA-256: $archiveHash"
Write-Host "  Drafts: $($outputRecords.Count) candidate-bound documents"
Write-Host "  Record: $approvalPath"
Write-Host "  Status: awaiting human copy approval; no upload or release authority recorded." -ForegroundColor Yellow
