mod anomalies;
#[cfg(test)]
mod anomaly_runtime_tests;
#[cfg(test)]
mod blessing_tests;
#[cfg(test)]
mod boss_gate_tests;
mod camp;
mod contracts;
mod discovery;
mod event_choices;
mod exploration_talents;
mod interactions;
mod map_gen;
mod map_objects;
mod navigation;
mod pressure;
#[cfg(test)]
mod secret_tests;
mod survey;
pub use camp::{camp_party, camp_sheltered};
pub use event_choices::{choose_special_event, event_choice_available, leave_special_event};
pub use survey::survey_floor;
#[cfg(test)]
mod tests;

use crate::data::{GameData, PassiveSkill, TowerBlessing};
use crate::engine::{monster_engine, town_engine};
use crate::state::{
    DailyCommitment, GameState, ResourceStack, TowerFoundEgg, TowerMapObject, TowerMapObjectKind,
    TowerRunGoal, TowerRunState,
};
use anomalies::{anomaly_effect, select_anomaly_id};
pub use contracts::contract_progress;
use contracts::refresh_contract;
use discovery::record_visible_discoveries;
use event_choices::resolve_special_location;
use exploration_talents::{party_has_passive, reveal_secret_in_current_room};
use interactions::resolve_hazard;
use map_gen::{generate_map, reveal_current_area};
use map_objects::{advance_wandering_enemy, WanderingAdvance};
use navigation::{explore_direction, route_direction};
use pressure::refresh_pressure;

pub struct TowerResult {
    pub summary: String,
    pub encounter: Option<TowerEncounterRequest>,
    pub returned_to_town: bool,
    pub completed_tower: bool,
}

pub struct TowerEncounterRequest {
    pub floor: u32,
    pub is_boss: bool,
    pub enemy_id: Option<String>,
}

pub fn start_run(state: &mut GameState, data: &GameData, goal: TowerRunGoal) -> TowerResult {
    if state.tower_run.is_some() {
        return result("The party is already inside the tower.");
    }

    let ready_members = available_party_ids(state);
    if ready_members.is_empty() {
        return result(
            "Assign at least one rested, uncommitted monster to the party before entering the tower. Tap Stable.",
        );
    }

    let start_floor = state
        .tower_progress
        .unlocked_floor
        .max(1)
        .min(max_floor(data));
    let Some(floor) = data.tower_floor(start_floor) else {
        return result(format!(
            "Missing tower floor data for floor {start_floor}. Tap Town to return."
        ));
    };

    for monster_id in ready_members {
        monster_engine::mark_commitment(state, monster_id, DailyCommitment::Tower);
    }

    let seed = tower_seed(state, start_floor, goal, 0);
    let map = generate_map(state, data, start_floor, goal, seed);
    let anomaly_id = select_anomaly_id(data, start_floor, seed);
    let anomaly_name = data
        .tower_anomaly(&anomaly_id)
        .map(|anomaly| anomaly.name.as_str())
        .unwrap_or("No anomaly");
    let guide_bonus = state.tower_discoveries.survey_bonus();
    let mut run = TowerRunState::new(start_floor, floor.pressure_limit, goal).with_map(map);
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
        "The party enters floor {}: {} under {}. Move through the map to find event sites, hazards, stairs, eggs, caches, and denizens.",
        floor.floor, floor.name, anomaly_name
    );
    let summary = format!("{summary}{guide_note}");
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

