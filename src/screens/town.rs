use macroquad::prelude::*;

use crate::assets;
use crate::data::GameData;
use crate::engine::town_engine;
use crate::screens::{town_layout, town_panels};
use crate::state::GameState;
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub use crate::engine::town_engine::TownCommand as TownAction;

pub fn handle_input(state: &GameState, data: &GameData, menu_open: bool) -> Option<TownAction> {
    if menu_open {
        return handle_menu_input();
    }

    if is_key_pressed(KeyCode::Space) {
        return Some(TownAction::Sleep);
    }
    if is_key_pressed(KeyCode::D) {
        return Some(TownAction::DungeonPrep);
    }
    if is_key_pressed(KeyCode::H) {
        return Some(TownAction::OpenHatchery);
    }
    if is_key_pressed(KeyCode::R) {
        return Some(TownAction::OpenStable);
    }
    if is_key_pressed(KeyCode::B) {
        return Some(TownAction::OpenBreeding);
    }
    if is_key_pressed(KeyCode::W) {
        return Some(TownAction::OpenWorkshop);
    }
    if is_key_pressed(KeyCode::T) {
        return Some(TownAction::OpenShop);
    }
    if is_key_pressed(KeyCode::C) {
        return Some(TownAction::Scavenge);
    }
    if is_key_pressed(KeyCode::Escape) {
        return Some(TownAction::OpenMenu);
    }

    if ui::button_clicked(town_layout::menu_button_rect(), true) {
        return Some(TownAction::OpenMenu);
    }

    for (index, building) in data.buildings.iter().enumerate() {
        let current_level = state.town.building_level(&building.id);
        if current_level > 0
            && facility_action(&building.id).is_some()
            && ui::button_clicked(town_layout::building_open_button_rect(index), true)
        {
            return facility_action(&building.id);
        }
        if current_level < building.max_level
            && ui::button_clicked(town_layout::building_button_rect(index), true)
        {
            return Some(TownAction::AdvanceBuilding(building.id.clone()));
        }
    }

    for npc in &data.npcs {
        if ui::button_clicked(town_layout::npc_button_rect(&npc.id), true) {
            return Some(TownAction::GreetNpc(npc.id.clone()));
        }
    }

    let shop_unlocked = state.town.building_level("shop") > 0;
    for (trade, rect) in town_layout::shop_buttons() {
        if ui::button_clicked(rect, shop_unlocked) {
            return Some(TownAction::Trade(trade));
        }
    }

    for (action, rect) in town_layout::action_buttons() {
        if ui::button_clicked(rect, true) {
            return Some(action);
        }
    }

    None
}

pub fn draw(state: &GameState, data: &GameData, status_message: &str, menu_open: bool) {
    draw_town_backdrop();
    draw_header(state);
    town_panels::draw(state, data);
    draw_town_tooltips(data);
    if menu_open {
        draw_escape_menu();
    }
    ui::draw_status(status_message);
}

fn handle_menu_input() -> Option<TownAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(TownAction::CloseMenu);
    }
    if is_key_pressed(KeyCode::S) || ui::button_clicked(town_layout::menu_save_rect(), true) {
        return Some(TownAction::Save);
    }
    if is_key_pressed(KeyCode::L) || ui::button_clicked(town_layout::menu_load_rect(), true) {
        return Some(TownAction::Load);
    }
    if is_key_pressed(KeyCode::T) || ui::button_clicked(town_layout::menu_title_rect(), true) {
        return Some(TownAction::BackToMenu);
    }
    if ui::button_clicked(town_layout::menu_tutorial_rect(), true) {
        return Some(TownAction::ReplayTutorial);
    }
    if is_key_pressed(KeyCode::Enter)
        || is_key_pressed(KeyCode::Space)
        || ui::button_clicked(town_layout::menu_resume_rect(), true)
    {
        return Some(TownAction::CloseMenu);
    }
    None
}

