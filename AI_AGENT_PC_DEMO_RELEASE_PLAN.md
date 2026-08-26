# Hatchspire Windows Demo — AI Agent Release Plan

Audit date: 26 August 2026

## Implementation status

- **Current phase:** Milestone 1 awaits scope approval; independent Milestone 4 settings and
  Milestone 5 save-shell work are in progress.
- **Scope state:** Awaiting human approval; no public-demo progression lock has been applied.
- **Working specification:** [`docs/pc_demo_definition.md`](docs/pc_demo_definition.md)
- **Release gate:** [`docs/pc_demo_release_checklist.md`](docs/pc_demo_release_checklist.md)
- **Next implementation step after approval:** encode the floor-3 finale boundary and its deterministic full-path tests.
- **Completed release slices:** proposed demo definition/release gate; visible New Game overwrite
  warning; unreadable-save quarantine and unsupported-newer-save recovery; change-aware atomic
  autosave with visible success/failure status; separately confirmed save reset; in-game build,
  save-location, and recovery help; independently persisted audio/display/motion preferences.
  Windowed UI scale now resizes the complete canvas across four tested sizes without cropping;
  supported historical saves normalize and rewrite at the current version. Each successful
  replacement save now retains one restorable previous state without discarding the file it replaces;
  native panics also append a local, manually shareable crash log beside the save. A post-publish
  packager now produces a self-identifying internal Windows preview with player/support/legal
  drafts, an exact checksum, and 120 available dependency license/notice texts. A regression gate
  covers all 37 embedded project inputs.
  A 240-cycle accelerated soak now stresses town, tower, combat, recovery, backup, and native reload.
  The exact packaged optimized EXE now passes a 16-scene boot/render smoke gate at 1280×720,
  1366×768, 1920×1080 fullscreen, and a deliberately small 960×540 window.
  That gate also records opt-in local update+draw CPU samples, enforces a provisional 16.667 ms p95
  budget, and reports diagnostic maxima; GPU, memory, and sustained pacing remain unmeasured.
  It also launches read-only from a working path containing spaces and Unicode without sidecar writes.
  Help, package manifests, and release smoke now share an exact `version+g<commit>` build identity.
  Catalog metadata now labels the build as an internal preview, leads with visible controls, and
  carries no inherited repository link or approved public-demo claim.

The working specification deliberately separates approved facts from proposed defaults. Updating
its decision record is the gate that authorizes scope-sensitive gameplay changes.

## Scope used for this plan

This plan treats the first public release as a **Windows 10/11 x64 downloadable demo on itch.io**. It excludes WebGL, Linux, macOS, Steam, achievements, cloud saves, online services, localization, and the content-complete 1.0 release.

The demo should be a curated slice of the existing game, not all ten floors presented as finished. The recommended promise is:

- 60–120 minutes for a first playthrough;
- the opening town, raising loop, and floors 1–3 of Mossy Ruins;
- at least one hatch, one recovery decision, one facility improvement, and several expeditions;
- a deliberately authored demo finale and a clear “continue in the full game” boundary;
- mouse and keyboard support, with every essential action still clickable;
- a portable `.zip` containing a self-contained Windows executable.

The human product owner must approve or replace that promise before the agent locks progression or writes store copy.

## Estimate verdict

**Three to six months is a correct, defensible calendar estimate for a polished public demo if one human owner is working part-time with an AI implementation agent. Four months is the planning target.** It is too long for a private prototype and should not be confused with the workspace’s existing 5–8 month estimate, which is for a full 1.0 made by a full-time team of four.

| Scenario | Calendar time | Conditions |
| --- | ---: | --- |
| Aggressive | 8–10 weeks | Scope freezes immediately; the human answers decisions within one day; approved audio/art is available; at least five outside testers participate in two rounds. |
| Recommended | 12–16 weeks | One part-time human owner; two or three playtest rounds; one meaningful UI/audio revision; no Steam work. |
| Conservative | 20–24 weeks | Slow human approvals, substantial art/audio curation, weak tester availability, or balance/onboarding needs more than three rounds. |

