use macroquad::prelude::*;

use crate::data::GameData;
use crate::engine::town_engine::ShopTrade;
use crate::state::GameState;
use crate::ui;
use macroquad_toolkit::ui::{draw_ui_text_ex, measure_ui_text};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShopAction {
    ToTown,
    Trade(ShopTrade),
}

pub fn handle_input() -> Option<ShopAction> {
    if is_key_pressed(KeyCode::Escape) || ui::controller_cancel_pressed() {
        return Some(ShopAction::ToTown);
    }
    if ui::button_clicked(town_button_rect(), true) {
        return Some(ShopAction::ToTown);
    }

    for (index, trade) in ShopTrade::ALL.iter().enumerate() {
        if ui::button_clicked(trade_button_rect(index), true) {
            return Some(ShopAction::Trade(*trade));
        }
    }

    None
}

pub fn draw(state: &GameState, data: &GameData, status_message: &str) {
    draw_backdrop();
    draw_header(state);
    draw_trades(data);
    draw_resources(state, data);
    ui::draw_status(status_message);
}

fn draw_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(25, 27, 31, 255),
    );
    draw_rectangle(
        0.0,
        486.0,
        ui::VIEW_WIDTH,
        234.0,
        Color::from_rgba(47, 42, 38, 255),
    );
    draw_circle(1090.0, 120.0, 88.0, Color::from_rgba(220, 180, 94, 26));
}

fn draw_header(state: &GameState) {
    ui::draw_panel(Rect::new(32.0, 24.0, ui::VIEW_WIDTH - 64.0, 78.0));
    draw_ui_text_ex(
        "Camp Shop",
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
            "Day {}  Shop Lv {}",
            state.day,
            state.town.building_level("shop")
        ),
        790.0,
        70.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    ui::draw_button(town_button_rect(), "Town", true);
}

fn draw_trades(data: &GameData) {
    let rect = Rect::new(32.0, 124.0, 760.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Trades", rect.x + 20.0, rect.y + 34.0);

    for (index, trade) in ShopTrade::ALL.iter().enumerate() {
        let Some(definition) = data.shop_trade(trade.id()) else {
            continue;
        };
        let y = rect.y + 84.0 + index as f32 * 108.0;
        draw_ui_text_ex(
            &definition.label,
            rect.x + 28.0,
            y,
            TextParams {
                font_size: 25,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &definition.detail,
            rect.x + 28.0,
            y + 28.0,
            TextParams {
                font_size: 18,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &trade_cost(data, *trade),
            rect.x + 28.0,
            y + 55.0,
            TextParams {
                font_size: 17,
                color: ui::TEXT,
                ..Default::default()
            },
        );
        ui::draw_button(trade_button_rect(index), "Trade", true);
    }
}

fn draw_resources(state: &GameState, data: &GameData) {
    let rect = Rect::new(824.0, 124.0, ui::VIEW_WIDTH - 856.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Inventory", rect.x + 20.0, rect.y + 34.0);

    for (index, resource) in data.resources.iter().enumerate() {
        let y = rect.y + 78.0 + index as f32 * 42.0;
        draw_ui_text_ex(
            &resource.name,
            rect.x + 24.0,
            y,
            TextParams {
                font_size: 21,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        let amount = state.resources.amount(&resource.id).to_string();
        draw_ui_text_ex(
            &amount,
            rect.x + rect.w - 28.0 - measure_ui_text(&amount, None, 21, 1.0).width,
            y,
            TextParams {
                font_size: 21,
                color: ui::ACCENT,
                ..Default::default()
            },
        );
    }
}

fn trade_cost(data: &GameData, trade: ShopTrade) -> String {
    let Some(definition) = data.shop_trade(trade.id()) else {
        return "Trade data unavailable".to_owned();
    };
    let format_stacks = |stacks: &[crate::data::ResourceAmount]| {
        stacks
            .iter()
            .map(|stack| {
                format!(
                    "{} {}",
                    stack.amount,
                    data.resource_name(&stack.resource_id)
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "Pay {}, gain {}",
        format_stacks(&definition.cost),
        format_stacks(&definition.reward)
    )
}

fn town_button_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH - 148.0, 44.0, 86.0, 34.0)
}

fn trade_button_rect(index: usize) -> Rect {
    Rect::new(650.0, 186.0 + index as f32 * 108.0, 96.0, 34.0)
}
