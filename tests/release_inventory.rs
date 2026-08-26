use std::fs;
use std::path::Path;

const VISUAL_INPUTS: [&str; 24] = [
    "hatchspire_title.png",
    "assets/generated/monster_art/monster_sprite_atlas_v1.png",
    "assets/generated/monster_art/canonical_egg_atlas_v1.png",
    "assets/generated/monster_art/enemy_boss_sprite_v2_atlas.png",
    "assets/generated/monster_art/monster_party_context_portrait_v3_atlas_v1.png",
    "assets/generated/town/town_facility_landmarks_v4_atlas.png",
    "assets/generated/town/town_service_npcs_v5_atlas.png",
    "assets/generated/combat/combat_vfx_atlas_v1.png",
    "assets/generated/dungeon/dungeon_deep_room_vignettes_v5_atlas.png",
    "assets/generated/dungeon/dungeon_sprite_atlas_v1.png",
    "assets/generated/dungeon/dungeon_enemy_atlas_v1.png",
    "assets/generated/dungeon/dungeon_biome_room_module_v2_atlas_v1.png",
    "assets/generated/dungeon/dungeon_biome_room_atlas_v1.png",
    "assets/generated/dungeon/dungeon_room_module_expedition_space_v2_atlas_v1.png",
    "assets/generated/dungeon/dungeon_room_module_atlas_v1.png",
    "assets/generated/dungeon/moss_gate_world_plate_v1.png",
    "assets/generated/dungeon/dungeon_interaction_landmarks_v5_atlas.png",
    "assets/generated/dungeon/dungeon_hazard_telegraphs_v5_atlas.png",
    "assets/generated/dungeon/dungeon_escalation_landmark_v2_atlas.png",
    "assets/generated/dungeon/dungeon_enemy_intent_wandering_atlas_v1.png",
    "assets/generated/dungeon/dungeon_enemy_intent_silhouette_v2_atlas_v1.png",
    "assets/generated/dungeon/dungeon_escalation_escape_cue_atlas_v1.png",
    "assets/generated/dungeon/dungeon_weather_discovery_v3_overlay.png",
    "assets/generated/dungeon/dungeon_secret_discovery_v2_atlas_v1.png",
];

const DATA_INPUTS: [&str; 13] = [
    "assets/data/config.json",
    "assets/data/resources.json",
    "assets/data/buildings.json",
    "assets/data/monster_species.json",
    "assets/data/egg_types.json",
    "assets/data/tower_floors.json",
    "assets/data/enemies.json",
    "assets/data/tower_specials.json",
    "assets/data/tower_hazards.json",
    "assets/data/tower_contracts.json",
    "assets/data/tower_anomalies.json",
    "assets/data/npcs.json",
    "assets/data/balance.json",
];

fn read(relative: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(root.join(relative))
        .unwrap_or_else(|error| panic!("{relative} must be readable: {error}"))
}

#[test]
fn every_embedded_project_input_exists_and_is_in_the_provenance_ledger() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let visual_sources = format!("{}\n{}", read("src/assets/mod.rs"), read("src/game.rs"));
    let data_source = read("src/data/loader.rs");
    let ledger = read("docs/shipped_asset_provenance.md");

    assert_eq!(
        visual_sources.matches("include_bytes!").count(),
        VISUAL_INPUTS.len()
    );
    assert_eq!(
        data_source.matches("include_json_str!").count(),
        DATA_INPUTS.len()
    );

    for relative in VISUAL_INPUTS.into_iter().chain(DATA_INPUTS) {
        assert!(
            root.join(relative).is_file(),
            "embedded input is missing: {relative}"
        );
        assert!(
            visual_sources.contains(relative) || data_source.contains(relative),
            "inventory names an input that is not embedded: {relative}"
        );
        assert!(
            ledger.contains(relative),
            "provenance ledger omits: {relative}"
        );
    }
}
