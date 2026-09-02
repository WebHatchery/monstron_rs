use super::*;
use crate::data::GameDataLoader;
use crate::state::GameState;

#[test]
fn only_a_successful_hatch_advances_the_contextual_guide() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.town.set_building_level("hatchery", 1);
    state.town.set_building_level("stable", 1);
    state
        .egg_inventory
        .add_egg("mossy_egg".to_owned(), 1, 1, 0xCAFE);

    let eggs_before = state.egg_inventory.eggs.len();
    egg_engine::hatch_egg(&mut state, &data, 1);
    mark_tutorial_hatch(&mut state, eggs_before);
    assert!(!state.story_flags.has(tutorial::EGG_HATCHED));

    state.egg_inventory.eggs[0].days_remaining = 0;
    let eggs_before = state.egg_inventory.eggs.len();
    egg_engine::hatch_egg(&mut state, &data, 1);
    mark_tutorial_hatch(&mut state, eggs_before);
    assert!(state.story_flags.has(tutorial::EGG_HATCHED));
}
