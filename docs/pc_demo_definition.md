# Hatchspire Windows Demo — Working Definition

Status: **PROPOSED — owner approval required before progression is locked**  
Plan source: `AI_AGENT_PC_DEMO_RELEASE_PLAN.md`  
Last updated: 27 August 2026

## One-page promise

Hatchspire's first public build is proposed as a free Windows 10/11 x64 demo for players who enjoy
monster raising, town recovery, and deliberate dungeon expeditions. Its purpose is general discovery
and structured public feedback, not a content-complete preview of all ten tower floors.

A first playthrough should take 60–120 minutes. The player begins beside the ruined tower with Pip,
restores the first useful camp facilities, prepares and completes several Mossy Ruins expeditions,
finds and hatches at least one egg, makes at least one recovery decision, and improves at least one
facility. The playable route ends after an authored floor-3 finale, followed by results, thanks,
feedback instructions, and a safe return to the title screen.

Every required action must have a visible mouse/touch target. Keyboard shortcuts may supplement
those controls. The release is a portable ZIP containing a self-contained Windows executable and
player-facing support, version, credits, notices, privacy, and known-issues documents.

## Proposed decisions

These defaults are not approved merely because they are written here. The owner should replace any
choice that is wrong, then fill in the approval record below.

| Decision | Proposed choice |
| --- | --- |
| Audience | PC players interested in approachable monster raising and expedition planning |
| Purpose | Free public-feedback demo for general discovery |
| Playtime | 60–120 minutes for a first playthrough |
| Content boundary | Opening town plus Mossy Ruins floors 1–3 |
| Endpoint | Authored floor-3 finale; floors 4–10 remain available only in development builds |
| Post-endpoint behavior | Lock further descent; allow results review and safe return to title |
| Save compatibility | Make no promise that demo saves migrate to the full game |
| Required systems | Pip, camp action, facility building, party preparation, expeditions, combat, retreat/recovery, one egg hatch, one facility improvement |
| Deferred systems | Breeding mastery, all floors after 3, online features, additional platforms, final-game balance |
| Emotional promise | Pip and the first hatchling make the damaged camp feel worth rebuilding; the tower feels dangerous but learnable |

## Approval record

Scope-sensitive implementation is authorized only when every row has a named owner and date.

| Decision | Approved choice | Date | Owner |
| --- | --- | --- | --- |
| Audience and purpose |  |  |  |
| Target playtime |  |  |  |
| Demo endpoint and post-endpoint behavior |  |  |  |
| Required raising/building systems |  |  |  |
| Save compatibility promise |  |  |  |
| Emotional promise and content tone |  |  |  |

## Numbered critical path

This is the route the tutorial, deterministic test fixture, and release capture set must prove.

1. Start **New Game** and meet Pip at the damaged camp.
2. Follow a visible first objective to perform one camp resource action.
3. Build or enter the first required facility through a visible control.
4. Open **Tower Prep**, review Pip's party slot, choose an expedition goal, and enter floor 1.
5. Use visible map routing and **EXPLORE**; encounter and complete the first combat.
6. Use **SURVEY** or **CAMP** in context, then return with resources using **RETREAT**.
7. Spend expedition resources on the required facility improvement.
8. Find an egg, return safely, care for it in the Hatchery, advance days, and hatch it.
9. Use the Stable to make an explicit party or recovery decision involving Pip or the hatchling.
10. Complete enough expeditions to unlock floors 2 and 3 without an unrecoverable resource deadlock.
11. Enter floor 3, receive a clear finale objective, and defeat or otherwise resolve the authored finale.
12. View results, thanks, and feedback instructions; return safely to the title screen.
13. Load the completed save and remain at the demo boundary without bypassing it or losing the completion record.

## Current implementation gaps on that path

- New Game enters town without onboarding or a visible first objective.
- The existing floor data unlocks floor 4 immediately after floor 3; there is no demo boundary.
- Floor 3 has no guardian or authored finale state.
- Tutorial completion, demo completion, and post-finale results are not represented in save data.
- Change-aware autosave, New Game confirmation, separately confirmed reset, native save-location
  help, visible historical-save migration, and one-step rollback are implemented.
- Settings persist audio groups, mute, fullscreen, reduced motion, and whole-canvas windowed UI
  scale independently; approved audio playback remains.
- A self-identifying internal Windows preview package can be generated with player/support/legal
  drafts, exact build identity, per-file hashes, an external archive checksum, the exact Windows
  dependency list, and 120 available registry-crate license/notice texts. Public approval remains.
- The 37 embedded project inputs have a regression-tested provenance ledger; all visual approvals
  remain pending.
- A 240-cycle accelerated soak covers town, tower, combat, recovery, backup-saving, and native
  reload boundaries. Real-time rendering, OS-event, and physical-device soak testing remains.
- The exact packaged release-profile EXE byte-matches the build and boots/renders all 18 seeded
  scenes in 1280×720, 1366×768, and 960×540 windows plus 1920×1080 fullscreen. Its opt-in 30-frame-per-scene CPU report
  enforces a provisional 16.667 ms p95 update+draw budget and reports diagnostic maxima; GPU,
  memory, sustained pacing, interaction,
  visual correctness, and clean-device testing remain.
- The same EXE launches from a working path with spaces and Unicode while read-only, without writing
  beside itself. A write-denying directory ACL and clean physical PCs remain manual gates.
- Help, crash-report guidance, package manifests, and release smoke share a `version+g<commit>` build
  identity; choosing the public demo version remains a human release-candidate decision.
- Catalog metadata is explicitly an internal preview, leads with visible mouse/touch controls, and
  makes no inherited repository or approved public-demo claim. Final store copy remains gated.
- Combat embeds the processed transparent VFX atlas and separates portrait, HP, role/status, and
  stat rows on every unit card. Active, victory, and defeat fixtures name the visible continuation
  path; broader intent, target, damage/status, and small-window readability remain.
- No full-demo deterministic test currently spans the numbered path above.

## Feature freeze

Until this demo reaches release-ready status, do not expand:

- floors 4–10 or their final balance;
- breeding beyond what is necessary to keep existing development content functional;
- new monsters, buildings, contracts, anomalies, hazards, or tower events;
- Steam, WebGL release, Linux, macOS, accounts, cloud saves, analytics, achievements,
  leaderboards, multiplayer, localization, voice acting, or automatic updating;
- engine replacement, large architecture rewrites, or non-release-facing tooling.

Development builds may retain the existing ten floors. Public-demo behavior must be selected by an
explicit build/release configuration and covered by tests; content should not be deleted merely to
hide it from the demo.

## Success metrics

The technical candidate passes when:

- a fresh save has exactly one obvious route through all 13 critical-path steps;
- every required action is clickable and every tutorial prompt names its visible control or gesture;
- deterministic tests prove completion, boundary enforcement, replay/load behavior, and recovery
  from one failed expedition;
- the intended path contains at least one hatch, recovery decision, facility improvement, and
  several expeditions;
- no blocker-class issue remains in the release path.

The public candidate additionally requires human evidence:

- at least five independent testers play the build;
- at least four reach the first completed expedition without live coaching;
- at least three complete the demo without live coaching;
- the approved playtime range holds across at least two tester skill bands;
- the exact ZIP passes on two clean physical Windows machines, including one modest-spec machine;
- the rights holder approves every shipped asset, store claim, and release action.

## Explicit exclusions

The demo does not promise the ten-floor campaign, final balancing, save migration into a later full
release, platforms other than Windows, online services, telemetry, achievements, localization,
voice acting, an installer, code signing, or a full-release date.
