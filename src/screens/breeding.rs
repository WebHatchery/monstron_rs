use macroquad::prelude::*;

use crate::assets;
use crate::data::{GameData, PassiveSkill};
use crate::engine::{breeding_engine, monster_engine, town_engine};
use crate::state::{GameState, MonsterInstance};
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BreedingAction {
    ToTown,
    Breed(u64, u64),
}

pub fn handle_input(state: &GameState) -> Option<BreedingAction> {
    if is_key_pressed(KeyCode::Escape) || ui::controller_cancel_pressed() {
        return Some(BreedingAction::ToTown);
    }
    if ui::button_clicked(town_button_rect(), true) {
        return Some(BreedingAction::ToTown);
    }

    for (index, (first, second)) in visible_pairs(state).iter().enumerate() {
        let enabled = breeding_engine::pair_is_compatible(first, second)
            && state.resources.amount("herbs") >= 2
            && town_engine::has_egg_capacity(state)
            && monster_engine::can_take_daily_action(state, first).is_ok()
            && monster_engine::can_take_daily_action(state, second).is_ok();
        if ui::button_clicked(pair_button_rect(index), enabled) {
            return Some(BreedingAction::Breed(first.id, second.id));
        }
    }

    None
}

pub fn draw(state: &GameState, data: &GameData, status_message: &str) {
    draw_backdrop();
    draw_header(state);
    draw_pairs(state, data);
    draw_grove_status(state, data);
    ui::draw_status(status_message);
}

fn draw_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(21, 29, 28, 255),
    );
    draw_circle(1120.0, 120.0, 120.0, Color::from_rgba(110, 186, 132, 28));
    draw_circle(1080.0, 152.0, 58.0, Color::from_rgba(224, 183, 98, 32));
    draw_rectangle(
        0.0,
        492.0,
        ui::VIEW_WIDTH,
        228.0,
        Color::from_rgba(36, 55, 43, 255),
    );
}

fn draw_header(state: &GameState) {
    ui::draw_panel(Rect::new(32.0, 24.0, ui::VIEW_WIDTH - 64.0, 78.0));
    draw_ui_text_ex(
        "Breeding Grove",
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
            "Day {}  Herbs {}  Eggs {}/{}",
            state.day,
            state.resources.amount("herbs"),
            state.egg_inventory.eggs.len(),
            town_engine::egg_capacity(state)
        ),
        740.0,
        70.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    ui::draw_button(town_button_rect(), "Town", true);
}

