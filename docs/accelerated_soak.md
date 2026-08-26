# Hatchspire accelerated long-session soak

Audit date: 27 August 2026

`tests/soak_flow.rs` compresses a long campaign into 240 deterministic daily cycles. Each cycle:

1. recovers until at least one party member is battle-ready, with a four-day deadlock guard;
2. starts a Balanced tower run and a floor-1 encounter;
3. resolves combat through attack, repeated flee attempts, or a forced defensive defeat pattern;
4. returns victories from the tower or verifies defeat/flee recovery reached town;
5. periodically scavenges and greets Mara, then advances the day;
6. atomically writes the full save through `macroquad-toolkit`, retaining the prior backup; and
7. reloads from the native slot to simulate a restart boundary.

The gate also proves combat and tower transient state are cleared at every save point, the monster
roster survives, the activity log remains capped at 80 entries, a backup exists, and the final
serialized state remains below 1 MB. Its unique temporary native save namespace is cleaned even
when an assertion unwinds.

Current deterministic baseline: 240 cycles, 180 victories, 60 recovery returns, day 271, and an
11,064-byte final serialized state.

Run it with:

```powershell
cargo test --test soak_flow -- --nocapture
```

## What this does not prove

This is an engine/persistence stress gate, not the required 2–4 hour real-time release soak. It does
not exercise rendering, frame pacing, GPU memory, OS window events, Alt+Tab, resize/fullscreen,
process termination, launcher/security behavior, audio, or physical hardware. Those remain manual
release-candidate gates, as do the scope-dependent tutorial and floor-3 finale path.
