use macroquad::prelude::Rect;

use crate::engine::town_engine::ShopTrade;
use crate::screens::town::TownAction;

pub(crate) const BUILDING_ROW_HEIGHT: f32 = 52.0;

pub(crate) fn building_button_rect(index: usize) -> Rect {
    Rect::new(
        612.0,
        148.0 + index as f32 * BUILDING_ROW_HEIGHT,
        72.0,
        34.0,
    )
}

pub(crate) fn building_open_button_rect(index: usize) -> Rect {
    Rect::new(
        692.0,
        148.0 + index as f32 * BUILDING_ROW_HEIGHT,
        68.0,
        34.0,
    )
}

pub(crate) fn npc_button_rect(npc_id: &str) -> Rect {
    let index = match npc_id {
        "mara" => 0,
        "bram" => 1,
        "lio" => 2,
        _ => 0,
    };
    Rect::new(1154.0, 357.0 + index as f32 * 32.0, 76.0, 26.0)
}

pub(crate) fn shop_buttons() -> [(ShopTrade, Rect); 3] {
    [
        (ShopTrade::BuyHerbs, Rect::new(982.0, 556.0, 86.0, 30.0)),
        (ShopTrade::BuyStone, Rect::new(1076.0, 556.0, 86.0, 30.0)),
        (ShopTrade::SellHerbs, Rect::new(1170.0, 556.0, 86.0, 30.0)),
    ]
}

pub(crate) fn action_buttons() -> Vec<(TownAction, Rect)> {
    vec![
        (TownAction::DungeonPrep, Rect::new(46.0, 536.0, 176.0, 34.0)),
        (TownAction::Sleep, Rect::new(46.0, 578.0, 176.0, 34.0)),
        (TownAction::Scavenge, Rect::new(46.0, 620.0, 176.0, 34.0)),
    ]
}

pub(crate) fn menu_button_rect() -> Rect {
    Rect::new(1100.0, 40.0, 92.0, 34.0)
}

pub(crate) fn menu_panel_rect() -> Rect {
    Rect::new(466.0, 166.0, 348.0, 326.0)
}

pub(crate) fn menu_resume_rect() -> Rect {
    Rect::new(536.0, 272.0, 208.0, 36.0)
}

pub(crate) fn menu_save_rect() -> Rect {
    Rect::new(536.0, 318.0, 208.0, 36.0)
}

pub(crate) fn menu_load_rect() -> Rect {
    Rect::new(536.0, 364.0, 208.0, 36.0)
}

pub(crate) fn menu_title_rect() -> Rect {
    Rect::new(536.0, 456.0, 208.0, 36.0)
}

pub(crate) fn menu_tutorial_rect() -> Rect {
    Rect::new(536.0, 410.0, 208.0, 36.0)
}
