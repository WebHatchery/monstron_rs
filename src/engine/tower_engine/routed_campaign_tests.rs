use super::*;
use crate::data::GameDataLoader;
use crate::engine::{combat_engine, day_engine};
use crate::state::{CombatOutcome, TowerMapObjectKind};

#[test]
fn routed_expedition_reaches_every_floor_and_completes_the_crown() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = campaign_party(&data);
    start_run_on_floor(&mut state, &data, TowerRunGoal::PushDeeper, 1);

    for floor in 1..=10 {
        assert_eq!(
            state.tower_run.as_ref().map(|run| run.current_floor),
            Some(floor)
        );
        if floor > 1 {
            recover_and_reenter(&mut state, &data, floor);
        }
        if data
            .tower_floor(floor)
            .is_some_and(|definition| !definition.guardian_enemy_id.is_empty())
        {
            route_through_interruptions(&mut state, &data, TowerMapObjectKind::Boss);
            assert!(state
                .tower_run
                .as_ref()
                .is_some_and(|run| run.boss_defeated));
        }

        let destination = if floor == 10 {
            TowerMapObjectKind::Exit
        } else {
            TowerMapObjectKind::Stairs
        };
        route_through_interruptions(&mut state, &data, destination);
    }

    assert!(state.tower_run.is_none());
    assert!(state.story_flags.has("verdant_crown_restored"));
    assert_eq!(state.tower_progress.unlocked_floor, 10);
    assert_eq!(state.tower_progress.best_floor, 10);
}

fn route_through_interruptions(
    state: &mut GameState,
    data: &crate::data::GameData,
    destination: TowerMapObjectKind,
) {
    for _ in 0..2_000 {
        if state.tower_run.is_none() {
            return;
        }
        if state
            .tower_run
            .as_ref()
            .is_some_and(|run| run.pressure >= run.pressure_limit)
        {
            camp_party(state, data);
            continue;
        }
        let floor_before = state.tower_run.as_ref().unwrap().current_floor;
        let target = state
            .tower_run
            .as_ref()
            .unwrap()
            .map
            .objects
            .iter()
            .find(|object| object.kind == destination)
            .map(|object| (object.x, object.y))
            .expect("generated floor should retain its routed destination");
        let result = route_party_to(state, data, target);
        if let Some(encounter) = result.encounter {
            combat_engine::start_named_encounter(
                state,
                data,
                encounter.floor,
                encounter.is_boss,
                encounter.enemy_id.as_deref(),
            );
            win_current_combat(state, data);
        }
        if state
            .tower_run
            .as_ref()
            .is_some_and(|run| run.pending_event.is_some())
        {
            leave_special_event(state, data);
        }
        if result.completed_tower
            || state.tower_run.is_none()
            || state
                .tower_run
                .as_ref()
                .is_some_and(|run| run.current_floor != floor_before)
            || !state
                .tower_run
                .as_ref()
                .unwrap()
                .map
                .objects
                .iter()
                .any(|object| object.kind == destination)
        {
            return;
        }
    }
    panic!("routed expedition did not reach {destination:?}");
}

fn win_current_combat(state: &mut GameState, data: &crate::data::GameData) {
    for _ in 0..300 {
        let Some(combat) = state.combat.as_ref() else {
            break;
        };
        if combat.outcome.is_some() {
            break;
        }
        combat_engine::player_action(state, data, combat_engine::CombatCommand::Skill);
    }
    assert_eq!(
        state.combat.as_ref().and_then(|combat| combat.outcome),
        Some(CombatOutcome::Victory)
    );
    combat_engine::finish_combat(state, data);
}

fn recover_and_reenter(state: &mut GameState, data: &crate::data::GameData, floor: u32) {
    return_to_town(state, data);
    for _ in 0..2 {
        day_engine::sleep(state, data);
    }
    start_run_on_floor(state, data, TowerRunGoal::PushDeeper, floor);
    assert_eq!(battle_ready_party_count(state), 6);
}

fn campaign_party(data: &crate::data::GameData) -> GameState {
    let mut state = GameState::new(data);
    state.monster_roster.monsters.clear();
    state.monster_roster.party_slots.fill(None);
    for (index, species_id) in [
        "slime",
        "rootling",
        "emberkit",
        "pebblepup",
        "rillfin",
        "glowmoth",
    ]
    .iter()
    .enumerate()
    {
        let species = data.species(species_id).unwrap();
        let id = state.monster_roster.add_monster(
            format!("{} Wayfinder", species.name),
            species,
            10_000 + index as u64,
        );
        let monster = state.monster_roster.monster_mut(id).unwrap();
        let curve = data.stat_curve(species_id).unwrap();
        for level in 2..=10 {
            monster.level = level;
            monster.max_hp += curve.hp_per_level;
            monster.attack += curve.attack_per_level;
            monster.defense += curve.defense_per_level;
            if level.is_multiple_of(curve.speed_interval) {
                monster.speed += curve.speed_per_interval;
            }
        }
        monster.hp = monster.max_hp;
        monster.bond = 5;
        state.monster_roster.party_slots[index] = Some(id);
    }
    state
}
