use super::*;
use crate::data::GameDataLoader;
use crate::engine::town_engine;

#[test]
fn first_expedition_steps_follow_persisted_progress_flags() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);

    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::Welcome)
    );
    state.story_flags.add(WELCOME);
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::Scavenge)
    );
    state.story_flags.add(SCAVENGED);
    assert_eq!(town_engine::egg_capacity(&state), 0);
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::BuildHatchery)
    );
    state.town.set_building_level("hatchery", 1);
    assert_eq!(town_engine::egg_capacity(&state), 3);
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::OpenHatchery)
    );
    state.story_flags.add(HATCHERY_OPENED);
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::OpenHatchery),
        "a reload in town must not strand the facility visit"
    );
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::LeaveHatchery)
    );
    state.story_flags.add(HATCHERY_VISITED);
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::OpenTowerPrep)
    );
    state.story_flags.add(PREP_OPENED);
    assert_eq!(
        current_step(&state, AppScreen::DungeonPrep, false),
        Some(TutorialStep::ChooseGoal)
    );
    state.story_flags.add(TOWER_ENTERED);
    assert_eq!(
        current_step(&state, AppScreen::Tower, false),
        Some(TutorialStep::Explore)
    );
    state.story_flags.add(EXPLORED);
    assert_eq!(
        current_step(&state, AppScreen::Tower, false),
        Some(TutorialStep::Survey)
    );
    state.story_flags.add(SURVEYED);
    assert_eq!(
        current_step(&state, AppScreen::Tower, false),
        Some(TutorialStep::Retreat)
    );
    state.story_flags.add(RETURNED);
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::Recover)
    );
    state.story_flags.add(RECOVERED);
    assert_eq!(
        current_step(&state, AppScreen::EndOfDay, false),
        Some(TutorialStep::EndDayContinue)
    );
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::OpenMenu)
    );
    assert_eq!(
        current_step(&state, AppScreen::Town, true),
        Some(TutorialStep::Save)
    );
    state.story_flags.add(SAVED);
    assert_eq!(
        current_step(&state, AppScreen::Town, true),
        Some(TutorialStep::Finished)
    );
    state.story_flags.add(COMPLETE);
    assert_eq!(current_step(&state, AppScreen::Town, false), None);
}

#[test]
fn combat_help_remains_available_after_the_expedition_guide() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.story_flags.add(COMPLETE);

    assert_eq!(
        current_step(&state, AppScreen::Combat, false),
        Some(TutorialStep::CombatIntro)
    );
    state.story_flags.add(COMBAT_INTRO);
    assert_eq!(
        current_step(&state, AppScreen::Combat, false),
        Some(TutorialStep::CombatAttack)
    );
    state.story_flags.add(COMBAT_ACTION);
    assert_eq!(current_step(&state, AppScreen::Combat, false), None);
}

#[test]
fn skipping_suppresses_every_tutorial_overlay() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.story_flags.add(SKIPPED);

    assert_eq!(current_step(&state, AppScreen::Town, false), None);
    assert_eq!(current_step(&state, AppScreen::Combat, false), None);
}
