use macroquad::prelude::*;

use crate::state::TowerRunGoal;
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaceholderAction {
    ToTown,
    ToTower(TowerRunGoal),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaceholderKind {
    DungeonPrep,
    EndOfDay,
}

pub fn handle_input(kind: PlaceholderKind) -> Option<PlaceholderAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(PlaceholderAction::ToTown);
    }

    if kind == PlaceholderKind::DungeonPrep && is_key_pressed(KeyCode::Enter) {
        return Some(PlaceholderAction::ToTower(TowerRunGoal::SafeRun));
    }

    if kind == PlaceholderKind::EndOfDay && is_key_pressed(KeyCode::Enter) {
        return Some(PlaceholderAction::ToTown);
    }

    for (action, rect, enabled) in buttons(kind) {
        if ui::button_clicked(rect, enabled) {
            return Some(action);
        }
    }

    None
}

pub fn draw(kind: PlaceholderKind, status_message: &str) {
    let rect = Rect::new(220.0, 120.0, ui::VIEW_WIDTH - 440.0, 440.0);
    ui::draw_panel(rect);

    let (title, body, hint) = copy(kind);
    ui::draw_centered_text(
        title,
        ui::VIEW_WIDTH * 0.5,
        rect.y + 76.0,
        38,
        ui::TEXT_BRIGHT,
    );
    ui::draw_centered_text(body, ui::VIEW_WIDTH * 0.5, rect.y + 135.0, 22, ui::TEXT);
    ui::draw_centered_text(hint, ui::VIEW_WIDTH * 0.5, rect.y + 174.0, 19, ui::TEXT_DIM);

    if kind == PlaceholderKind::DungeonPrep {
        draw_run_goal_reference(rect);
    }

    for (action, button_rect, enabled) in buttons(kind) {
        let label = match action {
            PlaceholderAction::ToTown => "Town",
            PlaceholderAction::ToTower(goal) => goal.label(),
        };
        ui::draw_button(button_rect, label, enabled);
    }

    ui::draw_status(status_message);
}

fn copy(kind: PlaceholderKind) -> (&'static str, &'static str, &'static str) {
    match kind {
        PlaceholderKind::DungeonPrep => (
            "Dungeon Prep",
            "Choose what this run is for before the party enters.",
            "Each goal changes eggs, materials, routes, and risk.",
        ),
        PlaceholderKind::EndOfDay => (
            "End Of Day",
            "The day has advanced and monsters have recovered.",
            "Tap TOWN to return to camp.",
        ),
    }
}

fn buttons(kind: PlaceholderKind) -> Vec<(PlaceholderAction, Rect, bool)> {
    let center_x = ui::VIEW_WIDTH * 0.5;
    match kind {
        PlaceholderKind::DungeonPrep => {
            let mut buttons = TowerRunGoal::CHOICES
                .iter()
                .enumerate()
                .map(|(index, goal)| {
                    (
                        PlaceholderAction::ToTower(*goal),
                        goal_button_rect(index),
                        true,
                    )
                })
                .collect::<Vec<_>>();
            buttons.push((
                PlaceholderAction::ToTown,
                Rect::new(center_x - 80.0, 522.0, 160.0, 30.0),
                true,
            ));
            buttons
        }
        PlaceholderKind::EndOfDay => vec![(PlaceholderAction::ToTown, end_day_town_rect(), true)],
    }
}

pub(crate) fn end_day_town_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH * 0.5 - 100.0, 390.0, 200.0, 46.0)
}

pub(crate) fn goal_button_rect(index: usize) -> Rect {
    Rect::new(298.0 + index as f32 * 138.0, 480.0, 126.0, 34.0)
}

fn draw_run_goal_reference(rect: Rect) {
    for (index, goal) in TowerRunGoal::CHOICES.iter().enumerate() {
        let x = rect.x + 42.0;
        let y = rect.y + 210.0 + index as f32 * 30.0;
        draw_ui_text_ex(
            goal.label(),
            x,
            y,
            TextParams {
                font_size: 18,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            goal.detail(),
            x + 124.0,
            y,
            TextParams {
                font_size: 14,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}
