//! Expedition entry and saved-map recovery.

use super::*;
use crate::state::DailyCommitment;

pub fn start_run(state: &mut GameState, data: &GameData, goal: TowerRunGoal) -> TowerResult {
    let deepest_floor = state
        .tower_progress
        .unlocked_floor
        .max(1)
        .min(max_floor(data));
    start_run_on_floor(state, data, goal, deepest_floor)
}

pub fn start_run_on_floor(
    state: &mut GameState,
    data: &GameData,
    goal: TowerRunGoal,
    requested_floor: u32,
) -> TowerResult {
    if state.tower_run.is_some() {
        return result("The party is already inside the tower.");
    }

    let max_available = state
        .tower_progress
        .unlocked_floor
        .max(1)
        .min(max_floor(data));
    if requested_floor == 0 || requested_floor > max_available {
        return result(format!(
            "Floor {requested_floor} is still sealed. Choose floor 1 through {max_available}."
        ));
    }

    let Some(floor) = data.tower_floor(requested_floor) else {
        return result(format!(
            "Missing tower floor data for floor {requested_floor}. Tap Town to return."
        ));
    };
    let ready_members = available_party_ids(state);
    if ready_members.is_empty() {
        return result(
            "Assign at least one rested, uncommitted monster to the party before entering the tower. Tap Stable.",
        );
    }

    for monster_id in ready_members {
        monster_engine::mark_commitment(state, monster_id, DailyCommitment::Tower);
    }

    let seed = tower_seed(state, requested_floor, goal, 0);
    let map = generate_map(state, data, requested_floor, goal, seed);
    let anomaly_id = select_anomaly_id(data, requested_floor, seed);
    let anomaly_name = data
        .tower_anomaly(&anomaly_id)
        .map(|anomaly| anomaly.name.as_str())
        .unwrap_or("No anomaly");
    let guide_bonus = state.tower_discoveries.survey_bonus();
    let mut run = TowerRunState::new(requested_floor, floor.pressure_limit, goal).with_map(map);
    run.survey_charges = run.survey_charges.saturating_add(guide_bonus).min(5);
    run.anomaly_id = anomaly_id;
    state.tower_run = Some(run);
    record_visible_discoveries(state, data, None);
    let guide_note = if guide_bonus > 0 {
        format!(" Field Guide expertise adds {guide_bonus} survey flare(s).")
    } else {
        String::new()
    };
    let summary = format!(
        "The party enters floor {}: {} under {}. Move through the map to find event sites, hazards, stairs, eggs, caches, and denizens.{guide_note}",
        floor.floor, floor.name, anomaly_name
    );
    state.activity_log.add(state.day, summary.clone());

    result(summary)
}

pub fn ensure_map(state: &mut GameState, data: &GameData) {
    let Some(run) = &state.tower_run else {
        return;
    };
    let needs_map = run.map.is_empty();

    if needs_map {
        let floor = run.current_floor.max(1).min(max_floor(data));
        let goal = run.goal;
        let seed = tower_seed(state, floor, goal, run.rooms_explored);
        let map = generate_map(state, data, floor, goal, seed);
        if let Some(run) = &mut state.tower_run {
            run.current_floor = floor;
            run.map = map;
            if run.anomaly_id.is_empty() {
                run.anomaly_id = select_anomaly_id(data, floor, seed);
            }
            run.add_event(format!("Generated a map for floor {floor}."));
        }
        record_visible_discoveries(state, data, None);
        return;
    }

    if let Some(run) = &mut state.tower_run {
        if run.anomaly_id.is_empty() {
            run.anomaly_id = select_anomaly_id(data, run.current_floor, run.map.seed);
        }
        let restored_visibility = run.map.ensure_visibility();
        let restored_room_kinds = run.map.ensure_room_kinds();
        let restored_room_art = run.map.ensure_room_art_variants();
        if restored_visibility || !run.map.is_visible(run.map.player_x, run.map.player_y) {
            reveal_current_area(&mut run.map);
        }
        if restored_visibility || restored_room_kinds || restored_room_art {
            run.add_event("Recovered the party's map notes and room markings.".to_owned());
        }
    }
    record_visible_discoveries(state, data, None);
}

#[cfg(test)]
mod tests;
