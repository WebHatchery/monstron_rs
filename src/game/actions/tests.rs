use super::*;
use crate::data::GameDataLoader;

#[test]
fn loaded_state_resumes_the_active_gameplay_screen() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    assert_eq!(loaded_screen(&state), AppScreen::Town);

    tower_engine::start_run_on_floor(&mut state, &data, TowerRunGoal::Balanced, 1);
    assert_eq!(loaded_screen(&state), AppScreen::Tower);
    combat_engine::start_encounter(&mut state, &data, 1, false);
    assert_eq!(loaded_screen(&state), AppScreen::Combat);

    state.combat = None;
    state.tower_run = None;
    state.story_flags.add("verdant_crown_restored");
    assert_eq!(loaded_screen(&state), AppScreen::Finale);
    state.story_flags.add(finale::EPILOGUE_SEEN);
    assert_eq!(loaded_screen(&state), AppScreen::Town);
}
