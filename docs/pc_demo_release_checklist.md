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
- [x] New Game presents a visible, clickable overwrite warning when progress exists.
- [x] Save reset has a separate confirmation, clears the active session, and states its consequences.
- [x] Unreadable saves can be retried, preserved through quarantine, or left unchanged.
- [x] Unsupported-newer saves are identified and left unchanged for a newer game build.
- [x] Help & Support displays the running version, native save path, and recovery/report guidance.
- [x] Master/music/SFX values, mute, fullscreen, and reduced motion persist outside game saves.
- [x] Windowed UI scale persists and preserves the complete canvas at 90%, 100%, 110%, and 125%.
- [x] Supported historical saves normalize and rewrite fully; unreadable partial saves enter recovery.
- [x] Each successful replacement save retains one rollback point; recovery preserves the displaced file before restoring it.
- [ ] No crash, hang, progress loss, chroma leak, overlap, clipping, or missing asset exists on the route.
- [ ] Master/music/SFX volume, mute, fullscreen/windowed, UI scale, and reduced motion persist.
- [ ] Full-demo deterministic run, release smoke test, and soak gate pass.
- [ ] Exact release ZIP passes on two clean physical Windows PCs and one modest-spec machine.
- [ ] Every shipped asset and dependency has an approved rights basis and required attribution.
- [ ] ZIP includes versioned executable, readme, support, known issues, credits, notices, and privacy text.
- [ ] Release manifest records version, commit, build date, file list, archive size, and SHA-256.
- [ ] Restricted itch download matches the recorded checksum and completes on a clean machine.
- [ ] No blocking or unaccepted important issue remains open.
- [ ] Human owner records GO and explicitly authorizes the public action.

## Important gates

- [ ] Capture review passes at 1280×720, 1366×768, 1920×1080, and a small resizable window.
- [ ] Combat clearly communicates HP, turn, intent, target, damage, status, and outcome.
- [ ] Audio feedback is restrained, normalized, attributable, and the game remains clear while muted.
- [ ] Local opt-in tester summary reports the agreed pacing and balance measures without telemetry.
- [ ] Frame-time and memory checks show no sustained degradation during the soak run.
- [ ] Alt+Tab, resize, fullscreen toggle, restart, non-ASCII path, spaces, and read-only launch folder pass.
- [ ] Antivirus/SmartScreen behavior and unsigned-build decision are recorded.
- [ ] At least five independent testers participate; required coaching/completion targets are met.
- [ ] Store copy, screenshots, system requirements, controls, limitations, and support path are approved.
- [ ] Rollback and first-patch procedures are rehearsed with a preserved previous build.

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
