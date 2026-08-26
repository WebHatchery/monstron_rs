# Hatchspire

Hatchspire is a monster-raising town RPG about rebuilding a camp beneath a ruined tower and preparing companions for deeper dungeon runs.

You begin with a loyal slime, a cold hatchery brazier, and just enough supplies to make the camp useful again. Each day is a loop of preparation, risk, recovery, and growth.

## Gameplay

- Rebuild camp facilities such as the hatchery, stable, workshop, shop, and breeding grove.
- Hatch, raise, rest, and organize tower-born monsters.
- Choose a party before entering the tower.
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
- Help & Support in Settings shows the build version, save path, and recovery guidance.
- Audio levels, mute, fullscreen, and reduced motion persist independently from game saves.
- Mouse: build, open, trade, and greet.
- Touch/mouse: every required tower action has a visible target, including room routing, EXPLORE, SURVEY, CAMP, RETREAT, event choices, and the Field Guide.

## Current Scope

Playable camp and tower loop with monsters, hatching, breeding, jobs, goal-driven expeditions, map routing, combat, fatigue, injuries, and recovery.

The ten-floor tower currently includes:

- 80 authored dungeon enemies across 11 active combat behaviors, six visual families, roaming hunters, and two floor guardians.
- 22 special locations with 44 persistent, touch-first event approaches, including party requirements, cargo costs, map effects, blessings, ambushes, and shelter-building outcomes.
- Deterministic room-art variants keep each generated chamber's visual identity stable through save/load, while completed landmarks leave an authored event trail on the map.
- Six hazards, six anomalies, six expedition contracts, persistent room purposes, and goal-aware automated routing.
- Concealed floor caches that use the secret-discovery atlas and can be exposed by SURVEY, loot-finder passives, or map-reading landmark events. Salvage runs hide two caches per floor.
- A persistent Field Guide that records enemies, hazards, landmarks, and tried approaches; knowledge ranks improve future survey kits and reveal known hunter tracks in explored rooms.
- Marked CAMP rooms with stronger recovery, Safe Run return routing, and event-created shelters that permanently reshape the current floor.
- Pressure escalation, wandering hunters, floor-specific guardian eggs, and sealed guardian thresholds on floors 5 and 10.

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
- `TODO.md` — open implementation, testing, and verification work.
