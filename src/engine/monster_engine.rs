use crate::data::GameData;
use crate::state::{DailyCommitment, GameState, MonsterInstance};

const FATIGUE_CAP: u32 = 6;

pub struct MonsterResult {
    pub summary: String,
}

pub fn toggle_party_member(
    state: &mut GameState,
    data: &GameData,
    monster_id: u64,
) -> MonsterResult {
    let Some(monster) = state.monster_roster.monster(monster_id) else {
        return MonsterResult {
            summary: "That monster is not in the roster. Tap Town and open Stable.".to_owned(),
        };
    };
    let monster_name = monster.name.clone();
    let species_name = data
        .species(&monster.species_id)
        .map(|species| species.name.clone())
        .unwrap_or_else(|| monster.species_id.clone());

    if let Some(slot_index) = state
        .monster_roster
        .party_slots
        .iter()
        .position(|slot| slot.is_some_and(|id| id == monster_id))
    {
        state.monster_roster.remove_from_party(slot_index);
        let summary = format!("Benched {} the {}.", monster_name, species_name);
        state.activity_log.add(state.day, summary.clone());
        return MonsterResult { summary };
    }

    if monster.is_injured() {
        return MonsterResult {
            summary: format!(
                "{monster_name} needs {} more day(s) of rest. Tap Town, then sleep.",
                monster.condition.injury_days
            ),
        };
    }

    if state.town.monster_job(monster_id).is_some() {
        return MonsterResult {
            summary: format!(
                "{monster_name} is committed to town work today. Tap Workshop to clear the job."
            ),
        };
    }

    if monster.condition.commitment != DailyCommitment::Free {
        return MonsterResult {
            summary: format!(
                "{} is already committed to {} today. Tap Town, then sleep.",
                monster_name,
                commitment_label(monster.condition.commitment)
            ),
        };
    }

    match state.monster_roster.assign_to_party(monster_id) {
        Ok(slot_index) => {
            let summary = format!(
                "Assigned {} the {} to party slot {}.",
                monster_name,
                species_name,
                slot_index + 1
            );
            state.activity_log.add(state.day, summary.clone());
            MonsterResult { summary }
        }
        Err(error) => MonsterResult { summary: error },
    }
}

pub fn remove_party_slot(state: &mut GameState, slot_index: usize) -> MonsterResult {
    match state.monster_roster.remove_from_party(slot_index) {
        Some(monster_id) => {
            let monster_name = state
                .monster_roster
                .monster(monster_id)
                .map(|monster| monster.name.clone())
                .unwrap_or_else(|| format!("Monster #{monster_id}"));
            let summary = format!("Removed {monster_name} from party slot {}.", slot_index + 1);
            state.activity_log.add(state.day, summary.clone());
            MonsterResult { summary }
        }
        None => MonsterResult {
            summary: "That party slot is already empty.".to_owned(),
        },
    }
}

pub fn rehome_monster(state: &mut GameState, monster_id: u64) -> MonsterResult {
    let Some(index) = state
        .monster_roster
        .monsters
        .iter()
        .position(|monster| monster.id == monster_id)
    else {
        return MonsterResult {
            summary: "That companion is no longer in the roster.".to_owned(),
        };
    };
    let monster_name = state.monster_roster.monsters[index].name.clone();
    if state.monster_roster.monsters.len() <= 1 {
        return MonsterResult {
            summary: format!("{monster_name} is the camp's last companion and cannot be rehomed."),
        };
    }
    if state.monster_roster.is_in_party(monster_id) {
        return MonsterResult {
            summary: format!("Bench {monster_name} before choosing a new home."),
        };
    }

    state.town.clear_monster_job(monster_id);
    state.monster_roster.monsters.remove(index);
    let summary =
        format!("{monster_name} left for a trusted new home. One Stable space is now available.");
    state.activity_log.add(state.day, summary.clone());
    MonsterResult { summary }
}

pub struct RecoveryResult {
    pub fatigue_reduced: usize,
    pub injuries_healed: usize,
    pub rested: usize,
}

