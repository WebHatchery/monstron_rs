use super::*;
use crate::data::GameDataLoader;

#[test]
fn first_expedition_steps_follow_persisted_progress_flags() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);

    assert_eq!(
        current_step(&state, AppScreen::Town),
        Some(TutorialStep::Welcome)
    );
    state.story_flags.add(WELCOME);
    assert_eq!(
        current_step(&state, AppScreen::Town),
        Some(TutorialStep::Scavenge)
    );
    state.story_flags.add(SCAVENGED);
    assert_eq!(
        current_step(&state, AppScreen::Town),
        Some(TutorialStep::OpenTowerPrep)
    );
    state.story_flags.add(PREP_OPENED);
    assert_eq!(
        current_step(&state, AppScreen::DungeonPrep),
        Some(TutorialStep::ChooseGoal)
    );
    state.story_flags.add(TOWER_ENTERED);
    assert_eq!(
        current_step(&state, AppScreen::Tower),
        Some(TutorialStep::Explore)
    );
    state.story_flags.add(EXPLORED);
    assert_eq!(
        current_step(&state, AppScreen::Tower),
        Some(TutorialStep::Survey)
    );
    state.story_flags.add(SURVEYED);
    assert_eq!(
        current_step(&state, AppScreen::Tower),
        Some(TutorialStep::Retreat)
    );
    state.story_flags.add(RETURNED);
    assert_eq!(
        current_step(&state, AppScreen::Town),
        Some(TutorialStep::Finished)
    );
    state.story_flags.add(COMPLETE);
    assert_eq!(current_step(&state, AppScreen::Town), None);
}

#[test]
fn combat_help_remains_available_after_the_expedition_guide() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.story_flags.add(COMPLETE);

    assert_eq!(
        current_step(&state, AppScreen::Combat),
        Some(TutorialStep::CombatIntro)
    );
    state.story_flags.add(COMBAT_INTRO);
    assert_eq!(
        current_step(&state, AppScreen::Combat),
        Some(TutorialStep::CombatAttack)
    );
    state.story_flags.add(COMBAT_ACTION);
    assert_eq!(current_step(&state, AppScreen::Combat), None);
}

#[test]
fn skipping_suppresses_every_tutorial_overlay() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.story_flags.add(SKIPPED);

    assert_eq!(current_step(&state, AppScreen::Town), None);
    assert_eq!(current_step(&state, AppScreen::Combat), None);
}