The schedule is driven by feedback cycles rather than code volume. An agent can implement and verify changes quickly, but it cannot manufacture independent player understanding or approve the game on the owner’s behalf.

## Evidence from the repository

### Already strong

- The complete town-building → monster-raising → tower → recovery loop exists.
- The implementation contains 20,065 physical lines across 116 Rust files; every Rust file is below the 800-line project limit.
- The automated suite currently passes 118 tests: 104 unit tests, 9 end-to-end game-flow tests,
  one asset-registry test, one release-inventory test, one release-metadata test, one
  accelerated-soak test, and one code-standards test.
- Formatting and strict lint checks pass.
- The Windows release already packages successfully as a self-contained 51.3 MB `hatchspire_windows.zip` containing `hatchspire.exe`.
- The exact packaged EXE byte-matches the optimized release build and renders all 16 seeded scenes
  at 1280×720 with valid, nontrivial PNG output.
- GitHub CI checks formatting, linting, tests, WebGL compilation, and a Windows release build.
- Save data is versioned and stored under the user’s application-data area through `macroquad-toolkit`.
- Deterministic combat replay, data validation, seeded captures, and verification screenshots provide unusually good foundations for regression work.
- There is substantial authored content: ten floors, 80 enemies, 22 special locations, hazards, anomalies, contracts, secrets, bosses, raising, breeding, jobs, facilities, and persistent discoveries.

### Release blockers or risks found

- There is no first-time tutorial or guided opening. New Game drops directly into the town.
- Defeating the floor-10 guardian does not produce an ending; the player only reaches a message that the stairs end. A shorter demo also has no authored finish line.
- The current combat verification capture visibly contains magenta/chroma sprite backgrounds, overlapping statistics, tiny text, and a presentation drop from the polished title/tower screens.
- Audio assets are absent. Settings now persist master/music/SFX levels, mute, fullscreen, and
  reduced motion independently from saves. Windowed UI scale preserves the full fixed canvas at
  90%, 100%, 110%, and 125%. Application of volume groups to approved audio remains.
- Save/load remains a single slot. The initial audit found no autosave or recovery shell; New Game
  confirmation, change-aware autosave, corrupt-save quarantine, and newer-save protection are now
  implemented, together with a separately confirmed save reset and native save-location help.
  One-step backup restore is also available from the recovery screen and preserves the displaced file.
- Draft credits, third-party notices, privacy, support, player readme, and known-issues files now
  exist. The exact Windows dependency inventory bundles 120 available registry-crate license/notice
  texts. No project/toolkit license has been chosen; `gilrs 0.10.10`, `gilrs-core 0.5.15`, and
  `quad-rand 0.2.3` ship no top-level license file despite declaring Apache/MIT or MIT; required
  notice selection is unreviewed; and human approval remains.
- A regression-tested ledger now identifies the 24 visual and 13 JSON inputs embedded in the
  executable, distinguishing them from the much larger experimental asset tree. Every visual row
  still lacks sufficient provenance and rights-holder approval for public distribution.
- `publish-itch.ps1` exists, but the required `itch.json` does not. The publisher cannot target an itch.io project yet.
- `game_page.json` now labels the build as an internal preview, describes the Windows target and
  development-only browser build, leads with visible mouse/touch controls, and omits the inherited
  `monstron_rs` link. Final approved public-demo copy and repository choice remain human gates.
- The ordinary publisher still creates an executable-only base ZIP. The post-publish preview
  packager replaces it with a verified internal archive containing the executable, build identity,
  readme, support, known issues, credits, privacy, dependency inventory, available license texts,
  and notice drafts, plus external manifest and SHA-256 sidecars. A deploy-only follow-up has been
  verified to synchronize that exact archive to local preview. Final approved content remains gated.
- Cargo remains at generic version `0.1.0`, but Help, manifests, and smoke output now append the
  exact Git revision and mark dirty builds. Assigning the approved demo version remains a
  release-candidate task.
