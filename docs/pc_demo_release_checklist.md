# Hatchspire Windows Demo — Release Gate

Status: active working checklist  
Specification: `docs/pc_demo_definition.md`

## Issue classes

### Blocking

A blocking issue prevents a release candidate. This includes a crash, hang, progress loss,
critical-path deadlock, accidental demo-boundary bypass, required action without a visible control,
illegible required UI, missing or chroma-leaking shipped art, unsupported/corrupt save trap, missing
license basis, wrong executable/package, or materially false store claim.

### Important

An important issue should be fixed for the demo unless the owner explicitly accepts and records it.
Examples include confusing optional UI, weak feedback, inconsistent presentation, balance outside
the approved range, minor input friction, avoidable frame-time spikes, or incomplete support text.

### Post-demo

A post-demo issue does not prevent the approved experience and is outside the frozen promise.
Examples include additional floors, content variety, new systems, extra platforms, localization,
online services, and improvements that do not affect demo comprehension, safety, or truthfulness.

Severity is based on player impact and the approved promise, not implementation difficulty.

## Blocking gates

- [ ] Owner approval record in `docs/pc_demo_definition.md` is complete.
- [ ] The 13-step critical path is implemented and has one obvious fresh-save route.
- [ ] A deliberate floor-3 finale, results/thanks screen, feedback instructions, and title return exist.
- [ ] Release builds enforce the demo boundary; development builds may retain floors 4–10.
- [ ] Tutorial, completion, and boundary state survive save/load or fail with a clear migration path.
- [ ] Every critical action has a visible mouse/touch target with precise tutorial wording.
- [ ] One failed expedition is recoverable without restarting the game.
- [x] State-changing actions autosave atomically and expose visible success or failure status.
- [x] All ten mutating screen routes pass through autosave; eight major completed-action boundaries survive native save/restart cycles.
- [x] New Game presents a visible, clickable overwrite warning when progress exists.
- [x] Save reset has a separate confirmation, clears the active session, and states its consequences.
- [x] Unreadable saves can be retried, preserved through quarantine, or left unchanged.
- [x] Unsupported-newer saves are identified and left unchanged for a newer game build.
- [x] Help & Support displays the running version, native save path, and recovery/report guidance.
- [x] The main menu provides a visible native Exit Game control.
- [x] Help and package manifests identify the exact Git build, including dirty working builds.
- [x] The packaged EXE carries verified Windows product, filename, internal-name, version, and custom-icon resources.
- [x] Master/music/SFX values, mute, fullscreen, and reduced motion persist outside game saves.
- [x] Windowed UI scale persists and preserves the complete canvas at 90%, 100%, 110%, and 125%.
- [x] Supported historical saves normalize and rewrite fully; unreadable partial saves enter recovery.
- [x] Each successful replacement save retains one rollback point; recovery preserves the displaced file before restoring it.
- [x] Native panics append a local crash log beside the save and Help explains how to report it manually.
- [ ] No crash, hang, progress loss, chroma leak, overlap, clipping, or missing asset exists on the route.
- [ ] Master/music/SFX volume, mute, fullscreen/windowed, UI scale, and reduced motion persist.
- [ ] Full-demo deterministic run, release smoke test, and soak gate pass.
- [ ] Exact release ZIP passes on two clean physical Windows PCs and one modest-spec machine.
- [ ] Every shipped asset and dependency has an approved rights basis and required attribution.
- [x] Internal ZIP includes the versioned executable, readme, support, known issues, credits, notices, and privacy text; human document approval remains separate.
- [x] Release smoke verifies manifest version, commit, UTC build date, every file hash, archive size, and SHA-256 against the exact ZIP.
- [x] Internal preview packager adds all draft documents, per-file hashes, build identity, and exact archive checksum.
- [x] Packaged support/privacy drafts disclose local tester counters/export; known issues no longer claim resolved combat defects.
- [x] Package audit lists the exact Windows dependency graph and bundles 120 available license/notice texts.
- [x] Missing project/toolkit license metadata and absent `gilrs`, `gilrs-core`, and `quad-rand` license files remain explicit blockers.
- [x] A regression-tested ledger identifies all 38 embedded project art/data/resource inputs.
- [x] A 240-cycle accelerated soak covers repeated town/tower/combat/recovery and native save reloads.
- [x] The packaged optimized EXE byte-matches the release build and renders all 19 seeded scenes at 1280×720.
- [x] Automated structural capture passes for all 19 scenes at 1366×768, 1920×1080, and 960×540.
- [x] Combat VFX uses the processed transparent atlas; unit-card portrait, HP, role, and stat rows do not overlap.
- [x] Deterministic combat captures cover active, victory, and defeat states with visible continuation wording.
- [x] Combat capture names rules-backed automatic Attack/Skill targets, the next enemy intent/target, damage, and status effects.
- [x] Release capture enforces a local, opt-in 16.667 ms p95 update+draw CPU budget across all 19 scenes.
- [x] Exact-package capture records nonzero sampled and OS-peak working-set diagnostics at all four resolutions.
- [x] The identical EXE launches read-only from a spaces/Unicode working path without sidecar writes.
- [x] The identical EXE completes five consecutive rendered starts/exits from that relocated path.
- [x] Internal catalog metadata removes the inherited repository link and unsupported public platform claim.
- [x] A deploy-only follow-up makes the local preview download byte-match the sealed archive checksum.
- [ ] Restricted itch download matches the recorded checksum and completes on a clean machine.
- [ ] No blocking or unaccepted important issue remains open.
- [ ] Human owner records GO and explicitly authorizes the public action.

## Important gates

- [ ] Capture review passes at 1280×720, 1366×768, 1920×1080, and a small resizable window.
- [x] Combat clearly communicates HP, turn, intent, target, damage, status, and outcome across the four-size package matrix.
- [ ] Audio feedback is restrained, normalized, attributable, and the game remains clear while muted.
- [x] Local opt-in tester summary reports saved pacing and balance measures without telemetry.
- [ ] Real-time frame pacing and memory checks show no sustained degradation during the 2–4 hour soak run.
- [ ] Alt+Tab, resize, fullscreen toggle, restart, non-ASCII path, spaces, and read-only launch folder pass.
- [ ] Antivirus/SmartScreen behavior and unsigned-build decision are recorded.
- [ ] At least five independent testers participate; required coaching/completion targets are met.
- [ ] Store copy, screenshots, system requirements, controls, limitations, and support path are approved.
- [x] Local rollback and first-patch switching are rehearsed with preserved exact builds; restricted-channel upload/download remains human-owned.

## Post-demo backlog

- [ ] Balance and finish floors 4–10 for the full release.
- [ ] Revisit breeding depth after the first demo evidence.
- [ ] Evaluate Steam and other platforms separately.
- [ ] Evaluate localization, accounts, cloud saves, analytics, achievements, and other online features.
- [ ] Decide whether later full releases migrate demo saves.

## Candidate record

| Field | Value |
| --- | --- |
| Version |  |
| Git commit |  |
| Build date |  |
| ZIP SHA-256 |  |
| ZIP size |  |
| Blocking issues open |  |
| Accepted important issues |  |
| Human decision | GO / NO-GO |
| Decision maker and time |  |
