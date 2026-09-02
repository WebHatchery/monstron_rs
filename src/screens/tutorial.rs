use macroquad::prelude::*;

use crate::engine::combat_engine::CombatCommand;
use crate::engine::town_engine::{self, TownCommand};
use crate::screens::{combat, hatchery, placeholder, tower, town_layout, AppScreen};
use crate::state::{GameState, TowerRunGoal};
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub const WELCOME: &str = "tutorial_welcome";
pub const SCAVENGED: &str = "tutorial_scavenged";
pub const HATCHERY_OPENED: &str = "tutorial_hatchery_opened";
pub const HATCHERY_VISITED: &str = "tutorial_hatchery_visited";
pub const PREP_OPENED: &str = "tutorial_prep_opened";
pub const TOWER_ENTERED: &str = "tutorial_tower_entered";
pub const EXPLORED: &str = "tutorial_explored";
pub const SURVEYED: &str = "tutorial_surveyed";
pub const RETURNED: &str = "tutorial_returned";
pub const RECOVERED: &str = "tutorial_recovered";
pub const SAVED: &str = "tutorial_saved";
pub const COMBAT_INTRO: &str = "tutorial_combat_intro";
pub const COMBAT_ACTION: &str = "tutorial_combat_action";
pub const COMPLETE: &str = "tutorial_complete";
pub const EGG_INTRO: &str = "tutorial_egg_intro";
pub const EGG_HATCHED: &str = "tutorial_egg_hatched";
pub const EGG_COMPLETE: &str = "tutorial_egg_complete";
pub const SKIPPED: &str = "tutorial_skipped";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TutorialAction {
    Continue,
    Skip,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TutorialStep {
    Welcome,
    Scavenge,
    BuildHatchery,
    OpenHatchery,
    LeaveHatchery,
    OpenTowerPrep,
    ChooseGoal,
    Explore,
    Survey,
    Retreat,
    Recover,
    EndDayContinue,
    OpenMenu,
    Save,
    CombatIntro,
    CombatAttack,
    Finished,
    EggIntro,
    OpenEggHatchery,
    CareEgg,
    LeaveEggHatchery,
    EggSleep,
    EggEndDayContinue,
    HatchEgg,
    EggBuildStable,
    EggUpgradeStable,
    EggFinished,
}

pub fn current_step(
    state: &GameState,
    screen: AppScreen,
    town_menu_open: bool,
) -> Option<TutorialStep> {
    if state.story_flags.has(SKIPPED) {
        return None;
    }
    if screen == AppScreen::Combat && !state.story_flags.has(COMBAT_INTRO) {
        return Some(TutorialStep::CombatIntro);
    }
    if screen == AppScreen::Combat && !state.story_flags.has(COMBAT_ACTION) {
        return Some(TutorialStep::CombatAttack);
    }
    if !state.story_flags.has(COMPLETE) {
        return first_expedition_step(state, screen, town_menu_open);
    }
    egg_care_step(state, screen)
}

fn first_expedition_step(
    state: &GameState,
    screen: AppScreen,
    town_menu_open: bool,
) -> Option<TutorialStep> {
    if !state.story_flags.has(WELCOME) {
        return (screen == AppScreen::Town).then_some(TutorialStep::Welcome);
    }
    if !state.story_flags.has(SCAVENGED) {
        return (screen == AppScreen::Town).then_some(TutorialStep::Scavenge);
    }
    if state.town.building_level("hatchery") == 0 {
        return (screen == AppScreen::Town).then_some(TutorialStep::BuildHatchery);
    }
    if !state.story_flags.has(HATCHERY_OPENED) {
        return (screen == AppScreen::Town).then_some(TutorialStep::OpenHatchery);
    }
    if !state.story_flags.has(HATCHERY_VISITED) {
        return match screen {
            AppScreen::Hatchery => Some(TutorialStep::LeaveHatchery),
            AppScreen::Town => Some(TutorialStep::OpenHatchery),
            _ => None,
        };
    }
    if !state.story_flags.has(PREP_OPENED) || !state.story_flags.has(TOWER_ENTERED) {
        return match screen {
            AppScreen::Town => Some(TutorialStep::OpenTowerPrep),
            AppScreen::DungeonPrep => Some(TutorialStep::ChooseGoal),
            _ => None,
        };
    }
    if !state.story_flags.has(EXPLORED) {
        return (screen == AppScreen::Tower).then_some(TutorialStep::Explore);
    }
    if !state.story_flags.has(SURVEYED) {
        return (screen == AppScreen::Tower).then_some(TutorialStep::Survey);
    }
    if !state.story_flags.has(RETURNED) {
        return (screen == AppScreen::Tower).then_some(TutorialStep::Retreat);
    }
    if !state.story_flags.has(RECOVERED) {
        return (screen == AppScreen::Town).then_some(TutorialStep::Recover);
    }
    if screen == AppScreen::EndOfDay {
        return Some(TutorialStep::EndDayContinue);
    }
    if !state.story_flags.has(SAVED) {
        return (screen == AppScreen::Town).then_some(if town_menu_open {
            TutorialStep::Save
        } else {
            TutorialStep::OpenMenu
        });
    }
    (screen == AppScreen::Town).then_some(TutorialStep::Finished)
}

fn egg_care_step(state: &GameState, screen: AppScreen) -> Option<TutorialStep> {
    if state.story_flags.has(EGG_COMPLETE) {
        return None;
    }
    if state.story_flags.has(EGG_HATCHED) {
        return matches!(screen, AppScreen::Hatchery | AppScreen::Town)
            .then_some(TutorialStep::EggFinished);
    }
    let egg = state.egg_inventory.eggs.first()?;
    if !state.story_flags.has(EGG_INTRO) {
        return (screen == AppScreen::Town).then_some(TutorialStep::EggIntro);
    }
    if screen == AppScreen::EndOfDay {
        return Some(TutorialStep::EggEndDayContinue);
    }
    if egg.days_remaining == 0 && !town_engine::has_monster_capacity(state) {
        let stable_can_expand =
            town_engine::monster_capacity(state) < town_engine::MAX_MONSTER_CAPACITY;
        return match screen {
            AppScreen::Hatchery if stable_can_expand => Some(TutorialStep::LeaveEggHatchery),
            AppScreen::Town if state.town.building_level("stable") == 0 => {
                Some(TutorialStep::EggBuildStable)
            }
            AppScreen::Town if stable_can_expand => Some(TutorialStep::EggUpgradeStable),
            _ => None,
        };
    }
    match screen {
        AppScreen::Town if egg.last_care_day == state.day && egg.days_remaining > 0 => {
            Some(TutorialStep::EggSleep)
        }
        AppScreen::Town => Some(TutorialStep::OpenEggHatchery),
        AppScreen::Hatchery if egg.days_remaining == 0 => Some(TutorialStep::HatchEgg),
        AppScreen::Hatchery if egg.last_care_day != state.day => Some(TutorialStep::CareEgg),
        AppScreen::Hatchery => Some(TutorialStep::LeaveEggHatchery),
        _ => None,
    }
}

pub fn handle_input(
    state: &GameState,
    screen: AppScreen,
    town_menu_open: bool,
) -> Option<TutorialAction> {
    let step = current_step(state, screen, town_menu_open)?;
    if ui::button_clicked(skip_rect(), true) {
        return Some(TutorialAction::Skip);
    }
    if matches!(
        step,
        TutorialStep::Welcome
            | TutorialStep::CombatIntro
            | TutorialStep::Finished
            | TutorialStep::EggIntro
            | TutorialStep::EggFinished
    ) && ui::button_clicked(continue_rect(step), true)
    {
        return Some(TutorialAction::Continue);
    }
    None
}

pub fn draw(state: &GameState, screen: AppScreen, town_menu_open: bool) {
    let Some(step) = current_step(state, screen, town_menu_open) else {
        return;
    };
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        color(2, 6, 8, 38),
    );
    if let Some(target) = target_rect(step) {
        let pulse = 2.0 + ((get_time() * 4.0).sin() as f32 + 1.0) * 1.5;
        draw_rectangle_lines(
            target.x - 5.0,
            target.y - 5.0,
            target.w + 10.0,
            target.h + 10.0,
            pulse,
            color(246, 196, 83, 255),
        );
    }

    let card = card_rect(step);
    draw_rectangle(card.x, card.y, card.w, card.h, color(7, 13, 15, 246));
    draw_rectangle_lines(
        card.x,
        card.y,
        card.w,
        card.h,
        2.0,
        color(207, 160, 70, 255),
    );
    draw_ui_text_ex(
        step_title(step),
        card.x + 24.0,
        card.y + 38.0,
        TextParams {
            font_size: 24,
            color: color(235, 207, 149, 255),
            ..Default::default()
        },
    );
    draw_instruction(step_instruction(step), card.x + 24.0, card.y + 74.0);
    if matches!(
        step,
        TutorialStep::Welcome
            | TutorialStep::CombatIntro
            | TutorialStep::Finished
            | TutorialStep::EggIntro
            | TutorialStep::EggFinished
    ) {
        ui::draw_button(continue_rect(step), continue_label(step), true);
    }
    ui::draw_button(skip_rect(), "SKIP GUIDE", true);
}

