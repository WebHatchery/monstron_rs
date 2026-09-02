# TODO — Hatchspire

This backlog is limited to implementation, test, harness, and documentation work that
can be completed by an AI coding agent. Subjective playtest tuning, visual approval,
and open-ended external toolchain decisions are intentionally excluded.

## Polish

- [x] Add a reusable tooltip primitive that supports pointer hover and a visible touch path.
- [x] Add contextual tooltips to town map, building upgrade, facility entry, and menu controls.
- [x] Add actionable validation messages for town building, shop trade, and NPC greeting failures.
- [x] Add actionable validation messages for hatchery, stable, breeding, and workshop failures.
- [x] Add tower and combat failure/recovery messages with a visible touch action for each.
- [x] Expose all 12 Stable slots and add confirmed rehoming so maximum capacity cannot block eggs.
- [x] Let players revisit every unlocked tower floor from a touch-first expedition-prep selector.
- [x] Keep live expedition messages and journal history inside their dungeon HUD bounds.
- [x] Make reached stairs—not entry, retreat, or ordinary combat—the authority for floor unlocks.
- [x] Verify every room and map object is reachable across all floors, goals, and representative seeds.
- [x] Show party readiness and a tested floor-level recommendation before each expedition.

## Onboarding

- [x] Add a persistent, skippable first-expedition guide from camp scavenging through tower return.
- [x] Add contextual first-combat guidance with an exact visible ATTACK target.
- [x] Add a Camp Menu replay path and verification captures at desktop and small-window sizes.
- [x] Build and visit the Hatchery before the guided expedition so discovered eggs have capacity.
- [x] Teach post-expedition recovery and the explicit manual-save path with visible controls.
- [x] Add contextual first-egg care, incubation, Stable-capacity recovery, and hatching guidance.

## Balance tooling

- [x] Define a serializable combat replay format containing the RNG seed, roster, encounter,
  commands, and expected outcome.
- [x] Record player commands and the RNG seed while a combat encounter is running.
- [x] Implement a replay runner that reconstructs an encounter and reports the first mismatch.
- [x] Add deterministic replay tests for victory, defeat, fleeing, and item use.
- [x] Regression-test every deep-floor enemy and guardian with a three-monster same-level party.
- [x] Regression-test deep victory, overnight recovery, and same-floor re-entry as one playable loop.
- [x] Prove a fresh three-monster party can level through generated-map revisits and beat both guardians.
- [x] Show individual level and XP-to-next-level progress in the Stable for training decisions.
- [x] Prove required early facilities and every maximum upgrade remain reachable from renewable income.
- [x] Prove routed dungeon eggs can be recovered, incubated, hatched, and assigned as a full early party.
- [x] Traverse generated maps from floor 1 to the finale through routed movement and live interruptions.
- [x] Make defeat recovery explicit in Town and prove the injured party can re-enter after sleeping.
- [x] Restore active Tower, Combat, and pending Finale screens correctly after save/load.
- [x] Keep the next campaign objective and guardian thresholds visible in Town through the ending.
- [x] Add original procedural ambience and gameplay cues governed by the existing audio settings.

## Verification captures

- [x] Extend the capture harness with seeded stable, breeding grove, workshop, shop, and
  combat scenes.
- [x] Capture town and hatchery verification screens in `docs/verification/`.
- [x] Capture stable, breeding grove, and workshop verification screens in
  `docs/verification/`.
- [x] Capture shop, tower, and combat verification screens in `docs/verification/`.

## Engineering — integration coverage

- [x] Add an integration test for the hatchery care and hatching flow.
- [x] Add an integration test for stable roster and recovery actions.
- [x] Add an integration test for shop trades and purchase validation.
- [x] Add an integration test for tower preparation, movement, return, and rewards.
- [x] Add an integration test for combat commands, resolution, and recovery.
- [x] Add an integration test for town purchases and building progression.

## Engineering — shared configuration

- [x] Move monster stat curves into shared typed game data and add integrity checks.
- [x] Move combat cooldown definitions into shared typed game data and add integrity checks.
- [x] Move shop inventory and trade costs into shared typed game data and add integrity checks.
- [x] Move tower rewards into shared typed game data and add integrity checks.

## Engineering — command boundaries

- [x] Standardise explicit town commands for building, trade, greeting, save, and navigation.
- [x] Move remaining town progression mutations behind the town command/reducer boundary.
- [x] Standardise explicit combat commands for attacks, skills, items, defence, and fleeing.
- [x] Move remaining combat progression mutations behind the combat command/reducer boundary.
- [x] Add regression tests proving rendering does not mutate town or combat progression.
