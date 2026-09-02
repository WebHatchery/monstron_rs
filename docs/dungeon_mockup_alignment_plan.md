# Dungeon Mockup Alignment Implementation Plan

Status: implemented and re-aligned through the full-screen Moss Gate acceptance pass on 2026-08-13. The earlier isolated-room/debug-overlay presentation was replaced with the reference composition for real; deeper floors use the same renderer and biome mapping.

The target is the world-first dungeon composition represented by `docs/ui_reference/living_tower_dungeon_mockup_v1.png`: a connected illustrated dungeon occupies the canvas, while a compact top bar, party rail, context drawer, expedition journal, and bottom action dock float over the world.

## Phase completion record

- Atlas registry: complete in `src/assets/mod.rs`; transparent room, fog, atmosphere, portrait, party, landmark, enemy, egg, cache and traversal atlases are addressable by typed helpers.
- Visual room graph: complete for the current run model; generated rooms are rendered as room-scale illustrated modules with purpose-aware scene selection.
- World camera/layered renderer: complete for the full-screen run view; room modules, connectors, entities, atmosphere, fog and overlays render in layers.
- Fog and lighting: complete for the acceptance pass using fog silhouettes, discovered tinting, landmark glows and biome atmosphere overlays.
- Physical landmarks: complete for cache, egg, enemy, boss, stairs and exit scene variants.
- Mockup HUD: complete with top run overlay, party rail, right context drawer, expedition journal, bottom action dock and persistent touch controls.
- Room-level interaction: complete as a visible-world tap-to-step affordance with directional touch fallback; tile movement remains authoritative.
- Data-driven biome variants: complete for room-family selection by floor group, with explicit hazard, recovery/return, and boss/reward atlas hooks for special rooms.

## 2026-08-13 acceptance correction

- The complete map is composed into one world stage, with seven or more overlapping illustrated chambers visible at the starting state instead of one oversized room surrounded by black void.
- Moss Gate rooms use purpose-specific camp, cache, encounter, nest, traversal, and shrine art; deeper floors retain these purposes with moss, flooded, ember, frost, root, and void biome families.
- Hidden rooms remain readable dark silhouettes under procedural mist, explored rooms retain muted detail, and the current room receives the strongest local light.
- The debug legend, minimap, exposed grid, and keyboard-labelled movement cluster were removed from the player-facing view.
- The top resource bar, compact party rail, right landmark drawer, expedition journal, and three-button touch dock now follow the mockup's placement, scale, black-gold framing, and visual hierarchy.
- Tapping a visible room converts its world-space target into an authoritative one-tile movement step; keyboard movement remains a secondary input path without appearing in player-facing text.
- Live entry, travel, discovery, and hazard messages wrap into a bounded two-line world overlay
  instead of disappearing beneath the context drawer.
- The verified acceptance capture is `docs/verification/ui_tower.png`.
- The Moss Gate acceptance floor now uses `moss_gate_world_plate_v1.png`, a dedicated HUD-free continuous ruin plate derived directly from the approved screenshot. This replaces the visibly incorrect floating-room and rectangular corridor reconstructions while live party, object, fog, and input state remain code-driven overlays.

## Phases

1. **Atlas registry and composition proof**
   - Add typed dungeon visual identifiers and atlas metadata.
   - Use transparent runtime atlases only.
   - Define cells, anchors, visual bounds, and layer order.
   - Capture a Moss Gate composition proof.

2. **Visual room graph**
   - Preserve `TowerMapState` as gameplay authority.
   - Derive room-purpose, biome, connector, landmark, and world-position presentation records.
   - Render rooms as connected illustrated modules instead of exposed square debug tiles.

3. **World camera and layered renderer**
   - Add a camera centered on the party.
   - Render background, discovered silhouettes, room modules, connectors, dressing, landmarks, entities, lights, and fog in layers.
   - Keep deterministic placement from the run seed.

4. **Fog and lighting**
   - Use unknown, discovered, and visible visual states.
   - Apply generated fog/silhouette and atmosphere overlays.
   - Add party, shrine, enemy, and boss light pools.

5. **Physical landmarks**
   - Convert egg, loot, enemy, boss, stairs, and exit objects into environmental scenes.
   - Keep small markers only as clarity accents.

6. **Mockup HUD**
   - Add top run bar, party rail, context drawer, expedition journal, and bottom action dock.
   - Keep controls touch-sized and explicit.
   - Retain directional movement as a fallback.

7. **Room-level interaction**
   - Add tap targets for adjacent rooms and landmarks.
   - Keep tile movement authoritative and available.
   - Surface Explore, Camp, Retreat, and contextual actions.

8. **Data-driven biome variants**
   - Map floors to moss, flooded, ember, frost, root, and void room families.
   - Add hazard, recovery, boss, and reward compositions.

9. **Verification**
   - Capture the tower state and replace `docs/verification/ui_tower.png`.
   - Run Rust tests and `publish.ps1`.
   - Commit each coherent milestone.

## Acceptance

The primary capture must show a full-canvas connected dungeon with room-scale art, soft fog, local lighting, physical landmarks, readable party identity, and a compact touch-first HUD. No magenta chroma assets or square tile grid may be visible in the final world presentation.