pub fn mark(state: &mut GameState, flag: &str) {
    state.story_flags.add(flag);
}

fn target_rect(step: TutorialStep) -> Option<Rect> {
    match step {
        TutorialStep::Scavenge => town_layout::action_buttons()
            .into_iter()
            .find(|(action, _)| *action == TownCommand::Scavenge)
            .map(|(_, rect)| rect),
        TutorialStep::BuildHatchery => Some(town_layout::building_button_rect(1)),
        TutorialStep::OpenHatchery => Some(town_layout::building_open_button_rect(1)),
        TutorialStep::LeaveHatchery => Some(hatchery::town_button_rect()),
        TutorialStep::OpenTowerPrep => town_layout::action_buttons()
            .into_iter()
            .find(|(action, _)| *action == TownCommand::DungeonPrep)
            .map(|(_, rect)| rect),
        TutorialStep::ChooseGoal => TowerRunGoal::CHOICES
            .iter()
            .position(|goal| *goal == TowerRunGoal::SafeRun)
            .map(placeholder::goal_button_rect),
        TutorialStep::Explore => Some(tower::explore_button_rect()),
        TutorialStep::Survey => Some(tower::survey_button_rect()),
        TutorialStep::Retreat => Some(tower::retreat_button_rect()),
        TutorialStep::Recover => town_layout::action_buttons()
            .into_iter()
            .find(|(action, _)| *action == TownCommand::Sleep)
            .map(|(_, rect)| rect),
        TutorialStep::EndDayContinue => Some(placeholder::end_day_town_rect()),
        TutorialStep::OpenMenu => Some(town_layout::menu_button_rect()),
        TutorialStep::Save => Some(town_layout::menu_save_rect()),
        TutorialStep::CombatAttack => combat::command_buttons()
            .into_iter()
            .find(|(command, _)| *command == CombatCommand::Attack)
            .map(|(_, rect)| rect),
        TutorialStep::OpenEggHatchery => Some(town_layout::building_open_button_rect(1)),
        TutorialStep::CareEgg => Some(hatchery::care_button_rect(0, 0)),
        TutorialStep::LeaveEggHatchery => Some(hatchery::town_button_rect()),
        TutorialStep::EggSleep => town_layout::action_buttons()
            .into_iter()
            .find(|(action, _)| *action == TownCommand::Sleep)
            .map(|(_, rect)| rect),
        TutorialStep::EggEndDayContinue => Some(placeholder::end_day_town_rect()),
        TutorialStep::HatchEgg => Some(hatchery::hatch_button_rect(0)),
        TutorialStep::EggBuildStable | TutorialStep::EggUpgradeStable => {
            Some(town_layout::building_button_rect(2))
        }
        TutorialStep::Welcome
        | TutorialStep::CombatIntro
        | TutorialStep::Finished
        | TutorialStep::EggIntro
        | TutorialStep::EggFinished => None,
    }
}

