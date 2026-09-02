use macroquad::prelude::*;

use crate::assets;
use crate::data::GameData;
use crate::engine::{monster_engine, town_engine};
use crate::state::GameState;
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

const ROSTER_PAGE_SIZE: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StableAction {
    ToTown,
    ToggleParty(u64),
    RemoveSlot(usize),
    Page(i32),
    RequestRehome(u64),
    ConfirmRehome(u64),
    CancelRehome,
}

pub fn handle_input(
    state: &GameState,
    roster_page: usize,
    pending_rehome: Option<u64>,
) -> Option<StableAction> {
    if let Some(monster_id) = pending_rehome {
        if is_key_pressed(KeyCode::Escape) || ui::button_clicked(cancel_rehome_rect(), true) {
            return Some(StableAction::CancelRehome);
        }
        if ui::button_clicked(confirm_rehome_rect(), true) {
            return Some(StableAction::ConfirmRehome(monster_id));
        }
        return None;
    }
    if is_key_pressed(KeyCode::Escape) {
        return Some(StableAction::ToTown);
    }

    if ui::button_clicked(town_button_rect(), true) {
        return Some(StableAction::ToTown);
    }

    for (index, slot) in state.monster_roster.party_slots.iter().enumerate() {
        if slot.is_some() && ui::button_clicked(slot_button_rect(index), true) {
            return Some(StableAction::RemoveSlot(index));
        }
    }

    if ui::button_clicked(previous_page_rect(), roster_page > 0) {
        return Some(StableAction::Page(-1));
    }
    if ui::button_clicked(
        next_page_rect(),
        roster_page + 1 < roster_page_count(state.monster_roster.monsters.len()),
    ) {
        return Some(StableAction::Page(1));
    }

    let bounds = roster_page_bounds(state.monster_roster.monsters.len(), roster_page);
    for (index, monster) in state.monster_roster.monsters[bounds].iter().enumerate() {
        let can_toggle = state.monster_roster.is_in_party(monster.id)
            || monster_engine::can_take_daily_action(state, monster).is_ok();
        if ui::button_clicked(roster_button_rect(index), can_toggle) {
            return Some(StableAction::ToggleParty(monster.id));
        }
        let can_rehome = state.monster_roster.monsters.len() > 1
            && !state.monster_roster.is_in_party(monster.id);
        if ui::button_clicked(rehome_button_rect(index), can_rehome) {
            return Some(StableAction::RequestRehome(monster.id));
        }
    }

    None
}

pub fn draw(
    state: &GameState,
    data: &GameData,
    status_message: &str,
    roster_page: usize,
    pending_rehome: Option<u64>,
) {
    draw_backdrop();
    draw_header(state);
    draw_party(state);
    draw_roster(state, data, roster_page);
    if let Some(monster_id) = pending_rehome {
        draw_rehome_confirmation(state, data, monster_id);
    }
    ui::draw_status(status_message);
}

fn draw_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(23, 28, 32, 255),
    );
    draw_rectangle(
        0.0,
        470.0,
        ui::VIEW_WIDTH,
        250.0,
        Color::from_rgba(45, 49, 43, 255),
    );
}

