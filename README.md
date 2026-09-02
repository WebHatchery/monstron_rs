# Hatchspire

Hatchspire is a monster-raising town RPG about rebuilding a camp beneath a ruined tower and preparing companions for deeper dungeon runs.

You begin with a loyal slime, a cold hatchery brazier, and just enough supplies to make the camp useful again. Each day is a loop of preparation, risk, recovery, and growth.

## Gameplay

- Rebuild camp facilities such as the hatchery, stable, workshop, shop, and breeding grove.
- Hatch, raise, rest, and organize tower-born monsters.
- Page through every Stable slot and safely rehome a benched companion when a full roster blocks
  further hatching.
- Review each companion's level and XP-to-next-level progress in the Stable before choosing a party.
- Choose a battle-ready party before entering the tower; expedition prep shows its average level
  beside the selected floor's suggested level and blocks entry when nobody is ready.
- Follow an optional, persistent first-expedition guide through Hatchery construction, dungeon
  exploration, recovery, and the first manual save using exact visible touch targets.
- Choose an expedition goal, survey hidden rooms, resolve landmark events, and retreat before pressure, injuries, and fatigue spiral.
- Spend rewards on stronger buildings, better recovery, and deeper expeditions.

## Goal

Turn a fragile camp into a working monster haven that can support increasingly dangerous tower runs.

## Controls

- Enter: new save or enter tower.
- Space: sleep.
- WASD / arrows: move through the tower view.
- Minimap: tracks discovered tower terrain.
- D: tower preparation.
- H: hatchery.
- R: stable or return from tower.
- B: breeding grove.
- W: workshop jobs.
- T: shop trades.
- C: scavenge supplies.
- A/S/D: combat attack, skill, or defend.
- I/F: combat herbs or flee.
- Esc: camp menu.
- S/L/T: save, load, or title inside menu.
- Progress autosaves after every successful state-changing action; manual save remains available.
- Save Options on the title screen deletes the single slot only after a separate confirmation.
- Help & Support in Settings shows the build version, save path, recovery guidance, and an explicit
  local-only tester-summary export. Nothing is uploaded.
- Audio levels, mute, fullscreen, and reduced motion persist independently from game saves.
- Window UI scale resizes the complete fixed canvas to 90%, 100%, 110%, or 125% without clipping controls.
- Supported older saves are normalized and rewritten at the current version when loaded.
- Each successful replacement save keeps the previous state available through RESTORE BACKUP on the recovery screen.
- Native crashes append a local `crash_log.txt` beside the save; nothing is uploaded automatically.
- Mouse: build, open, trade, and greet.
- Touch/mouse: tap a chamber to travel toward it at a readable pace. Travel pauses at loot,
  hazards, landmarks, and encounters; tap another room to redirect, or use STEP NOW, SURVEY,
  CAMP, or RETREAT for direct control. Live expedition results wrap inside the world HUD, while
  bounded journal excerpts keep route changes and discoveries readable without covering controls.
- Tower Prep defaults to the deepest unlocked floor, with visible PREV FLOOR and NEXT FLOOR
  controls for revisiting any earlier route under a different expedition goal. It shows the
  battle-ready party count, average level, and a suggested level matching the selected floor.
- A deeper floor unlocks only after the party reaches the current floor's stairs; entering,
  fighting, or retreating alone cannot bypass exploration.
- The first-expedition guide can be skipped at any step and replayed from the Camp Menu.
- When the first egg reaches camp, contextual guidance covers daily care, incubation, Stable
  expansion or confirmed rehoming, and hatching without interrupting later free play.

## Current Scope

Playable camp and tower loop with monsters, hatching, breeding, jobs, goal-driven expeditions, map routing, combat, fatigue, injuries, and recovery.

The ten-floor tower currently includes:

- 80 authored dungeon enemies across 11 active combat behaviors, six visual families, roaming hunters, and two floor guardians.
- 22 special locations with 44 persistent, touch-first event approaches, including party requirements, cargo costs, map effects, blessings, ambushes, and shelter-building outcomes.
- Deterministic room-art variants keep each generated chamber's visual identity stable through save/load, while completed landmarks leave an authored event trail on the map.
- Six hazards, six anomalies, six expedition contracts, persistent room purposes, and goal-aware
  routed travel that visibly crosses the map and stops for discoveries.
- Direct selection of every unlocked floor, so earlier nests, caches, landmarks, enemies, and
  Field Guide discoveries remain available after deeper routes open.
- Concealed floor caches that use the secret-discovery atlas and can be exposed by SURVEY, loot-finder passives, or map-reading landmark events. Salvage runs hide two caches per floor.
- A persistent Field Guide that records enemies, hazards, landmarks, and tried approaches; knowledge ranks improve future survey kits and reveal known hunter tracks in explored rooms.
- Marked CAMP rooms with stronger recovery, Safe Run return routing, and event-created shelters that permanently reshape the current floor.
- Pressure escalation, wandering hunters, floor-specific guardian eggs, and sealed guardian thresholds on floors 5 and 10.
- Stair-authoritative sequential progression backed by connectivity checks across every floor,
  expedition goal, and a broad deterministic seed sweep.
