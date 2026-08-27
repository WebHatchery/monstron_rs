//! Explicit, local-only export of saved pacing and balance facts.

use std::path::PathBuf;

use crate::data::GameData;
use crate::state::GameState;

const GAME_NAME: &str = "hatchspire";
const REPORT_FILE: &str = "tester_summary.txt";
const TEST_PATH_ENV: &str = "HATCHSPIRE_TESTER_SUMMARY_PATH";

#[cfg(test)]
mod tests;

pub fn render(state: &GameState, data: &GameData, build_id: &str) -> String {
    let metrics = &state.playtest_metrics;
    let expedition_return_rate = percent(metrics.expeditions_returned, metrics.expeditions_started);
    let resolved_combats = metrics
        .combat_victories
        .saturating_add(metrics.combat_defeats)
        .saturating_add(metrics.combat_flees);
    let combat_win_rate = percent(metrics.combat_victories, resolved_combats);

    let resources = data
        .resources
        .iter()
        .map(|resource| {
            format!(
                "- {}: {}",
                resource.name,
                state.resources.amount(&resource.id)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let facilities = data
        .buildings
        .iter()
        .map(|building| {
            format!(
                "- {}: level {}",
                building.name,
                state.town.building_level(&building.id)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "HATCHSPIRE LOCAL TESTER SUMMARY\n\
Build: {build_id}\n\
\n\
This file was created only because the player tapped EXPORT LOCAL SUMMARY.\n\
It contains saved game counters only. Hatchspire did not transmit or upload it.\n\
Counters begin at zero for a new game or when an older save first loads in a metrics-capable build.\n\
\n\
PACING\n\
- Current day: {day}\n\
- Recorded progression actions: {actions}\n\
- Days advanced: {days_advanced}\n\
- Expeditions started: {expeditions_started}\n\
- Expeditions returned: {expeditions_returned}\n\
- Expedition return rate: {expedition_return_rate}\n\
- Rooms explored: {rooms_explored}\n\
- Highest floor reached: {best_floor}\n\
- Highest floor unlocked: {unlocked_floor}\n\
\n\
BALANCE\n\
- Combat encounters: {combat_encounters}\n\
- Victories: {victories}\n\
- Defeats: {defeats}\n\
- Fled combats: {flees}\n\
- Combat win rate: {combat_win_rate}\n\
- Monsters hatched: {hatched}\n\
- Current roster size: {roster}\n\
- Eggs currently held: {eggs}\n\
- Facility levels gained: {facility_levels_gained}\n\
\n\
CURRENT RESOURCES\n{resources}\n\
\n\
CURRENT FACILITIES\n{facilities}\n",
        day = state.day,
        actions = metrics.progression_actions,
        days_advanced = metrics.days_advanced,
        expeditions_started = metrics.expeditions_started,
        expeditions_returned = metrics.expeditions_returned,
        rooms_explored = metrics.rooms_explored,
        best_floor = state.tower_progress.best_floor,
        unlocked_floor = state.tower_progress.unlocked_floor,
        combat_encounters = metrics.combat_encounters,
        victories = metrics.combat_victories,
        defeats = metrics.combat_defeats,
        flees = metrics.combat_flees,
        hatched = metrics.monsters_hatched,
        roster = state.monster_roster.monsters.len(),
        eggs = state.egg_inventory.eggs.len(),
        facility_levels_gained = metrics.facility_levels_gained,
    )
}

pub fn export(state: &GameState, data: &GameData) -> Result<PathBuf, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = report_path()?;
        let content = render(state, data, crate::build_info::BUILD_ID);
        macroquad_toolkit::persistence::save_string_atomic(&path, &content)?;
        Ok(path)
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = (state, data);
        Err("Local tester-summary export is available in the Windows build.".to_owned())
    }
}

pub fn location_description() -> String {
    if !is_supported() {
        return "Local tester-summary export is available in the Windows build.".to_owned();
    }
    report_path()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|error| error)
}

pub const fn is_supported() -> bool {
    cfg!(not(target_arch = "wasm32"))
}

fn report_path() -> Result<PathBuf, String> {
    macroquad_toolkit::persistence::get_configured_save_path(
        GAME_NAME,
        REPORT_FILE,
        Some(TEST_PATH_ENV),
    )
    .ok_or_else(|| "Windows could not determine the tester-summary location.".to_owned())
}

fn percent(numerator: u32, denominator: u32) -> String {
    if denominator == 0 {
        "not available".to_owned()
    } else {
        format!("{}%", u64::from(numerator) * 100 / u64::from(denominator))
    }
}
