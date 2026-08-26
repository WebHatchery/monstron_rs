# Hatchspire release-profile smoke gate

Audit date: 27 August 2026

`scripts/release_smoke.ps1` validates the optimized Windows executable that is actually inside the
post-publish preview archive.

The gate:

1. requires a clean working tree unless explicitly run as an internal dirty-tree test;
2. reads `BUILD_INFO.json` from `dist/hatchspire_windows.zip`;
3. requires the package commit and version to match Git HEAD and Cargo;
4. hashes `hatchspire.exe` inside the ZIP and requires it to match the release build byte-for-byte;
5. boots that optimized build through all 16 deterministic capture scenes in one process;
6. records 30 update+draw CPU samples per scene and enforces a 16.667 ms p95 and 250 ms single-frame
   ceiling;
7. requires a successful process exit within the capture timeout;
8. validates every output as a nontrivial 1280×720 PNG; then
9. launches the identical EXE from a working path containing spaces and Unicode, marks the EXE
   read-only, and fails if the game writes any sidecar beside it.

Run the complete internal package sequence with:

```powershell
.\publish.ps1
.\scripts\package_windows_preview.ps1
.\scripts\release_smoke.ps1
.\publish.ps1 -DeployOnly
```

Current baseline: all 16 scenes pass at 1280×720. Three fresh-process development-machine runs had a
worst p95 update+draw time of 0.830 ms and a worst individual time of 178.250 ms. The package
manifest and command output, rather than this mutable document, record the candidate's exact commit
and executable SHA-256. See `docs/performance_probe.md` for the measurement boundary.

## What this does not prove

The smoke gate verifies startup, seeded update/draw CPU time, capture output, and clean process
completion. It does not play the controls, compare visual correctness, traverse the scope-dependent full demo,
measure GPU presentation, sustained frame time or memory, exercise OS window events, or replace clean-machine and
physical-device testing. A read-only executable and sidecar-free relocated launch do not reproduce
a directory with write-denying ACLs. The canonical `docs/verification` images still require human
visual review.
