# TODO — Hatchspire

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
- [ ] Make introductory/completion tutorial instructions name their visible next
  button (`CONTINUE`, `SHOW ACTIONS`, `FINISH GUIDE`, or `SHOW THE EGG`). Update
  `game_page.json` to describe direct room tapping instead of the obsolete
  focus-then-`ROUTE` interaction; verify the touch flow and replace affected
  captures directly in `docs/verification/` (§7.5).
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
