use super::*;

#[test]
fn combatant_cards_stay_below_the_side_headings_and_inside_formation() {
    for slot in 0..6 {
        for rect in [ally_slot_rect(slot), enemy_slot_rect(slot)] {
            assert!(rect.y >= 220.0);
            assert!(rect.y + rect.h <= 454.0);
        }
    }
}

#[test]
fn combatant_cards_do_not_overlap_within_either_side() {
    for slots in [
        (0..6).map(ally_slot_rect).collect::<Vec<_>>(),
        (0..6).map(enemy_slot_rect).collect::<Vec<_>>(),
    ] {
        for left in 0..slots.len() {
            for right in (left + 1)..slots.len() {
                assert!(!slots[left].overlaps(&slots[right]));
            }
        }
    }
}

#[test]
fn tactics_panel_names_automatic_targets_and_the_queued_threat() {
    let data = crate::data::GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    crate::engine::combat_engine::start_named_encounter(
        &mut state,
        &data,
        1,
        false,
        Some("moss_mite"),
    );
    let combat = state.combat.expect("combat should start");

    let lines = tactics_lines(&combat);

    assert!(lines[0].contains("Attack -> Moss Mite F1 (auto)"));
    assert!(lines[1].contains("(auto)"));
    assert!(lines[2].contains("Next: Moss Mite"));
    assert!(lines[2].contains("Pip F1"));
    assert_eq!(lines[3], "Active status: none");
}

#[test]
fn tactics_panel_spells_out_active_status_names() {
    let data = crate::data::GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    crate::engine::combat_engine::start_named_encounter(
        &mut state,
        &data,
        1,
        false,
        Some("moss_mite"),
    );
    let combat = state.combat.as_mut().expect("combat should start");
    combat.allies[0].is_guarding = true;
    combat.allies[0].is_defending = true;
    combat.enemies[0].is_marked = true;

    assert_eq!(active_statuses(combat), "Active: GUARDING | MARKED");
}

#[test]
fn conditional_enemy_intent_text_only_promises_effects_that_will_trigger() {
    let data = crate::data::GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    crate::engine::combat_engine::start_named_encounter(
        &mut state,
        &data,
        6,
        false,
        Some("rime_marrow"),
    );
    let combat = state.combat.as_mut().expect("combat should start");
    combat.round = 2;
    let enemy = &combat.enemies[0];
    assert_eq!(enemy_attack_verb(combat, enemy), "hit");

    combat.enemies[0].hp -= 1;
    let enemy = &combat.enemies[0];
    assert_eq!(enemy_attack_verb(combat, enemy), "heal + hit");
}
