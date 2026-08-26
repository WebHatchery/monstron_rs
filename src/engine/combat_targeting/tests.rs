use super::*;
use crate::data::{GameDataLoader, MonsterRole};
use crate::engine::combat_engine::start_named_encounter;
use crate::state::GameState;

fn encounter(enemy_id: &str) -> CombatState {
    encounter_at(enemy_id, 1)
}

fn encounter_at(enemy_id: &str, floor: u32) -> CombatState {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    start_named_encounter(&mut state, &data, floor, false, Some(enemy_id));
    state.combat.expect("combat should start")
}

#[test]
fn player_previews_use_the_frontline_target_selected_by_attacks() {
    let combat = encounter("moss_mite");

    assert_eq!(
        player_attack_target(&combat),
        Some(CombatTarget {
            side: CombatSide::Enemy,
            index: 0,
        })
    );
    assert_eq!(player_skill_target(&combat), player_attack_target(&combat));
}

#[test]
fn queued_enemy_preview_names_the_next_living_enemy_and_frontline_target() {
    let combat = encounter("moss_mite");
    let preview = queued_enemy_intent(&combat).expect("an enemy should be queued");

    assert_eq!(preview.actor_index, 0);
    assert_eq!(preview.kind, EnemyIntentKind::Attack);
    assert_eq!(preview.target.side, CombatSide::Ally);
    assert_eq!(preview.target.index, 0);
    assert!(!preview.intercepted);
}

#[test]
fn preview_reports_when_a_guarding_tank_redirects_a_back_row_hit() {
    let mut combat = encounter("moss_mite");
    combat.round = 2;
    combat.enemies[0].enemy_behavior = Some(EnemyBehavior::Harrier);
    combat.allies[0].role = Some(MonsterRole::Tank);
    combat.allies[0].is_guarding = true;
    let mut back_row = combat.allies[0].clone();
    back_row.name = "Back Row Ally".to_owned();
    back_row.role = Some(MonsterRole::Support);
    back_row.slot = 3;
    combat.allies.push(back_row);

    let preview = enemy_intent(&combat, 0).expect("the harrier should have an intent");

    assert_eq!(preview.target.index, 0);
    assert!(preview.intercepted);
}

#[test]
fn preview_reports_non_attack_turns_for_bulwarks() {
    let mut combat = encounter_at("mirror_crab", 5);
    combat.round = 3;

    let preview = enemy_intent(&combat, 0).expect("the bulwark should have an intent");

    assert_eq!(preview.kind, EnemyIntentKind::Brace);
    assert_eq!(preview.target.side, CombatSide::Enemy);
    assert_eq!(preview.target.index, 0);
}