fn draw_pairs(state: &GameState, data: &GameData) {
    let rect = Rect::new(32.0, 124.0, 780.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Compatible Pairs", rect.x + 20.0, rect.y + 34.0);

    let pairs = visible_pairs(state);
    if pairs.is_empty() {
        draw_ui_text_ex(
            "Two monsters are needed before the grove can prepare an egg.",
            rect.x + 24.0,
            rect.y + 108.0,
            TextParams {
                font_size: 23,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        return;
    }

    for (index, (first, second)) in pairs.iter().enumerate() {
        draw_pair_row(
            rect,
            index,
            first,
            second,
            data,
            state,
            state.resources.amount("herbs") >= 2,
            town_engine::has_egg_capacity(state),
            state.tower_progress.best_floor.max(1),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_pair_row(
    rect: Rect,
    index: usize,
    first: &MonsterInstance,
    second: &MonsterInstance,
    data: &GameData,
    state: &GameState,
    has_cost: bool,
    has_egg_capacity: bool,
    origin_floor: u32,
) {
    let y = rect.y + 76.0 + index as f32 * 62.0;
    let compatible = breeding_engine::pair_is_compatible(first, second);
    assets::draw_monster_badge(&first.species_id, rect.x + 22.0, y - 32.0, 38.0);
    assets::draw_monster_badge(&second.species_id, rect.x + 68.0, y - 32.0, 38.0);

    draw_ui_text_ex(
        &format!(
            "{} + {}",
            monster_label(first, data),
            monster_label(second, data)
        ),
        rect.x + 122.0,
        y,
        TextParams {
            font_size: 21,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!(
            "{}  Bond {}+{}  {} lineage",
            breeding_engine::compatibility_label(first, second),
            first.bond,
            second.bond,
            breeding_engine::lineage_quality_label(breeding_engine::lineage_quality(
                first,
                second,
                origin_floor
            ))
        ),
        rect.x + 122.0,
        y + 24.0,
        TextParams {
            font_size: 16,
            color: if compatible { ui::TEXT_DIM } else { ui::WARN },
            ..Default::default()
        },
    );
    let available = monster_engine::can_take_daily_action(state, first).is_ok()
        && monster_engine::can_take_daily_action(state, second).is_ok();
    let forecast = breeding_engine::forecast_pair(state, data, first, second)
        .map(|forecast| forecast_summary(data, &forecast))
        .unwrap_or_else(|| "No stable egg forecast.".to_owned());
    let forecast_line = if !has_egg_capacity {
        "Hatchery egg capacity is full."
    } else if available {
        forecast.as_str()
    } else {
        "One parent is already committed today."
    };
    draw_ui_text_ex(
        forecast_line,
        rect.x + 122.0,
        y + 43.0,
        TextParams {
            font_size: 13,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    ui::draw_button(
        pair_button_rect(index),
        "Breed",
        compatible && has_cost && available && has_egg_capacity,
    );
}

fn draw_grove_status(state: &GameState, data: &GameData) {
    let rect = Rect::new(836.0, 124.0, ui::VIEW_WIDTH - 868.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Grove Lineage", rect.x + 20.0, rect.y + 34.0);

    let grove_level = state.town.building_level("breeding_grove");
    let cost_text = breeding_engine::breeding_cost()
        .iter()
        .map(|(resource_id, amount)| format!("{} {}", amount, data.resource_name(resource_id)))
        .collect::<Vec<_>>()
        .join(", ");
    let lines = [
        format!("Grove level: {}", grove_level),
        format!(
            "Egg slots: {}/{}",
            state.egg_inventory.eggs.len(),
            town_engine::egg_capacity(state)
        ),
        format!("Pairing cost: {}", cost_text),
        format!("Origin floor: {}", state.tower_progress.best_floor.max(1)),
        format!(
            "Mutation chance: {}%",
            breeding_engine::mutation_chance(state.tower_progress.best_floor.max(1))
        ),
    ];

    for (index, line) in lines.iter().enumerate() {
        draw_ui_text_ex(
            line,
            rect.x + 22.0,
            rect.y + 78.0 + index as f32 * 28.0,
            TextParams {
                font_size: 19,
                color: ui::TEXT,
                ..Default::default()
            },
        );
    }

    draw_recent_bred_eggs(state, data, rect.x + 22.0, rect.y + 224.0);
}

fn draw_recent_bred_eggs(state: &GameState, data: &GameData, x: f32, y: f32) {
    draw_ui_text_ex(
        "Recent Grove Eggs",
        x,
        y,
        TextParams {
            font_size: 20,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );

    let mut rows = state
        .egg_inventory
        .eggs
        .iter()
        .filter(|egg| egg.inheritance.is_some())
        .rev()
        .take(3)
        .peekable();
    if rows.peek().is_none() {
        draw_ui_text_ex(
            "No bred eggs are waiting yet.",
            x,
            y + 42.0,
            TextParams {
                font_size: 18,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
        return;
    }

    for (index, egg) in rows.enumerate() {
        let row_y = y + 46.0 + index as f32 * 54.0;
        let egg_name = data
            .egg_type(&egg.egg_type_id)
            .map(|egg_type| egg_type.name.as_str())
            .unwrap_or(egg.egg_type_id.as_str());
        let mutation = egg
            .inheritance
            .as_ref()
            .is_some_and(|inheritance| inheritance.mutated);
        assets::draw_egg_badge(&egg.egg_type_id, x, row_y - 32.0, 36.0);
        draw_ui_text_ex(
            &format!("{} #{}", egg_name, egg.id),
            x + 50.0,
            row_y,
            TextParams {
                font_size: 18,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            if mutation {
                "mutated lineage"
            } else {
                "inherited lineage"
            },
            x + 50.0,
            row_y + 20.0,
            TextParams {
                font_size: 15,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

fn visible_pairs(state: &GameState) -> Vec<(&MonsterInstance, &MonsterInstance)> {
    let monsters = state
        .monster_roster
        .monsters
        .iter()
        .take(6)
        .collect::<Vec<_>>();
    let mut pairs = Vec::new();
    for first_index in 0..monsters.len() {
        for second_index in first_index + 1..monsters.len() {
            pairs.push((monsters[first_index], monsters[second_index]));
            if pairs.len() >= 6 {
                return pairs;
            }
        }
    }
    pairs
}

fn monster_label(monster: &MonsterInstance, data: &GameData) -> String {
    let species_name = data
        .species(&monster.species_id)
        .map(|species| species.name.as_str())
        .unwrap_or(monster.species_id.as_str());
    format!("{} the {}", monster.name, species_name)
}

fn forecast_summary(data: &GameData, forecast: &breeding_engine::BreedingForecast) -> String {
    let egg_text = forecast
        .egg_options
        .iter()
        .take(2)
        .map(|(name, chance)| format!("{chance}% {}", short_egg_name(name)))
        .collect::<Vec<_>>()
        .join(", ");
    let species_text = forecast
        .species_options
        .iter()
        .take(2)
        .map(|species_id| {
            data.species(species_id)
                .map(|species| species.name.clone())
                .unwrap_or_else(|| species_id.clone())
        })
        .collect::<Vec<_>>()
        .join("/");
    let element_text = forecast
        .element_options
        .iter()
        .take(2)
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("/");
    let temperament_text = forecast
        .temperament_options
        .iter()
        .take(2)
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("/");
    let passive_text = forecast
        .passive_options
        .first()
        .map(|passive| passive_short(*passive))
        .unwrap_or("-");
    format!(
        "{} | {} | {} | {} | {} | {} lineage | Mut {}%",
        egg_text,
        species_text,
        element_text,
        temperament_text,
        passive_text,
        breeding_engine::lineage_quality_label(forecast.lineage_quality),
        forecast.mutation_chance
    )
}

fn short_egg_name(name: &str) -> &str {
    name.strip_suffix(" Egg").unwrap_or(name)
}

fn passive_short(passive: PassiveSkill) -> &'static str {
    match passive {
        PassiveSkill::FindsSmallLoot => "Loot",
        PassiveSkill::ResistsPoison => "Resist",
        PassiveSkill::DetectsEggs => "Egg sense",
        PassiveSkill::FindsStone => "Stone",
        PassiveSkill::BurnsBrambles => "Brambles",
        PassiveSkill::SoothesInjuries => "Soothe",
    }
}

fn town_button_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH - 148.0, 44.0, 86.0, 34.0)
}

fn pair_button_rect(index: usize) -> Rect {
    Rect::new(704.0, 166.0 + index as f32 * 62.0, 78.0, 30.0)
}