- Native panics now append a local crash log beside the save. Windows icon/version metadata,
  clean-machine launch tests, a write-denying launch-directory ACL test, antivirus/SmartScreen
  records, gamepad support, GPU/memory profiling, and a 2–4 hour real-time render/device soak remain
  absent. The exact EXE does pass a read-only spaces/Unicode relocation test and a provisional
  16.667 ms p95 update+draw CPU gate across all release-capture scenes, while a deterministic
  240-cycle engine/native-persistence soak covers 180 victories and 60 recovery returns without deadlock.
- Current automated flows prove systems in isolation and in short chains, not that a new player can understand, enjoy, and finish the demo.

## Agent-owned work

The agent can complete everything in this section once the human supplies the decisions and rights approvals called out in `HUMAN_PC_DEMO_RELEASE_CHECKLIST.md`.

### 1. Turn the approved promise into an executable release gate

- Write a one-page demo definition: audience, playtime, included systems, endpoint, excluded systems, and success metrics.
- Convert it into a numbered critical path from New Game to the demo finale.
- Add a feature freeze list. Hide or clearly label anything outside the demo promise instead of expanding it.
- Add a release checklist with blocking, important, and post-demo issue classes.
- Keep the existing ten-floor content available to development builds if useful, while making the public demo boundary explicit and testable.

Exit criteria:

- A fresh save has one obvious route through every promised system.
- The demo has a deliberate finale, results/thank-you screen, feedback link or instructions, and a safe return to title.
- Automated tests prove the endpoint cannot be bypassed accidentally and old saves fail or migrate clearly.

Estimated agent effort: 3–6 working days after the scope decision.

### 2. Build first-session onboarding

- Add a short, stateful tutorial that teaches the visible controls in context.
- Teach the first camp action, facility entry, party preparation, map movement, Explore/Survey/Camp/Retreat, combat actions, rewards, recovery, saving, and the demo goal.
- Make prompts name the exact visible button or direct gesture.
- Add skip/replay tutorial controls and persist completion.
- Prevent overlays from blocking their own required target.
- Add deterministic tutorial tests and capture every tutorial step at 1280×720 plus one smaller supported window.

Exit criteria:

- A player can complete the demo without reading the repository README.
- Every required action is possible by mouse alone.
- Five independent testers can reach the first completed expedition; at least four do so without live coaching. This last measurement must be gathered by humans.

Estimated agent effort: 5–9 working days, plus human testing time.

### 3. Repair visual hierarchy and combat presentation

- Remove magenta/chroma leakage from every shipped sprite and atlas region.
- Fix combat card/stat collisions and make HP, turn ownership, intent, target, damage, status, and victory/defeat states readable.
- Bring combat, town, facilities, and ending screens closer to the quality bar already set by the title and tower screens.
- Audit cropping, scaling, contrast, tooltip placement, button states, and text clipping at supported Windows sizes.
- Add capture scenes for tutorial, demo finale, settings, save warning, victory, defeat, empty states, and error states.
- Replace verification images representing the same states rather than adding duplicates.

Exit criteria:

- No chroma key, placeholder block, clipped label, overlapping text, illegible status, or missing image appears in the release path.
- Screenshot review passes at 1280×720, 1366×768, 1920×1080, and a deliberately small resizable window.

Estimated agent effort: 6–12 working days, depending on asset curation feedback.

### 4. Add a minimum professional audio and settings pass

- Implement master, music, and SFX volume, mute, fullscreen/windowed mode, UI scale, and reduced-motion/screen-shake controls using the shared toolkit where appropriate.
- Persist settings independently of game saves.
- Add UI confirmation sounds, combat feedback, discovery/reward cues, danger/pressure cues, and restrained town/tower ambience or music.
- Normalize loudness and avoid repetitive stacking.
- Ensure the game is fully playable muted.
- Maintain an audio attribution/provenance table.

Exit criteria:

- All volume groups work from 0–100%, settings survive restart, and no sound is painfully loud or endlessly repeated.
- The human owner approves every externally sourced or generated audio asset and its license.

Estimated agent effort: 5–10 working days after approved audio sources exist.

### 5. Harden saves, recovery, and the application shell

