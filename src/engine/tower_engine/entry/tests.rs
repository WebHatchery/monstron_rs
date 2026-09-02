use super::*;
use crate::data::GameDataLoader;

#[test]
fn unlocked_earlier_floors_can_be_revisited() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.tower_progress.unlocked_floor = 6;

    let result = start_run_on_floor(&mut state, &data, TowerRunGoal::Scout, 2);

    assert!(result.summary.contains("floor 2: Lantern Hollows"));
    assert_eq!(state.tower_run.as_ref().unwrap().current_floor, 2);
    assert_eq!(state.tower_run.as_ref().unwrap().map.floor, 2);
}

#[test]
fn sealed_floor_is_rejected_without_committing_the_party() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.tower_progress.unlocked_floor = 3;

    let result = start_run_on_floor(&mut state, &data, TowerRunGoal::Balanced, 4);

    assert!(result.summary.contains("still sealed"));
    assert!(state.tower_run.is_none());
    assert!(state
        .monster_roster
        .monsters
        .iter()
        .all(|monster| monster.condition.commitment == DailyCommitment::Free));
}

#[test]
fn default_entry_still_uses_the_deepest_unlocked_floor() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.tower_progress.unlocked_floor = 4;

    start_run(&mut state, &data, TowerRunGoal::Balanced);

    assert_eq!(state.tower_run.as_ref().unwrap().current_floor, 4);
}
