use macroquad::prelude::*;

use crate::assets;
use crate::data::GameData;
use crate::engine::{
    monster_engine,
    town_engine::{self, ShopTrade},
};
use crate::screens::{town::TownAction, town_layout};
use crate::state::GameState;
use crate::ui;
use macroquad_toolkit::ui::{draw_ui_text_ex, measure_ui_text};

const BODY_FONT: u16 = 20;
const DETAIL_FONT: u16 = 18;
const LABEL_FONT: u16 = 22;
const ROSTER_PREVIEW_LIMIT: usize = 3;

pub(crate) fn draw(state: &GameState, data: &GameData) {
    draw_resources(state, data);
    draw_buildings(state, data);
    draw_roster(state, data);
    draw_npcs(state, data);
    draw_tower_progress(state, data);
    draw_log(state);
    draw_shop(state);
    draw_actions();
}

fn draw_resources(state: &GameState, data: &GameData) {
    let rect = Rect::new(24.0, 112.0, 220.0, 212.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Resources", rect.x + 16.0, rect.y + 32.0);

    for (index, resource) in data.resources.iter().enumerate() {
        let y = rect.y + 64.0 + index as f32 * 28.0;
        draw_ui_text_ex(
            &resource.name,
            rect.x + 16.0,
            y,
            TextParams {
                font_size: BODY_FONT,
                color: ui::TEXT,
                ..Default::default()
            },
        );
        let amount = state.resources.amount(&resource.id);
        let label = amount.to_string();
        draw_ui_text_ex(
            &label,
            rect.x + rect.w - 20.0 - measure_ui_text(&label, None, BODY_FONT, 1.0).width,
            y,
            TextParams {
                font_size: BODY_FONT,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
    }
}

fn draw_buildings(state: &GameState, data: &GameData) {
    let rect = Rect::new(260.0, 112.0, 520.0, 360.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Buildings", rect.x + 18.0, rect.y + 32.0);

    for (index, building) in data.buildings.iter().enumerate() {
        let current_level = state.town.building_level(&building.id);
        let y = rect.y + 64.0 + index as f32 * town_layout::BUILDING_ROW_HEIGHT;
        let status = if current_level == 0 {
            "Unbuilt".to_owned()
        } else {
            format!("Lv {}/{}", current_level, building.max_level)
        };
        let cost = town_engine::next_cost(data, &building.id, current_level);
        let cost_text = if current_level >= building.max_level {
            "Maxed".to_owned()
        } else {
            town_engine::cost_text(data, &cost)
        };

        draw_ui_text_ex(
            &building.name,
            rect.x + 18.0,
            y,
            TextParams {
                font_size: LABEL_FONT,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &status,
            rect.x + 196.0,
            y,
            TextParams {
                font_size: BODY_FONT,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &cost_text,
            rect.x + 18.0,
            y + 25.0,
            TextParams {
                font_size: DETAIL_FONT,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );

        let label = if current_level == 0 {
            "Build"
        } else if current_level >= building.max_level {
            "Max"
        } else {
            "Upgrade"
        };
        ui::draw_button(
            town_layout::building_button_rect(index),
            label,
            current_level < building.max_level,
        );
        if let Some(open_label) = open_label(&building.id) {
            ui::draw_button(
                town_layout::building_open_button_rect(index),
                open_label,
                current_level > 0,
            );
        }
    }
}

fn draw_roster(state: &GameState, data: &GameData) {
    let rect = Rect::new(800.0, 112.0, ui::VIEW_WIDTH - 824.0, 180.0);
    ui::draw_panel(rect);
    ui::draw_section_title(
        &format!(
            "Monster Roster ({}/{})",
            state.monster_roster.monsters.len(),
            town_engine::monster_capacity(state)
        ),
        rect.x + 18.0,
        rect.y + 32.0,
    );

    if state.monster_roster.monsters.is_empty() {
        draw_ui_text_ex(
            "No monsters have joined yet.",
            rect.x + 18.0,
            rect.y + 82.0,
            TextParams {
                font_size: BODY_FONT,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        return;
    }

    for (index, monster) in state
        .monster_roster
        .monsters
        .iter()
        .take(ROSTER_PREVIEW_LIMIT)
        .enumerate()
    {
        let y = rect.y + 58.0 + index as f32 * 38.0;
        assets::draw_monster_badge(&monster.species_id, rect.x + 18.0, y - 22.0, 28.0);
        let species_name = data
            .species(&monster.species_id)
            .map(|species| species.name.as_str())
            .unwrap_or(monster.species_id.as_str());
        draw_ui_text_ex(
            &format!("{} the {}", monster.name, species_name),
            rect.x + 56.0,
            y,
            TextParams {
                font_size: 18,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!(
                "Lv {}  HP {}/{}  {}",
                monster.level,
                monster.hp,
                monster.max_hp,
                monster_engine::condition_label(monster)
            ),
            rect.x + 56.0,
            y + 18.0,
            TextParams {
                font_size: 14,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }

    let hidden_count = state
        .monster_roster
        .monsters
        .len()
        .saturating_sub(ROSTER_PREVIEW_LIMIT);
    if hidden_count > 0 {
        draw_ui_text_ex(
            &format!("Open the Stable to manage +{hidden_count} more."),
            rect.x + 18.0,
            rect.y + 166.0,
            TextParams {
                font_size: 14,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

fn draw_npcs(state: &GameState, data: &GameData) {
    let rect = Rect::new(800.0, 312.0, ui::VIEW_WIDTH - 824.0, 160.0);
    ui::draw_panel(rect);
    ui::draw_section_title("NPC Services", rect.x + 18.0, rect.y + 32.0);

    for (index, npc) in data.npcs.iter().enumerate() {
        let y = rect.y + 62.0 + index as f32 * 32.0;
        assets::draw_npc(&npc.id, rect.x + 18.0, y - 28.0, 28.0);
        draw_ui_text_ex(
            &format!("{} - {}", npc.name, npc.service),
            rect.x + 52.0,
            y,
            TextParams {
                font_size: BODY_FONT,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!("F {}", state.npc_friendship(&npc.id)),
            rect.x + 294.0,
            y,
            TextParams {
                font_size: DETAIL_FONT,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        ui::draw_button(town_layout::npc_button_rect(&npc.id), "Greet", true);
    }
}

fn draw_tower_progress(state: &GameState, data: &GameData) {
    let rect = Rect::new(24.0, 344.0, 220.0, 142.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Tower", rect.x + 16.0, rect.y + 32.0);
    draw_ui_text_ex(
        &format!(
            "Reached {}  ·  Open {}",
            state.tower_progress.best_floor, state.tower_progress.unlocked_floor
        ),
        rect.x + 16.0,
        rect.y + 70.0,
        TextParams {
            font_size: DETAIL_FONT,
            color: ui::TEXT,
            ..Default::default()
        },
    );
    let objective = campaign_objective(state, data);
    draw_ui_text_ex(
        &objective.heading,
        rect.x + 16.0,
        rect.y + 98.0,
        TextParams {
            font_size: 14,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &objective.action,
        rect.x + 16.0,
        rect.y + 122.0,
        TextParams {
            font_size: 14,
            color: ui::TEXT,
            ..Default::default()
        },
    );
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CampaignObjective {
    heading: String,
    action: String,
}

fn campaign_objective(state: &GameState, data: &GameData) -> CampaignObjective {
    if state.story_flags.has("verdant_crown_restored") {
        return CampaignObjective {
            heading: "CROWN RESTORED".to_owned(),
            action: "Free expeditions remain open.".to_owned(),
        };
    }

    let max_floor = data
        .tower_floors
        .iter()
        .map(|floor| floor.floor)
        .max()
        .unwrap_or(1);
    let open_floor = state.tower_progress.unlocked_floor.clamp(1, max_floor);
    let Some(floor) = data.tower_floor(open_floor) else {
        return CampaignObjective {
            heading: "NEXT · CLIMB THE TOWER".to_owned(),
            action: "Reach the open floor's stairs.".to_owned(),
        };
    };
    if !floor.guardian_enemy_id.is_empty() {
        if state.tower_progress.guardian_defeated(open_floor) {
            return CampaignObjective {
                heading: "NEXT · OPEN THRESHOLD".to_owned(),
                action: if open_floor == max_floor {
                    "Return and cross the Crown threshold.".to_owned()
                } else {
                    "Return and reach the open stairs.".to_owned()
                },
            };
        }
        let guardian = data
            .enemy(&floor.guardian_enemy_id)
            .map_or(floor.name.as_str(), |enemy| enemy.name.as_str());
        return CampaignObjective {
            heading: format!("NEXT · {}", guardian.to_uppercase()),
            action: if open_floor == max_floor {
                "Defeat it and cross the threshold.".to_owned()
            } else {
                "Defeat it, then reach the stairs.".to_owned()
            },
        };
    }

    CampaignObjective {
        heading: "NEXT · CLIMB THE TOWER".to_owned(),
        action: "Reach the open floor's stairs.".to_owned(),
    }
}

fn draw_log(state: &GameState) {
    let rect = Rect::new(260.0, 492.0, 520.0, 132.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Activity Log", rect.x + 18.0, rect.y + 30.0);

    let visible = state.activity_log.entries.iter().rev().take(2);
    for (index, entry) in visible.enumerate() {
        let y = rect.y + 62.0 + index as f32 * 32.0;
        draw_ui_text_ex(
            &short_log_line(entry.day, &entry.message),
            rect.x + 18.0,
            y,
            TextParams {
                font_size: DETAIL_FONT,
                color: ui::TEXT,
                ..Default::default()
            },
        );
    }
}

fn draw_shop(state: &GameState) {
    let rect = Rect::new(800.0, 492.0, ui::VIEW_WIDTH - 824.0, 132.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Shop", rect.x + 18.0, rect.y + 30.0);

    let shop_unlocked = state.town.building_level("shop") > 0;
    let helper = if shop_unlocked {
        "Buy basics or sell surplus herbs."
    } else {
        "Build the shop to unlock trades."
    };
    draw_ui_text_ex(
        helper,
        rect.x + 18.0,
        rect.y + 58.0,
        TextParams {
            font_size: DETAIL_FONT,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );

    for (trade, button_rect) in town_layout::shop_buttons() {
        let label = match trade {
            ShopTrade::BuyHerbs => "Buy Herbs",
            ShopTrade::BuyStone => "Buy Stone",
            ShopTrade::SellHerbs => "Sell Herbs",
        };
        ui::draw_button(button_rect, label, shop_unlocked);
    }
}

fn draw_actions() {
    let rect = Rect::new(24.0, 492.0, 220.0, 176.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Camp Actions", rect.x + 16.0, rect.y + 30.0);

    for (action, button_rect) in town_layout::action_buttons() {
        let label = match action {
            TownAction::Scavenge => "Scavenge",
            TownAction::Sleep => "Sleep",
            TownAction::DungeonPrep => "Tower",
            TownAction::OpenMenu
            | TownAction::CloseMenu
            | TownAction::OpenHatchery
            | TownAction::OpenStable
            | TownAction::OpenBreeding
            | TownAction::OpenWorkshop
            | TownAction::OpenShop
            | TownAction::Save
            | TownAction::Load
            | TownAction::BackToMenu
            | TownAction::ReplayTutorial
            | TownAction::AdvanceBuilding(_)
            | TownAction::Trade(_)
            | TownAction::GreetNpc(_) => "",
        };
        ui::draw_button(button_rect, label, true);
    }
}

fn open_label(building_id: &str) -> Option<&'static str> {
    match building_id {
        "hatchery" | "stable" | "breeding_grove" | "workshop" | "shop" => Some("Open"),
        _ => None,
    }
}

fn short_log_line(day: u32, message: &str) -> String {
    const MAX_CHARS: usize = 58;
    let prefix = format!("Day {day}: ");
    let available = MAX_CHARS.saturating_sub(prefix.len());
    let mut body = message.chars().take(available).collect::<String>();
    if message.chars().count() > available {
        body.push_str("...");
    }
    format!("{prefix}{body}")
}

#[cfg(test)]
mod tests;