- Deep-floor balance regressions exercise every floor 4–10 enemy and both guardians with a
  three-companion party, plus a victory, overnight recovery, and same-floor re-entry cycle.
- Natural-progression coverage starts three companions at level one, trains only through encounters
  present on generated revisit maps, and clears both guardians by level ten within 45 victories.
- The required Hatchery and Stable foundations are reachable after two visible camp gathers, while
  repeatable town income can eventually complete every facility without a resource dead end.
- A generated floor-one Egg Hunt is integration-tested through routed nest exploration, safe return,
  daily care, incubation, hatching, and Stable assignment into a three-companion expedition party.
- A routed campaign regression walks generated corridors from floor 1 through floor 10, handles
  pressure camps, combats, hazards, loot, eggs, and landmark interruptions, defeats both guardians,
  reaches each physical stair, and completes the Verdant Crown exit.
- Defeat returns to Town with an exact visible SLEEP recovery instruction; the Town roster exposes
  each condition, and a regression proves three recovery nights restore deep-floor re-entry.
- Loading reconstructs unresolved Combat and Tower screens, preserves an unseen Crown epilogue,
  and records finale choices so completed post-game saves resume in Town without replay loops.
- Town keeps the next campaign objective visible after onboarding, naming both guardian thresholds
  and replacing the ascent goal with a post-story free-expedition state after the Crown is restored.
- An authored floor-10 conclusion that banks the final expedition, records the restored Verdant
  Crown, celebrates the active party, and keeps post-story expeditions available.

## Design

The design spine is a single circle:

```text
Town building -> Monster raising -> Tower depth -> Town building
```

Three pillars carry it:

- **Town growth.** The camp starts broken beside the tower and grows into a monster-focused settlement. Buildings are functional systems (hatchery, stable, workshop, shop, breeding grove), not decorations.
- **Monster raising.** A monster is an adventurer *and* a citizen, and should matter in combat, exploration, town work, breeding, and flavour. Each carries a species, element, temperament, role, passive, and town skill. The starter slime stays relevant through bond progression and unique utility.
- **Tower dungeon.** Ten floors across Mossy Ruins (1-3), Crystal Cracks (4-6), and Sunken Garden (7-10), with a Mirror Matriarch threshold on floor 5 and the Verdant Crown on floor 10. The tower is a persistent knowledge game as well as the source of eggs, materials, landmarks, hazards, and encounters.

The emotional goal is that the town exists *because* the monsters are helping build it, rather than buildings being abstract menu upgrades.

Standing design constraints:

- Build for native and WebGL from the start.
- Keep systems data-driven so species, eggs, buildings, NPCs, and floors expand without engine rewrites.
- Prefer deterministic, inspectable simulation over hidden randomness.
- UI returns intent/action objects; state mutation lives in engines and reducers.
- The old Unity Monstron prototype is inspiration only, never a port target. `Hatchspire` is both the player-facing title and the Rust crate/package identity.

## Documentation

- `docs/monster_art_pipeline.md` — art DNA, prompt export, and the local ComfyUI generation workflow.
- `docs/shipped_asset_provenance.md` — exact embedded art/data inventory and human rights-approval ledger.
- `docs/accelerated_soak.md` — deterministic 240-cycle engine/native-persistence stress gate and its limits.
- `docs/autosave_boundary_matrix.md` — native restart checks at eight completed progression boundaries.
- `docs/release_profile_smoke.md` — exact-package optimized boot/render smoke gate across all 20 scenes.
- `docs/performance_probe.md` — opt-in optimized CPU budget plus four-resolution process-memory diagnostics.
- `docs/local_tester_summary.md` — explicit local-only pacing/balance export and migration boundary.
- `docs/windows_rollback_procedure.md` — preserved-build rollback and first-patch rehearsal.
- `TODO.md` — open implementation, testing, and verification work.

## Internal Windows preview package

Run the required publisher first, then add the player-facing drafts and exact build manifest:

```powershell
.\publish.ps1
.\scripts\package_windows_preview.ps1
.\scripts\release_smoke.ps1
.\publish.ps1 -DeployOnly
```

The packager requires a clean working tree, replaces `dist/hatchspire_windows.zip`, verifies its
contents, and writes `dist/hatchspire_windows_manifest.json` plus `dist/hatchspire_windows.sha256`.
The smoke gate then proves the packaged EXE matches the release build, renders all 20 seeded scenes
at four target window sizes, satisfies the provisional optimized CPU regression budget, and records
nonzero sampled/OS-peak process-memory diagnostics for every resolution process. The
deploy-only pass synchronizes that sealed archive, rather than the initial executable-only base ZIP,
to the local preview. The result remains an internal preview; it does not
waive the human scope, rights, support, visual-review, device-testing, or release approvals.
