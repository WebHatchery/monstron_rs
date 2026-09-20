# TODO — Hatchspire

## UI_STYLE review — 2026-09-20

Audit scope: read `AGENTS.md`, `UI_STYLE.md`, `CODE_STANDARDS.md`,
`GAME_DEVELOPMENT_GUIDE.md`, and the README's gameplay/design spine. No separate
GDD or `PROJECT_AGENTS.md` was found. Inspected screen dispatch, UI helpers,
Town/facility/prep/combat/tower rendering and input, map transforms, tutorials,
and the existing capture/review workflow. This is planning only.

Evidence: visually inspected existing 1280×720 captures in `docs/verification/`:
`ui_town.png`, `ui_tower.png`, `ui_tower_long.png`, `ui_combat.png`,
`ui_dungeon_prep_unlocked.png`, `ui_stable_full.png`, `ui_hatchery.png`,
`ui_breeding.png`, `ui_workshop.png`, `ui_tutorial_town.png`, and
`ui_save_recovery.png`. These are historical captures, not fresh renders of
HEAD. For example, Stable now draws level/XP information and its slot button
position differs from the full-roster capture; do not report that capture's
overlap as a verified current defect. No runtime, browser, or physical touch
session was performed. No game code changed, so publish was not run for this
documentation-only audit.

The inspected screens already use Hatchspire-specific content; no literal
template demo copy was verified. The repeated header/list/reference-panel
composition still needs adaptation to each decision (UI-02/04/07/08/09), not
a speculative template-removal task. Preserve existing positives: Camp Menu
already separates save/load from ordinary Town actions, the Field Guide has
visible close/page controls, and tutorials already support skip/replay.

Implementation order: settle the screen briefs, then compose Town, tower, and
combat before visual polish. Establish responsive/input foundations alongside
those layouts before rebuilding the facility screens. Apply the feedback and
onboarding tasks across the resulting screens. Every task below requires
normal and minimum-size review; use 1280×720 and provisional 960×540 until the
viewport contract is settled, plus 1366×768 and 1920×1080 regression checks.
Measure the actual browser canvas, not just the host window or logical canvas.

### Verified findings — code and historical visual evidence as noted

- [ ] **UI-01 — Record decision-led screen briefs and the viewport contract.**
  **Scope:** `README.md`, `src/screens/mod.rs`, `src/main.rs::window_conf`,
  `src/ui/mod.rs::virtual_camera`, `src/settings.rs::window_dimensions`.
  **Observed (code/docs):** the README describes the loop but lacks the seven
  UI_STYLE §1 screen-brief answers and explicit normal/minimum viewport support.
  The game scales a fixed 1280×720 canvas; the release packet tests 960×540 but
  this does not establish browser/touch usability.
  **Change:** document the decision, dominant focus, primary action, supporting
  and deferred information, camera/layout, and touch feedback for Town, prep,
  tower, combat, and facility management. Declare native and embedded-browser
  support, including the treatment of narrow/portrait canvases. Set measurable
  rendered text/target floors before changing layouts.
  **Accept/verify:** each normal-play brief has at most 2–3 strong attention
  regions and keeps utility navigation separate from gameplay decisions.
  Validate the proposed sizes in actual native/browser canvases; use at least
  44×44 CSS-pixel touch targets and 16px essential body text as initial design
  targets, adjusting the composition rather than shrinking below them.

- [ ] **UI-02 — Recompose Town around camp growth and the next useful action.**
  **Scope:** `src/screens/town.rs::{draw,draw_header,draw_town_backdrop}`,
  `town_panels.rs::draw`, `town_layout.rs`, and tutorial target rectangles.
  **Observed (code + `ui_town.png`):** Resources, Buildings, Roster, NPC Services,
  Tower, Camp Actions, Activity Log, and Shop compete as eight bordered panels.
  Landmark art is largely covered; Tower, Sleep, and Scavenge have identical
  weight, and the Shop duplicates the dedicated trading screen.
  **Change:** give most of the screen to an actionable camp view or focused
  facility-selection view with readable landmarks. Replace the permanent
  building-cost table with one selected facility inspector containing Build,
  Upgrade, or Open and exact costs/shortfalls. Move trading into Shop, detailed
  roster into Stable, NPC details into selection, and history behind a visible
  journal control. Keep a compact resource/readiness summary and next objective;
  emphasize Scavenge during foundation shortages, recovery when needed, and
  Tower preparation when ready without blocking other valid actions. Keep Menu
  in a quiet utility area, preserving Save/Load/Title and replay-guide access.
  **Accept/verify:** camp, selected decision, and compact status are the only
  strong regions; art communicates usable destinations rather than hiding
  behind a dashboard. At normal/minimum sizes, tap through new-game gathering,
  Hatchery construction, insufficient-resource upgrade, injured-party sleep,
  late-game facilities, guardian objective, and post-story free expeditions.
  Costs, readiness, recovery, and navigation remain discoverable without hover.