fn draw_header(state: &GameState) {
    ui::draw_panel(Rect::new(32.0, 24.0, ui::VIEW_WIDTH - 64.0, 78.0));
    draw_ui_text_ex(
        "Stable",
        58.0,
        72.0,
        TextParams {
            font_size: 36,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!(
            "Roster {}/{}  Party {}",
            state.monster_roster.monsters.len(),
            town_engine::monster_capacity(state),
            state
                .monster_roster
                .party_slots
                .iter()
                .filter(|slot| slot.is_some())
                .count()
        ),
        842.0,
        70.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    ui::draw_button(town_button_rect(), "Town", true);
}

fn draw_party(state: &GameState) {
    let rect = Rect::new(32.0, 124.0, ui::VIEW_WIDTH - 64.0, 154.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Six-Slot Party", rect.x + 20.0, rect.y + 34.0);

    for (index, slot) in state.monster_roster.party_slots.iter().enumerate() {
        let x = rect.x + 22.0 + index as f32 * 196.0;
        let y = rect.y + 62.0;
        draw_rectangle(x, y, 176.0, 70.0, Color::from_rgba(28, 36, 40, 230));
        draw_rectangle_lines(x, y, 176.0, 70.0, 1.0, ui::PANEL_EDGE);
        let row = if index < 3 { "Front" } else { "Back" };
        draw_ui_text_ex(
            &format!("{} {}", row, index + 1),
            x + 10.0,
            y + 20.0,
            TextParams {
                font_size: 16,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );

        if let Some(monster_id) = slot {
            if let Some(monster) = state.monster_roster.monster(*monster_id) {
                assets::draw_monster_badge(&monster.species_id, x + 10.0, y + 28.0, 34.0);
                draw_ui_text_ex(
                    &monster.name,
                    x + 54.0,
                    y + 45.0,
                    TextParams {
                        font_size: 18,
                        color: ui::TEXT_BRIGHT,
                        ..Default::default()
                    },
                );
                draw_ui_text_ex(
                    &format!(
                        "Lv {} · {}",
                        monster.level,
                        monster_engine::condition_label(monster)
                    ),
                    x + 54.0,
                    y + 63.0,
                    TextParams {
                        font_size: 14,
                        color: ui::TEXT_DIM,
                        ..Default::default()
                    },
                );
                ui::draw_button(slot_button_rect(index), "Bench", true);
            }
        } else {
            draw_ui_text_ex(
                "Empty",
                x + 58.0,
                y + 52.0,
                TextParams {
                    font_size: 20,
                    color: ui::TEXT_DIM,
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_roster(state: &GameState, data: &GameData, roster_page: usize) {
    let rect = Rect::new(32.0, 300.0, ui::VIEW_WIDTH - 64.0, 306.0);
    ui::draw_panel(rect);
    ui::draw_section_title(
        &format!(
            "Roster ({}/{})",
            state.monster_roster.monsters.len(),
            town_engine::monster_capacity(state)
        ),
        rect.x + 20.0,
        rect.y + 34.0,
    );

    let page_count = roster_page_count(state.monster_roster.monsters.len());
    draw_ui_text_ex(
        &format!(
            "Page {}/{}",
            roster_page.min(page_count - 1) + 1,
            page_count
        ),
        974.0,
        rect.y + 32.0,
        TextParams {
            font_size: 16,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    ui::draw_button(previous_page_rect(), "Prev", roster_page > 0);
    ui::draw_button(next_page_rect(), "Next", roster_page + 1 < page_count);

    let bounds = roster_page_bounds(state.monster_roster.monsters.len(), roster_page);
    for (index, monster) in state.monster_roster.monsters[bounds].iter().enumerate() {
        let column = index % 2;
        let row = index / 2;
        let x = rect.x + 20.0 + column as f32 * 594.0;
        let y = rect.y + 68.0 + row as f32 * 58.0;
        let species_name = data
            .species(&monster.species_id)
            .map(|species| species.name.as_str())
            .unwrap_or(monster.species_id.as_str());
        assets::draw_monster_badge(&monster.species_id, x, y - 28.0, 42.0);
        draw_ui_text_ex(
            &format!(
                "{} the {} · {}",
                monster.name,
                species_name,
                level_progress_label(monster.level, monster.xp)
            ),
            x + 58.0,
            y,
            TextParams {
                font_size: 18,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!(
                "{} {}  {}  {}  Plan: {}",
                monster.element,
                monster.role,
                monster.temperament,
                monster_engine::condition_label(monster),
                monster_engine::daily_plan_label(state, monster)
            ),
            x + 58.0,
            y + 24.0,
            TextParams {
                font_size: 15,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        let in_party = state.monster_roster.is_in_party(monster.id);
        let label = if in_party {
            "Bench"
        } else if monster_engine::can_take_daily_action(state, monster).is_err() {
            "Rest"
        } else {
            "Party"
        };
        ui::draw_button(
            roster_button_rect(index),
            label,
            in_party || monster_engine::can_take_daily_action(state, monster).is_ok(),
        );
        ui::draw_button(
            rehome_button_rect(index),
            "Rehome",
            state.monster_roster.monsters.len() > 1 && !in_party,
        );
    }
}

fn level_progress_label(level: u32, xp: u32) -> String {
    format!("Lv {level} · XP {xp}/{}", level.saturating_mul(20))
}

fn draw_rehome_confirmation(state: &GameState, data: &GameData, monster_id: u64) {
    let Some(monster) = state.monster_roster.monster(monster_id) else {
        return;
    };
    let species = data
        .species(&monster.species_id)
        .map(|species| species.name.as_str())
        .unwrap_or(monster.species_id.as_str());
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(4, 7, 9, 190),
    );
    let rect = Rect::new(380.0, 220.0, 520.0, 260.0);
    ui::draw_panel(rect);
    ui::draw_centered_text(
        "Choose A New Home?",
        rect.x + rect.w * 0.5,
        rect.y + 54.0,
        30,
        ui::TEXT_BRIGHT,
    );
    ui::draw_centered_text(
        &format!(
            "{} the {} will leave this save permanently.",
            monster.name, species
        ),
        rect.x + rect.w * 0.5,
        rect.y + 104.0,
        19,
        ui::TEXT,
    );
    ui::draw_centered_text(
        "This frees one Stable space for a ready egg.",
        rect.x + rect.w * 0.5,
        rect.y + 136.0,
        17,
        ui::TEXT_DIM,
    );
    ui::draw_button(cancel_rehome_rect(), "CANCEL", true);
    ui::draw_button(confirm_rehome_rect(), "REHOME", true);
}

fn town_button_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH - 148.0, 44.0, 86.0, 34.0)
}

fn slot_button_rect(index: usize) -> Rect {
    Rect::new(150.0 + index as f32 * 196.0, 190.0, 70.0, 26.0)
}

fn roster_button_rect(index: usize) -> Rect {
    let column = index % 2;
    let row = index / 2;
    Rect::new(
        542.0 + column as f32 * 594.0,
        344.0 + row as f32 * 58.0,
        78.0,
        30.0,
    )
}

fn rehome_button_rect(index: usize) -> Rect {
    let column = index % 2;
    let row = index / 2;
    Rect::new(
        454.0 + column as f32 * 594.0,
        344.0 + row as f32 * 58.0,
        82.0,
        30.0,
    )
}

pub(crate) fn first_rehome_button_rect(state: &GameState) -> Option<Rect> {
    let bounds = roster_page_bounds(state.monster_roster.monsters.len(), 0);
    state.monster_roster.monsters[bounds]
        .iter()
        .enumerate()
        .find(|(_, monster)| !state.monster_roster.is_in_party(monster.id))
        .map(|(index, _)| rehome_button_rect(index))
}

fn previous_page_rect() -> Rect {
    Rect::new(1062.0, 310.0, 72.0, 28.0)
}

fn next_page_rect() -> Rect {
    Rect::new(1142.0, 310.0, 72.0, 28.0)
}

fn cancel_rehome_rect() -> Rect {
    Rect::new(432.0, 404.0, 190.0, 42.0)
}

fn confirm_rehome_rect() -> Rect {
    Rect::new(658.0, 404.0, 190.0, 42.0)
}

pub(crate) fn roster_page_count(roster_len: usize) -> usize {
    roster_len.div_ceil(ROSTER_PAGE_SIZE).max(1)
}

pub(crate) fn roster_page_bounds(roster_len: usize, roster_page: usize) -> std::ops::Range<usize> {
    let page = roster_page.min(roster_page_count(roster_len) - 1);
    let start = page * ROSTER_PAGE_SIZE;
    start..(start + ROSTER_PAGE_SIZE).min(roster_len)
}

#[cfg(test)]
mod tests;
