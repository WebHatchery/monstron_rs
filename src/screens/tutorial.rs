use macroquad::prelude::*;

use crate::engine::combat_engine::CombatCommand;
use crate::engine::town_engine::TownCommand;
use crate::screens::{combat, placeholder, tower, town_layout, AppScreen};
use crate::state::{GameState, TowerRunGoal};
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub const WELCOME: &str = "tutorial_welcome";
pub const SCAVENGED: &str = "tutorial_scavenged";
pub const PREP_OPENED: &str = "tutorial_prep_opened";
pub const TOWER_ENTERED: &str = "tutorial_tower_entered";
pub const EXPLORED: &str = "tutorial_explored";
pub const SURVEYED: &str = "tutorial_surveyed";
pub const RETURNED: &str = "tutorial_returned";
pub const COMBAT_INTRO: &str = "tutorial_combat_intro";
pub const COMBAT_ACTION: &str = "tutorial_combat_action";
pub const COMPLETE: &str = "tutorial_complete";
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
    OpenTowerPrep,
    ChooseGoal,
    Explore,
    Survey,
    Retreat,
    CombatIntro,
    CombatAttack,
    Finished,
}

pub fn current_step(state: &GameState, screen: AppScreen) -> Option<TutorialStep> {
    if state.story_flags.has(SKIPPED) {
        return None;
    }
    if screen == AppScreen::Combat && !state.story_flags.has(COMBAT_INTRO) {
        return Some(TutorialStep::CombatIntro);
    }
    if screen == AppScreen::Combat && !state.story_flags.has(COMBAT_ACTION) {
        return Some(TutorialStep::CombatAttack);
    }
    if state.story_flags.has(COMPLETE) {
        return None;
    }
    if !state.story_flags.has(WELCOME) {
        return (screen == AppScreen::Town).then_some(TutorialStep::Welcome);
    }
    if !state.story_flags.has(SCAVENGED) {
        return (screen == AppScreen::Town).then_some(TutorialStep::Scavenge);
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
    (screen == AppScreen::Town).then_some(TutorialStep::Finished)
}

pub fn handle_input(state: &GameState, screen: AppScreen) -> Option<TutorialAction> {
    let step = current_step(state, screen)?;
    if ui::button_clicked(skip_rect(), true) {
        return Some(TutorialAction::Skip);
    }
    if matches!(
        step,
        TutorialStep::Welcome | TutorialStep::CombatIntro | TutorialStep::Finished
    ) && ui::button_clicked(continue_rect(step), true)
    {
        return Some(TutorialAction::Continue);
    }
    None
}

pub fn draw(state: &GameState, screen: AppScreen) {
    let Some(step) = current_step(state, screen) else {
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

    let card = if matches!(step, TutorialStep::Welcome | TutorialStep::Finished) {
        Rect::new(350.0, 242.0, 580.0, 194.0)
    } else {
        Rect::new(340.0, 92.0, 600.0, 128.0)
    };
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
        TutorialStep::Welcome | TutorialStep::CombatIntro | TutorialStep::Finished
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
        TutorialStep::CombatAttack => combat::command_buttons()
            .into_iter()
            .find(|(command, _)| *command == CombatCommand::Attack)
            .map(|(_, rect)| rect),
        TutorialStep::Welcome | TutorialStep::CombatIntro | TutorialStep::Finished => None,
    }
}

fn step_title(step: TutorialStep) -> &'static str {
    match step {
        TutorialStep::Welcome => "PIP IS READY",
        TutorialStep::Scavenge => "GATHER FOR THE ROAD",
        TutorialStep::OpenTowerPrep => "PREPARE AN EXPEDITION",
        TutorialStep::ChooseGoal => "CHOOSE THE FIRST ROUTE",
        TutorialStep::Explore => "STEP INTO THE TOWER",
        TutorialStep::Survey => "READ THE HIDDEN ROOMS",
        TutorialStep::Retreat => "BRING THE PARTY HOME",
        TutorialStep::CombatIntro => "A DENIZEN BLOCKS THE PATH",
        TutorialStep::CombatAttack => "TAKE THE FIRST TURN",
        TutorialStep::Finished => "FIRST EXPEDITION READY",
    }
}

fn step_instruction(step: TutorialStep) -> &'static str {
    match step {
        TutorialStep::Welcome => {
            "Follow the gold guide to learn the camp and your first tower run."
        }
        TutorialStep::Scavenge => "Tap SCAVENGE to gather the camp's first travel supplies.",
        TutorialStep::OpenTowerPrep => "Tap TOWER to review the expedition goals.",
        TutorialStep::ChooseGoal => "Tap SAFE RUN to enter floor 1 with Pip.",
        TutorialStep::Explore => {
            "Tap EXPLORE, or tap a visible chamber, to move through the dungeon."
        }
        TutorialStep::Survey => {
            "Tap SURVEY to reveal a hidden room before choosing the next route."
        }
        TutorialStep::Retreat => "Tap RETREAT to bank everything found during this practice run.",
        TutorialStep::CombatIntro => {
            "Each living companion takes a turn. Read the target and intent panels."
        }
        TutorialStep::CombatAttack => "Tap ATTACK to strike the highlighted automatic target.",
        TutorialStep::Finished => {
            "You can now explore freely. Replay this guide from the Camp Menu."
        }
    }
}

fn continue_label(step: TutorialStep) -> &'static str {
    match step {
        TutorialStep::Welcome => "CONTINUE",
        TutorialStep::CombatIntro => "SHOW ACTIONS",
        TutorialStep::Finished => "FINISH GUIDE",
        _ => "CONTINUE",
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
