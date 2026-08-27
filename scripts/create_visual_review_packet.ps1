<#
.SYNOPSIS
    Creates a candidate-stamped human review gallery from exact-package smoke captures.

.DESCRIPTION
    Verifies the sealed package and the release-smoke capture manifest, rehashes all 76 PNGs, and
    copies them into an ignored self-contained review packet. The generated records are blank and
    do not claim visual approval or release authorization.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$CaptureRoot = "target\release-smoke",
    [string]$CaptureSummaryPath = "target\release-smoke\capture_summary.json",
    [string]$OutputRoot = "target\visual-review-packet"
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

function Get-DisplayName {
    param([string]$Scene)

    (Get-Culture).TextInfo.ToTitleCase($Scene.Replace('_', ' '))
}

$projectDir = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$targetDir = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
$archive = Resolve-ProjectPath $projectDir $ArchivePath
$manifestFile = Resolve-ProjectPath $projectDir $ManifestPath
$captureDir = Resolve-ProjectPath $projectDir $CaptureRoot
$captureSummaryFile = Resolve-ProjectPath $projectDir $CaptureSummaryPath
$outputRootDir = Resolve-ProjectPath $projectDir $OutputRoot
Assert-ChildPath $targetDir $captureDir
Assert-ChildPath $targetDir $captureSummaryFile
Assert-ChildPath $targetDir $outputRootDir

foreach ($required in @($archive, $manifestFile, $captureSummaryFile)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Required visual-review evidence is missing: $required"
    }
}

$manifest = Get-Content -LiteralPath $manifestFile -Raw | ConvertFrom-Json
$summary = Get-Content -LiteralPath $captureSummaryFile -Raw | ConvertFrom-Json
$archiveInfo = Get-Item -LiteralPath $archive
$archiveHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($manifest.schema_version -ne 1 -or
    [string]$manifest.archive -ne $archiveInfo.Name -or
    [long]$manifest.archive_bytes -ne $archiveInfo.Length -or
    [string]$manifest.archive_sha256 -ne $archiveHash -or
    [bool]$manifest.working_tree_dirty -or [bool]$manifest.toolkit_working_tree_dirty) {
    throw "External manifest does not identify a clean exact archive."
}

$exeRecord = @($manifest.included_files | Where-Object { $_.path -eq "hatchspire.exe" })
if ($exeRecord.Count -ne 1) {
    throw "External manifest must identify exactly one hatchspire.exe."
}
if ($summary.schema_version -ne 1 -or
    [string]$summary.status -ne "technical_capture_passed_human_review_pending" -or
    [string]$summary.build_id -ne [string]$manifest.build_id -or
    [string]$summary.git_commit -ne [string]$manifest.git_commit -or
    [string]$summary.toolkit_build_id -ne [string]$manifest.toolkit_build_id -or
    [string]$summary.toolkit_git_commit -ne [string]$manifest.toolkit_git_commit -or
    [string]$summary.archive_sha256 -ne $archiveHash -or
    [string]$summary.executable_sha256 -ne [string]$exeRecord[0].sha256 -or
    [bool]$summary.human_visual_review_recorded -or [bool]$summary.release_approval_granted) {
    throw "Capture summary does not identify the exact unapproved package."
}

$expectedResolutions = @("1280x720", "1366x768", "1920x1080", "960x540")
$scenes = @($summary.scenes | ForEach-Object { [string]$_ })
$captures = @($summary.captures)
if ($scenes.Count -ne 19 -or $scenes.Count -ne [int]$summary.scene_count -or
    $captures.Count -ne 76 -or $captures.Count -ne [int]$summary.capture_count -or
    [int]$summary.resolution_count -ne $expectedResolutions.Count) {
    throw "Capture summary must contain 19 scenes and 76 four-resolution captures."
}
if (@($scenes | Select-Object -Unique).Count -ne $scenes.Count) {
    throw "Capture summary contains duplicate scene names."
}