fn step_title(step: TutorialStep) -> &'static str {
    match step {
        TutorialStep::Welcome => "PIP IS READY",
        TutorialStep::Scavenge => "GATHER FOR THE ROAD",
        TutorialStep::BuildHatchery => "LIGHT THE HATCHERY BRAZIER",
        TutorialStep::OpenHatchery => "VISIT THE NEW HATCHERY",
        TutorialStep::LeaveHatchery => "EGGS NOW HAVE A HOME",
        TutorialStep::OpenTowerPrep => "PREPARE AN EXPEDITION",
        TutorialStep::ChooseGoal => "CHOOSE THE FIRST ROUTE",
        TutorialStep::Explore => "STEP INTO THE TOWER",
        TutorialStep::Survey => "READ THE HIDDEN ROOMS",
        TutorialStep::Retreat => "BRING THE PARTY HOME",
        TutorialStep::Recover => "REST AFTER THE EXPEDITION",
        TutorialStep::EndDayContinue => "BEGIN THE NEXT DAY",
        TutorialStep::OpenMenu => "OPEN THE CAMP LEDGER",
        TutorialStep::Save => "KEEP THIS CAMP SAFE",
        TutorialStep::CombatIntro => "A DENIZEN BLOCKS THE PATH",
        TutorialStep::CombatAttack => "TAKE THE FIRST TURN",
        TutorialStep::Finished => "FIRST EXPEDITION READY",
        TutorialStep::EggIntro => "AN EGG CAME HOME",
        TutorialStep::OpenEggHatchery => "VISIT THE WAITING EGG",
        TutorialStep::CareEgg => "WARM THE FIRST EGG",
        TutorialStep::LeaveEggHatchery => "LET THE EGG REST",
        TutorialStep::EggSleep => "PASS A WARM NIGHT",
        TutorialStep::EggEndDayContinue => "THE SHELL STIRS",
        TutorialStep::HatchEgg => "WELCOME A NEW COMPANION",
        TutorialStep::EggBuildStable => "MAKE ROOM IN THE STABLE",
        TutorialStep::EggUpgradeStable => "EXPAND THE STABLE",
        TutorialStep::EggFinished => "A NEW KEEPER JOINS THE CAMP",
    }
}