fn facility_action(building_id: &str) -> Option<TownAction> {
    match building_id {
        "hatchery" => Some(TownAction::OpenHatchery),
        "stable" => Some(TownAction::OpenStable),
        "breeding_grove" => Some(TownAction::OpenBreeding),
        "workshop" => Some(TownAction::OpenWorkshop),
        "shop" => Some(TownAction::OpenShop),
        _ => None,
    }
}

fn draw_town_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(24, 29, 34, 255),
    );
    draw_circle(1050.0, 130.0, 90.0, Color::from_rgba(218, 178, 92, 35));
    draw_rectangle(
        0.0,
        430.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT - 430.0,
        Color::from_rgba(37, 55, 45, 255),
    );

    for index in 0..6 {
        let x = 28.0 + index as f32 * 206.0;
        let y = if index % 2 == 0 { 182.0 } else { 252.0 };
        assets::draw_landmark(index, x, y, 190.0, 190.0);
    }
}

fn draw_header(state: &GameState) {
    ui::draw_panel(Rect::new(24.0, 20.0, ui::VIEW_WIDTH - 48.0, 72.0));
    draw_ui_text_ex(
        "Tower Camp",
        48.0,
        66.0,
        TextParams {
            font_size: 34,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!("Town Rank {}", town_engine::town_rank(state)),
        262.0,
        64.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!("Day {}", state.day),
        ui::VIEW_WIDTH - 170.0,
        66.0,
        TextParams {
            font_size: 28,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    ui::draw_button(town_layout::menu_button_rect(), "Menu", true);
    ui::draw_tooltip_target(ui::Tooltip {
        rect: town_layout::menu_button_rect(),
        title: "Camp Menu",
        detail: "Save, load, or return to the title.",
    });
}

fn draw_town_tooltips(data: &GameData) {
    for (index, building) in data.buildings.iter().enumerate() {
        ui::draw_tooltip_target(ui::Tooltip {
            rect: town_layout::building_button_rect(index),
            title: "Building upgrade",
            detail: "Spend the listed supplies to improve this building.",
        });
        if facility_action(&building.id).is_some() {
            ui::draw_tooltip_target(ui::Tooltip {
                rect: town_layout::building_open_button_rect(index),
                title: "Enter facility",
                detail: "Open this facility's care, trade, or work actions.",
            });
        }
    }
    for index in 0..6 {
        let x = 28.0 + index as f32 * 206.0;
        let y = if index % 2 == 0 { 182.0 } else { 252.0 };
        ui::draw_tooltip_target(ui::Tooltip {
            rect: Rect::new(x, y, 190.0, 190.0),
            title: "Camp map",
            detail: "Explore the camp map to find its facilities.",
        });
    }
}

fn draw_escape_menu() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(5, 8, 10, 155),
    );

    let rect = town_layout::menu_panel_rect();
    ui::draw_panel(rect);
    ui::draw_centered_text(
        "Camp Menu",
        rect.x + rect.w * 0.5,
        rect.y + 54.0,
        30,
        ui::TEXT_BRIGHT,
    );
    draw_ui_text_ex(
        "Tap RESUME to close this menu.",
        rect.x + 92.0,
        rect.y + 90.0,
        TextParams {
            font_size: 17,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    ui::draw_button(town_layout::menu_resume_rect(), "Resume", true);
    ui::draw_button(town_layout::menu_save_rect(), "Save", true);
    ui::draw_button(town_layout::menu_load_rect(), "Load", true);
    ui::draw_button(town_layout::menu_tutorial_rect(), "Replay Guide", true);
    ui::draw_button(town_layout::menu_title_rect(), "Title", true);
    ui::draw_tooltip_target(ui::Tooltip {
        rect: town_layout::menu_save_rect(),
        title: "Save camp",
        detail: "Write the current day and roster to the save slot.",
    });
    ui::draw_tooltip_target(ui::Tooltip {
        rect: town_layout::menu_load_rect(),
        title: "Load camp",
        detail: "Restore the last saved day and progression.",
    });
}
