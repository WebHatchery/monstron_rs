use macroquad::prelude::*;

use crate::assets;
use crate::data::GameData;
use crate::engine::{egg_engine, town_engine};
use crate::state::EggCareFocus;
use crate::state::GameState;
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HatcheryAction {
    ToTown,
    CareEgg(u64, EggCareFocus),
    HatchEgg(u64),
}

pub fn handle_input(state: &GameState) -> Option<HatcheryAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(HatcheryAction::ToTown);
    }

    if ui::button_clicked(town_button_rect(), true) {
        return Some(HatcheryAction::ToTown);
    }

    for (index, egg) in state.egg_inventory.eggs.iter().take(6).enumerate() {
        for (care_index, care_focus) in care_choices().iter().enumerate() {
            if ui::button_clicked(
                care_button_rect(index, care_index),
                care_enabled(
                    egg.days_remaining,
                    egg.last_care_day,
                    state.day,
                    *care_focus,
                ),
            ) {
                return Some(HatcheryAction::CareEgg(egg.id, *care_focus));
            }
        }

        let hatch_enabled = egg.days_remaining == 0 && town_engine::has_monster_capacity(state);
        if ui::button_clicked(hatch_button_rect(index), hatch_enabled) {
            return Some(HatcheryAction::HatchEgg(egg.id));
        }
    }

    None
}

pub fn draw(state: &GameState, data: &GameData, status_message: &str) {
    draw_backdrop();
    draw_header(state);
    draw_egg_inventory(state, data);
    draw_reference(data);
    ui::draw_status(status_message);
}

fn draw_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(24, 28, 32, 255),
    );
    draw_circle(1120.0, 130.0, 100.0, Color::from_rgba(129, 187, 138, 30));
    draw_rectangle(
        0.0,
        500.0,
        ui::VIEW_WIDTH,
        220.0,
        Color::from_rgba(40, 50, 42, 255),
    );
}

fn draw_header(state: &GameState) {
    ui::draw_panel(Rect::new(32.0, 24.0, ui::VIEW_WIDTH - 64.0, 78.0));
    draw_ui_text_ex(
        "Hatchery",
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
            "Day {}  Eggs {}/{}  Monsters {}/{}",
            state.day,
            state.egg_inventory.eggs.len(),
            town_engine::egg_capacity(state),
            state.monster_roster.monsters.len(),
            town_engine::monster_capacity(state)
        ),
        604.0,
        70.0,
        TextParams {
            font_size: 22,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    ui::draw_button(town_button_rect(), "Town", true);
}

fn draw_egg_inventory(state: &GameState, data: &GameData) {
    let rect = Rect::new(32.0, 124.0, 780.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title(
        &format!(
            "Egg Inventory ({}/{})",
            state.egg_inventory.eggs.len(),
            town_engine::egg_capacity(state)
        ),
        rect.x + 20.0,
        rect.y + 34.0,
    );

    if state.egg_inventory.eggs.is_empty() {
        let empty_label = if town_engine::egg_capacity(state) == 0 {
            "Build the Hatchery before keeping tower eggs."
        } else {
            "No eggs are waiting. Bring eggs back from tower runs."
        };
        draw_ui_text_ex(
            empty_label,
            rect.x + 24.0,
            rect.y + 108.0,
            TextParams {
                font_size: 24,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        return;
    }

    for (index, egg) in state.egg_inventory.eggs.iter().take(6).enumerate() {
        let y = rect.y + 76.0 + index as f32 * 66.0;
        let egg_type = data.egg_type(&egg.egg_type_id);
        let name = egg_type
            .map(|egg_type| egg_type.name.as_str())
            .unwrap_or(egg.egg_type_id.as_str());
        let rarity = egg_type
            .map(|egg_type| egg_type.rarity.as_str())
            .unwrap_or("?");
        assets::draw_egg_badge(&egg.egg_type_id, rect.x + 24.0, y - 34.0, 42.0);
        draw_ui_text_ex(
            &format!("{} #{}", name, egg.id),
            rect.x + 82.0,
            y,
            TextParams {
                font_size: 22,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!(
                "{}  Floor {}  {} day(s) remaining  {}",
                rarity, egg.origin_floor, egg.days_remaining, egg.care_focus
            ),
            rect.x + 82.0,
            y + 24.0,
            TextParams {
                font_size: 17,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!(
                "{}  {}",
                egg_engine::likely_species_text(egg, data),
                egg_engine::trait_preview_text(egg, data)
            ),
            rect.x + 82.0,
            y + 42.0,
            TextParams {
                font_size: 13,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        for (care_index, care_focus) in care_choices().iter().enumerate() {
            ui::draw_button(
                care_button_rect(index, care_index),
                care_focus.label(),
                care_enabled(
                    egg.days_remaining,
                    egg.last_care_day,
                    state.day,
                    *care_focus,
                ),
            );
        }
        let hatch_ready = egg.days_remaining == 0;
        let hatch_enabled = hatch_ready && town_engine::has_monster_capacity(state);
        let hatch_label = if hatch_ready && !hatch_enabled {
            "Full"
        } else {
            "Hatch"
        };
        ui::draw_button(hatch_button_rect(index), hatch_label, hatch_enabled);
    }
}

fn draw_reference(data: &GameData) {
    let rect = Rect::new(836.0, 124.0, ui::VIEW_WIDTH - 868.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Known Egg Types", rect.x + 20.0, rect.y + 34.0);

    for (index, egg_type) in data.egg_types.iter().enumerate() {
        let column = index / 8;
        let x = rect.x + 20.0 + column as f32 * 186.0;
        let row_y = rect.y + 76.0 + (index % 8) as f32 * 45.0;
        assets::draw_egg_badge(&egg_type.id, x, row_y - 28.0, 34.0);
        draw_ui_text_ex(
            &egg_type.name,
            x + 40.0,
            row_y,
            TextParams {
                font_size: 19,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!(
                "{}  Floor {}  Hatch {}d",
                egg_type.rarity, egg_type.discovery_floor, egg_type.hatch_days
            ),
            x + 40.0,
            row_y + 20.0,
            TextParams {
                font_size: 15,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

pub(crate) fn town_button_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH - 148.0, 44.0, 86.0, 34.0)
}

fn care_choices() -> [EggCareFocus; 4] {
    [
        EggCareFocus::Warm,
        EggCareFocus::Soothe,
        EggCareFocus::Study,
        EggCareFocus::Stabilise,
    ]
}

fn care_enabled(
    days_remaining: u32,
    last_care_day: u32,
    current_day: u32,
    care_focus: EggCareFocus,
) -> bool {
    last_care_day != current_day && (care_focus != EggCareFocus::Warm || days_remaining > 0)
}

pub(crate) fn care_button_rect(row: usize, column: usize) -> Rect {
    Rect::new(
        488.0 + column as f32 * 62.0,
        160.0 + row as f32 * 66.0,
        58.0,
        28.0,
    )
}

pub(crate) fn hatch_button_rect(index: usize) -> Rect {
    Rect::new(736.0, 160.0 + index as f32 * 66.0, 60.0, 28.0)
}

trait EggCareLabel {
    fn label(self) -> &'static str;
}

impl EggCareLabel for EggCareFocus {
    fn label(self) -> &'static str {
        match self {
            EggCareFocus::None => "-",
            EggCareFocus::Warm => "Warm",
            EggCareFocus::Soothe => "Soothe",
            EggCareFocus::Study => "Study",
            EggCareFocus::Stabilise => "Stable",
        }
    }
}