- Add autosave at safe progression boundaries and an explicit saved indicator.
- Warn before New Game overwrites an existing slot.
- Add delete/reset save with confirmation and clear wording.
- Use the toolkit’s atomic save, migration, backup, and quarantine facilities where appropriate.
- Show the native save location in a help/support screen.
- Handle corrupt, unsupported-newer, and partially migrated saves without trapping the player.
- Persist and restore window/settings state safely.
- Add version/build information, credits, support details, and a clean quit path.

Exit criteria:

- Force-closing immediately after each major demo action never destroys the last known-good save.
- Corrupt-save tests and manual recovery instructions pass.
- New Game cannot silently erase progress.

Estimated agent effort: 4–7 working days.

### 6. Balance and instrument the demo without adding surveillance

- Create seeded full-demo simulation/replay fixtures and report progression curves.
- Record local, opt-in debug summaries suitable for a tester to attach manually; do not add network telemetry unless separately approved.
- Implemented independently: release capture writes an opt-in local JSON Lines summary of average,
  p95, and maximum optimized update+draw CPU time; it sends nothing over the network.
- Measure time to first expedition, battle duration, damage/strain, retreat rate, resource income/spend, facility timing, hatching timing, and demo completion.
- Add debug shortcuts or seed files excluded from release mode so reported states are reproducible.
- Tune from observed playtest evidence, not only simulation averages.

Exit criteria:

- No required progression can deadlock.
- A new player can recover from one failed expedition without restarting.
- The intended demo route fits the approved duration across at least two tester skill bands.

Estimated agent effort: 4–8 working days across multiple feedback rounds.

### 7. Establish Windows quality gates

- Add release-profile smoke tests and a seeded headless run covering the full demo path.
- Implemented independently: exact-package identity checks and release-profile boot/render smoke
  now cover all 16 registered scenes at four target window sizes. The scope-dependent full-demo
  path and human visual review still await approval.
- Add a 2–4 hour soak scenario that repeatedly enters town, facilities, tower, combat, save/load, and quit/restart.
- Implemented independently: a 240-cycle accelerated soak repeatedly crosses town, tower, combat,
  recovery, atomic backup-save, and native reload boundaries. It does not replace the real-time soak.
- Track frame time and memory at supported resolutions; investigate sustained degradation.
- Implemented independently: all 16 optimized 1280×720 capture scenes enforce a provisional
  16.667 ms p95 update+draw CPU budget and report diagnostic maxima. GPU presentation, memory,
  sustained real-time behavior, and performance at other resolutions remain.
- Test fresh install, paths containing spaces and non-ASCII characters, read-only launch folder, missing/corrupt save, Alt+Tab, resizing, fullscreen toggling, and repeated restart.
- Implemented independently: the exact packaged EXE launches read-only from a spaces/Unicode working
  path and writes no sidecars there. Clean-install, write-denying ACL, and OS-event tests remain.
- Verify on integrated graphics or an equivalent low-spec machine when the human provides access.
- Add a release manifest containing version, commit, build date, SHA-256, archive size, and included files.
- Add Windows application icon/version metadata if the chosen packaging path supports it cleanly.

Exit criteria:

- Zero release-path crashes or progress-loss defects remain.
- No known blocker or high-severity issue is open.
- The exact release ZIP passes on at least two physical Windows machines owned by different people.

Estimated agent effort: 5–9 working days, interleaved with human device testing.

### 8. Produce legal/support artifacts for human approval

- Inventory every asset actually compiled into the Windows executable.
- Record creator/source, creation method, license or ownership basis, required attribution, modification, and human approval for each shipped asset.
- Draft `CREDITS.md`, `THIRD_PARTY_NOTICES.md`, end-user readme, known issues, support instructions, and a privacy statement stating that no telemetry is collected if that remains true.
- Audit dependency licenses and flag incompatible, unclear, or missing terms.
- Remove public claims the evidence does not support.

Exit criteria:

- Every shipped asset and dependency has a documented permission basis.
- The human rights holder explicitly signs off; an agent’s audit is not legal approval.

Estimated agent effort: 3–6 working days after provenance information is available.

