# Hatchspire shipped-asset provenance ledger

Audit date: 27 August 2026

This ledger covers project files embedded into the Windows executable. Macroquad's built-in
resources and compiled Rust dependencies are handled by the dependency-notice audit. Runtime asset
packaging is empty because these inputs are compiled into the binary.

Every row remains blocked until the rights holder records creator/source, creation method,
ownership or license basis, required attribution, modifications, and explicit approval. A file's
presence in the repository is not proof that it may be published.

## Visual inputs

| Embedded file | Creator/source | Method | Rights basis / attribution | Modifications | Human approval |
| --- | --- | --- | --- | --- | --- |
| `hatchspire_title.png` | Unresolved | Unresolved | Unresolved | Unresolved | Pending |
| `assets/generated/monster_art/monster_sprite_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/monster_art/canonical_egg_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/monster_art/enemy_boss_sprite_v2_atlas.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/monster_art/monster_party_context_portrait_v3_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/town/town_facility_landmarks_v4_atlas.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/town/town_service_npcs_v5_atlas.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/combat/combat_vfx_atlas_chroma_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Chroma processing unresolved | Pending |
| `assets/generated/dungeon/dungeon_deep_room_vignettes_v5_atlas.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_sprite_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_enemy_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_biome_room_module_v2_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_biome_room_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_room_module_expedition_space_v2_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_room_module_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/moss_gate_world_plate_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_interaction_landmarks_v5_atlas.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_hazard_telegraphs_v5_atlas.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_escalation_landmark_v2_atlas.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_enemy_intent_wandering_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_enemy_intent_silhouette_v2_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_escalation_escape_cue_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_weather_discovery_v3_overlay.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |
| `assets/generated/dungeon/dungeon_secret_discovery_v2_atlas_v1.png` | Unresolved | Generated; tool/model record required | Unresolved | Unresolved | Pending |

## Embedded game-data inputs

These JSON files appear to be project-authored data, but the publisher still must attest ownership
and inherited-source status.

| Embedded file | Ownership/source | Attribution | Human approval |
| --- | --- | --- | --- |
| `assets/data/config.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/resources.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/buildings.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/monster_species.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/egg_types.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/tower_floors.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/enemies.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/tower_specials.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/tower_hazards.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/tower_contracts.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/tower_anomalies.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/npcs.json` | Project-authored; attestation pending | None identified | Pending |
| `assets/data/balance.json` | Project-authored; attestation pending | None identified | Pending |

## Findings

- 37 project inputs are embedded: 24 visual files and 13 JSON data files.
- No audio file is embedded.
- `asset_registry.json` correctly remains empty because no external runtime asset is required.
- All 24 visual rows lack sufficient provenance and approval for a public release.
- The repository and internal macroquad-toolkit do not declare license metadata.