- [ ] **UI-03 — Give tower travel an unobscured map and one contextual inspector.**
  **Scope:** `src/screens/tower.rs::{draw,draw_run_overlay,draw_action_dock}`,
  `tower/context_drawer.rs::draw`, `tower/journal.rs`,
  `tower/map_view.rs::{WORLD,world_transform,world_tap_action,room_rect}`.
  **Observed (code + tower captures):** a full-time context card repeats floor
  identity and exploration prose even without an object; journal, anomaly,
  party rail, oversized Field Guide control, and status banner compete with the
  world. `WORLD` extends beneath side overlays; map input has no general overlay
  exclusion. The transform independently scales X/Y into a fixed rectangle
  and offers no player-relative zoom. `SET` is drawn without a corresponding
  settings action, while an invisible top-right return rectangle is actionable.
  **Change:** collapse history and anomaly explanation into visible disclosures;
  retain pressure, party condition, cargo risk, and relevant anomaly effects in
  one quiet HUD. Show the inspector only for explicit inspection or an immediate
  encounter, with a visible close control; remove its empty-state card. Reduce
  ornamental double borders. Remove `SET` and the invisible return target;
  retain clearly labelled RETREAT as a gameplay risk decision. Recompute the
  usable world area and use a shared rendering/picking transform with useful,
  aspect-preserving framing; enlarge the map into reclaimed space. Ensure UI
  taps are consumed before world routing. Add visible zoom/recenter controls
  only if dense-floor inspection demonstrates they are needed.
  **Accept/verify:** map is the first focus, dock the second, contextual decision
  the third only when needed. Tap edge rooms and redirect routes on floors 1,
  5, and 10 at both sizes; opening/closing Guide, journal, or inspector must not
  route or retreat the party. Check resize/display scaling and any added zoom.
  Verify high pressure, exhausted Survey, sealed stairs, pending events, and
  full cargo without hiding costs, hazards, or the retreat path. The specific
  overlap-induced misrouting behavior still needs interactive reproduction.

- [ ] **UI-04 — Make combatants, current actor, and consequences dominate combat.**
  **Scope:** `src/screens/combat.rs::{draw_formation,draw_combatant,draw_actions,
  draw_tactics,draw_log,draw_rewards}` and combat tutorial copy.
  **Observed (code + `ui_combat.png`):** 30px portraits and 11–12px stat lines
  sit inside small equal cards while a large Battle Log and separate tactics
  panel occupy the right side. All living cards share styling, so the current
  actor is not visually distinguished. Automatic targets and enemy intent
  require looking away from the units; the header only says Party/Enemy turn.
  **Change:** enlarge the encounter/formation, visually identify current actor
  and affected targets with non-color-only cues, and put HP, urgent conditions,
  and enemy intent beside the affected units. Defer ATK/DEF/SPD/MOR to a tap
  inspector. Collapse history to a recent result plus reopenable log. Keep
  valid combat choices equally available, but make the action area prominent;
  show Herbs stock/cost beside Herbs. On outcome, emphasize result and Continue
  with defeat recovery guidance. Do not classify Flee as menu navigation.
  **Accept/verify:** the player can identify who acts, who is affected, and the
  next threat at a glance, with no more than three strong regions. Review a
  full formation, guardian, long intent, Guard/Mark, zero Herbs, victory, defeat,
  and flee at both sizes. Tap unit inspection/dismissal and every command;
  preserve automatic targeting rules and readable rewards/recovery after effects.