$captureByKey = @{}
foreach ($capture in $captures) {
    $scene = [string]$capture.scene
    $resolution = [string]$capture.resolution
    $relativePath = [string]$capture.path
    $key = "$scene|$resolution"
    if ($scene -notin $scenes -or $resolution -notin $expectedResolutions -or
        [string]::IsNullOrWhiteSpace($relativePath) -or $captureByKey.ContainsKey($key)) {
        throw "Capture summary contains an unknown or duplicate capture: $key"
    }
    $source = [IO.Path]::GetFullPath((Join-Path $captureDir $relativePath))
    Assert-ChildPath $captureDir $source
    if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
        throw "Capture is missing: $relativePath"
    }
    $sourceInfo = Get-Item -LiteralPath $source
    $sourceHash = (Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash.ToLowerInvariant()
    $width = Get-PngDimension $source 16
    $height = Get-PngDimension $source 20
    if ($sourceInfo.Length -ne [long]$capture.bytes -or
        $sourceHash -ne [string]$capture.sha256 -or
        $width -ne [int]$capture.width -or $height -ne [int]$capture.height) {
        throw "Capture no longer matches its technical manifest: $relativePath"
    }
    $captureByKey[$key] = [pscustomobject]@{
        Scene = $scene
        Resolution = $resolution
        Source = $source
        Bytes = $sourceInfo.Length
        Sha256 = $sourceHash
        Width = $width
        Height = $height
        Fullscreen = [bool]$capture.fullscreen
    }
}
foreach ($scene in $scenes) {
    foreach ($resolution in $expectedResolutions) {
        if (-not $captureByKey.ContainsKey("$scene|$resolution")) {
            throw "Capture matrix is missing $scene at $resolution."
        }
    }
}

$candidateDir = Join-Path $outputRootDir ([string]$manifest.git_commit)
$capturesOutputDir = Join-Path $candidateDir "captures"
Assert-ChildPath $targetDir $candidateDir
if (Test-Path -LiteralPath $candidateDir) {
    throw "Review packet already exists; refusing to overwrite possible human evidence: $candidateDir"
}
New-Item -ItemType Directory -Path $capturesOutputDir -Force | Out-Null

$packetCaptures = [Collections.Generic.List[object]]::new()
foreach ($scene in $scenes) {
    foreach ($resolution in $expectedResolutions) {
        $capture = $captureByKey["$scene|$resolution"]
        $destinationDir = Join-Path $capturesOutputDir $resolution
        New-Item -ItemType Directory -Path $destinationDir -Force | Out-Null
        $destination = Join-Path $destinationDir "ui_$scene.png"
        Assert-ChildPath $candidateDir $destination
        Copy-Item -LiteralPath $capture.Source -Destination $destination -Force
        if ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash.ToLowerInvariant() -ne
            $capture.Sha256) {
            throw "Copied review capture hash mismatch: $scene at $resolution"
        }
        $packetCaptures.Add([ordered]@{
            scene = $scene
            resolution = $resolution
            width = $capture.Width
            height = $capture.Height
            fullscreen = $capture.Fullscreen
            path = "captures/$resolution/ui_$scene.png"
            bytes = $capture.Bytes
            sha256 = $capture.Sha256
        })
    }
}

$generatedUtc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
$packetManifest = [ordered]@{
    schema_version = 1
    status = "awaiting_human_visual_review"
    generated_utc = $generatedUtc
    build_id = [string]$manifest.build_id
    git_commit = [string]$manifest.git_commit
    toolkit_build_id = [string]$manifest.toolkit_build_id
    toolkit_git_commit = [string]$manifest.toolkit_git_commit
    archive_sha256 = $archiveHash
    executable_sha256 = [string]$exeRecord[0].sha256
    scene_count = $scenes.Count
    resolution_count = $expectedResolutions.Count
    capture_count = $packetCaptures.Count
    captures = @($packetCaptures)
    human_visual_review_recorded = $false
    release_approval_granted = $false
}
$packetManifestPath = Join-Path $candidateDir "CAPTURE_MANIFEST.json"
$packetManifest | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $packetManifestPath -Encoding utf8

$reviewRows = foreach ($scene in $scenes) {
    "| $(Get-DisplayName $scene) |  |  |  |  |  |"
}
$review = @"
# Hatchspire candidate visual review

Status: **AWAITING HUMAN VISUAL REVIEW — NOT RELEASE APPROVAL**  
Generated UTC: $generatedUtc

## Exact candidate

| Field | Value |
| --- | --- |
| Build ID | $($manifest.build_id) |
| Hatchspire commit | $($manifest.git_commit) |
| Toolkit build | $($manifest.toolkit_build_id) |
| ZIP SHA-256 | $archiveHash |
| EXE SHA-256 | $($exeRecord[0].sha256) |
| Scenes | $($scenes.Count) |
| Captures | $($packetCaptures.Count) |

Open `REVIEW_GALLERY.html` beside this file. Review every image at its native size, not only the
thumbnail. Record Pass, Fail, or Needs discussion for every resolution. Blank cells do not pass.

