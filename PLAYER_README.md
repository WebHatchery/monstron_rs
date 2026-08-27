# Hatchspire Windows Preview

Hatchspire is a monster-raising town RPG about rebuilding a camp beneath a ruined tower, caring
for tower-born companions, and preparing them for increasingly dangerous expeditions.

This archive is an internal pre-release preview. It is not yet the approved public demo: onboarding,
the authored demo finale, audio, rights approval, and outside testing remain incomplete.

## Install and start

1. Extract the entire ZIP to a writable folder.
2. Open the extracted folder.
3. Run `hatchspire.exe`.

Do not run the executable from inside the ZIP. The target platform is Windows 10/11 x64; final
minimum hardware requirements have not yet been approved.

## Controls and quitting

Every required action has a visible mouse/touch target. Keyboard shortcuts are optional. Open the
camp menu with its visible button and choose **Exit Game** for a clean quit.

## Saves and recovery

Progress autosaves after successful state-changing actions. A successful replacement save retains
one previous state. If a save cannot be loaded, the recovery screen can restore that backup while
preserving the file it replaces. New Game and Save Reset both require visible confirmation.

Settings and saves are kept in the current Windows user's local application-data folder. Open
**Settings → Help & Support** in the game to see the exact saved-camp path and running version.
That screen can also create `tester_summary.txt` beside the save when you tap **EXPORT LOCAL
SUMMARY**. The plain-text pacing/balance summary is never uploaded automatically; inspect it before
choosing whether to share it with the private test team.

See `docs/SUPPORT.md`, `docs/KNOWN_ISSUES.md`, and `docs/PRIVACY.md` before testing.