- [ ] **UI-05 — Build responsive controls and safe, explicit touch inspection.**
  **Scope:** `src/ui/mod.rs::{virtual_camera,button_clicked,draw_tooltip_target}`,
  `src/settings.rs`, screen layout functions, and tower picking.
  **Observed (code):** 26px NPC/Stable controls and 28px care/job controls become
  19.5/21px high at 960×540; 10–14px information shrinks too. Window UI scale
  changes window dimensions, not information layout. Tooltips depend on hover
  or holding the same action target; there is no distinct tap-to-inspect action.
  Historical Town captures show repeated question-mark badges over controls/art.
  **Change:** implement compact/reflowed layouts using toolkit layout/viewport
  helpers, moving secondary content to selection or separate views before
  shrinking controls. Prefer a toolkit upgrade if a shared capability is absent.
  Replace scattered tooltip badges with a clearly labelled, non-destructive
  inspection/help path and visible dismissal. Retain release activation while
  requiring a matching press origin so a drag ending on another control cannot
  activate it. Ensure map press routing cannot leak through overlays.
  **Accept/verify:** meet UI-01's rendered text/target floors, with spacing for
  adjacent choices. Test native resize, actual embedded-browser canvas, display
  scaling, and touch-only opening, inspection, scrolling, dismissal, and back
  paths. Specifically press one target and release over another, drag off a
  destructive action, and dismiss a modal above a room: no accidental action.
  These are code-derived risks, not an assertion of a completed touch test.

- [ ] **UI-06 — Separate current state from temporary action feedback.**
  **Scope:** `src/game.rs::status_message`, `src/game/actions.rs`,
  `src/game/save_flow.rs`, `src/ui/mod.rs::{draw_status,draw_status_at}`,
  `tower.rs::draw_run_status`, and Town/tower/combat history renderers.
  **Observed (code + captures):** the latest status string has no expiry and is
  rendered every frame, alongside permanent logs; tower entry prose remains a
  wide overlay above the world. General status rendering is a single unbounded
  line, unlike the tower's bounded but truncated text.
  **Change:** distinguish transient success/event notices from persistent
  actionable warnings. Update affected resource/HP/egg/route state in place;
  expire ordinary notices and reclaim their space. Keep full results retrievable
  in a readable history rather than only the last two truncated tower entries.
  Retain save failures and recovery instructions until resolved or acknowledged;
  prevent routine autosave messages from displacing consequential results.
  **Accept/verify:** after notices end, gains, injury, route destination, and
  outcomes remain understandable from current state or history. At both sizes,
  exercise rapid repeated trades, long discoveries, route interruptions,
  combat results, sleep, save failure, and reduced motion; no permanent success
  prose or clipped essential error text. Check audio behavior because snapshots
  currently observe `status_message`.

- [ ] **UI-07 — Focus Hatchery on the selected egg and earned knowledge.**
  **Scope:** `src/screens/hatchery.rs::{draw_egg_inventory,draw_reference,
  care_button_rect,hatch_button_rect}` and egg guidance.
  **Observed (code + `ui_hatchery.png`):** `draw_reference` lists every data egg
  type, including the final guardian egg, regardless of discovery. It consumes
  a full-height panel while a single owned egg and five small controls occupy
  only a narrow row; capacity appears in both header and inventory heading.
  **Change:** replace the permanent catalogue with an explicit discovered-egg
  reference view, leaving unknown entries unnamed until earned. Use the main
  area for readable egg selection and one care/hatching inspector with larger
  art, incubation, today's care, expected effect, and adjacent enabled reasons.
  Keep one capacity summary and a visible Stable recovery route when full;
  distinguish the care choice labelled Stable from the Stable facility.
  **Accept/verify:** first-egg play exposes relevant care choices without late
  game spoilers; ready Hatch is prominent, and Full explains how to make space.
  At both sizes, tap through empty inventory, multiple eggs, already cared,
  ready/full roster, opening Stable, cancelling rehome, and successful hatching.

