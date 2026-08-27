# Hatchspire release-capture CPU and memory probe

Audit date: 27 August 2026

The exact packaged optimized Windows executable records a local performance sample while
`scripts/release_smoke.ps1` renders its 19 deterministic scenes. Each scene contributes 30 frames
at 1280x720. The sample contains the average, 95th-percentile, and maximum CPU time spent inside
Hatchspire's `update` and `draw` calls.

The smoke gate requires every scene's p95 update+draw time to be at most 16.667 ms, a provisional
60 Hz regression budget. The limit can be overridden explicitly for diagnostic runs, but the
default release gate enforces it.

Five fresh-process runs of the original 16-scene set established the initial baseline. Their worst
p95 was 0.830 ms in `autosave_notice`; the other values were 0.589, 0.581, 0.589, and 0.523 ms. The
first expanded 18-scene run had a 0.715 ms worst p95 in `save_migration_notice`.
Maximum samples are retained for diagnosis but are not a release threshold: one run's first-use
`tower` frame reached 1,260.245 ms while that scene's p95 was 0.556 ms. Lazy glyph/texture work and
host scheduling make a single cold-frame cap too noisy for this short probe. These values are local
regression evidence, not minimum-system-requirement claims.

The opt-in JSON Lines report is written to the ignored file
`target/release-smoke/performance.jsonl`. Hatchspire creates no report during ordinary play, sends
nothing over the network, and does not treat the output as telemetry.

## Capture-process memory

The shared capture harness samples the exact executable's resident working set every 25 ms while
each resolution batch is alive. It writes a local JSON report containing sample count, first,
median, p95, final, and largest sampled working sets plus Windows' lifetime peak. Release smoke
requires a nonzero, internally ordered report for each of the 1280×720, 1366×768, 1920×1080
fullscreen, and 960×540 processes.

The reports live at `target/release-smoke/memory_<resolution>.json`. The command summary separates
the worst p95/final samples from transient sampled/OS maxima. A ceiling can be supplied with
`-MaxSampledWorkingSetMb`, but there is deliberately no default memory threshold yet. Identical
short capture runs produced roughly 260–295 MB maxima normally while cold peaks varied above 1 GB;
one later fullscreen run reached about 1.61 GB. A fixed 512 MB limit was therefore rejected as
flaky, while a ceiling high enough to admit every observation would not be a meaningful release
claim. Distribution evidence now makes transient and sustained behavior distinguishable, but the
values remain diagnostics until a real-time ordinary-play baseline supports a defensible threshold.

## What this does not prove

The CPU timer stops before `next_frame`, so it excludes GPU command execution, presentation,
display sync, driver latency, PNG export, and waiting between frames. The process-memory sample
includes screenshot readback/export overhead and sequential fixture loading, so it is not a normal-
play memory profile. Thirty deterministic frames per scene do not measure sustained pacing, GPU
memory, resource growth, input latency, OS events, audio, or thermal behavior. These are short
regression diagnostics, not a substitute for the required 2–4 hour real-time run and tests on clean
physical Windows machines, including a modest-spec machine.
