<#
.SYNOPSIS
    Exercises live resize, minimize, and restore events on the exact packaged Windows executable.

.DESCRIPTION
    Verifies and extracts the clean package, starts one visible deterministic capture, resizes its
    client area through Win32, minimizes and restores it, then requires a valid final render and
    clean process exit. This host diagnostic does not replace interactive physical-device testing.
#>
param(
    [string]$ArchivePath = "dist\hatchspire_windows.zip",
    [string]$ManifestPath = "dist\hatchspire_windows_manifest.json",
    [string]$ChecksumPath = "dist\hatchspire_windows.sha256",
    [int]$InitialWidth = 1280,
    [int]$InitialHeight = 720,
    [int]$ResizedWidth = 960,
    [int]$ResizedHeight = 540,
    [int]$DurationSeconds = 8
)

$ErrorActionPreference = "Stop"

if ($InitialWidth -lt 640 -or $InitialHeight -lt 360 -or
    $ResizedWidth -lt 640 -or $ResizedHeight -lt 360) {
    throw "Initial and resized client areas must be at least 640x360."
}
if ($DurationSeconds -lt 6 -or $DurationSeconds -gt 60) {
    throw "DurationSeconds must be between 6 and 60."
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

function Get-PngDimension {
    param([string]$Path, [int]$Offset)

    $stream = [IO.File]::OpenRead($Path)
    try {
        $stream.Position = $Offset
        $bytes = New-Object byte[] 4
        if ($stream.Read($bytes, 0, 4) -ne 4) { throw "Invalid PNG: $Path" }
        [Array]::Reverse($bytes)
        [BitConverter]::ToUInt32($bytes, 0)
    } finally {
        $stream.Dispose()
    }
}

function Wait-Until {
    param([scriptblock]$Condition, [int]$TimeoutMilliseconds, [string]$Failure)

    $deadline = [DateTime]::UtcNow.AddMilliseconds($TimeoutMilliseconds)
    while ([DateTime]::UtcNow -lt $deadline) {
        if (& $Condition) { return }
        Start-Sleep -Milliseconds 50
    }
    throw $Failure
}

if (-not ("HatchspireNativeWindow" -as [type])) {
    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public static class HatchspireNativeWindow {
    public delegate bool EnumWindowsCallback(IntPtr window, IntPtr parameter);

    [StructLayout(LayoutKind.Sequential)]
    public struct Rect { public int Left, Top, Right, Bottom; }

    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool GetClientRect(IntPtr window, out Rect rect);

    [DllImport("user32.dll", SetLastError = true)]
    public static extern int GetWindowLong(IntPtr window, int index);

    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool AdjustWindowRectEx(ref Rect rect, int style, bool menu, int exStyle);

    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool MoveWindow(IntPtr window, int x, int y, int width, int height, bool repaint);

    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool ShowWindowAsync(IntPtr window, int command);

    [DllImport("user32.dll")]
    public static extern bool IsIconic(IntPtr window);

    [DllImport("user32.dll")]
    private static extern bool EnumWindows(EnumWindowsCallback callback, IntPtr parameter);

    [DllImport("user32.dll")]
    private static extern uint GetWindowThreadProcessId(IntPtr window, out uint processId);

    [DllImport("user32.dll")]
    private static extern bool IsWindowVisible(IntPtr window);

    public static IntPtr FindVisibleWindow(int wantedProcessId) {
        IntPtr found = IntPtr.Zero;
        int largestArea = 0;
        EnumWindows((window, parameter) => {
            uint processId;
            GetWindowThreadProcessId(window, out processId);
            if (processId == wantedProcessId && IsWindowVisible(window)) {
                Rect rect;
                if (GetClientRect(window, out rect)) {
                    int area = Math.Max(0, rect.Right - rect.Left) * Math.Max(0, rect.Bottom - rect.Top);
                    if (area > largestArea) {
                        largestArea = area;
                        found = window;
                    }
                }
            }
            return true;
        }, IntPtr.Zero);
        return found;
    }
}
"@
}

function Get-ClientSize {
    param([IntPtr]$Window)

    $rect = [HatchspireNativeWindow+Rect]::new()
    if (-not [HatchspireNativeWindow]::GetClientRect($Window, [ref]$rect)) {
        throw "Could not read the Hatchspire client area."
    }
    [pscustomobject]@{ Width = $rect.Right - $rect.Left; Height = $rect.Bottom - $rect.Top }
}

function Set-ClientSize {
    param([IntPtr]$Window, [int]$Width, [int]$Height)

    $style = [HatchspireNativeWindow]::GetWindowLong($Window, -16)
    $exStyle = [HatchspireNativeWindow]::GetWindowLong($Window, -20)
    $rect = [HatchspireNativeWindow+Rect]::new()
    $rect.Right = $Width
    $rect.Bottom = $Height
    if (-not [HatchspireNativeWindow]::AdjustWindowRectEx([ref]$rect, $style, $false, $exStyle)) {
        throw "Could not calculate the resized Hatchspire window frame."
    }
    if (-not [HatchspireNativeWindow]::MoveWindow(
            $Window, 80, 80, $rect.Right - $rect.Left, $rect.Bottom - $rect.Top, $true)) {
        throw "Could not resize the Hatchspire window."
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
if ($manifest.schema_version -ne 1 -or [string]$manifest.archive -ne $archiveInfo.Name -or
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
    throw "Window-event release evidence requires a clean Hatchspire and toolkit package."
}

$targetRoot = [IO.Path]::GetFullPath((Join-Path $projectDir "target"))
$evidenceDir = Join-Path $targetRoot "window-event-smoke"
$runDir = Join-Path $targetRoot ("window-event-smoke-run-" + [Guid]::NewGuid().ToString("N"))
$executable = Join-Path $runDir "hatchspire.exe"
$capturePath = Join-Path $evidenceDir "ui_resized_restored.png"
$captureManifest = Join-Path $evidenceDir "capture.tsv"
$stdoutPath = Join-Path $evidenceDir "stdout.log"
$stderrPath = Join-Path $evidenceDir "stderr.log"
$summaryPath = Join-Path $evidenceDir "summary.json"
Assert-ChildPath $targetRoot $evidenceDir
Assert-ChildPath $targetRoot $runDir

$process = $null
$primaryError = $null
try {
    New-Item -ItemType Directory -Path $runDir -Force | Out-Null
    New-Item -ItemType Directory -Path $evidenceDir -Force | Out-Null
    Remove-Item -LiteralPath $capturePath, $captureManifest, $stdoutPath, $stderrPath, $summaryPath `
        -Force -ErrorAction SilentlyContinue

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

    Set-Content -LiteralPath $captureManifest -Value "mainmenu`t$capturePath" -Encoding utf8
    $frames = [int][Math]::Ceiling(($DurationSeconds * 1000.0) / 16.667)
    Set-Item Env:HATCHSPIRE_CAPTURE_MANIFEST $captureManifest
    Set-Item Env:HATCHSPIRE_CAPTURE_FRAMES $frames
    Set-Item Env:HATCHSPIRE_CAPTURE_MIN_FRAME_MS "16.667"
    Set-Item Env:HATCHSPIRE_HEADLESS "0"
    Set-Item Env:HATCHSPIRE_WINDOW_WIDTH $InitialWidth
    Set-Item Env:HATCHSPIRE_WINDOW_HEIGHT $InitialHeight

    $started = [DateTime]::UtcNow
    $process = Start-Process -FilePath $executable -WorkingDirectory $runDir -PassThru `
        -RedirectStandardOutput $stdoutPath -RedirectStandardError $stderrPath
    Wait-Until {
        $process.Refresh()
        $process.HasExited -or
            [HatchspireNativeWindow]::FindVisibleWindow($process.Id) -ne [IntPtr]::Zero
    } 10000 "Hatchspire did not create a window within 10 seconds."
    if ($process.HasExited) { throw "Hatchspire exited before its window could be exercised." }
    $window = [HatchspireNativeWindow]::FindVisibleWindow($process.Id)

    $initialObserved = Get-ClientSize $window
    if ($initialObserved.Width -lt 640 -or $initialObserved.Height -lt 360) {
        throw "Hatchspire created an invalid initial client area: $($initialObserved.Width)x$($initialObserved.Height)."
    }
    Set-ClientSize $window $ResizedWidth $ResizedHeight
    Wait-Until {
        $size = Get-ClientSize $window
        $size.Width -eq $ResizedWidth -and $size.Height -eq $ResizedHeight
    } 5000 "Hatchspire did not process the live resize event."

    [HatchspireNativeWindow]::ShowWindowAsync($window, 6) | Out-Null
    Wait-Until { [HatchspireNativeWindow]::IsIconic($window) } 5000 `
        "Hatchspire did not enter the minimized state."
    Start-Sleep -Milliseconds 500
    [HatchspireNativeWindow]::ShowWindowAsync($window, 9) | Out-Null
    Wait-Until { -not [HatchspireNativeWindow]::IsIconic($window) } 5000 `
        "Hatchspire did not restore from the minimized state."
    Wait-Until {
        $size = Get-ClientSize $window
        $size.Width -eq $ResizedWidth -and $size.Height -eq $ResizedHeight
    } 5000 "Hatchspire did not preserve its resized client area after restore."
    $restoredSize = Get-ClientSize $window

    if (-not $process.WaitForExit(($DurationSeconds + 60) * 1000)) {
        $process.Kill()
        throw "Hatchspire did not exit after the window-event capture."
    }
    $process.WaitForExit()
    if ($process.ExitCode -ne 0) {
        $details = @(
            if (Test-Path -LiteralPath $stdoutPath) { Get-Content $stdoutPath -Tail 40 }
            if (Test-Path -LiteralPath $stderrPath) { Get-Content $stderrPath -Tail 40 }
        ) -join [Environment]::NewLine
        throw "Hatchspire exited with code $($process.ExitCode). $details"
    }
    if (-not (Test-Path -LiteralPath $capturePath -PathType Leaf) -or
        (Get-Item $capturePath).Length -lt 20000) {
        throw "The resized/restored capture is missing or blank."
    }
    $captureWidth = Get-PngDimension $capturePath 16
    $captureHeight = Get-PngDimension $capturePath 20
    if ($captureWidth -lt 640 -or $captureHeight -lt 360) {
        throw "The resized/restored capture has an invalid framebuffer size."
    }

    [ordered]@{
        schema_version = 1
        status = "host_diagnostic_only"
        build_id = [string]$manifest.build_id
        git_commit = [string]$manifest.git_commit
        toolkit_build_id = [string]$manifest.toolkit_build_id
        toolkit_git_commit = [string]$manifest.toolkit_git_commit
        archive_sha256 = $archiveHash
        executable_sha256 = $exeHash
        started_utc = $started.ToString("yyyy-MM-ddTHH:mm:ssZ")
        elapsed_wall_milliseconds = [long]([DateTime]::UtcNow - $started).TotalMilliseconds
        requested_initial_width = $InitialWidth
        requested_initial_height = $InitialHeight
        initial_client_width = $initialObserved.Width
        initial_client_height = $initialObserved.Height
        resized_client_width = $restoredSize.Width
        resized_client_height = $restoredSize.Height
        capture_width = $captureWidth
        capture_height = $captureHeight
        minimize_observed = $true
        restore_observed = $true
        exit_code = $process.ExitCode
        capture = $capturePath
    } | ConvertTo-Json | Set-Content -LiteralPath $summaryPath -Encoding utf8

    Write-Host "Exact-package Windows event smoke passed:" -ForegroundColor Green
    Write-Host "  Build ID: $($manifest.build_id)"
    Write-Host "  Resize: ${InitialWidth}x$InitialHeight -> ${ResizedWidth}x$ResizedHeight"
    Write-Host "  Minimize/restore: observed; final render and clean exit verified"
    Write-Host "  Evidence: $summaryPath"
    Write-Host "  Host diagnostic only; interactive Alt+Tab/fullscreen and physical-device gates remain open." -ForegroundColor Yellow
} catch {
    $primaryError = $_
    throw
} finally {
    Remove-Item Env:HATCHSPIRE_CAPTURE_MANIFEST, Env:HATCHSPIRE_CAPTURE_FRAMES, `
        Env:HATCHSPIRE_CAPTURE_MIN_FRAME_MS, Env:HATCHSPIRE_HEADLESS, `
        Env:HATCHSPIRE_WINDOW_WIDTH, `
        Env:HATCHSPIRE_WINDOW_HEIGHT -ErrorAction SilentlyContinue
    if ($null -ne $process) {
        $process.Refresh()
        if (-not $process.HasExited) {
            $process.Kill()
            $process.WaitForExit(5000) | Out-Null
        }
        $process.WaitForExit()
        $process.Dispose()
    }
    if (Test-Path -LiteralPath $runDir) {
        Assert-ChildPath $targetRoot $runDir
        for ($attempt = 1; $attempt -le 20; $attempt++) {
            try {
                Remove-Item -LiteralPath $runDir -Recurse -Force
                break
            } catch {
                if ($attempt -eq 20) {
                    if ($null -eq $primaryError) { throw }
                    Write-Warning "Window-event temporary directory cleanup failed: $($_.Exception.Message)"
                }
                Start-Sleep -Milliseconds 250
            }
        }
    }
}