pub fn move_party(state: &mut GameState, data: &GameData, dx: i32, dy: i32) -> TowerResult {
    if state.tower_run.is_none() {
        return result("No tower run is active. Tap Town to choose a run.");
    }
    if dx == 0 && dy == 0 {
        return result("The party waits and listens.");
    }

    ensure_map(state, data);

    if state
        .tower_run
        .as_ref()
        .is_some_and(|run| run.pressure >= run.pressure_limit)
    {
        return result("The tower is fully awake. Tap CAMP to recover or RETREAT with the cargo.");
    }

    let has_loot_finder = party_has_passive(state, PassiveSkill::FindsSmallLoot);
    let (object, hunter_moved, secrets_revealed) = {
        let Some(run) = &mut state.tower_run else {
            return result("No tower run is active. Tap Town to choose a run.");
        };
        if run.map.is_empty() {
            return result(
                "No dungeon map is available. Tap Return to Town and re-enter the tower.",
            );
        }

        let next_x = run.map.player_x as i32 + dx;
        let next_y = run.map.player_y as i32 + dy;
        if next_x < 0 || next_y < 0 || !run.map.is_passable(next_x as u32, next_y as u32) {
            return result("A wall blocks the way. Tap a highlighted adjacent tile.");
        }

        run.map.player_x = next_x as u32;
        run.map.player_y = next_y as u32;
        if run.route_target == Some((run.map.player_x, run.map.player_y)) {
            run.route_target = None;
        }
        run.rooms_explored += 1;
        run.camp_cooldown = run.camp_cooldown.saturating_sub(1);
        let anomaly = anomaly_effect(run, data);
        let base_pressure_interval: u32 = match anomaly {
            Some(crate::data::TowerAnomalyEffect::QuietVeil) => 6,
            Some(crate::data::TowerAnomalyEffect::EchoingRain) => 3,
            _ => 4,
        };
        let pressure_interval = if run.has_blessing(TowerBlessing::QuietSteps) {
            base_pressure_interval.saturating_add(2).min(7)
        } else {
            base_pressure_interval
        };
        if run.rooms_explored.is_multiple_of(pressure_interval) {
            run.pressure = run.pressure.saturating_add(1).min(run.pressure_limit);
        }
        reveal_current_area(&mut run.map);
        let secrets_revealed = if has_loot_finder {
            reveal_secret_in_current_room(&mut run.map)
        } else {
            0
        };
        let object = run
            .map
            .object_index_at(run.map.player_x, run.map.player_y)
            .map(|index| run.map.objects.remove(index));
        if object.is_some() {
            // Discoveries are deliberate travel interruption points. The player
            // chooses whether to resume after reading the result or resolving
            // the encounter instead of being pulled onward by an old room tap.
            run.route_target = None;
        }
        let hunter_interval = if anomaly == Some(crate::data::TowerAnomalyEffect::HunterTracks) {
            1
        } else {
            2
        };
        let advance = if object.is_none() && run.rooms_explored.is_multiple_of(hunter_interval) {
            advance_wandering_enemy(&mut run.map)
        } else {
            None
        };
        match advance {
            Some(WanderingAdvance::Encounter(hunter)) => (Some(hunter), false, secrets_revealed),
            Some(WanderingAdvance::Moved) => (object, true, secrets_revealed),
            None => (object, false, secrets_revealed),
        }
    };

    record_visible_discoveries(state, data, object.as_ref());

    let Some(object) = object else {
        let pressure_warning = state
            .tower_run
            .as_ref()
            .filter(|run| run.pressure + 2 >= run.pressure_limit)
            .map(|run| {
                format!(
                    " Tower pressure is {}/{}.",
                    run.pressure, run.pressure_limit
                )
            })
            .unwrap_or_default();
        let hunter_warning = if hunter_moved {
            " A wandering hunter closes in."
        } else {
            ""
        };
        let talent_notice = if secrets_revealed > 0 {
            " A loot-finder hears a hollow seam: a concealed cache appears on the map."
        } else {
            ""
        };
        return with_contract_refresh(
            result(format!(
                "The party advances through the dungeon.{pressure_warning}{hunter_warning}{talent_notice}"
            )),
            state,
            data,
        );
    };

    let mut outcome = resolve_map_object(state, data, object);
    if secrets_revealed > 0 && !outcome.summary.contains("surveyed secret") {
        outcome
            .summary
            .push_str(" A loot-finder also exposes a concealed cache in this room.");
    }
    with_contract_refresh(outcome, state, data)
}

