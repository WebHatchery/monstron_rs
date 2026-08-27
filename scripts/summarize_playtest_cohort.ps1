<#
.SYNOPSIS
    Summarizes anonymous human playtest session JSON for one exact candidate.

.DESCRIPTION
    Validates session identity and eligibility, deduplicates anonymous testers using their latest
    programme round, and evaluates the release plan's cohort thresholds. It does not infer human
    observations or grant release approval.
#>
param(
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$PacketRoot = "target\playtest-packet"
)

$ErrorActionPreference = "Stop"

function Resolve-ProjectPath {
    param([string]$ProjectDir, [string]$Path)
    if ([IO.Path]::IsPathRooted($Path)) { [IO.Path]::GetFullPath($Path) }
    else { [IO.Path]::GetFullPath((Join-Path $ProjectDir $Path)) }
}

$projectDir = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$manifestFile = Resolve-ProjectPath $projectDir $ManifestPath
$packetRootDir = Resolve-ProjectPath $projectDir $PacketRoot
if (-not (Test-Path -LiteralPath $manifestFile -PathType Leaf)) {
    throw "Package manifest is missing: $manifestFile"
}
$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json
$packetDir = Join-Path $packetRootDir ([string]$manifest.git_commit)
$candidatePath = Join-Path $packetDir "CANDIDATE.json"
$sessionsDir = Join-Path $packetDir "sessions"
if (-not (Test-Path -LiteralPath $candidatePath -PathType Leaf) -or
    -not (Test-Path -LiteralPath $sessionsDir -PathType Container)) {
    throw "Matching playtest packet is missing for $($manifest.git_commit)."
}
$candidate = Get-Content -LiteralPath $candidatePath -Raw | ConvertFrom-Json
if ([string]$candidate.status -ne "awaiting_independent_playtests" -or
    [string]$candidate.git_commit -ne [string]$manifest.git_commit -or
    [string]$candidate.archive_sha256 -ne [string]$manifest.archive_sha256 -or
    [bool]$candidate.release_approval_granted) {
    throw "Playtest packet does not identify the current unapproved candidate."
}

$allowedFamiliarity = @("unfamiliar", "familiar", "other")
$allowedWouldPlay = @("yes", "no", "unsure")
$expectedFields = @(
    "schema_version", "candidate_git_commit", "candidate_archive_sha256",
    "anonymous_tester_id", "programme_round", "independent_of_development",
    "consent_to_use_anonymized_results", "observer_attested", "genre_familiarity",
    "no_live_coaching", "reached_first_completed_expedition", "completed_approved_demo",
    "understood_town_raise_tower_loop", "crash_or_progress_loss", "would_play_more",
    "play_minutes", "issue_ids", "local_tester_summary_shared"
)
$sessionFiles = @(Get-ChildItem -LiteralPath $sessionsDir -File -Filter "*.json" | Sort-Object Name)
$eligible = [Collections.Generic.List[object]]::new()
$excluded = [Collections.Generic.List[object]]::new()
foreach ($file in $sessionFiles) {
    try {
        $session = Get-Content -LiteralPath $file.FullName -Raw | ConvertFrom-Json
        $id = [string]$session.anonymous_tester_id
        $actualFields = @($session.PSObject.Properties.Name)
        if (@($actualFields | Where-Object { $_ -notin $expectedFields }).Count -gt 0 -or
            @($expectedFields | Where-Object { $_ -notin $actualFields }).Count -gt 0) {
            throw "record fields differ from the anonymous session schema"
        }
        if ($session.schema_version -ne 1 -or $id -notmatch '^T[0-9]{2,4}$') {
            throw "invalid schema or anonymous tester ID"
        }
        if ([string]$session.candidate_git_commit -ne [string]$candidate.git_commit -or
            [string]$session.candidate_archive_sha256 -ne [string]$candidate.archive_sha256) {
            throw "candidate identity mismatch"
        }
        if ([int]$session.programme_round -lt 1 -or
            [string]$session.genre_familiarity -notin $allowedFamiliarity -or
            [string]$session.would_play_more -notin $allowedWouldPlay) {
            throw "invalid round or enumerated response"
        }
        foreach ($field in @(
            "independent_of_development", "consent_to_use_anonymized_results", "observer_attested",
            "no_live_coaching", "reached_first_completed_expedition", "completed_approved_demo",
            "understood_town_raise_tower_loop", "crash_or_progress_loss"
        )) {
            if ($session.$field -isnot [bool]) { throw "$field must be true or false" }
        }
        if ($null -eq $session.play_minutes -or [int]$session.play_minutes -lt 0 -or
            [int]$session.play_minutes -gt 1440) {
            throw "play_minutes must be between 0 and 1440"
        }
        if ($session.local_tester_summary_shared -isnot [bool] -or
            @($session.issue_ids | Where-Object { [string]$_ -notmatch '^[A-Za-z0-9#_-]{1,32}$' }).Count -gt 0) {
            throw "local-summary choice or issue ID list is invalid"
        }
        if (-not [bool]$session.independent_of_development -or
            -not [bool]$session.consent_to_use_anonymized_results -or
            -not [bool]$session.observer_attested) {
            $excluded.Add([ordered]@{ file = $file.Name; tester_id = $id; reason = "eligibility_or_consent" })
            continue
        }
        $eligible.Add([pscustomobject]@{
            File = $file.Name
            TesterId = $id
            Round = [int]$session.programme_round
            Familiarity = [string]$session.genre_familiarity
            NoCoaching = [bool]$session.no_live_coaching
            FirstExpedition = [bool]$session.reached_first_completed_expedition
            DemoCompleted = [bool]$session.completed_approved_demo
            LoopUnderstood = [bool]$session.understood_town_raise_tower_loop
            CrashOrLoss = [bool]$session.crash_or_progress_loss
            WouldPlayMore = [string]$session.would_play_more
            PlayMinutes = [int]$session.play_minutes
        })
    } catch {
        $excluded.Add([ordered]@{ file = $file.Name; tester_id = ""; reason = $_.Exception.Message })
    }
}

