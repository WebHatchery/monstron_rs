//! Saved, local-only counters used for an explicitly exported tester summary.

use serde::{Deserialize, Serialize};

use super::{CombatOutcome, GameState};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaytestMetrics {
    pub progression_actions: u64,
    pub days_advanced: u32,
    pub expeditions_started: u32,
    pub expeditions_returned: u32,
    pub rooms_explored: u32,
    pub combat_encounters: u32,
    pub combat_victories: u32,
    pub combat_defeats: u32,
    pub combat_flees: u32,
    pub monsters_hatched: u32,
    pub facility_levels_gained: u32,
}

impl PlaytestMetrics {
    pub(crate) fn record_transition(before: &GameState, after: &mut GameState) {
        let before_outcome = before.combat.as_ref().and_then(|combat| combat.outcome);
        let after_outcome = after.combat.as_ref().and_then(|combat| combat.outcome);
        let rooms_gained = rooms_gained(before, after);
        let facility_levels_gained = facility_levels(after).saturating_sub(facility_levels(before));
        let monsters_hatched = after
            .monster_roster
            .monsters
            .len()
            .saturating_sub(before.monster_roster.monsters.len())
            as u32;
        let metrics = &mut after.playtest_metrics;

        metrics.progression_actions = metrics.progression_actions.saturating_add(1);
        metrics.days_advanced = metrics
            .days_advanced
            .saturating_add(after.day.saturating_sub(before.day));
        metrics.expeditions_started = metrics.expeditions_started.saturating_add(u32::from(
            before.tower_run.is_none() && after.tower_run.is_some(),
        ));
        metrics.expeditions_returned = metrics.expeditions_returned.saturating_add(u32::from(
            before.tower_run.is_some() && after.tower_run.is_none(),
        ));
        metrics.rooms_explored = metrics.rooms_explored.saturating_add(rooms_gained);
        metrics.combat_encounters = metrics
            .combat_encounters
            .saturating_add(u32::from(before.combat.is_none() && after.combat.is_some()));

        if before_outcome != after_outcome {
            match after_outcome {
                Some(CombatOutcome::Victory) => {
                    metrics.combat_victories = metrics.combat_victories.saturating_add(1);
                }
                Some(CombatOutcome::Defeat) => {
                    metrics.combat_defeats = metrics.combat_defeats.saturating_add(1);
                }
                Some(CombatOutcome::Fled) => {
                    metrics.combat_flees = metrics.combat_flees.saturating_add(1);
                }
                None => {}
            }
        }

        metrics.monsters_hatched = metrics.monsters_hatched.saturating_add(monsters_hatched);
        metrics.facility_levels_gained = metrics
            .facility_levels_gained
            .saturating_add(facility_levels_gained);
    }
}

fn rooms_gained(before: &GameState, after: &GameState) -> u32 {
    let before_rooms = before
        .tower_run
        .as_ref()
        .map_or(0, |run| run.rooms_explored);
    let after_rooms = after
        .tower_run
        .as_ref()
        .map_or(before_rooms, |run| run.rooms_explored);
    after_rooms.saturating_sub(before_rooms)
}

fn facility_levels(state: &GameState) -> u32 {
    state
        .town
        .buildings
        .iter()
        .map(|building| building.level)
        .sum()
}
