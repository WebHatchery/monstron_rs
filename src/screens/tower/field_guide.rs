use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

use super::TowerAction;
use crate::assets;
use crate::data::{GameData, TowerHazardDefinition};
use crate::state::GameState;
use crate::ui;

const ENTRIES_PER_PAGE: usize = 6;

pub(super) fn handle_input(
    state: &GameState,
    data: &GameData,
    requested_page: usize,
) -> Option<TowerAction> {
    let page_count = entries(state, data).len().max(1).div_ceil(ENTRIES_PER_PAGE);
    let page = requested_page.min(page_count - 1);
    if is_key_pressed(KeyCode::Escape)
        || ui::controller_cancel_pressed()
        || ui::button_clicked(close_rect(), true)
    {
        Some(TowerAction::CloseGuide)
    } else if ui::button_clicked(previous_rect(), page > 0) {
        Some(TowerAction::GuidePage(-1))
    } else if ui::button_clicked(next_rect(), page + 1 < page_count) {
        Some(TowerAction::GuidePage(1))
    } else {
        None
    }
}

pub(super) fn draw(state: &GameState, data: &GameData, requested_page: usize) {
    ui::begin_controller_modal();
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(2, 5, 7, 220),
    );
    let panel = Rect::new(88.0, 48.0, 1104.0, 624.0);
    ui::draw_panel(panel);
    draw_ui_text_ex(
        "TOWER FIELD GUIDE",
        panel.x + 28.0,
        panel.y + 42.0,
        TextParams {
            font_size: 30,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    let entries = entries(state, data);
    let page_count = entries.len().max(1).div_ceil(ENTRIES_PER_PAGE);
    let page = requested_page.min(page_count - 1);
    draw_ui_text_ex(
        &format!(
            "Discovered {} / {} records  ·  Event approaches {}/{}     {}     Page {} / {}",
            entries.len(),
            data.enemies.len() + data.tower_special_locations.len() + data.tower_hazards.len(),
            state.tower_discoveries.event_ids.len(),
            data.tower_events.len(),
            preparation_label(state),
            page + 1,
            page_count
        ),
        panel.x + 28.0,
        panel.y + 70.0,
        TextParams {
            font_size: 16,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );

    for (slot, entry) in entries
        .iter()
        .skip(page * ENTRIES_PER_PAGE)
        .take(ENTRIES_PER_PAGE)
        .enumerate()
    {
        let column = slot % 2;
        let row = slot / 2;
        let card = Rect::new(
            panel.x + 28.0 + column as f32 * 530.0,
            panel.y + 94.0 + row as f32 * 150.0,
            510.0,
            132.0,
        );
        draw_card(entry, card);
    }
    if entries.is_empty() {
        ui::draw_centered_text(
            "Explore visible rooms to record enemies, landmarks, and hazards.",
            ui::VIEW_WIDTH * 0.5,
            ui::VIEW_HEIGHT * 0.5,
            22,
            ui::TEXT_DIM,
        );
    }
    ui::draw_button(previous_rect(), "PREVIOUS", page > 0);
    ui::draw_button(next_rect(), "NEXT", page + 1 < page_count);
    ui::draw_button(close_rect(), "CLOSE", true);
}

fn preparation_label(state: &GameState) -> String {
    let records = state.tower_discoveries.record_count();
    match records {
        30.. => "Field prep +2 SURVEY".to_owned(),
        12.. => format!("Field prep +1 SURVEY · rank 2 at {}/30", records.min(30)),
        _ => format!("Field prep unlocks at {}/12", records.min(12)),
    }
}

enum GuideEntry<'a> {
    Enemy(&'a crate::data::EnemyDefinition),
    Location(&'a crate::data::TowerSpecialLocationDefinition, usize),
    Hazard(&'a TowerHazardDefinition),
}

fn entries<'a>(state: &GameState, data: &'a GameData) -> Vec<GuideEntry<'a>> {
    let mut entries = Vec::new();
    for enemy in &data.enemies {
        if state.tower_discoveries.enemy_ids.contains(&enemy.id) {
            entries.push(GuideEntry::Enemy(enemy));
        }
    }
    for location in &data.tower_special_locations {
        if state
            .tower_discoveries
            .special_location_ids
            .contains(&location.id)
        {
            let tried = location
                .event_ids
                .iter()
                .filter(|event_id| state.tower_discoveries.event_ids.contains(event_id))
                .count();
            entries.push(GuideEntry::Location(location, tried));
        }
    }
    for hazard in &data.tower_hazards {
        if state.tower_discoveries.hazard_ids.contains(&hazard.id) {
            entries.push(GuideEntry::Hazard(hazard));
        }
    }
    entries
}

fn draw_card(entry: &GuideEntry<'_>, card: Rect) {
    draw_rectangle(
        card.x,
        card.y,
        card.w,
        card.h,
        Color::from_rgba(12, 19, 20, 245),
    );
    draw_rectangle_lines(
        card.x,
        card.y,
        card.w,
        card.h,
        1.0,
        Color::from_rgba(87, 119, 103, 220),
    );
    let (kind, name, detail, floors) = match entry {
        GuideEntry::Enemy(enemy) => (
            format!("ENEMY · {:?}", enemy.behavior).to_uppercase(),
            enemy.name.as_str(),
            enemy.description.as_str(),
            format!(
                "Floors {}–{} · Pack {}",
                enemy.min_floor, enemy.max_floor, enemy.pack_size
            ),
        ),
        GuideEntry::Location(location, tried) => (
            "LANDMARK · EVENT".to_owned(),
            location.name.as_str(),
            location.description.as_str(),
            format!(
                "Floors {}–{} · Approaches tried {}/{}",
                location.min_floor,
                location.max_floor,
                tried,
                location.event_ids.len()
            ),
        ),
        GuideEntry::Hazard(hazard) => (
            "HAZARD · COUNTER".to_owned(),
            hazard.name.as_str(),
            hazard.description.as_str(),
            hazard_counter(hazard),
        ),
    };
    draw_entry_art(entry, card.x + 10.0, card.y + 22.0, 88.0);
    draw_ui_text_ex(
        &kind,
        card.x + 108.0,
        card.y + 22.0,
        TextParams {
            font_size: 12,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        name,
        card.x + 108.0,
        card.y + 48.0,
        TextParams {
            font_size: 21,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_wrapped(detail, card.x + 108.0, card.y + 72.0, card.w - 120.0);
    draw_ui_text_ex(
        &floors,
        card.x + 108.0,
        card.y + 119.0,
        TextParams {
            font_size: 12,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
}

fn draw_entry_art(entry: &GuideEntry<'_>, x: f32, y: f32, size: f32) {
    match entry {
        GuideEntry::Enemy(enemy) => {
            assets::draw_dungeon_enemy_visual(enemy.visual, x, y, size, size)
        }
        GuideEntry::Location(location, _) => {
            assets::draw_special_location(location.visual, x, y, size, size)
        }
        GuideEntry::Hazard(hazard) => assets::draw_tower_hazard(hazard.visual, x, y, size, size),
    }
}

fn hazard_counter(hazard: &TowerHazardDefinition) -> String {
    let counter = hazard
        .counter_passive
        .map(|passive| passive.to_string())
        .or_else(|| hazard.counter_element.map(|element| element.to_string()))
        .unwrap_or_else(|| "No known counter".to_owned());
    format!(
        "Floors {}–{} · Counter: {counter}",
        hazard.min_floor, hazard.max_floor
    )
}

fn draw_wrapped(text: &str, x: f32, y: f32, width: f32) {
    macroquad_toolkit::ui::draw_text_block_ex(
        text,
        x,
        y - 12.0,
        width,
        40.0,
        macroquad_toolkit::ui::TextStyle::new(12.0, ui::TEXT_DIM).with_line_gap(4.0),
        12.0,
    );
}

fn previous_rect() -> Rect {
    Rect::new(390.0, 620.0, 150.0, 42.0)
}
fn next_rect() -> Rect {
    Rect::new(556.0, 620.0, 150.0, 42.0)
}
fn close_rect() -> Rect {
    Rect::new(1008.0, 620.0, 150.0, 42.0)
}

#[cfg(test)]
mod tests;