- [ ] **UI-08 — Make the entire roster usable in Workshop and Breeding.**
  **Scope:** `src/screens/workshop.rs::{handle_input,draw_assignments,
  draw_worker_row,draw_reference}`, `breeding.rs::{visible_pairs,draw_pairs,
  draw_pair_row,draw_grove_status}`, and `stable.rs` roster/party layout.
  **Observed (code):** Workshop rendering and input stop at six monsters;
  Breeding takes the first six monsters and then only six pairs, with no paging.
  Later roster members/valid pairs are inaccessible despite Stable supporting
  twelve. Historical facility captures also show tiny forecast text and large
  permanent reference panels; Stable repeats party members across two views.
  **Change:** provide paged/scrollable full-roster selection with stable IDs.
  In Breeding, select two parents from the full roster and present one readable
  comparison/forecast, exact cost, availability, compatibility, and egg-space
  reason beside Breed. In Workshop, show assignments as a deliberate management
  list and reveal job choices/output for the selected worker; move general job
  prose to help. Keep Stable's useful party comparison and pagination, but
  consolidate repeated facts and expose rehome contextually with confirmation.
  **Accept/verify:** at both sizes, select the twelfth monster for work and as
  either parent, choose a pair outside the current first six combinations, clear
  jobs, compare long forecasts, and recover from full egg capacity. Assignment
  and selection survive paging without acting on a different monster. Review
  current Stable level/XP/condition layout with a fresh full-roster capture before
  fixing any historical overlap; preserve safe rehome cancellation and warnings.

- [ ] **UI-09 — Turn expedition prep into a clear choice and departure flow.**
  **Scope:** `src/screens/placeholder.rs::{draw,draw_run_goal_reference,buttons,
  goal_button_rect,draw_floor_selector}`, prep actions and tutorial targets.
  **Observed (code + prep capture):** descriptions are in a separate list from
  five identically styled goal buttons, which immediately start the expedition.
  Town sits directly below these gameplay choices inside the same panel/action
  grouping. Repeated instructions do not make the immediate departure explicit.
  **Change:** attach each goal's risk/reward description to its selectable row;
  show selection, then a prominent `Enter Tower` action naming floor and goal.
  Retain party readiness/level warning beside departure and explicit previous/
  next floor controls. Move Town to a spatially distinct quiet back control.
  Remove duplicated introductory prose after first use and update Enter-key and
  guide semantics to match the selected goal.
  **Accept/verify:** a player understands goal tradeoffs and whether a tap selects
  or departs; navigation cannot be mistaken for a goal. At both sizes, tap every
  goal, revisit unlocked floors, cancel to Town, enter, and test zero-ready and
  under-level parties without hiding warnings or allowing locked-floor entry.

- [ ] **UI-10 — Align onboarding and action reasons with the new composition.**
  **Scope:** `src/screens/tutorial.rs`, `game_page.json`, Town/Shop affordability,
  `tower.rs::draw_action_dock`, facility action descriptions and help.
  **Observed (code):** introductory/completion guide copy omits the exact next
  control in several steps; the host page still describes focus-then-ROUTE and
  an Exit Game control absent from Camp Menu. Tower Survey is painted like an
  available action even when input disables it; Camp cooldown is tiny text.
  Town build and Shop trade controls do not show affordability in enabled state.
  **Change:** merge the earlier tutorial-copy task here: name `CONTINUE`,
  `SHOW ACTIONS`, `FINISH GUIDE`, or `SHOW THE EGG` where appropriate and update
  host instructions to actual direct room tapping and available menu controls.
  Update tutorial highlights after layouts move, keep Skip and Replay Guide,
  and remove completed/ordinary-play instruction wallpaper. Give disabled
  actions an adjacent short reason and exact shortage/cooldown; preserve engine
  validation and do not require hover or trial-and-error to learn a cost.
  **Accept/verify:** complete and skip/replay both first-expedition and first-egg
  guides by touch at both sizes, including combat and full-roster recovery.
  Check zero-survey/high-pressure, Camp cooldown, and unaffordable build/trade
  states. Prompts identify actual visible controls, never obscure their targets,
  and disappear after completion; all utilities/recovery remain reachable.

### Further inspection — not verified current visual/input defects

