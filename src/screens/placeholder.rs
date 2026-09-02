use macroquad::prelude::*;

use crate::data::GameData;
use crate::state::TowerRunGoal;
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaceholderAction {
    ToTown,
    ToTower(TowerRunGoal),
    SelectFloor(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaceholderKind {
    DungeonPrep,
    EndOfDay,
}

pub fn handle_input(
    kind: PlaceholderKind,
    selected_floor: u32,
    unlocked_floor: u32,
    can_enter: bool,
) -> Option<PlaceholderAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(PlaceholderAction::ToTown);
    }

    if kind == PlaceholderKind::DungeonPrep && can_enter && is_key_pressed(KeyCode::Enter) {
        return Some(PlaceholderAction::ToTower(TowerRunGoal::SafeRun));
    }
    if kind == PlaceholderKind::DungeonPrep && is_key_pressed(KeyCode::Left) {
        return Some(PlaceholderAction::SelectFloor(-1));
    }
    if kind == PlaceholderKind::DungeonPrep && is_key_pressed(KeyCode::Right) {
        return Some(PlaceholderAction::SelectFloor(1));
    }

    if kind == PlaceholderKind::EndOfDay && is_key_pressed(KeyCode::Enter) {
        return Some(PlaceholderAction::ToTown);
    }

    for (action, rect, enabled) in buttons(kind, selected_floor, unlocked_floor, can_enter) {
        if ui::button_clicked(rect, enabled) {
            return Some(action);
        }
    }

    None
}

pub fn draw(
    kind: PlaceholderKind,
    data: &GameData,
    status_message: &str,
    selected_floor: u32,
    unlocked_floor: u32,
    ready_party_count: usize,
    ready_party_average_level: Option<u32>,
) {
    let rect = Rect::new(220.0, 96.0, ui::VIEW_WIDTH - 440.0, 500.0);
    ui::draw_panel(rect);

    let (title, body, hint) = copy(kind);
    ui::draw_centered_text(
        title,
        ui::VIEW_WIDTH * 0.5,
        rect.y + 62.0,
        38,
        ui::TEXT_BRIGHT,
    );
    ui::draw_centered_text(body, ui::VIEW_WIDTH * 0.5, rect.y + 104.0, 22, ui::TEXT);
    ui::draw_centered_text(hint, ui::VIEW_WIDTH * 0.5, rect.y + 136.0, 19, ui::TEXT_DIM);

    if kind == PlaceholderKind::DungeonPrep {
        draw_floor_selector(
            data,
            rect,
            selected_floor,
            unlocked_floor,
            ready_party_count,
            ready_party_average_level,
        );
        draw_run_goal_reference(rect);
    }

    for (action, button_rect, enabled) in
        buttons(kind, selected_floor, unlocked_floor, ready_party_count > 0)
    {
        let label = match action {
            PlaceholderAction::ToTown => "Town",
            PlaceholderAction::ToTower(goal) => goal.label(),
            PlaceholderAction::SelectFloor(-1) => "Prev Floor",
            PlaceholderAction::SelectFloor(_) => "Next Floor",
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
            "Choose a goal; reach the floor's stairs to unlock the next route.",
        ),
        PlaceholderKind::EndOfDay => (
            "End Of Day",
            "The day has advanced and monsters have recovered.",
            "Tap TOWN to return to camp.",
        ),
    }
}

fn buttons(
    kind: PlaceholderKind,
    selected_floor: u32,
    unlocked_floor: u32,
    can_enter: bool,
) -> Vec<(PlaceholderAction, Rect, bool)> {
    let center_x = ui::VIEW_WIDTH * 0.5;
    match kind {
        PlaceholderKind::DungeonPrep => {
            let max_floor = unlocked_floor.max(1);
            let mut buttons = vec![
                (
                    PlaceholderAction::SelectFloor(-1),
                    previous_floor_rect(),
                    selected_floor > 1,
                ),
                (
                    PlaceholderAction::SelectFloor(1),
                    next_floor_rect(),
                    selected_floor < max_floor,
                ),
            ];
            buttons.extend(
                TowerRunGoal::CHOICES
                    .iter()
                    .enumerate()
                    .map(|(index, goal)| {
                        (
                            PlaceholderAction::ToTower(*goal),
                            goal_button_rect(index),
                            can_enter,
                        )
                    }),
            );
            buttons.push((
                PlaceholderAction::ToTown,
                Rect::new(center_x - 80.0, 552.0, 160.0, 30.0),
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
    Rect::new(298.0 + index as f32 * 138.0, 508.0, 126.0, 34.0)
}

fn draw_run_goal_reference(rect: Rect) {
    draw_ui_text_ex(
        "EXPEDITION GOAL",
        rect.x + 42.0,
        rect.y + 282.0,
        TextParams {
            font_size: 16,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    for (index, goal) in TowerRunGoal::CHOICES.iter().enumerate() {
        let x = rect.x + 42.0;
        let y = rect.y + 306.0 + index as f32 * 22.0;
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

fn draw_floor_selector(
    data: &GameData,
    rect: Rect,
    selected_floor: u32,
    unlocked_floor: u32,
    ready_party_count: usize,
    ready_party_average_level: Option<u32>,
) {
    let selected = normalize_floor_selection(selected_floor, unlocked_floor);
    let floor = data.tower_floor(selected);
    let name = floor.map_or("Unknown Floor", |floor| floor.name.as_str());
    let theme = floor.map_or("No expedition notes are available.", |floor| {
        floor.theme.as_str()
    });
    let card = Rect::new(rect.x + 190.0, rect.y + 154.0, rect.w - 380.0, 108.0);
    ui::draw_panel(card);
    ui::draw_centered_text(
        &format!("Floor {selected} of {} · {name}", unlocked_floor.max(1)),
        ui::VIEW_WIDTH * 0.5,
        card.y + 34.0,
        24,
        ui::TEXT_BRIGHT,
    );
    ui::draw_centered_text(theme, ui::VIEW_WIDTH * 0.5, card.y + 66.0, 15, ui::TEXT_DIM);
    let (readiness, readiness_color) = if ready_party_count == 0 {
        (
            "NO READY PARTY · Tap TOWN, then open Stable".to_owned(),
            ui::WARN,
        )
    } else {
        let average = ready_party_average_level.unwrap_or(1);
        let color = if average < selected {
            ui::WARN
        } else {
            ui::ACCENT
        };
        (
            format!("{ready_party_count} READY · PARTY AVG LV {average} · SUGGESTED LV {selected}"),
            color,
        )
    };
    ui::draw_centered_text(
        &readiness,
        ui::VIEW_WIDTH * 0.5,
        card.y + 91.0,
        15,
        readiness_color,
    );
}

fn previous_floor_rect() -> Rect {
    Rect::new(262.0, 274.0, 130.0, 46.0)
}

fn next_floor_rect() -> Rect {
    Rect::new(888.0, 274.0, 130.0, 46.0)
}

pub(crate) fn normalize_floor_selection(selected_floor: u32, unlocked_floor: u32) -> u32 {
    selected_floor.clamp(1, unlocked_floor.max(1))
}

#[cfg(test)]
mod tests;
