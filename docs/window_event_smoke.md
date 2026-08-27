# Hatchspire exact-package Windows event smoke

Audit date: 27 August 2026

`scripts/window_event_smoke.ps1` exercises live Win32 resize, minimize, and restore events against
the exact executable extracted from a sealed clean Windows package. It verifies the manifest,
checksum, archive, source identities, and executable hash before starting one visible deterministic
capture process.

Run it after sealing a clean package:

```powershell
.\scripts\window_event_smoke.ps1
```

The default run creates a 1280×720 client area, resizes it to exactly 960×540 using the current
window styles, verifies the live client size, minimizes it, observes the iconic state, restores it,
and verifies that the resized client area survives. The process must then produce a nontrivial
960×540 render and exit cleanly. The default wall-clock duration is eight seconds.

Ignored evidence at `target/window-event-smoke/summary.json` records the exact build/toolkit/archive/
EXE identities, timestamps, elapsed time, requested and observed sizes, minimize/restore observations,
capture dimensions, and exit code. The PNG and process logs live beside it; the extracted executable
and temporary directory are removed after the run.

This is a development-host diagnostic. It does not reproduce a player's Alt+Tab sequence, interactive
fullscreen toggle, display-scale changes, sleep/wake, multiple monitors, GPU-driver differences, or
physical-device conditions. Those remain human gates even when this probe passes.
