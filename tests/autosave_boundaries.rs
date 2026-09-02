use std::time::{SystemTime, UNIX_EPOCH};

use hatchspire::data::{GameData, GameDataLoader};
use hatchspire::engine::{
    combat_engine::{self, CombatCommand, CombatDestination},
    day_engine, tower_engine, town_engine,
};
use hatchspire::save::SaveData;
use hatchspire::state::{CombatOutcome, GameState, TowerRunGoal};
use macroquad_toolkit::persistence::{
    delete_slot, load_from_slot, save_to_slot_with_version_and_backup, slot_backup_exists,
};

const SLOT: &str = "autosave_boundary";
const GAME_COORDINATOR: &str = include_str!("../src/game.rs");

struct SaveCleanup {
    game_name: String,
}

impl Drop for SaveCleanup {
    fn drop(&mut self) {
        for slot in [
            SLOT,
            "autosave_boundary_backup",
            "autosave_boundary_before_restore",
        ] {
            let _ = delete_slot(&self.game_name, slot);
        }
    }
}

fn unique_game_name() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should follow the Unix epoch")
        .as_nanos();
    format!("hatchspire_autosave_{}_{nanos}", std::process::id())
}

fn checkpoint(game_name: &str, data: &GameData, state: GameState, label: &str) -> GameState {
    let expected = serde_json::to_value(&state).expect("checkpoint state should serialize");
    let save = SaveData {
        version: data.config.save_version,
        state,
    };
    save_to_slot_with_version_and_backup(game_name, SLOT, &save, env!("CARGO_PKG_VERSION"))
        .unwrap_or_else(|error| panic!("{label} checkpoint failed: {error}"));
    let loaded: SaveData = load_from_slot(game_name, SLOT)
        .unwrap_or_else(|error| panic!("{label} restart failed: {error}"));
    assert_eq!(loaded.version, data.config.save_version, "{label}");
    assert_eq!(
        serde_json::to_value(&loaded.state).expect("reloaded state should serialize"),
        expected,
        "{label} changed across the native restart boundary"
    );
    loaded.state
}

fn empty_adjacent_step(state: &GameState) -> (i32, i32) {
    let run = state.tower_run.as_ref().expect("tower run should exist");
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .find(|(dx, dy)| {
            let x = run.map.player_x as i32 + dx;
            let y = run.map.player_y as i32 + dy;
            x >= 0
                && y >= 0
                && run.map.is_passable(x as u32, y as u32)
                && !run
                    .map
                    .objects
                    .iter()
                    .any(|object| object.x == x as u32 && object.y == y as u32)
        })
        .expect("start room should have an empty adjacent route")
}

#[test]
fn completed_demo_actions_survive_native_save_restart_boundaries() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let game_name = unique_game_name();
    let _cleanup = SaveCleanup {
        game_name: game_name.clone(),
    };

    let mut state = checkpoint(&game_name, &data, GameState::new(&data), "new camp");

    let scavenged = town_engine::scavenge_supplies(&mut state);
    assert!(scavenged.summary.contains("scavenges"));
    state = checkpoint(&game_name, &data, state, "town resource action");

    state.resources.add("wood", 40);
    state.resources.add("herbs", 20);
    let built = town_engine::advance_building(&mut state, &data, "hatchery");
    assert!(built.summary.contains("Built Hatchery"));
    state = checkpoint(&game_name, &data, state, "facility improvement");

    let started = tower_engine::start_run(&mut state, &data, TowerRunGoal::Balanced);
    assert!(
        started.summary.contains("enters floor"),
        "{}",
        started.summary
    );
    state = checkpoint(&game_name, &data, state, "tower entry");

    let (dx, dy) = empty_adjacent_step(&state);
    let before_position = state
        .tower_run
        .as_ref()
        .map(|run| (run.map.player_x, run.map.player_y));
    let movement = tower_engine::move_party(&mut state, &data, dx, dy);
    let after_position = state
        .tower_run
        .as_ref()
        .map(|run| (run.map.player_x, run.map.player_y));
    assert_ne!(after_position, before_position, "{}", movement.summary);
    state = checkpoint(&game_name, &data, state, "tower movement");

    combat_engine::start_encounter(&mut state, &data, 1, false);
    assert!(state.combat.is_some());
    combat_engine::player_action(&mut state, &data, CombatCommand::Attack);
    assert!(state
        .combat
        .as_ref()
        .is_some_and(|combat| !combat.command_history.is_empty()));
    state = checkpoint(&game_name, &data, state, "unresolved combat turn");

    for _ in 0..160 {
        if state
            .combat
            .as_ref()
            .is_some_and(|combat| combat.outcome.is_some())
        {
            break;
        }
        combat_engine::player_action(&mut state, &data, CombatCommand::Attack);
    }
    assert!(state
        .combat
        .as_ref()
        .is_some_and(|combat| combat.outcome == Some(CombatOutcome::Victory)));
    let finish = combat_engine::finish_combat(&mut state, &data);
    assert_eq!(finish.destination, CombatDestination::Tower);
    let returned = tower_engine::return_to_town(&mut state, &data);
    assert!(returned.returned_to_town, "{}", returned.summary);
    state = checkpoint(&game_name, &data, state, "combat reward and town return");

    let day_before = state.day;
    day_engine::sleep(&mut state, &data);
    assert_eq!(state.day, day_before + 1);
    let state = checkpoint(&game_name, &data, state, "day advance and recovery");

    assert!(state.combat.is_none());
    assert!(state.tower_run.is_none());
    assert!(slot_backup_exists(&game_name, SLOT));
}

#[test]
fn every_progression_screen_routes_mutation_through_the_autosave_wrapper() {
    for action in [
        "game.apply_town_action(action)",
        "game.apply_hatchery_action(action)",
        "game.apply_stable_action(action)",
        "game.apply_breeding_action(action)",
        "game.apply_workshop_action(action)",
        "game.apply_shop_action(action)",
        "game.apply_tower_action(action)",
        "game.apply_combat_action(action)",
        "game.apply_tutorial_action(action)",
    ] {
        let routed = format!("self.apply_progression(|game| {action})");
        assert!(
            GAME_COORDINATOR.contains(&routed),
            "progression route bypasses autosave: {action}"
        );
    }
    assert_eq!(
        GAME_COORDINATOR
            .matches("self.apply_progression(|game| game.apply_placeholder_action(action))")
            .count(),
        2,
        "both dungeon preparation and end-of-day progression must autosave"
    );
    assert_eq!(
        GAME_COORDINATOR.matches("self.apply_progression(").count(),
        11,
        "review and register every new progression route in this persistence gate"
    );
}