pub fn recover_monsters(state: &mut GameState) -> RecoveryResult {
    let assigned_ids = state
        .town
        .assignments
        .iter()
        .map(|assignment| assignment.monster_id)
        .collect::<Vec<_>>();
    let mut fatigue_reduced = 0;
    let mut injuries_healed = 0;
    let mut rested = 0;

    for monster in &mut state.monster_roster.monsters {
        monster.hp = monster.max_hp;
        let is_working = assigned_ids.contains(&monster.id);
        let is_resting = !is_working
            && matches!(
                monster.condition.commitment,
                DailyCommitment::Free | DailyCommitment::Rest
            );
        let fatigue_recovery = if is_resting {
            2
        } else if is_working {
            0
        } else {
            1
        };

        if monster.condition.fatigue > 0 && fatigue_recovery > 0 {
            monster.condition.fatigue = monster.condition.fatigue.saturating_sub(fatigue_recovery);
            fatigue_reduced += 1;
        }

        if monster.condition.injury_days > 0 && is_resting {
            monster.condition.injury_days -= 1;
            if monster.condition.injury_days == 0 {
                injuries_healed += 1;
            }
        }

        if is_resting {
            monster.bond += 1;
            rested += 1;
        }
        monster.condition.commitment = DailyCommitment::Free;
    }

    RecoveryResult {
        fatigue_reduced,
        injuries_healed,
        rested,
    }
}

pub fn add_fatigue(monster: &mut MonsterInstance, amount: u32) {
    monster.condition.fatigue = (monster.condition.fatigue + amount).min(FATIGUE_CAP);
}

pub fn add_injury(monster: &mut MonsterInstance, days: u32) {
    monster.condition.injury_days = monster.condition.injury_days.max(days);
    monster.hp = monster.hp.max(1);
    add_fatigue(monster, 2);
}

pub fn condition_label(monster: &MonsterInstance) -> String {
    if monster.condition.injury_days > 0 {
        format!("Injured {}d", monster.condition.injury_days)
    } else if monster.condition.fatigue >= 5 {
        "Exhausted".to_owned()
    } else if monster.condition.fatigue >= 3 {
        "Tired".to_owned()
    } else if monster.condition.fatigue > 0 {
        "Winded".to_owned()
    } else {
        "Ready".to_owned()
    }
}

pub fn commitment_label(commitment: DailyCommitment) -> &'static str {
    match commitment {
        DailyCommitment::Free => "free",
        DailyCommitment::Tower => "tower exploration",
        DailyCommitment::Breeding => "breeding care",
        DailyCommitment::Rest => "rest",
    }
}

pub fn daily_plan_label(state: &GameState, monster: &MonsterInstance) -> String {
    if let Some(job) = state.town.monster_job(monster.id) {
        return crate::engine::job_engine::job_label(job).to_owned();
    }
    match monster.condition.commitment {
        DailyCommitment::Free => "Open".to_owned(),
        DailyCommitment::Tower => "Tower".to_owned(),
        DailyCommitment::Breeding => "Breeding".to_owned(),
        DailyCommitment::Rest => "Rest".to_owned(),
    }
}

pub fn can_take_daily_action(state: &GameState, monster: &MonsterInstance) -> Result<(), String> {
    if monster.is_injured() {
        return Err(format!(
            "{} needs {} more day(s) of rest.",
            monster.name, monster.condition.injury_days
        ));
    }
    if state.town.monster_job(monster.id).is_some() {
        return Err(format!("{} is committed to town work today.", monster.name));
    }
    if monster.condition.commitment != DailyCommitment::Free {
        return Err(format!(
            "{} is already committed to {} today.",
            monster.name,
            commitment_label(monster.condition.commitment)
        ));
    }
    Ok(())
}

pub fn mark_commitment(
    state: &mut GameState,
    monster_id: u64,
    commitment: DailyCommitment,
) -> bool {
    let Some(monster) = state.monster_roster.monster_mut(monster_id) else {
        return false;
    };
    monster.condition.commitment = commitment;
    true
}

#[cfg(test)]
mod tests;
