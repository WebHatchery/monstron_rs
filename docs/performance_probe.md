# Hatchspire release-capture CPU probe

Audit date: 27 August 2026

The exact packaged optimized Windows executable records a local performance sample while
`scripts/release_smoke.ps1` renders its 16 deterministic scenes. Each scene contributes 30 frames
at 1280x720. The sample contains the average, 95th-percentile, and maximum CPU time spent inside
Hatchspire's `update` and `draw` calls.

The smoke gate currently requires:

- every scene's p95 update+draw time to be at most 16.667 ms; and
- every individual update+draw sample to be at most 250 ms.

The p95 limit is the provisional 60 Hz regression budget. The separate maximum limit retains the
first-use work performed by scene transitions—such as lazy glyph or texture preparation—without
allowing an unbounded stall. Both limits can be overridden explicitly for diagnostic runs, but the
default release gate enforces them.

Three fresh-process runs on the development machine established the initial baseline. The worst p95
was 0.830 ms in `autosave_notice`; the worst individual sample was 178.250 ms in `tower`. Across the
other two runs, worst p95 values were 0.589 and 0.581 ms, while worst individual values were 153.883
and 152.867 ms. These values are local regression evidence, not minimum-system-requirement claims.

The opt-in JSON Lines report is written to the ignored file
`target/release-smoke/performance.jsonl`. Hatchspire creates no report during ordinary play, sends
nothing over the network, and does not treat the output as telemetry.

## What this does not prove

The timer stops before `next_frame`, so it excludes GPU command execution, presentation, display
sync, driver latency, PNG export, and waiting between frames. Thirty deterministic frames per scene
do not measure sustained pacing, process memory, GPU memory, resource growth, input latency, OS
events, audio, or thermal behavior. The required 2–4 hour real-time run and tests on clean physical
Windows machines—including a modest-spec machine—remain release-candidate gates.
