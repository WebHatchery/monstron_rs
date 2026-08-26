use crate::data::{EnemyBehavior, MonsterRole};
use crate::state::{CombatSide, CombatState, CombatTurn, Combatant};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CombatTarget {
    pub side: CombatSide,
    pub index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EnemyIntentKind {
    Attack,
    Brace,
    Rally,
    Ward,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EnemyIntent {
    pub actor_index: usize,
    pub target: CombatTarget,
    pub kind: EnemyIntentKind,
    pub intercepted: bool,
}

pub(crate) fn player_attack_target(combat: &CombatState) -> Option<CombatTarget> {
    first_target(&combat.enemies).map(|index| CombatTarget {
        side: CombatSide::Enemy,
        index,
    })
}

pub(crate) fn player_skill_target(combat: &CombatState) -> Option<CombatTarget> {
    let turn = combat.current_turn()?;
    if turn.side != CombatSide::Ally {
        return None;
    }
    let actor = combat.allies.get(turn.slot)?;
    let (side, index) = match actor.role {
        Some(MonsterRole::Support) => {
            let index = most_wounded_target(&combat.allies).unwrap_or_else(|| {
                combat
                    .allies
                    .iter()
                    .enumerate()
                    .filter(|(_, ally)| ally.is_alive())
                    .min_by_key(|(_, ally)| ally.morale)
                    .map_or(turn.slot, |(index, _)| index)
            });
            (CombatSide::Ally, index)
        }
        Some(MonsterRole::Tank) => (CombatSide::Ally, turn.slot),
        Some(MonsterRole::Striker) => (
            CombatSide::Enemy,
            wounded_target(&combat.enemies).or_else(|| first_target(&combat.enemies))?,
        ),
        Some(MonsterRole::Scout) | None => (CombatSide::Enemy, first_target(&combat.enemies)?),
    };
    Some(CombatTarget { side, index })
}

pub(crate) fn queued_enemy_intent(combat: &CombatState) -> Option<EnemyIntent> {
    let actor_index = combat
        .turn_order
        .iter()
        .skip(combat.turn_index + 1)
        .find(|turn| turn.side == CombatSide::Enemy && turn_is_alive(combat, **turn))?
        .slot;
    enemy_intent(combat, actor_index)
}

fn enemy_intent(combat: &CombatState, actor_index: usize) -> Option<EnemyIntent> {
    let actor = combat.enemies.get(actor_index)?;
    let behavior = actor.enemy_behavior.unwrap_or(EnemyBehavior::Standard);
    if behavior == EnemyBehavior::Bulwark && combat.round.is_multiple_of(3) {
        return Some(intent(
            actor_index,
            CombatSide::Enemy,
            actor_index,
            EnemyIntentKind::Brace,
        ));
    }
    if behavior == EnemyBehavior::Packleader && actor_index == 0 && combat.round.is_multiple_of(2) {
        if let Some(target) = packleader_target(combat, actor_index) {
            return Some(intent(
                actor_index,
                CombatSide::Enemy,
                target,
                EnemyIntentKind::Rally,
            ));
        }
    }
    if behavior == EnemyBehavior::Warden && actor_index == 0 && combat.round.is_multiple_of(3) {
        if let Some(target) = warden_target(combat, actor_index) {
            return Some(intent(
                actor_index,
                CombatSide::Enemy,
                target,
                EnemyIntentKind::Ward,
            ));
        }
    }
    let (target, guard) = enemy_target(combat, actor_index)?;
    Some(EnemyIntent {
        actor_index,
        target: CombatTarget {
            side: CombatSide::Ally,
            index: target,
        },
        kind: EnemyIntentKind::Attack,
        intercepted: guard.is_some(),
    })
}

fn intent(
    actor_index: usize,
    side: CombatSide,
    index: usize,
    kind: EnemyIntentKind,
) -> EnemyIntent {
    EnemyIntent {
        actor_index,
        target: CombatTarget { side, index },
        kind,
        intercepted: false,
    }
}

pub(crate) fn packleader_target(combat: &CombatState, actor_index: usize) -> Option<usize> {
    combat
        .enemies
        .iter()
        .enumerate()
        .filter(|(index, enemy)| *index != actor_index && enemy.is_alive())
        .min_by_key(|(_, enemy)| (enemy.attack, enemy.slot))
        .map(|(index, _)| index)
}

pub(crate) fn warden_target(combat: &CombatState, actor_index: usize) -> Option<usize> {
    combat
        .enemies
        .iter()
        .enumerate()
        .filter(|(index, enemy)| *index != actor_index && enemy.is_alive())
        .min_by_key(|(_, enemy)| (enemy.hp * 100 / enemy.max_hp.max(1), enemy.slot))
        .map(|(index, _)| index)
}

pub(crate) fn enemy_target(
    combat: &CombatState,
    enemy_index: usize,
) -> Option<(usize, Option<usize>)> {
    let behavior = combat.enemies[enemy_index]
        .enemy_behavior
        .unwrap_or(EnemyBehavior::Standard);
    if behavior == EnemyBehavior::Ambusher && combat.round == 1 {
        let target = combat
            .allies
            .iter()
            .enumerate()
            .filter(|(_, ally)| ally.is_alive())
            .min_by_key(|(_, ally)| (ally.hp, ally.defense, ally.slot))
            .map(|(index, _)| index)?;
        if combat.allies[target].slot >= 3 {
            if let Some(guard) = guarding_front_tank(&combat.allies) {
                return Some((guard, Some(guard)));
            }
        }
        return Some((target, None));
    }
    let back_targeted = if behavior == EnemyBehavior::Harrier {
        (combat.round + enemy_index as u32).is_multiple_of(2)
    } else {
        (combat.round + combat.floor + enemy_index as u32).is_multiple_of(4)
    };
    if back_targeted {
        if let Some(back_target) = row_target(&combat.allies, 3..6) {
            if let Some(guard) = guarding_front_tank(&combat.allies) {
                return Some((guard, Some(guard)));
            }
            return Some((back_target, None));
        }
    }
    first_target(&combat.allies).map(|target| (target, None))
}

pub(crate) fn first_target(combatants: &[Combatant]) -> Option<usize> {
    row_target(combatants, 0..3).or_else(|| row_target(combatants, 3..6))
}

pub(crate) fn wounded_target(combatants: &[Combatant]) -> Option<usize> {
    combatants
        .iter()
        .enumerate()
        .filter(|(_, unit)| unit.is_alive() && unit.hp < unit.max_hp)
        .min_by_key(|(_, unit)| unit.hp)
        .map(|(index, _)| index)
}

fn most_wounded_target(combatants: &[Combatant]) -> Option<usize> {
    combatants
        .iter()
        .enumerate()
        .filter(|(_, unit)| unit.is_alive() && unit.hp < unit.max_hp)
        .max_by_key(|(_, unit)| unit.max_hp - unit.hp)
        .map(|(index, _)| index)
}

fn row_target(combatants: &[Combatant], mut slots: std::ops::Range<usize>) -> Option<usize> {
    slots
        .find(|slot| {
            combatants
                .iter()
                .any(|unit| unit.slot == *slot && unit.is_alive())
        })
        .and_then(|slot| {
            combatants
                .iter()
                .position(|unit| unit.slot == slot && unit.is_alive())
        })
}

fn guarding_front_tank(combatants: &[Combatant]) -> Option<usize> {
    combatants.iter().position(|unit| {
        unit.is_alive() && unit.slot < 3 && unit.role == Some(MonsterRole::Tank) && unit.is_guarding
    })
}

fn turn_is_alive(combat: &CombatState, turn: CombatTurn) -> bool {
    match turn.side {
        CombatSide::Ally => combat
            .allies
            .get(turn.slot)
            .is_some_and(Combatant::is_alive),
        CombatSide::Enemy => combat
            .enemies
            .get(turn.slot)
            .is_some_and(Combatant::is_alive),
    }
}
