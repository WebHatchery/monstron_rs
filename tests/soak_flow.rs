use std::time::{SystemTime, UNIX_EPOCH};

use hatchspire::data::GameDataLoader;
use hatchspire::engine::{
    combat_engine::{self, CombatCommand, CombatDestination},
    day_engine, tower_engine, town_engine,
};
use hatchspire::save::SaveData;
use hatchspire::state::{GameState, TowerRunGoal};
use macroquad_toolkit::persistence::{
    delete_slot, load_from_slot, save_to_slot_with_version_and_backup, slot_backup_exists,
};

const SLOT: &str = "accelerated_soak";
const CYCLES: usize = 240;

struct SaveCleanup {
    game_name: String,
}

impl Drop for SaveCleanup {
    fn drop(&mut self) {
        for slot in [
            SLOT,
            "accelerated_soak_backup",
            "accelerated_soak_before_restore",
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
    format!("hatchspire_soak_{}_{nanos}", std::process::id())
}

#[test]
fn accelerated_long_session_repeats_town_tower_combat_recovery_and_restart() {
    let data = GameDataLoader::load_embedded().expect("embedded game data should load");
    let game_name = unique_game_name();
    let _cleanup = SaveCleanup {
        game_name: game_name.clone(),
    };
    let mut state = GameState::new(&data);
    state.resources.add("wood", 500);
    state.resources.add("stone", 500);
    state.resources.add("herbs", 500);
    state.resources.add("coins", 500);

    for building in ["hatchery", "stable", "breeding_grove", "workshop", "shop"] {
        let result = town_engine::advance_building(&mut state, &data, building);
        assert!(result.summary.contains("Built"), "{}", result.summary);
    }

    let mut victories = 0;
    let mut recoveries = 0;
    for cycle in 0..CYCLES {
        for _ in 0..4 {
            if tower_engine::battle_ready_party_count(&state) > 0 {
                break;
            }
            day_engine::sleep(&mut state, &data);
        }
        assert!(
            tower_engine::battle_ready_party_count(&state) > 0,
            "party deadlocked before cycle {cycle}"
        );

        state.tower_progress.unlocked_floor = 1;
        let started = tower_engine::start_run(&mut state, &data, TowerRunGoal::Balanced);
        assert!(
            state.tower_run.is_some(),
            "cycle {cycle}: {}",
            started.summary
        );
        combat_engine::start_encounter(&mut state, &data, 1, false);
        assert!(
            state.combat.is_some(),
            "cycle {cycle}: combat did not start"
        );

        let command = if cycle % 17 == 0 {
            CombatCommand::Defend
        } else if cycle % 5 == 0 {
            CombatCommand::Flee
        } else {
            CombatCommand::Attack
        };
        for _ in 0..160 {
            if state
                .combat
                .as_ref()
                .is_some_and(|combat| combat.outcome.is_some())
            {
                break;
            }
            combat_engine::reduce_command(&mut state, &data, command);
        }
        assert!(
            state
                .combat
                .as_ref()
                .is_some_and(|combat| combat.outcome.is_some()),
            "combat did not resolve in cycle {cycle}"
        );

        let finish = combat_engine::finish_combat(&mut state, &data);
        if finish.destination == CombatDestination::Tower {
            victories += 1;
            let returned = tower_engine::return_to_town(&mut state, &data);
            assert!(
                returned.returned_to_town,
                "cycle {cycle}: {}",
                returned.summary
            );
        } else {
            recoveries += 1;
            assert!(state.tower_run.is_none(), "recovery should return to town");
        }
        assert!(state.combat.is_none());

        if cycle % 10 == 0 {
            town_engine::scavenge_supplies(&mut state);
        }
        if cycle % 13 == 0 {
            town_engine::greet_npc(&mut state, &data, "mara");
        }
        day_engine::sleep(&mut state, &data);

        let save = SaveData {
            version: data.config.save_version,
            state,
        };
        save_to_slot_with_version_and_backup(&game_name, SLOT, &save, env!("CARGO_PKG_VERSION"))
            .unwrap_or_else(|error| panic!("cycle {cycle} save failed: {error}"));
        let loaded: SaveData = load_from_slot(&game_name, SLOT)
            .unwrap_or_else(|error| panic!("cycle {cycle} restart failed: {error}"));
        state = loaded.state;

        assert!(state.combat.is_none());
        assert!(state.tower_run.is_none());
        assert!(!state.monster_roster.monsters.is_empty());
        assert!(state.activity_log.entries.len() <= 80);
    }

    assert!(victories > CYCLES / 2, "too few victories: {victories}");
    assert!(recoveries > 0, "the recovery branch was never exercised");
    assert!(slot_backup_exists(&game_name, SLOT));
    assert!(state.day > CYCLES as u32);
    let final_save_bytes = serde_json::to_vec(&state)
        .expect("final soak state should serialize")
        .len();
    assert!(final_save_bytes < 1_000_000);
    println!(
        "accelerated soak: {CYCLES} cycles, {victories} victories, {recoveries} recovery returns, day {}, {}-byte final state",
        state.day, final_save_bytes
    );
}
