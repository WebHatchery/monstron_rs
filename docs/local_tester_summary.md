# Local tester summary

Audit date: 27 August 2026

The Windows build exposes **EXPORT LOCAL SUMMARY** in **Settings → Help & Support** whenever an
active or readable saved camp exists. The counters update only as part of ordinary autosaved game
progress; no separate report is sent over the network, copied to the clipboard, or written by
opening the screen. The player must tap the visible export control. Hatchspire then atomically
replaces `tester_summary.txt` beside its save files and shows the exact local path on screen.

The text file identifies the exact build and records these agreed pacing and balance facts:

- current day, recorded progression actions, and days advanced;
- expeditions started/returned, return rate, rooms explored, and highest reached/unlocked floor;
- combat encounters, victories, defeats, fled combats, and win rate;
- monsters hatched, current roster and egg counts, and facility levels gained; and
- current resource and facility snapshots.

Counters are part of the autosaved game state and use saturating integer updates. An older save
loads safely with zeroed counters, so a migrated save reports only actions performed after it first
loads in a metrics-capable build. A fresh save is required for a complete playtest-session summary.
Opening Help or exporting does not change game progress.

This artifact is intentionally not telemetry. It contains no player name, machine identifier,
account, input log, wall-clock timestamp, file content, or automatic upload mechanism. A tester may
inspect the plain-text file and choose whether to share it manually. The summary helps aggregate
local pacing and balance evidence; it does not establish the human coaching, comprehension, or
subjective-quality measures in the release plan.
