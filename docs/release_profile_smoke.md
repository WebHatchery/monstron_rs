# Hatchspire release-profile smoke gate

Audit date: 27 August 2026

`scripts/release_smoke.ps1` validates the optimized Windows executable that is actually inside the
post-publish preview archive.

The gate:

1. requires a clean working tree unless explicitly run as an internal dirty-tree test;
2. requires the external manifest and checksum sidecar to match the exact archive name, byte size,
   SHA-256, clean/dirty state, version, commit, build identity, and strict UTC build date;
3. hashes every ZIP entry and independently checks the external file list plus the embedded
   `BUILD_INFO.json` payload list;
4. requires the versioned executable and all player/support/legal documents in the ZIP, and requires
   both manifests to identify exactly `quad-rand 0.2.3` as the sole missing dependency license file;
5. requires the package commit and version to match Git HEAD and Cargo;
6. hashes `hatchspire.exe` inside the ZIP and requires it to match the release build byte-for-byte;
7. verifies the exact executable's Windows product/file name and Cargo-derived version fields, then
   compares its rendered 32 px icon pixels with the tracked multi-resolution source;
8. boots that optimized build through all 19 deterministic capture scenes in one process;
9. records 30 update+draw CPU samples per scene, enforces a 16.667 ms p95 ceiling, and reports the
   diagnostic maximum without treating noisy first-use work as a release threshold;
10. requires internally ordered first/median/p95/final/maximum sampled working sets and an OS peak
   from each resolution process, with an optional explicit sampled-memory ceiling but no unstable
   default cap;
11. requires a successful process exit within the capture timeout;
12. boots the same executable again in 1366×768 and 960×540 windows plus 1920×1080 fullscreen,
   validating all 76 scene outputs as nontrivial PNGs with exact dimensions; then
13. launches and exits the identical EXE five consecutive times from a working path containing
   spaces and Unicode, marks the EXE read-only, validates every rendered boot, and fails if any
   launch writes a sidecar beside it.

Run the complete internal package sequence with:

```powershell
.\publish.ps1
.\scripts\package_windows_preview.ps1
.\scripts\release_smoke.ps1
.\publish.ps1 -DeployOnly
```

Current baseline: all 19 scenes pass at all four resolutions. The first expanded 18-scene run had a
0.715 ms worst p95 update+draw time; five earlier 16-scene runs peaked at 0.830 ms. A diagnostic first-use
sample reached 1,260.245 ms while its scene p95 remained 0.556 ms. The package
manifest and command output, rather than this mutable document, record the candidate's exact commit
and executable SHA-256. See `docs/performance_probe.md` for the measurement boundary.

## What this does not prove

The smoke gate verifies startup, seeded update/draw CPU time, short-batch process-memory reporting,
capture output, and clean process completion. It does not play the controls, compare visual
correctness, or traverse the
scope-dependent full demo,
measure GPU presentation, sustained frame time or ordinary-play memory, exercise OS window events, or replace clean-machine and
physical-device testing. The fullscreen 1080p capture avoids Windows reducing a decorated
1920×1080 window to its desktop work area; it does not validate interactive fullscreen toggling.
Structural success at four resolutions is not human screenshot approval.
A read-only executable and sidecar-free relocated launch do not reproduce
a directory with write-denying ACLs. The canonical `docs/verification` images still require human
visual review.
