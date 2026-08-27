<#
.SYNOPSIS
    Creates a candidate-stamped independent-playtest evidence packet.

.DESCRIPTION
    Verifies the clean sealed package, matching committed source, and candidate storefront packet,
    then writes anonymous session templates and observer guidance. It records no human result and
    grants no distribution, upload, or release authority.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$StorefrontRoot = "target\storefront-review-packet",
    [string]$OutputRoot = "target\playtest-packet"
)

$ErrorActionPreference = "Stop"

function Resolve-ProjectPath {
    param([string]$ProjectDir, [string]$Path)
    if ([IO.Path]::IsPathRooted($Path)) { [IO.Path]::GetFullPath($Path) }
    else { [IO.Path]::GetFullPath((Join-Path $ProjectDir $Path)) }
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
$targetDir = Join-Path $projectDir "target"
$archive = Resolve-ProjectPath $projectDir $ArchivePath
$manifestFile = Resolve-ProjectPath $projectDir $ManifestPath
$storefrontRootDir = Resolve-ProjectPath $projectDir $StorefrontRoot
$outputRootDir = Resolve-ProjectPath $projectDir $OutputRoot
Assert-ChildPath $targetDir $storefrontRootDir
Assert-ChildPath $targetDir $outputRootDir
foreach ($required in @($archive, $manifestFile)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Required playtest evidence is missing: $required"
    }
}

$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json
$archiveInfo = Get-Item -LiteralPath $archive
$archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($manifest.schema_version -ne 1 -or [string]$manifest.archive -ne $archiveInfo.Name -or
    [long]$manifest.archive_bytes -ne $archiveInfo.Length -or
    [string]$manifest.archive_sha256 -ne $archiveHash -or
    [bool]$manifest.working_tree_dirty -or [bool]$manifest.toolkit_working_tree_dirty) {
    throw "External manifest does not identify a clean exact archive."
}

$headCommit = (& git -C $projectDir rev-parse HEAD).Trim()
$dirtyLines = @(& git -C $projectDir status --porcelain)
if ($LASTEXITCODE -ne 0 -or $headCommit -notmatch '^[0-9a-f]{40}$' -or $dirtyLines.Count -gt 0) {
    throw "Playtest packets require a committed clean working tree."
}
if ($headCommit -ne [string]$manifest.git_commit) {
    throw "Playtest packet source HEAD does not match the sealed package commit."
}

$storefrontDir = Join-Path $storefrontRootDir ([string]$manifest.git_commit)
$storefrontCandidatePath = Join-Path $storefrontDir "CANDIDATE.json"
$storefrontCopyPath = Join-Path $storefrontDir "ITCH_STORE_PAGE_REVIEW.md"
foreach ($required in @($storefrontCandidatePath, $storefrontCopyPath)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Matching candidate storefront evidence is missing: $required"
    }
}
$storefront = Get-Content -LiteralPath $storefrontCandidatePath -Raw | ConvertFrom-Json
if ([string]$storefront.status -ne "awaiting_human_storefront_approval" -or
    [string]$storefront.git_commit -ne [string]$manifest.git_commit -or
    [string]$storefront.archive_sha256 -ne $archiveHash -or
    [bool]$storefront.human_storefront_review_recorded -or
    [bool]$storefront.upload_authorized -or [bool]$storefront.release_approval_granted) {
    throw "Storefront packet does not identify the same unapproved candidate."
}
$storefrontCopyHash = (Get-FileHash -LiteralPath $storefrontCopyPath -Algorithm SHA256).Hash.ToLowerInvariant()
$storefrontCopyRecord = @($storefront.rendered_documents |
    Where-Object { $_.path -eq "ITCH_STORE_PAGE_REVIEW.md" })
if ($storefrontCopyRecord.Count -ne 1 -or
    [string]$storefrontCopyRecord[0].sha256 -ne $storefrontCopyHash) {
    throw "Storefront instructions no longer match their candidate manifest."
}

$candidateDir = Join-Path $outputRootDir ([string]$manifest.git_commit)
Assert-ChildPath $targetDir $candidateDir
if (Test-Path -LiteralPath $candidateDir) {
    throw "Playtest packet already exists; refusing to overwrite possible human evidence: $candidateDir"
}
$sessionsDir = Join-Path $candidateDir "sessions"
New-Item -ItemType Directory -Path $sessionsDir -Force | Out-Null

$generatedUtc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
$candidate = [ordered]@{
    schema_version = 1
    status = "awaiting_independent_playtests"
    generated_utc = $generatedUtc
    build_id = [string]$manifest.build_id
    git_commit = [string]$manifest.git_commit
    toolkit_build_id = [string]$manifest.toolkit_build_id
    toolkit_git_commit = [string]$manifest.toolkit_git_commit
    archive = $archiveInfo.Name
    archive_bytes = [long]$archiveInfo.Length
    archive_sha256 = $archiveHash
    storefront_copy_sha256 = $storefrontCopyHash
    required_eligible_testers = 5
    required_first_expedition_without_coaching = 4
    required_demo_completion_without_coaching = 3
    required_loop_understanding = 4
    allowed_crash_or_progress_loss = 0
    human_results_recorded = $false
    distribution_instructions_approved = $false
    release_approval_granted = $false
}
$candidate | ConvertTo-Json -Depth 4 |
    Set-Content -LiteralPath (Join-Path $candidateDir "CANDIDATE.json") -Encoding utf8

