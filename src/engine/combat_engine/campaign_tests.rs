use super::*;
use crate::data::{GameData, GameDataLoader, MonsterRole};
use crate::engine::{day_engine, tower_engine};
use crate::state::{DailyCommitment, TowerRunGoal};

#[test]
fn three_member_same_level_party_can_clear_every_deep_encounter() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let deep_enemies = data
        .enemies
        .iter()
        .filter(|enemy| enemy.max_floor >= 4 && enemy.min_floor <= 10)
        .collect::<Vec<_>>();
    let mut scenarios = 0;

    for enemy in deep_enemies {
        let floor = enemy.max_floor.clamp(4, 10);
        for day in 1..=4 {
            let mut state = trained_party(&data, floor, u64::from(day * 100 + floor));
            state.monster_roster.party_slots[3..].fill(None);
            state.day = day;
            start_named_encounter(&mut state, &data, floor, enemy.is_boss, Some(&enemy.id));
            run_tactical_policy(&mut state, &data);
            assert!(
                state
                    .combat
                    .as_ref()
                    .is_some_and(|combat| combat.outcome == Some(CombatOutcome::Victory)),
                "three-member level-{floor} party failed against {} on day {day}",
                enemy.id
            );
            scenarios += 1;
        }
    }

    assert!(scenarios >= 100, "expected broad deep-floor coverage");
}

#[test]
fn deep_victory_sleep_and_reentry_form_a_recoverable_cycle() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let floor = 7;
    let mut state = trained_party(&data, floor, 707);
    state.monster_roster.party_slots[3..].fill(None);
    state.tower_progress.unlocked_floor = floor;
    let party_ids = state.monster_roster.party_slots[..3]
        .iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();

    tower_engine::start_run_on_floor(&mut state, &data, TowerRunGoal::SafeRun, floor);
    assert!(state.tower_run.is_some());
    let enemy_id = data
        .enemies
        .iter()
        .find(|enemy| !enemy.is_boss && enemy.min_floor <= floor && enemy.max_floor >= floor)
        .expect("floor seven should have an enemy")
        .id
        .clone();
    start_named_encounter(&mut state, &data, floor, false, Some(&enemy_id));
    run_tactical_policy(&mut state, &data);
    assert_eq!(
        state.combat.as_ref().and_then(|combat| combat.outcome),
        Some(CombatOutcome::Victory)
    );
    assert_eq!(
        finish_combat(&mut state, &data).destination,
        CombatDestination::Tower
    );
    assert!(party_ids.iter().all(|id| {
        state.monster_roster.monster(*id).is_some_and(|monster| {
            monster.condition.commitment == DailyCommitment::Tower && monster.condition.fatigue > 0
        })
    }));

    tower_engine::return_to_town(&mut state, &data);
    day_engine::sleep(&mut state, &data);
    assert!(party_ids.iter().all(|id| {
        state.monster_roster.monster(*id).is_some_and(|monster| {
            monster.condition.commitment == DailyCommitment::Free
                && monster.hp == monster.max_hp
                && monster.is_battle_ready()
        })
    }));
    tower_engine::start_run_on_floor(&mut state, &data, TowerRunGoal::SafeRun, floor);
    assert_eq!(
        state.tower_run.as_ref().map(|run| run.current_floor),
        Some(floor)
    );
}

fn trained_party(data: &GameData, level: u32, seed: u64) -> GameState {
    let mut state = GameState::new(data);
    state.monster_roster.monsters.clear();
    state.monster_roster.party_slots.fill(None);
    for (index, species_id) in [
        "rootling",
        "emberkit",
        "pebblepup",
        "rillfin",
        "glowmoth",
        "slime",
    ]
    .iter()
    .enumerate()
    {
        let species = data.species(species_id).unwrap();
        let id = state.monster_roster.add_monster(
            format!("{} Keeper", species.name),
            species,
            seed + index as u64,
        );
        let monster = state.monster_roster.monster_mut(id).unwrap();
        let curve = data.stat_curve(species_id).unwrap();
        for next_level in 2..=level {
            monster.level = next_level;
            monster.max_hp += curve.hp_per_level;
            monster.attack += curve.attack_per_level;
            monster.defense += curve.defense_per_level;
            if next_level.is_multiple_of(curve.speed_interval) {
                monster.speed += curve.speed_per_interval;
            }
        }
        monster.hp = monster.max_hp;
        monster.bond = 5;
        state.monster_roster.party_slots[index] = Some(id);
    }
    state
}

fn run_tactical_policy(state: &mut GameState, data: &GameData) {
    for _ in 0..300 {
        let Some(combat) = state.combat.as_ref() else {
            return;
        };
        if combat.outcome.is_some() {
            return;
        }
        let Some(turn) = combat.current_turn() else {
            return;
        };
        let ally = &combat.allies[turn.slot];
        let command = match ally.role {
            Some(MonsterRole::Tank) if !ally.is_guarding => CombatCommand::Skill,
            Some(MonsterRole::Support)
                if combat
                    .allies
                    .iter()
                    .any(|member| member.hp > 0 && member.hp * 3 < member.max_hp * 2) =>
            {
                CombatCommand::Skill
            }
            Some(MonsterRole::Scout)
                if !combat
                    .enemies
                    .iter()
                    .any(|enemy| enemy.hp > 0 && enemy.is_marked) =>
            {
                CombatCommand::Skill
            }
            Some(MonsterRole::Striker) => CombatCommand::Skill,
            _ => CombatCommand::Attack,
        };
        player_action(state, data, command);
    }
}
