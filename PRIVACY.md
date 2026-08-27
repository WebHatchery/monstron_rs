# Hatchspire Windows preview privacy

The current Windows game binary does not send analytics, telemetry, crash reports, save data,
settings, or gameplay data over the network.

Hatchspire stores the following data locally for the current Windows user:

- one active saved camp and its recovery copies;
- saved local pacing/balance counters used for the optional tester summary;
- audio, display, motion, and UI-scale settings; and
- `crash_log.txt` when a native panic occurs.

The game creates `tester_summary.txt` beside the save only when the player taps **EXPORT LOCAL
SUMMARY** in Help. The text identifies the build and summarizes saved progression, expedition,
combat, roster, facility, and resource facts. It contains no player name, machine identifier,
account, wall-clock timestamp, or input log, and it is never uploaded automatically.

The game does not upload those files. They leave the computer only if the player chooses to attach
them to a manual support report. A player can remove the active save and recovery copies through the
separately confirmed **Save Reset** control. Settings, `crash_log.txt`, and `tester_summary.txt`
deletion are currently manual.

This statement describes the Windows binary only. A future download page, payment provider,
community service, or support form may process data under its own terms and must be reviewed before
public release. If networking, accounts, analytics, or automatic crash upload are added, this
statement must be revised before that build is distributed.
