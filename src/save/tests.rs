use super::*;
use crate::data::GameDataLoader;
use crate::engine::{breeding_engine, day_engine, job_engine};
use crate::state::{GameState, TownJobKind};

#[test]
fn save_versions_newer_than_the_game_are_not_treated_as_supported() {
    assert_eq!(compatibility(1, 1), SaveCompatibility::Supported);
    assert_eq!(compatibility(0, 1), SaveCompatibility::Supported);
    assert_eq!(compatibility(2, 1), SaveCompatibility::Newer);
}

#[test]
fn phase6_and_phase7_fields_round_trip_through_save_data() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.town.set_building_level("breeding_grove", 1);
    state.town.set_building_level("hatchery", 1);
    state.town.set_building_level("workshop", 1);
    state.resources.add("herbs", 10);

    let rillfin = data.species("rillfin").expect("rillfin should exist");
    let second_id = state
        .monster_roster
        .add_monster("Ripple".to_owned(), rillfin, 0xBEE5_7001);
    breeding_engine::breed_pair(&mut state, &data, 1, second_id);
    day_engine::sleep(&mut state, &data);
    job_engine::assign_job(&mut state, &data, 1, TownJobKind::Forage);

    let save_data = SaveData {
        version: data.config.save_version,
        state,
    };
    let json = serde_json::to_string(&save_data).expect("save should serialize");
    assert!(json.contains("inheritance"));
    assert!(json.contains("assignments"));
    assert!(json.contains("art_profile"));
    assert!(json.contains("lineage_quality"));

    let loaded: SaveData = serde_json::from_str(&json).expect("save should deserialize");
    assert!(loaded.state.egg_inventory.eggs[0].inheritance.is_some());
    assert_eq!(loaded.state.town.assignments[0].job, TownJobKind::Forage);
}

#[test]
fn older_saves_without_phase6_or_phase7_fields_still_load() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state
        .egg_inventory
        .add_egg("mossy_egg".to_owned(), 1, 1, 0xF00D);
    state.town.set_monster_job(1, TownJobKind::Forage);

    let save_data = SaveData {
        version: data.config.save_version,
        state,
    };
    let mut value = serde_json::to_value(&save_data).expect("save should become json");
    value["state"]["town"]
        .as_object_mut()
        .expect("town should be an object")
        .remove("assignments");
    value["state"]
        .as_object_mut()
        .expect("state should be an object")
        .remove("tower_discoveries");
    for egg in value["state"]["egg_inventory"]["eggs"]
        .as_array_mut()
        .expect("eggs should be an array")
    {
        egg.as_object_mut()
            .expect("egg should be an object")
            .remove("inheritance");
    }
    for monster in value["state"]["monster_roster"]["monsters"]
        .as_array_mut()
        .expect("monsters should be an array")
    {
        monster
            .as_object_mut()
            .expect("monster should be an object")
            .remove("condition");
        monster
            .as_object_mut()
            .expect("monster should be an object")
            .remove("art_profile");
    }

    let loaded: SaveData = serde_json::from_value(value).expect("old save should load");
    assert!(loaded.state.town.assignments.is_empty());
    assert!(loaded.state.egg_inventory.eggs[0].inheritance.is_none());
    assert_eq!(loaded.state.monster_roster.monsters[0].condition.fatigue, 0);
    assert!(loaded.state.tower_discoveries.enemy_ids.is_empty());
    assert!(loaded.state.monster_roster.monsters[0]
        .art_profile
        .is_empty());
}

#[test]
fn older_bred_eggs_without_art_fields_still_load() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.town.set_building_level("breeding_grove", 1);
    state.town.set_building_level("hatchery", 1);
    state.resources.add("herbs", 10);

    let rillfin = data.species("rillfin").expect("rillfin should exist");
    let second_id = state
        .monster_roster
        .add_monster("Ripple".to_owned(), rillfin, 0xBEE5_7001);
    breeding_engine::breed_pair(&mut state, &data, 1, second_id);

    let save_data = SaveData {
        version: data.config.save_version,
        state,
    };
    let mut value = serde_json::to_value(&save_data).expect("save should become json");
    let inheritance = value["state"]["egg_inventory"]["eggs"][0]["inheritance"]
        .as_object_mut()
        .expect("inheritance should be an object");
    inheritance.remove("lineage_quality");
    inheritance.remove("art_profile");

    let loaded: SaveData = serde_json::from_value(value).expect("old bred egg should load");
    let inheritance = loaded.state.egg_inventory.eggs[0]
        .inheritance
        .as_ref()
        .expect("inheritance should still exist");
    assert_eq!(inheritance.lineage_quality, 0);
    assert!(inheritance.art_profile.is_empty());
}