Check required text readability, clipping, overlap, chroma leakage, missing or malformed art,
incorrect scale, obvious generation defects, visual hierarchy, touch-target clarity, and whether the
scene is suitable to represent Hatchspire publicly. Structural capture success is not taste approval.

## Scene matrix

| Scene | 1280×720 | 1366×768 | 1920×1080 fullscreen | 960×540 | Evidence / issue IDs |
| --- | --- | --- | --- | --- | --- |
$($reviewRows -join [Environment]::NewLine)

## Cross-scene review

- Art direction and palette consistency: PASS / FAIL / NEEDS DISCUSSION
- Monster identity and scale consistency: PASS / FAIL / NEEDS DISCUSSION
- Required controls remain visibly tappable: PASS / FAIL / NEEDS DISCUSSION
- Required text remains readable at every size: PASS / FAIL / NEEDS DISCUSSION
- No chroma leakage, overlap, clipping, missing art, or obvious generation defect: PASS / FAIL / NEEDS DISCUSSION
- Title, icon, game UI, screenshots, and public visual promise agree: PASS / FAIL / NEEDS DISCUSSION
- Issue IDs and notes: ____________________

## Human decision

- Overall capture review: PASS / FAIL / INCOMPLETE
- Reviewer: ____________________
- Review date and time: ____________________
- Reviewer attestation: ____________________
- Accepted visual issues and rationale: ____________________
- Public release authorized by this record: **NO — use the separate final GO record**
"@
$reviewPath = Join-Path $candidateDir "VISUAL_REVIEW_RECORD.md"
Set-Content -LiteralPath $reviewPath -Value $review -Encoding utf8

$htmlRows = foreach ($scene in $scenes) {
    $display = [Net.WebUtility]::HtmlEncode((Get-DisplayName $scene))
    $cells = foreach ($resolution in $expectedResolutions) {
        $relative = "captures/$resolution/ui_$scene.png"
        $encodedPath = [Net.WebUtility]::HtmlEncode($relative)
        "<td><a href=`"$encodedPath`"><img src=`"$encodedPath`" alt=`"$display at $resolution`"></a><div>$resolution</div></td>"
    }
    "<tr><th scope=`"row`">$display</th>$($cells -join '')</tr>"
}
$html = @"
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Hatchspire visual review — $($manifest.build_id)</title>
<style>
body{font-family:Segoe UI,Arial,sans-serif;margin:1.5rem;background:#15131b;color:#f1e9d2}a{color:#9fd6ff}
header{max-width:90rem;margin:auto}.warning{padding:.8rem;background:#4b2e22;border:1px solid #d6995e}
table{border-collapse:collapse;margin:1.5rem auto;width:100%;max-width:110rem}th,td{border:1px solid #554e61;padding:.5rem;text-align:center;vertical-align:top}
th{background:#25212e;position:sticky;left:0}img{display:block;width:100%;max-width:28rem;height:auto;margin:auto;background:#000}td div{margin-top:.35rem;color:#c9bfaa}
</style>
</head>
<body>
<header><h1>Hatchspire candidate visual review</h1>
<p class="warning"><strong>Awaiting human visual review.</strong> Technical capture passed; no image or release is approved by this packet.</p>
<p>Build <strong>$($manifest.build_id)</strong><br>Commit $($manifest.git_commit)<br>ZIP SHA-256 $archiveHash</p>
<p>Click any image to inspect its native pixels. Record decisions in <a href="VISUAL_REVIEW_RECORD.md">VISUAL_REVIEW_RECORD.md</a>.</p></header>
<table><thead><tr><th>Scene</th><th>1280×720</th><th>1366×768</th><th>1920×1080 fullscreen</th><th>960×540</th></tr></thead>
<tbody>$($htmlRows -join [Environment]::NewLine)</tbody></table>
</body>
</html>
"@
$galleryPath = Join-Path $candidateDir "REVIEW_GALLERY.html"
Set-Content -LiteralPath $galleryPath -Value $html -Encoding utf8

Write-Host "Candidate visual-review packet created:" -ForegroundColor Green
Write-Host "  Build ID: $($manifest.build_id)"
Write-Host "  Captures: $($packetCaptures.Count) exact PNGs across $($scenes.Count) scenes"
Write-Host "  Gallery: $galleryPath"
Write-Host "  Status: awaiting human visual review; no image or release approval recorded." -ForegroundColor Yellow
