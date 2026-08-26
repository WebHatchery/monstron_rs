# Hatchspire release-profile smoke gate

Audit date: 27 August 2026

`scripts/release_smoke.ps1` validates the optimized Windows executable that is actually inside the
post-publish preview archive.

The gate:

1. requires a clean working tree unless explicitly run as an internal dirty-tree test;
2. reads `BUILD_INFO.json` from `dist/hatchspire_windows.zip`;
3. requires the package commit and version to match Git HEAD and Cargo;
4. hashes `hatchspire.exe` inside the ZIP and requires it to match the release build byte-for-byte;
5. boots that optimized build through all 18 deterministic capture scenes in one process;
6. records 30 update+draw CPU samples per scene, enforces a 16.667 ms p95 ceiling, and reports the
   diagnostic maximum without treating noisy first-use work as a release threshold;
7. requires a successful process exit within the capture timeout;
8. boots the same executable again in 1366×768 and 960×540 windows plus 1920×1080 fullscreen,
   validating all 72 scene outputs as nontrivial PNGs with exact dimensions; then
9. launches the identical EXE from a working path containing spaces and Unicode, marks the EXE
   read-only, and fails if the game writes any sidecar beside it.

Run the complete internal package sequence with:

```powershell
.\publish.ps1
.\scripts\package_windows_preview.ps1
.\scripts\release_smoke.ps1
.\publish.ps1 -DeployOnly
```

Current baseline: all 18 scenes pass at all four resolutions. The first expanded 18-scene run had a
0.715 ms worst p95 update+draw time; five earlier 16-scene runs peaked at 0.830 ms. A diagnostic first-use
sample reached 1,260.245 ms while its scene p95 remained 0.556 ms. The package
manifest and command output, rather than this mutable document, record the candidate's exact commit
and executable SHA-256. See `docs/performance_probe.md` for the measurement boundary.

## What this does not prove

The smoke gate verifies startup, seeded update/draw CPU time, capture output, and clean process
completion. It does not play the controls, compare visual correctness, or traverse the
scope-dependent full demo,
measure GPU presentation, sustained frame time or memory, exercise OS window events, or replace clean-machine and
physical-device testing. The fullscreen 1080p capture avoids Windows reducing a decorated
1920×1080 window to its desktop work area; it does not validate interactive fullscreen toggling.
Structural success at four resolutions is not human screenshot approval.
A read-only executable and sidecar-free relocated launch do not reproduce
a directory with write-denying ACLs. The canonical `docs/verification` images still require human
visual review.
