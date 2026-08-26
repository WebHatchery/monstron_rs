<#
.SYNOPSIS
    Headless screenshot harness for Hatchspire.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook
    (HATCHSPIRE_CAPTURE_*) provided by macroquad_toolkit::capture in
    src/main.rs. Scenes are seeded via Game::begin_capture_scene:
      - "mainmenu" -> boot state (main menu)
      - "new_game_warning" -> destructive New Game confirmation
      - "save_recovery" -> unreadable-save preservation choices
      - "autosave_notice" -> visible successful autosave status
      - "save_migration_notice" -> successful historical-save upgrade status
      - "save_reset_warning" -> destructive save reset confirmation
      - "help" -> build, save location, and recovery instructions
      - "settings" -> persisted audio, display, and motion preferences
      - "town"     -> fresh save, town screen
      - "hatchery" -> seeded hatchery screen
      - "stable"   -> seeded stable roster and recovery scene
      - "breeding" -> seeded breeding grove scene
      - "workshop" -> seeded workshop assignments scene
      - "shop"     -> seeded shop scene
      - "tower"    -> seeded active first-floor dungeon run
      - "combat"   -> seeded first-floor combat scene

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [string[]]$Scenes = @("mainmenu", "new_game_warning", "save_recovery", "autosave_notice", "save_migration_notice", "save_reset_warning", "help", "settings", "town", "hatchery", "stable", "breeding", "workshop", "shop", "tower", "combat"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
