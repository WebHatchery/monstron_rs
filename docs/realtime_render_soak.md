# Hatchspire sustained exact-package render soak

Audit date: 27 August 2026

`scripts/realtime_render_soak.ps1` runs the exact `hatchspire.exe` from the sealed Windows ZIP for
real wall-clock time. It verifies the external manifest and checksum, extracts the archive into an
isolated temporary directory, rehashes the executable, and then cycles all 19 release scenes in one
process. The capture harness holds each deterministic frame for a configured minimum duration so the
process remains alive long enough to expose sustained resource growth instead of finishing an
accelerated render loop in seconds.

The default command is the release-evidence form:

```powershell
.\scripts\realtime_render_soak.ps1
```

It opens a visible 1280×720 window for two hours. A visible run between two and four hours is labelled
`release_evidence`; shorter runs and all headless runs are labelled `validation_only`. The four-hour
maximum keeps the release contract bounded. A deliberately short harness check is:

```powershell
.\scripts\realtime_render_soak.ps1 -DurationSeconds 10 -Headless
```

Use `-AllowDirty` only while developing or validating the harness against a package whose manifest
explicitly identifies the dirty tree. It cannot turn a dirty package into release evidence.

The ignored `target/realtime-render-soak/summary.json` identifies the exact archive, executable,
commit, requested and elapsed duration, visibility, scene/frame counts, worst p95 and single-frame
CPU samples, and sampled working-set distribution. Supporting captures, per-scene timing records,
and process-memory evidence live beside it. The provisional p95 update+draw limit remains 16.667 ms;
there is no default memory ceiling until representative physical-machine evidence supports one.

This automated render soak does not exercise the demo's interaction path, save/load and restart
boundaries, audio, Alt+Tab, resizing, fullscreen switching, controller behavior, GPU memory,
presentation latency, thermals, or clean-device compatibility. It complements the accelerated
240-cycle gameplay/persistence soak and short four-resolution release smoke; it does not replace a
human-observed 2–4 hour play/device run on representative physical Windows hardware.
