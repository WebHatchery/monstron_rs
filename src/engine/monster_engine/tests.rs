use super::*;
use crate::data::GameDataLoader;

#[test]
fn recovery_reduces_strain_and_heals_injury_timer() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    let monster = state.monster_roster.monster_mut(1).unwrap();
    monster.condition.fatigue = 5;
    monster.condition.injury_days = 1;
    monster.hp = 1;

    let result = recover_monsters(&mut state);
    let recovered = state.monster_roster.monster(1).unwrap();

    assert_eq!(result.fatigue_reduced, 1);
    assert_eq!(result.injuries_healed, 1);
    assert_eq!(result.rested, 1);
    assert_eq!(recovered.condition.fatigue, 3);
    assert_eq!(recovered.condition.injury_days, 0);
    assert_eq!(recovered.hp, recovered.max_hp);
}

#[test]
fn rehoming_requires_a_benched_companion_and_clears_town_work() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    let species = data.species("rootling").expect("rootling should exist");
    let second_id = state
        .monster_roster
        .add_monster("Fern".to_owned(), species, 0xF00D);
    state.monster_roster.assign_to_party(second_id).unwrap();
    state
        .town
        .set_monster_job(second_id, crate::state::TownJobKind::Forage);

    let blocked = rehome_monster(&mut state, second_id);
    assert!(blocked.summary.contains("Bench Fern"));
    assert!(state.monster_roster.monster(second_id).is_some());

    let slot = state
        .monster_roster
        .party_slots
        .iter()
        .position(|slot| *slot == Some(second_id))
        .unwrap();
    state.monster_roster.remove_from_party(slot);
    let rehomed = rehome_monster(&mut state, second_id);
    assert!(rehomed.summary.contains("trusted new home"));
    assert!(state.monster_roster.monster(second_id).is_none());
    assert!(state.town.monster_job(second_id).is_none());

    let last = rehome_monster(&mut state, 1);
    assert!(last.summary.contains("last companion"));
    assert_eq!(state.monster_roster.monsters.len(), 1);
}
