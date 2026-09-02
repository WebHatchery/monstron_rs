use crate::data::GameData;
use crate::engine::combat_support::{
    add_boss_egg, advance_to_player_or_outcome, advance_turn, ally_attack, ally_skill, award_xp,
    build_allies, build_named_enemies, combined_rewards, defend, encounter_xp, flee_chance,
    flee_succeeds, rebuild_turn_order, record_floor_reached, reward_text, sync_allies,
    victory_rewards,
};
pub(crate) use crate::engine::combat_targeting::{
    player_attack_target, player_skill_target, queued_enemy_intent, CombatTarget, EnemyIntent,
    EnemyIntentKind,
};
use crate::engine::{monster_engine, tower_engine};
use crate::state::{CombatOutcome, CombatSide, CombatState, GameState};
use crate::state::{CombatReplayCommand, CombatReplayStep};

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CombatCommand {
    Attack,
    Skill,
    Defend,
    Item,
    Flee,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CombatDestination {
    Combat,
    Tower,
    Town,
}

pub struct CombatResult {
    pub summary: String,
}

pub struct CombatFinish {
    pub summary: String,
    pub destination: CombatDestination,
}

pub fn start_encounter(
    state: &mut GameState,
    data: &GameData,
    floor: u32,
    is_boss: bool,
) -> CombatResult {
    start_named_encounter(state, data, floor, is_boss, None)
}

pub fn start_named_encounter(
    state: &mut GameState,
    data: &GameData,
    floor: u32,
    is_boss: bool,
    enemy_id: Option<&str>,
) -> CombatResult {
    if state.combat.is_some() {
        return CombatResult {
            summary: "A combat encounter is already active. Tap the visible combat action."
                .to_owned(),
        };
    }

    let allies = build_allies(state);
    if allies.is_empty() {
        state.tower_run = None;
        return CombatResult {
            summary: "No battle-ready monsters remain. Tap Town to recover the party.".to_owned(),
        };
    }

    let enemies = build_named_enemies(data, floor, is_boss, enemy_id);
    if enemies.is_empty() {
        return CombatResult {
            summary: format!("No enemy data is available for floor {floor}. Tap Town to return."),
        };
    }

    let mut combat = CombatState {
        floor,
        round: 1,
        turn_index: 0,
        turn_order: Vec::new(),
        allies: allies.clone(),
        rewards: combined_rewards(data, &enemies),
        xp_reward: encounter_xp(data, &enemies),
        enemies: enemies.clone(),
        log: Vec::new(),
        outcome: None,
        is_boss,
        rng_seed: encounter_seed(state, floor, is_boss),
        replay_roster: allies.clone(),
        replay_enemies: enemies.clone(),
        replay_turn_order: Vec::new(),
        replay_round: 1,
        replay_turn_index: 0,
        command_history: Vec::new(),
    };
    combat.add_log(format!("Encounter started on floor {floor}."));
    rebuild_turn_order(&mut combat);
    advance_to_player_or_outcome(&mut combat);
    combat.replay_roster = combat.allies.clone();
    combat.replay_enemies = combat.enemies.clone();
    combat.replay_turn_order = combat.turn_order.clone();
    combat.replay_round = combat.round;
    combat.replay_turn_index = combat.turn_index;

    let summary = if is_boss {
        "Boss combat started.".to_owned()
    } else {
        "Enemy combat started.".to_owned()
    };
    state.combat = Some(combat);
    CombatResult { summary }
}

pub fn player_action(
    state: &mut GameState,
    data: &GameData,
    command: CombatCommand,
) -> CombatResult {
    if command == CombatCommand::Item {
        let result = use_item(state);
        if let Some(combat) = &mut state.combat {
            record_replay_step(combat, command);
        }
        return result;
    }

    let Some(combat) = &mut state.combat else {
        return CombatResult {
            summary: "No combat encounter is active. Tap Town to return.".to_owned(),
        };
    };

    if combat.outcome.is_some() {
        return CombatResult {
            summary: "The encounter is already resolved. Tap Continue.".to_owned(),
        };
    }

    let Some(turn) = combat.current_turn() else {
        return CombatResult {
            summary: "Combat turn order is empty. Tap Flee to recover the encounter.".to_owned(),
        };
    };
    if turn.side != CombatSide::Ally {
        advance_to_player_or_outcome(combat);
        return CombatResult {
            summary: "Enemies moved before the party could act. Tap an enabled action.".to_owned(),
        };
    }

    let summary = match command {
        CombatCommand::Attack => ally_attack(combat, turn.slot),
        CombatCommand::Skill => ally_skill(combat, data, turn.slot),
        CombatCommand::Defend => defend(combat, turn.slot),
        CombatCommand::Flee => {
            let chance = flee_chance(combat);
            if flee_succeeds(combat) {
                combat.outcome = Some(CombatOutcome::Fled);
                combat.add_log(format!("The party breaks away from the fight ({chance}%)."));
                "The party flees toward town.".to_owned()
            } else {
                combat.add_log(format!("Escape failed despite a {chance}% chance."));
                "The party fails to find an escape route.".to_owned()
            }
        }
        CombatCommand::Item => unreachable!("item command is handled before borrowing combat"),
    };

    if combat.outcome.is_none() {
        advance_turn(combat);
        advance_to_player_or_outcome(combat);
    }

    record_replay_step(combat, command);
    CombatResult { summary }
}

pub fn reduce_command(
    state: &mut GameState,
    data: &GameData,
    command: CombatCommand,
) -> CombatResult {
    player_action(state, data, command)
}

pub fn combat_digest(combat: &CombatState) -> String {
    let allies = combat
        .allies
        .iter()
        .map(|ally| {
            format!(
                "{}:{}:{}:{}",
                ally.slot, ally.hp, ally.is_defending, ally.is_guarding
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let enemies = combat
        .enemies
        .iter()
        .map(|enemy| {
            format!(
                "{}:{}:{}:{}",
                enemy.slot, enemy.hp, enemy.is_defending, enemy.is_marked
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "round={};turn={};index={};outcome={:?};allies={};enemies={}",
        combat.round,
        combat.turn_index,
        combat.turn_order.len(),
        combat.outcome,
        allies,
        enemies
    )
}

fn record_replay_step(combat: &mut CombatState, command: CombatCommand) {
    let command = match command {
        CombatCommand::Attack => CombatReplayCommand::Attack,
        CombatCommand::Skill => CombatReplayCommand::Skill,
        CombatCommand::Defend => CombatReplayCommand::Defend,
        CombatCommand::Item => CombatReplayCommand::Item,
        CombatCommand::Flee => CombatReplayCommand::Flee,
    };
    combat.command_history.push(CombatReplayStep {
        command,
        digest: combat_digest(combat),
    });
}

pub fn finish_combat(state: &mut GameState, data: &GameData) -> CombatFinish {
    let Some(combat) = state.combat.take() else {
        return CombatFinish {
            summary: "No combat encounter is active. Tap Town to return.".to_owned(),
            destination: CombatDestination::Town,
        };
    };

    match combat.outcome {
        Some(CombatOutcome::Victory) => finish_victory(state, data, combat),
        Some(CombatOutcome::Defeat) => finish_defeat(state, combat),
        Some(CombatOutcome::Fled) => finish_flee(state, data, combat),
        None => {
            state.combat = Some(combat);
            CombatFinish {
                summary: "The fight is still underway.".to_owned(),
                destination: CombatDestination::Combat,
            }
        }
    }
}

fn use_item(state: &mut GameState) -> CombatResult {
    let Some(combat) = state.combat.as_ref() else {
        return CombatResult {
            summary: "No combat encounter is active. Tap Town to return.".to_owned(),
        };
    };
    let Some(turn) = combat.current_turn() else {
        return CombatResult {
            summary: "Combat turn order is empty. Tap Flee to recover the encounter.".to_owned(),
        };
    };
    if turn.side != CombatSide::Ally {
        return CombatResult {
            summary: "Items can only be used on an allied turn. Tap an enabled action.".to_owned(),
        };
    }
    if state.resources.amount("herbs") <= 0 {
        return CombatResult {
            summary: "No herbs are available for a field dressing. Tap Flee or return to Town."
                .to_owned(),
        };
    }

    let _ = state.resources.spend(&[("herbs".to_owned(), 1)]);
    let combat = state.combat.as_mut().expect("combat was checked above");
    let heal = 10;
    let name = combat.allies[turn.slot].name.clone();
    let ally = &mut combat.allies[turn.slot];
    ally.hp = (ally.hp + heal).min(ally.max_hp);
    let summary = format!("{name} uses herbs and recovers {heal} HP.");
    combat.add_log(summary.clone());
    advance_turn(combat);
    advance_to_player_or_outcome(combat);

    CombatResult { summary }
}

fn finish_victory(state: &mut GameState, data: &GameData, combat: CombatState) -> CombatFinish {
    sync_allies(state, &combat);
    award_xp(state, data, combat.xp_reward);
    apply_victory_strain(state, &combat);
    record_floor_reached(state, combat.floor);
    let rewards = victory_rewards(&combat, data);

    if let Some(run) = &mut state.tower_run {
        for reward in &rewards {
            run.add_cargo(&reward.resource_id, reward.amount);
        }
        if combat.is_boss {
            add_boss_egg(run, data, combat.floor);
            run.boss_defeated = true;
            let exits = run
                .map
                .objects
                .iter()
                .filter(|object| object.kind == crate::state::TowerMapObjectKind::Exit)
                .map(|object| (object.x, object.y))
                .collect::<Vec<_>>();
            for (x, y) in exits {
                run.map
                    .set_visibility(x, y, crate::state::TowerTileVisibility::Visible);
            }
            run.add_event("The guardian falls; the crown threshold opens.".to_owned());
        }
        run.add_event(format!("Won combat on floor {}.", combat.floor));
    } else {
        for reward in &rewards {
            state.resources.add(&reward.resource_id, reward.amount);
        }
    }

    let summary = format!(
        "Victory on floor {}. Gained {} XP and {}. The party gains expedition strain.",
        combat.floor,
        combat.xp_reward,
        reward_text(data, &rewards)
    );
    state.activity_log.add(state.day, summary.clone());
    CombatFinish {
        summary,
        destination: CombatDestination::Tower,
    }
}

fn finish_defeat(state: &mut GameState, combat: CombatState) -> CombatFinish {
    for ally in &combat.allies {
        if let Some(monster_id) = ally.monster_id {
            if let Some(monster) = state.monster_roster.monster_mut(monster_id) {
                monster.hp = 1;
                monster_engine::add_injury(monster, 2);
                monster_engine::add_fatigue(monster, 1);
            }
        }
    }
    state.tower_run = None;
    let summary = format!(
        "The party was defeated on floor {} and rescued back to town. Run cargo was lost, and the party needs recovery.",
        combat.floor
    );
    state.activity_log.add(state.day, summary.clone());
    CombatFinish {
        summary,
        destination: CombatDestination::Town,
    }
}

fn finish_flee(state: &mut GameState, data: &GameData, combat: CombatState) -> CombatFinish {
    sync_allies(state, &combat);
    apply_flee_strain(state, &combat);
    let summary = if state.tower_run.is_some() {
        let tower_summary = tower_engine::return_to_town(state, data).summary;
        format!("Fled combat. The party gains light strain. {tower_summary}")
    } else {
        "Fled combat and returned to town with light strain.".to_owned()
    };
    state.activity_log.add(state.day, summary.clone());
    CombatFinish {
        summary,
        destination: CombatDestination::Town,
    }
}

fn apply_victory_strain(state: &mut GameState, combat: &CombatState) {
    for ally in &combat.allies {
        let Some(monster_id) = ally.monster_id else {
            continue;
        };
        let Some(monster) = state.monster_roster.monster_mut(monster_id) else {
            continue;
        };
        if ally.hp <= 0 {
            monster_engine::add_injury(monster, 1);
        } else {
            monster_engine::add_fatigue(monster, 1);
        }
    }
}

fn apply_flee_strain(state: &mut GameState, combat: &CombatState) {
    for ally in &combat.allies {
        let Some(monster_id) = ally.monster_id else {
            continue;
        };
        let Some(monster) = state.monster_roster.monster_mut(monster_id) else {
            continue;
        };
        monster_engine::add_fatigue(monster, 1);
    }
}

fn encounter_seed(state: &GameState, floor: u32, is_boss: bool) -> u64 {
    0x00C0_ABA7_u64
        ^ u64::from(state.day).wrapping_mul(97)
        ^ u64::from(floor).wrapping_mul(193)
        ^ u64::from(is_boss as u8).wrapping_mul(389)
        ^ state.monster_roster.next_id.wrapping_mul(577)
}

#[cfg(test)]
mod tests;