- [ ] **UI-11 — Close the current-build visual and interaction evidence gaps.**
  **Scope:** `src/game/capture.rs`, capture scenes in `src/state/playtest.rs`,
  `docs/verification/`, `docs/visual_review_packet.md`, browser host and screen
  input paths. Depends on the relevant implementation tasks above.
  **Gap:** existing screenshots do not prove current-build usability at the
  minimum viewport or real touch behavior. Floor-one art is a special backdrop
  while later floors use generated room rendering; room hit rectangles expand
  and can overlap. These warrant checks, not an invented finding that visible
  paths or selection are wrong. Current dense Stable, landmark modal layering,
  late-game long text, and all save-recovery variants need fresh inspection.
  **Action:** after meaningful game changes run `.\publish.ps1` without
  parameters, then capture supported first-use, normal, dense, selected,
  pressure/failure, and recovery states at the agreed sizes. Replace equivalent
  captures directly in `docs/verification/`; record build identity, actual canvas
  dimensions, attention regions, text/target measurements, and input results.
  Exercise new game → camp → preparation → room route → event/combat → retreat
  → sleep → egg care/hatch → party assignment, plus Guide/history/inspector
  dismissal and save/backup recovery. Compare floor-one and deep-floor visual
  geometry to reachable rooms and tap targets after resize; verify dragging and
  overlays do not trigger underlying controls. Inspect narrow browser behavior
  according to UI-01's declared support policy.
  **Accept:** each changed screen satisfies UI_STYLE §9 with evidence for both
  normal and minimum sizes, including touch-only paths; remaining failures or
  unavailable physical-device checks are explicitly recorded. Do not substitute
  compilation, capture-file existence, or an old human-release packet for this
  review, or claim this agent audit grants the separate human release approval.

## Existing implementation and standards backlog

- [ ] Migrate all tests and test-only helpers from `src/` into `tests/`
  (53 test files, 161 cases). Exercise the public library API, introduce only
  intentional public seams, and remove test module declarations from production
  sources before expanding coverage (CODE_STANDARDS §11.4).
- [ ] Review suites by major feature, especially tower navigation, combat commands,
  and combat support; consolidate related inputs into table-driven cases toward
  five tests per feature. Preserve distinct regressions and explain suites that
  need more than five cases (§11.3).
- [ ] Remove `parse_json<T>` from `src/data/loader.rs`; call the toolkit's labeled
  embedded loader or `include_json!` directly, retaining project schemas,
  semantic validation, and source-specific errors (§5.3).
- [ ] Move remaining gameplay balance into typed JSON, including scavenging rewards
  in `town_engine.rs` and fatigue/recovery/bond values in `monster_engine.rs`.
  Validate the new fields and derive reward messages from loaded values (§5.3).
- [ ] Externalize player-facing labels, tutorial copy, and action/result messages
  from `src/screens/`, `src/game/actions.rs`, and engines into JSON under `assets/`;
  load through the toolkit and validate required text keys (§5.3).
- [ ] Split functions exceeding 100 lines by responsibility, starting with
  `Game::begin_capture_scene`, `Game::update_gameplay`, `Game::draw`, content
  validation/index/fallback builders, and tower/egg/breeding/combat handlers.
  Keep files below 800 total lines, extract cohesive action modules from the
  758-line `src/game/actions.rs`, and migrate touched `mod.rs` roots to named
  module files when restructuring (§2.2–2.3, §4.1).
- [ ] Move tutorial progression selection/flags and `tutorial::mark` from
  `src/screens/tutorial.rs` into state/engine modules; keep screen input/rendering
  read-only and apply progression through the existing action dispatcher
  (§2.1, §5.1, §7.1).
- [ ] Add missing `//!` purpose documentation to production and test modules,
  including crate roots. Correct the stale “non-test lines” description in
  `tests/code_standards.rs` to describe total physical lines (§2.2, §9.2).
- [ ] Replace oversized argument lists in `GameData::from_parts`, atlas drawing,
  and breeding rows with cohesive data/context structs; remove the blanket
  Clippy allowance in `src/main.rs` and explain any remaining targeted allowances
  with comments (§4.3, §10.2).
- [ ] Remove same-scope variable shadowing, starting with repeated
  `status_message` bindings in `Game::new` and `targets` in the UI controller registry;
  use distinct descriptive bindings or explicit reassignment (§10.3).