pub fn explore_party(state: &mut GameState, data: &GameData) -> TowerResult {
    ensure_map(state, data);
    let focused_direction = state.tower_run.as_ref().and_then(|run| {
        run.route_target
            .and_then(|target| route_direction(&run.map, run.goal, target.0, target.1))
    });
    if let Some(direction) = focused_direction {
        return move_party(state, data, direction.0, direction.1);
    }
    if let Some(run) = &mut state.tower_run {
        run.route_target = None;
    }
    let shelter_plan = state.tower_run.as_ref().and_then(|run| {
        (run.goal == TowerRunGoal::SafeRun
            && run.camp_cooldown == 0
            && run.pressure + 2 >= run.pressure_limit)
            .then(|| {
                if camp_sheltered(run) {
                    None
                } else {
                    camp::camp_room_center(run)
                        .and_then(|target| route_direction(&run.map, run.goal, target.0, target.1))
                }
            })
    });
    if shelter_plan == Some(None) {
        return result(
            "The Safe Run route has reached the marked shelter. Tap CAMP to calm the tower.",
        );
    }
    if let Some(Some(direction)) = shelter_plan {
        let mut outcome = move_party(state, data, direction.0, direction.1);
        outcome
            .summary
            .push_str(" Safe Run is routing back toward the marked shelter.");
        return outcome;
    }
    let Some(direction) = state
        .tower_run
        .as_ref()
        .and_then(|run| explore_direction(&run.map, run.goal))
    else {
        return result("No unexplored route is reachable. Tap RETREAT or choose a visible room.");
    };
    move_party(state, data, direction.0, direction.1)
}

pub fn route_party_to(state: &mut GameState, data: &GameData, target: (u32, u32)) -> TowerResult {
    ensure_map(state, data);
    let Some(run) = &mut state.tower_run else {
        return result("No tower run is active. Tap Town to choose a run.");
    };
    if !run.map.is_passable(target.0, target.1) {
        return result("That room is not reachable. Tap a visible chamber.");
    }
    let Some(direction) = route_direction(&run.map, run.goal, target.0, target.1) else {
        run.route_target = None;
        return result("No route reaches that room. Tap another chamber or SURVEY.");
    };
    run.route_target = Some(target);
    let mut outcome = move_party(state, data, direction.0, direction.1);
    if state.tower_run.is_none() {
        return outcome;
    }
    if state
        .tower_run
        .as_ref()
        .is_some_and(|run| (run.map.player_x, run.map.player_y) == target)
    {
        if let Some(run) = &mut state.tower_run {
            run.route_target = None;
        }
        outcome
            .summary
            .push_str(" The party reaches the focused room.");
    } else {
        outcome
            .summary
            .push_str(" The party follows the marked route; tap another room to redirect.");
    }
    outcome
}

pub fn room_tap_direction(run: &TowerRunState, target: (u32, u32)) -> Option<(i32, i32)> {
    route_direction(&run.map, run.goal, target.0, target.1)
}

pub fn return_to_town(state: &mut GameState, data: &GameData) -> TowerResult {
    let Some(run) = state.tower_run.take() else {
        return result("No tower run is active. Tap Town to choose a run.");
    };

    record_floor_reached(state, data, run.current_floor);

    for stack in &run.cargo {
        state.resources.add(&stack.resource_id, stack.amount);
    }

    let egg_capacity = town_engine::egg_capacity(state);
    let available_egg_slots = egg_capacity.saturating_sub(state.egg_inventory.eggs.len());
    let mut egg_count = 0;
    for found_egg in run.found_eggs.iter().take(available_egg_slots) {
        state.egg_inventory.add_egg(
            found_egg.egg_type_id.clone(),
            found_egg.hatch_days,
            found_egg.origin_floor,
            found_egg.palette_seed,
        );
        egg_count += 1;
    }
    let eggs_left_behind = run.found_eggs.len().saturating_sub(egg_count);

    let cargo_label = cargo_text(data, &run.cargo);
    let mut summary = format!(
        "Returned from floor {} with {} and {} egg(s).",
        run.current_floor, cargo_label, egg_count
    );
    if eggs_left_behind > 0 {
        summary.push_str(&format!(
            " Hatchery capacity left {} egg(s) behind ({}/{}).",
            eggs_left_behind,
            state.egg_inventory.eggs.len(),
            egg_capacity
        ));
    }
    state.activity_log.add(state.day, summary.clone());

    TowerResult {
        summary,
        encounter: None,
        returned_to_town: true,
        completed_tower: false,
    }
}

