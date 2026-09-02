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

#[test]
fn first_egg_guidance_covers_care_incubation_hatching_and_capacity() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.town.set_building_level("hatchery", 1);
    state.story_flags.add(COMPLETE);
    state
        .egg_inventory
        .add_egg("mossy_egg".to_owned(), 2, 1, 0xCAFE);

    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::EggIntro)
    );
    state.story_flags.add(EGG_INTRO);
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::OpenEggHatchery)
    );
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::CareEgg)
    );

    let day = state.day;
    let egg = state.egg_inventory.eggs.first_mut().unwrap();
    egg.days_remaining = 1;
    egg.last_care_day = day;
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::LeaveEggHatchery)
    );
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::EggSleep)
    );
    assert_eq!(
        current_step(&state, AppScreen::EndOfDay, false),
        Some(TutorialStep::EggEndDayContinue)
    );

    state.day += 1;
    state.egg_inventory.eggs[0].days_remaining = 0;
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::HatchEgg)
    );

    let starter = state.monster_roster.monsters[0].clone();
    for id in 2..=3 {
        let mut monster = starter.clone();
        monster.id = id;
        state.monster_roster.monsters.push(monster);
    }
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::LeaveEggHatchery)
    );
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::EggBuildStable)
    );
    state.town.set_building_level("stable", 1);
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::HatchEgg)
    );

    state.town.set_building_level("stable", 3);
    while state.monster_roster.monsters.len() < town_engine::MAX_MONSTER_CAPACITY {
        let mut monster = starter.clone();
        monster.id = state.monster_roster.monsters.len() as u64 + 1;
        state.monster_roster.monsters.push(monster);
    }
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::LeaveEggHatchery)
    );
    assert_eq!(
        current_step(&state, AppScreen::Town, false),
        Some(TutorialStep::EggOpenStable),
        "the guide must route around a disabled max-level Stable upgrade"
    );
    assert_eq!(
        current_step(&state, AppScreen::Stable, false),
        Some(TutorialStep::EggRehome)
    );

    state.egg_inventory.eggs.clear();
    state.story_flags.add(EGG_HATCHED);
    assert_eq!(
        current_step(&state, AppScreen::Hatchery, false),
        Some(TutorialStep::EggFinished)
    );
    state.story_flags.add(EGG_COMPLETE);
    assert_eq!(current_step(&state, AppScreen::Town, false), None);
}