fn step_instruction(step: TutorialStep) -> &'static str {
    match step {
        TutorialStep::Welcome => {
            "Follow the gold guide to learn the camp and your first tower run."
        }
        TutorialStep::Scavenge => "Tap SCAVENGE to gather the camp's first travel supplies.",
        TutorialStep::BuildHatchery => {
            "Tap the Hatchery BUILD button so tower eggs can come home with you."
        }
        TutorialStep::OpenHatchery => "Tap OPEN beside the Hatchery to visit its warm nests.",
        TutorialStep::LeaveHatchery => {
            "Future tower eggs appear here for care and hatching. Tap TOWN to return."
        }
        TutorialStep::OpenTowerPrep => "Tap TOWER to review the expedition goals.",
        TutorialStep::ChooseGoal => "Tap SAFE RUN to enter floor 1 with Pip.",
        TutorialStep::Explore => {
            "Tap EXPLORE, or tap a visible chamber, to move through the dungeon."
        }
        TutorialStep::Survey => {
            "Tap SURVEY to reveal a hidden room before choosing the next route."
        }
        TutorialStep::Retreat => "Tap RETREAT to bank everything found during this practice run.",
        TutorialStep::Recover => {
            "Tap SLEEP. A new day warms stored eggs and restores tired companions."
        }
        TutorialStep::EndDayContinue => "Tap TOWN to return to the recovered camp.",
        TutorialStep::OpenMenu => "Tap MENU to reach the manual save control.",
        TutorialStep::Save => "Tap SAVE. Progress also autosaves after successful actions.",
        TutorialStep::CombatIntro => {
            "Each living companion takes a turn. Read the target and intent panels."
        }
        TutorialStep::CombatAttack => "Tap ATTACK to strike the highlighted automatic target.",
        TutorialStep::Finished => {
            "You can now explore freely. Replay this guide from the Camp Menu."
        }
        TutorialStep::EggIntro => {
            "The Hatchery kept a tower egg safe. Follow the gold guide to hatch it."
        }
        TutorialStep::OpenEggHatchery => "Tap OPEN beside the Hatchery to inspect the first egg.",
        TutorialStep::CareEgg => "Tap WARM to shorten incubation and record today's care.",
        TutorialStep::LeaveEggHatchery => {
            "Tap TOWN. The egg needs a new day, or more Stable space, before hatching."
        }
        TutorialStep::EggSleep => "Tap SLEEP to warm the cared-for egg overnight.",
        TutorialStep::EggEndDayContinue => "Tap TOWN to check the egg on the new day.",
        TutorialStep::HatchEgg => "Tap HATCH to welcome the ready creature into the roster.",
        TutorialStep::EggBuildStable => {
            "Tap the Stable BUILD button to create more room for companions."
        }
        TutorialStep::EggUpgradeStable => {
            "Tap the Stable UPGRADE button to create more room for companions."
        }
        TutorialStep::EggFinished => {
            "Care, time, and Stable space turn recovered eggs into new party members."
        }
    }
}