pub fn cargo_text(data: &GameData, cargo: &[ResourceStack]) -> String {
    if cargo.is_empty() {
        return "no materials".to_owned();
    }

    cargo
        .iter()
        .filter(|stack| stack.amount > 0)
        .map(|stack| {
            format!(
                "{} {}",
                stack.amount,
                data.resource_name(&stack.resource_id)
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn party_count(state: &GameState) -> usize {
    state
        .monster_roster
        .party_slots
        .iter()
        .filter(|slot| slot.is_some())
        .count()
}

pub fn battle_ready_party_count(state: &GameState) -> usize {
    state
        .monster_roster
        .party_slots
        .iter()
        .filter_map(|slot| state.monster_roster.monster((*slot)?))
        .filter(|monster| monster.is_battle_ready() && state.town.monster_job(monster.id).is_none())
        .count()
}

fn resolve_map_object(
    state: &mut GameState,
    data: &GameData,
    object: TowerMapObject,
) -> TowerResult {
    if object.kind == TowerMapObjectKind::SecretCache && !object.revealed {
        if let Some(run) = &mut state.tower_run {
            run.map.objects.push(object);
        }
        return result("The room facade holds, but something sounds hollow. Tap SURVEY to inspect hidden chambers.");
    }
    match object.kind {
        TowerMapObjectKind::Loot | TowerMapObjectKind::SecretCache => {
            let secret = object.kind == TowerMapObjectKind::SecretCache;
            let resource_name = data.resource_name(&object.resource_id).to_owned();
            if let Some(run) = &mut state.tower_run {
                let blessing_bonus = if run.has_blessing(TowerBlessing::CacheSense) {
                    2
                } else {
                    0
                };
                let anomaly_bonus = if anomaly_effect(run, data)
                    == Some(crate::data::TowerAnomalyEffect::CacheBloom)
                {
                    2
                } else {
                    0
                };
                let amount = object.amount + blessing_bonus + anomaly_bonus;
                run.add_cargo(&object.resource_id, amount);
                let summary = if secret {
                    format!(
                        "Opened a surveyed secret: found {} {} behind the room facade.",
                        amount, resource_name
                    )
                } else {
                    format!("Found {} {} in a tower cache.", amount, resource_name)
                };
                run.add_event(summary.clone());
                result(summary)
            } else {
                result("No tower run is active. Tap Town to choose a run.")
            }
        }
        TowerMapObjectKind::Egg => {
            let egg_name = data
                .egg_type(&object.egg_type_id)
                .map(|egg| egg.name.as_str())
                .unwrap_or(object.egg_type_id.as_str())
                .to_owned();
            if let Some(run) = &mut state.tower_run {
                let hatch_bonus = if anomaly_effect(run, data)
                    == Some(crate::data::TowerAnomalyEffect::NestingPulse)
                {
                    1
                } else {
                    0
                };
                run.found_eggs.push(TowerFoundEgg {
                    egg_type_id: object.egg_type_id,
                    hatch_days: object.hatch_days.saturating_sub(hatch_bonus),
                    origin_floor: run.current_floor,
                    palette_seed: object.palette_seed,
                });
                let summary = format!("Found a {egg_name} in a quiet nest.");
                run.add_event(summary.clone());
                result(summary)
            } else {
                result("No tower run is active. Tap Town to choose a run.")
            }
        }
        TowerMapObjectKind::Enemy | TowerMapObjectKind::Boss => {
            let is_boss = object.kind == TowerMapObjectKind::Boss;
            let floor = state
                .tower_run
                .as_ref()
                .map(|run| run.current_floor)
                .unwrap_or(1);
            let label = if is_boss {
                "boss".to_owned()
            } else if object.wandering {
                data.enemy(&object.enemy_id)
                    .map(|enemy| format!("wandering {}", enemy.name))
                    .unwrap_or_else(|| "wandering enemy".to_owned())
            } else {
                "enemy".to_owned()
            };
            let verb = if object.wandering {
                "catches the party"
            } else {
                "blocks the tile"
            };
            let summary = format!("A {label} {verb}. Combat starts.");
            if let Some(run) = &mut state.tower_run {
                run.add_event(summary.clone());
            }
            TowerResult {
                summary,
                encounter: Some(TowerEncounterRequest {
                    floor,
                    is_boss,
                    enemy_id: (!object.enemy_id.is_empty()).then_some(object.enemy_id),
                }),
                returned_to_town: false,
                completed_tower: false,
            }
        }
        TowerMapObjectKind::Hazard => resolve_hazard(state, data, &object.hazard_id),
        TowerMapObjectKind::SpecialLocation => resolve_special_location(state, data, object),
        TowerMapObjectKind::Stairs => resolve_stairs(state, data, object),
        TowerMapObjectKind::Exit => resolve_exit(state, data, object),
    }
}

fn resolve_exit(state: &mut GameState, data: &GameData, object: TowerMapObject) -> TowerResult {
    let sealed = state
        .tower_run
        .as_ref()
        .is_some_and(|run| guardian_gate_is_sealed(data, run));
    if sealed {
        if let Some(run) = &mut state.tower_run {
            run.map.objects.push(object);
            let summary =
                "The crown threshold is sealed. Defeat the floor guardian or tap RETREAT.";
            run.add_event(summary.to_owned());
            return result(summary);
        }
    }
    if state
        .tower_run
        .as_ref()
        .is_some_and(|run| run.current_floor == max_floor(data) && run.boss_defeated)
    {
        return complete_tower(state, data);
    }
    return_to_town(state, data)
}

fn resolve_stairs(state: &mut GameState, data: &GameData, object: TowerMapObject) -> TowerResult {
    let sealed = state
        .tower_run
        .as_ref()
        .is_some_and(|run| guardian_gate_is_sealed(data, run));
    if sealed {
        if let Some(run) = &mut state.tower_run {
            run.map.objects.push(object);
            let summary = "A floor guardian seals the deeper stair. Defeat it or tap RETREAT.";
            run.add_event(summary.to_owned());
            return result(summary);
        }
    }
    advance_floor(state, data)
}

fn guardian_gate_is_sealed(data: &GameData, run: &TowerRunState) -> bool {
    !run.boss_defeated
        && data
            .tower_floor(run.current_floor)
            .is_some_and(|floor| floor.is_boss_floor || !floor.guardian_enemy_id.is_empty())
}

fn advance_floor(state: &mut GameState, data: &GameData) -> TowerResult {
    let Some(run) = &state.tower_run else {
        return result("No tower run is active. Tap Town to choose a run.");
    };
    let next_floor = (run.current_floor + 1).min(max_floor(data));
    if next_floor == run.current_floor {
        if !run.boss_defeated {
            return result("The living crown bars the final threshold. Defeat its guardian.");
        }
        return complete_tower(state, data);
    }

    let Some(next_floor_data) = data.tower_floor(next_floor) else {
        return result(format!("Missing tower floor data for floor {next_floor}."));
    };

    record_floor_reached(state, data, next_floor);
    let goal = state
        .tower_run
        .as_ref()
        .map(|run| run.goal)
        .unwrap_or_default();
    let step_count = state
        .tower_run
        .as_ref()
        .map(|run| run.rooms_explored)
        .unwrap_or(0);
    let seed = tower_seed(state, next_floor, goal, step_count);
    let map = generate_map(state, data, next_floor, goal, seed);
    let anomaly_id = select_anomaly_id(data, next_floor, seed);
    let anomaly_name = data
        .tower_anomaly(&anomaly_id)
        .map(|anomaly| anomaly.name.as_str())
        .unwrap_or("No anomaly");
    let guide_bonus = state.tower_discoveries.survey_bonus();

    if let Some(run) = &mut state.tower_run {
        run.current_floor = next_floor;
        run.stats.floors_descended += 1;
        run.pressure_limit = next_floor_data.pressure_limit;
        run.boss_defeated = false;
        run.survey_charges = crate::state::survey_charges_for(run.goal)
            .saturating_add(guide_bonus)
            .min(5);
        run.anomaly_id = anomaly_id;
        run.route_target = None;
        run.map = map;
        let guide_note = if guide_bonus > 0 {
            format!(" Field Guide expertise packs {guide_bonus} extra survey flare(s).")
        } else {
            String::new()
        };
        let summary = format!(
            "Descended to floor {}: {} under {}. A fresh map unfolds.{guide_note}",
            next_floor_data.floor, next_floor_data.name, anomaly_name
        );
        run.add_event(summary.clone());
        return result(summary);
    }

    result("No tower run is active. Tap Town to choose a run.")
}

fn available_party_ids(state: &GameState) -> Vec<u64> {
    state
        .monster_roster
        .party_slots
        .iter()
        .filter_map(|slot| {
            let monster = state.monster_roster.monster((*slot)?)?;
            if monster_engine::can_take_daily_action(state, monster).is_ok() {
                Some(monster.id)
            } else {
                None
            }
        })
        .collect()
}

fn record_floor_reached(state: &mut GameState, data: &GameData, floor: u32) {
    state.tower_progress.best_floor = state.tower_progress.best_floor.max(floor);

    if let Some(floor_data) = data.tower_floor(floor) {
        let unlocked = floor_data.unlocks_floor.max(floor);
        state.tower_progress.unlocked_floor = state
            .tower_progress
            .unlocked_floor
            .max(unlocked)
            .min(max_floor(data));
    }
}

fn tower_seed(state: &GameState, floor: u32, goal: TowerRunGoal, salt: u32) -> u64 {
    0x544F_5745_524D_4150
        ^ u64::from(state.day).wrapping_mul(97)
        ^ u64::from(floor).wrapping_mul(193)
        ^ u64::from(salt).wrapping_mul(389)
        ^ state.egg_inventory.next_id.wrapping_mul(53)
        ^ (goal as u64).wrapping_mul(577)
}

fn max_floor(data: &GameData) -> u32 {
    data.tower_floors
        .iter()
        .map(|floor| floor.floor)
        .max()
        .unwrap_or(1)
}

fn result(summary: impl Into<String>) -> TowerResult {
    TowerResult {
        summary: summary.into(),
        encounter: None,
        returned_to_town: false,
        completed_tower: false,
    }
}

fn complete_tower(state: &mut GameState, data: &GameData) -> TowerResult {
    let Some(run) = state.tower_run.as_ref() else {
        return result("No tower run is active. Tap Town to choose a run.");
    };
    let floor = run.current_floor;
    let rooms = run.rooms_explored;
    let landmarks = run.stats.landmarks_resolved;
    let first_victory = !state.story_flags.has("verdant_crown_restored");

    let mut outcome = return_to_town(state, data);
    state.story_flags.add("verdant_crown_restored");
    let return_note = outcome.summary;
    outcome.summary = if first_victory {
        format!(
            "The Verdant Crown opens to daylight. The party completes Hatchspire after {rooms} steps and {landmarks} resolved landmarks on floor {floor}. {return_note}"
        )
    } else {
        format!(
            "The party reaches the living crown again after {rooms} steps. Hatchspire remembers its keepers. {return_note}"
        )
    };
    state.activity_log.add(state.day, outcome.summary.clone());
    outcome.completed_tower = true;
    outcome
}

fn with_contract_refresh(
    mut outcome: TowerResult,
    state: &mut GameState,
    data: &GameData,
) -> TowerResult {
    if let Some(pressure_summary) = refresh_pressure(state, data) {
        outcome.summary.push(' ');
        outcome.summary.push_str(&pressure_summary);
    }
    if let Some(contract_summary) = state
        .tower_run
        .as_mut()
        .and_then(|run| refresh_contract(run, data))
    {
        outcome.summary.push(' ');
        outcome.summary.push_str(&contract_summary);
    }
    outcome
}