$latestByTester = [ordered]@{}
foreach ($session in ($eligible | Sort-Object TesterId, Round, File)) {
    $latestByTester[$session.TesterId] = $session
}
$testers = @($latestByTester.Values)
$count = $testers.Count
$unfamiliar = @($testers | Where-Object Familiarity -eq "unfamiliar").Count
$familiar = @($testers | Where-Object Familiarity -eq "familiar").Count
$firstUncoached = @($testers | Where-Object { $_.NoCoaching -and $_.FirstExpedition }).Count
$completeUncoached = @($testers | Where-Object { $_.NoCoaching -and $_.DemoCompleted }).Count
$understood = @($testers | Where-Object LoopUnderstood).Count
$crashOrLoss = @($testers | Where-Object CrashOrLoss).Count
$wouldYes = @($testers | Where-Object WouldPlayMore -eq "yes").Count
$wouldNo = @($testers | Where-Object WouldPlayMore -eq "no").Count
$wouldUnsure = @($testers | Where-Object WouldPlayMore -eq "unsure").Count
$minutes = @($testers | ForEach-Object { $_.PlayMinutes })

$enoughEvidence = $count -ge [int]$candidate.required_eligible_testers
$targetsMet = $enoughEvidence -and $unfamiliar -ge 2 -and $familiar -ge 2 -and
    $firstUncoached -ge [int]$candidate.required_first_expedition_without_coaching -and
    $completeUncoached -ge [int]$candidate.required_demo_completion_without_coaching -and
    $understood -ge [int]$candidate.required_loop_understanding -and
    $crashOrLoss -le [int]$candidate.allowed_crash_or_progress_loss
$status = if (-not $enoughEvidence) { "insufficient_evidence" }
elseif ($targetsMet) { "targets_met" }
else { "targets_not_met" }

$summary = [ordered]@{
    schema_version = 1
    status = $status
    generated_utc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
    build_id = [string]$candidate.build_id
    git_commit = [string]$candidate.git_commit
    archive_sha256 = [string]$candidate.archive_sha256
    session_files = $sessionFiles.Count
    eligible_unique_testers = $count
    excluded_records = @($excluded)
    unfamiliar_testers = $unfamiliar
    familiar_testers = $familiar
    first_expedition_without_coaching = $firstUncoached
    demo_completion_without_coaching = $completeUncoached
    understood_loop = $understood
    crash_or_progress_loss = $crashOrLoss
    would_play_more = [ordered]@{ yes = $wouldYes; no = $wouldNo; unsure = $wouldUnsure }
    play_minutes = [ordered]@{
        minimum = if ($minutes.Count) { ($minutes | Measure-Object -Minimum).Minimum } else { $null }
        maximum = if ($minutes.Count) { ($minutes | Measure-Object -Maximum).Maximum } else { $null }
        average = if ($minutes.Count) { [Math]::Round(($minutes | Measure-Object -Average).Average, 1) } else { $null }
    }
    human_evidence_mechanically_summarized = $count -gt 0
    release_approval_granted = $false
}
$summaryPath = Join-Path $packetDir "COHORT_SUMMARY.json"
$summary | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $summaryPath -Encoding utf8

$report = @"
# Hatchspire playtest cohort summary

Status: **$($status.ToUpperInvariant()) — NOT RELEASE APPROVAL**  
Candidate: $($candidate.build_id)  
ZIP SHA-256: $($candidate.archive_sha256)

| Measure | Target | Result |
| --- | ---: | ---: |
| Eligible unique independent testers | 5 minimum | $count |
| Unfamiliar with monster-raising games | 2 minimum | $unfamiliar |
| Familiar with monster-raising games | 2 minimum | $familiar |
| First expedition without coaching | 4 minimum | $firstUncoached |
| Demo completion without coaching | 3 minimum | $completeUncoached |
| Understand town → raise → tower loop | 4 minimum | $understood |
| Crash or progress loss | 0 | $crashOrLoss |
| Would play more — yes / no / unsure | No forced target | $wouldYes / $wouldNo / $wouldUnsure |

Session files: $($sessionFiles.Count)  
Excluded records: $($excluded.Count)  
Playtime minutes, minimum / average / maximum: $($summary.play_minutes.minimum) / $($summary.play_minutes.average) / $($summary.play_minutes.maximum)

This report mechanically summarizes human-entered anonymous records. The release owner must review
source consent, observations, issue severity, qualitative notes, skill bands, and multiple-round
evidence. `targets_met` does not authorize upload or public release.
"@
Set-Content -LiteralPath (Join-Path $packetDir "COHORT_SUMMARY.md") -Value $report -Encoding utf8

Write-Host "Independent-playtest cohort summary created:" -ForegroundColor Green
Write-Host "  Status: $status"
Write-Host "  Eligible unique testers: $count"
Write-Host "  First expedition / demo completion / loop understanding: $firstUncoached / $completeUncoached / $understood"
Write-Host "  Crash or progress loss: $crashOrLoss"
Write-Host "  Evidence: $summaryPath"
Write-Host "  Mechanical summary only; no upload or release approval granted." -ForegroundColor Yellow
