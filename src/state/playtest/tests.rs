use super::*;
use crate::data::GameDataLoader;
use crate::state::{CombatState, TowerRunGoal, TowerRunState};

fn state() -> GameState {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    GameState::new(&data)
}

#[test]
fn transition_records_pacing_without_double_counting_existing_totals() {
    let before = state();
    let mut after = before.clone();
    after.day += 2;
    after.tower_run = Some(TowerRunState::new(1, 20, TowerRunGoal::Balanced));
    after.tower_run.as_mut().unwrap().rooms_explored = 3;
    after.town.set_building_level("hatchery", 1);

    PlaytestMetrics::record_transition(&before, &mut after);

    assert_eq!(after.playtest_metrics.progression_actions, 1);
    assert_eq!(after.playtest_metrics.days_advanced, 2);
    assert_eq!(after.playtest_metrics.expeditions_started, 1);
    assert_eq!(after.playtest_metrics.rooms_explored, 3);
    assert_eq!(after.playtest_metrics.facility_levels_gained, 1);
}

#[test]
fn combat_outcome_is_counted_only_when_it_first_appears() {
    let mut before = state();
    before.combat = Some(CombatState {
        floor: 1,
        round: 1,
        turn_index: 0,
        turn_order: Vec::new(),
        allies: Vec::new(),
        enemies: Vec::new(),
        rewards: Vec::new(),
        xp_reward: 0,
        log: Vec::new(),
        outcome: None,
        is_boss: false,
        rng_seed: 1,
        replay_roster: Vec::new(),
        replay_enemies: Vec::new(),
        replay_turn_order: Vec::new(),
        replay_round: 1,
        replay_turn_index: 0,
        command_history: Vec::new(),
    });
    let mut resolved = before.clone();
    resolved.combat.as_mut().unwrap().outcome = Some(CombatOutcome::Victory);
    PlaytestMetrics::record_transition(&before, &mut resolved);
    assert_eq!(resolved.playtest_metrics.combat_victories, 1);

    let mut continued = resolved.clone();
    PlaytestMetrics::record_transition(&resolved, &mut continued);
    assert_eq!(continued.playtest_metrics.combat_victories, 1);
}