fn continue_label(step: TutorialStep) -> &'static str {
    match step {
        TutorialStep::Welcome => "CONTINUE",
        TutorialStep::CombatIntro => "SHOW ACTIONS",
        TutorialStep::Finished => "FINISH GUIDE",
        TutorialStep::EggIntro => "SHOW THE EGG",
        TutorialStep::EggFinished => "CONTINUE",
        _ => "CONTINUE",
    }
}

fn card_rect(step: TutorialStep) -> Rect {
    if matches!(
        step,
        TutorialStep::Welcome
            | TutorialStep::Finished
            | TutorialStep::EggIntro
            | TutorialStep::EggFinished
    ) {
        Rect::new(350.0, 242.0, 580.0, 194.0)
    } else if matches!(
        step,
        TutorialStep::BuildHatchery
            | TutorialStep::OpenHatchery
            | TutorialStep::OpenEggHatchery
            | TutorialStep::CareEgg
            | TutorialStep::HatchEgg
            | TutorialStep::EggBuildStable
            | TutorialStep::EggUpgradeStable
    ) {
        Rect::new(340.0, 478.0, 600.0, 128.0)
    } else {
        Rect::new(340.0, 92.0, 600.0, 128.0)
    }
}

fn draw_instruction(text: &str, x: f32, y: f32) {
    let mut line = String::new();
    let mut row = 0;
    for word in text.split_whitespace() {
        if !line.is_empty() && line.len() + word.len() + 1 > 62 {
            draw_ui_text_ex(
                &line,
                x,
                y + row as f32 * 22.0,
                TextParams {
                    font_size: 17,
                    color: ui::TEXT,
                    ..Default::default()
                },
            );
            line.clear();
            row += 1;
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    draw_ui_text_ex(
        &line,
        x,
        y + row as f32 * 22.0,
        TextParams {
            font_size: 17,
            color: ui::TEXT,
            ..Default::default()
        },
    );
}

fn continue_rect(step: TutorialStep) -> Rect {
    if step == TutorialStep::CombatIntro {
        Rect::new(510.0, 166.0, 260.0, 42.0)
    } else {
        Rect::new(510.0, 382.0, 260.0, 42.0)
    }
}

fn skip_rect() -> Rect {
    Rect::new(1086.0, 102.0, 160.0, 38.0)
}

fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

#[cfg(test)]
mod tests;
