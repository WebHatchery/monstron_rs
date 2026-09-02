use super::*;
use crate::data::GameDataLoader;
use crate::engine::combat_engine;
use crate::state::CombatOutcome;

#[test]
fn entering_and_retreating_does_not_unlock_a_deeper_floor() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);

    start_run(&mut state, &data, TowerRunGoal::SafeRun);
    return_to_town(&mut state, &data);

    assert_eq!(state.tower_progress.best_floor, 1);
    assert_eq!(state.tower_progress.unlocked_floor, 1);
}

#[test]
fn ordinary_combat_victory_does_not_unlock_a_deeper_floor() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    start_run(&mut state, &data, TowerRunGoal::Balanced);
    combat_engine::start_encounter(&mut state, &data, 1, false);
    state.combat.as_mut().unwrap().outcome = Some(CombatOutcome::Victory);

    combat_engine::finish_combat(&mut state, &data);

    assert_eq!(state.tower_progress.best_floor, 1);
    assert_eq!(state.tower_progress.unlocked_floor, 1);
}

#[test]
fn each_cleared_stair_unlocks_exactly_the_floor_it_reaches() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    start_run(&mut state, &data, TowerRunGoal::PushDeeper);

    for expected_floor in 2..=10 {
        if state
            .tower_run
            .as_ref()
            .is_some_and(|run| guardian_gate_is_sealed(&data, run))
        {
            state.tower_run.as_mut().unwrap().boss_defeated = true;
        }
        let stairs = state
            .tower_run
            .as_ref()
            .unwrap()
            .map
            .objects
            .iter()
            .find(|object| object.kind == TowerMapObjectKind::Stairs)
            .cloned()
            .expect("every non-final floor should expose a stair");

        let result = resolve_map_object(&mut state, &data, stairs);

        assert!(result.summary.contains(&format!("floor {expected_floor}")));
        assert!(result
            .summary
            .contains(&format!("Floor {expected_floor} unlocked")));
        assert_eq!(state.tower_progress.best_floor, expected_floor);
        assert_eq!(state.tower_progress.unlocked_floor, expected_floor);
        assert_eq!(
            state.tower_run.as_ref().unwrap().current_floor,
            expected_floor
        );
    }
}