### 9. Prepare the itch.io Windows release

- Create `itch.json` after the human supplies the exact owner/project slug.
- Keep the channel Windows-only and exclude HTML5 publishing from the release procedure.
- Update platform, controls, repository, demo status, version, and support metadata.
- Package a clean Windows ZIP with the executable and approved player-facing documents.
- Run the project publisher with no parameters, then the itch publisher in dry-run/preview mode.
- Draft the itch page description, short description, tags, system requirements, installation instructions, content notes, known issues, release notes, FAQ, and update post.
- Prepare approved screenshots, cover art, thumbnail, and optional short trailer assets.
- Generate checksums and archive the exact release candidate.

Itch.io’s official Butler documentation confirms that it can push a ZIP to a named channel, but the itch project page must already exist and Butler must be authenticated. Those account actions belong to the human. Sources: [Pushing builds](https://itch.io/docs/butler/pushing.html), [Butler authentication](https://itch.io/docs/butler/login.html), and [project access modes](https://itch.io/docs/creators/access-control.amp).

Exit criteria:

- The human downloads the processed build from a restricted or unlisted page on a clean machine and completes the demo.
- The public release candidate is byte-for-byte identified by its recorded checksum.

Estimated agent effort: 3–5 working days, excluding account setup and human approval.

### 10. Support the launch and first patch

- Triage incoming reports into reproducible defects, usability findings, balance feedback, and requests.
- Prepare minimal patches with regression tests and updated release notes.
- Preserve save compatibility within the demo line or warn explicitly before breaking it.
- Maintain a known-issues list and a rollback-ready previous build.
- Produce a seven-day and thirty-day review for the human owner.

Exit criteria:

- A rollback build and patch procedure have both been rehearsed before launch.
- The human decides which reports change scope and when to publish each patch.

## Recommended calendar

| Weeks | Agent focus | Required human input |
| --- | --- | --- |
| 1–2 | Freeze promise, create demo endpoint, write critical-path tests, inventory shipped assets | Approve audience, length, endpoint, platform, price, and rights direction |
| 3–5 | Onboarding, combat/readability fixes, settings, first audio integration | Weekly build playthrough and fast visual/audio decisions |
| 6–8 | Save hardening, accessibility minimum, balance harness, full-path capture | Recruit and observe first outside test round |
| 9–11 | Fix findings, second balance/onboarding pass, soak/performance work | Second outside test round on multiple PCs |
| 12–13 | Store assets/copy, legal/support drafts, restricted itch build | Approve copy/assets/rights; configure itch account and page |
| 14–15 | Release-candidate fixes, clean-machine tests, checksum and rollback rehearsal | Final go/no-go playthrough and visibility decision |
| 16 | Buffer or public release; hold scope for urgent fixes only | Publish, communicate, and handle community decisions |

Do not schedule a public date before the first independent playtest round. A target month is safe; a target day is premature.

## Definition of release-ready

All items below must be true:

- The approved critical path is complete and ends deliberately.
- Five or more people outside development have played; at least three complete the demo without live coaching.
- The exact ZIP passes on two clean Windows machines and one modest-spec machine.
- All automated checks, the soak run, capture review, save recovery, and packaging validation pass.
- No known crash, progress loss, blocker, illegible required UI, missing asset, chroma leak, or unlicensed asset remains.
- Settings, save behavior, version, support path, credits, and known limitations are visible to players.
- The itch page, Windows channel, pricing, access level, and download tags are correct.
- The human owner has approved the build, store page, rights ledger, and public release action.

## Explicitly outside the first demo

Unless the human changes the scope, defer these:

- Steamworks integration and Steam store review;
- installers, automatic updating, and code-signing expenditure;
- WebGL, macOS, Linux, Steam Deck certification, or console work;
- cloud saves, accounts, analytics, crash-upload services, achievements, leaderboards, multiplayer, and workshop support;
- full localization, voice acting, and a commissioned orchestral soundtrack;
- guaranteeing all ten floors are balanced as final-release content;
- major engine replacement or broad architectural refactoring.