$sessionTemplate = [ordered]@{
    schema_version = 1
    candidate_git_commit = [string]$manifest.git_commit
    candidate_archive_sha256 = $archiveHash
    anonymous_tester_id = ""
    programme_round = 0
    independent_of_development = $null
    consent_to_use_anonymized_results = $null
    observer_attested = $null
    genre_familiarity = "unrecorded"
    no_live_coaching = $null
    reached_first_completed_expedition = $null
    completed_approved_demo = $null
    understood_town_raise_tower_loop = $null
    crash_or_progress_loss = $null
    would_play_more = "unrecorded"
    play_minutes = $null
    issue_ids = @()
    local_tester_summary_shared = $false
}
$sessionTemplate | ConvertTo-Json -Depth 4 |
    Set-Content -LiteralPath (Join-Path $candidateDir "SESSION_TEMPLATE.json") -Encoding utf8

$observerGuide = @"
# Hatchspire independent-playtest observer guide

Candidate: $($manifest.build_id)  
ZIP SHA-256: $archiveHash

## Before play

1. Confirm the tester did not build Hatchspire and record only an anonymous ID in packet JSON.
2. Obtain and retain consent outside Git before observing, recording, collecting quotes, hardware
   details, saves, logs, or a local tester summary. Consent to one item does not imply another.
3. Give the tester the approved player-facing download/page instructions, not the repository README.
4. Verify the downloaded ZIP hash. Start from a fresh save for complete pacing counters.
5. Explain that the game uploads nothing and that sharing any local file is optional.

## During play

- Observe silently where consent permits. Do not identify controls, explain goals, suggest tactics,
  correct misunderstandings, or rescue progress while measuring uncoached outcomes.
- Record the moment and visible context of hesitation, misreading, boredom, failure, crash, progress
  loss, or a request for help. Put free-form notes outside the JSON record.
- If safety, distress, or consent requires intervention, intervene and mark no-live-coaching false.
- Do not count automated capture, a developer session, or a coached walkthrough as an independent test.

## Neutral follow-up prompts

- What did you think your next goal was?
- What did you believe would happen when you selected that control?
- How would you describe the camp → raising → tower loop in your own words?
- Where, if anywhere, did you feel stuck or uncertain?
- What felt slow, repetitive, unfair, or especially satisfying?
- Would you choose to play more? Why or why not?

Do not explain the intended answer before recording the tester's response. A human observer must set
the structured booleans and attest the record; the game and summarizer cannot infer comprehension.
"@
Set-Content -LiteralPath (Join-Path $candidateDir "OBSERVER_GUIDE.md") -Value $observerGuide -Encoding utf8

$sessionRecord = @"
# Hatchspire independent session record

Candidate: $($manifest.build_id)  
ZIP SHA-256: $archiveHash

Keep this completed narrative outside Git. Put only anonymous structured outcomes in a copied JSON
template under `sessions/`.

| Field | Human entry |
| --- | --- |
| Anonymous tester ID |  |
| Programme round and date |  |
| Independent of development | Yes / No |
| Genre familiarity | Unfamiliar / Familiar / Other |
| Consent scope and record location |  |
| Downloaded ZIP hash matched | Yes / No |
| Fresh save used | Yes / No |
| Live coaching provided | Yes / No; when and why |
| First completed expedition reached | Yes / No |
| Approved demo completed | Yes / No / Not implemented |
| Town → raise → tower loop understood | Yes / No; evidence in tester's words |
| Crash or progress loss | Yes / No; issue ID |
| Would voluntarily play more | Yes / No / Unsure; why |
| Play duration |  |
| Local tester summary offered/shared voluntarily |  |
| Issue IDs |  |
| Observer attestation and time |  |

Observed hesitation/failure timeline: ____________________

Neutral follow-up answers: ____________________
"@
Set-Content -LiteralPath (Join-Path $candidateDir "SESSION_RECORD_TEMPLATE.md") -Value $sessionRecord -Encoding utf8

$cohortRecord = @"
# Hatchspire independent-playtest cohort record

Candidate: $($manifest.build_id)  
ZIP SHA-256: $archiveHash  
Status: **AWAITING HUMAN EVIDENCE — NOT RELEASE APPROVAL**

| Anonymous tester | Round | Familiarity | No coaching | First expedition | Demo complete | Loop understood | Crash/loss | Would play more | Issue IDs |
| --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T01 |  |  |  |  |  |  |  |  |  |  |
| T02 |  |  |  |  |  |  |  |  |  |  |
| T03 |  |  |  |  |  |  |  |  |  |  |
| T04 |  |  |  |  |  |  |  |  |  |  |
| T05 |  |  |  |  |  |  |  |  |  |  |

Run `scripts/summarize_playtest_cohort.ps1` after adding consented anonymous session JSON files.
Mechanical targets do not replace issue review, tester consent, qualitative judgment, or final GO.
"@
Set-Content -LiteralPath (Join-Path $candidateDir "COHORT_RECORD.md") -Value $cohortRecord -Encoding utf8

Set-Content -LiteralPath (Join-Path $sessionsDir "README.md") -Encoding utf8 -Value @"
# Anonymous session JSON

Copy `..\SESSION_TEMPLATE.json` once per session, use a `.json` filename such as `T01.json`, and fill
every field deliberately. Do not store personal identity or free-form notes here. The summarizer
ignores this README and reads only JSON files.
"@

Write-Host "Candidate independent-playtest packet created:" -ForegroundColor Green
Write-Host "  Build ID: $($manifest.build_id)"
Write-Host "  ZIP SHA-256: $archiveHash"
Write-Host "  Packet: $candidateDir"
Write-Host "  Status: awaiting independent human evidence; no distribution or release approval recorded." -ForegroundColor Yellow
