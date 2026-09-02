use macroquad::prelude::*;

mod context_drawer;
mod event_prompt;
mod field_guide;
mod journal;
mod map_view;

use crate::assets;
use crate::data::GameData;
use crate::engine::{tower_engine, town_engine};
use crate::state::{GameState, TowerRunState};
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TowerAction {
    Move(i32, i32),
    RouteTo(u32, u32),
    Explore,
    Survey,
    Camp,
    ChooseEvent(String),
    LeaveEvent,
    ReturnToTown,
    ToTown,
    OpenGuide,
    CloseGuide,
    GuidePage(i32),
}

pub fn handle_input(
    state: &GameState,
    data: &GameData,
    guide_open: bool,
    guide_page: usize,
) -> Option<TowerAction> {
    if guide_open {
        return field_guide::handle_input(state, data, guide_page);
    }
    if is_key_pressed(KeyCode::Escape) {
        return if state.tower_run.is_some() {
            Some(TowerAction::ReturnToTown)
        } else {
            Some(TowerAction::ToTown)
        };
    }

    if ui::button_clicked(guide_button_rect(state.tower_run.is_some()), true) {
        return Some(TowerAction::OpenGuide);
    }

    if let Some(run) = &state.tower_run {
        if run.pending_event.is_some() {
            return event_prompt::handle_input(state, data, run);
        }
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            return Some(TowerAction::Move(0, -1));
        }
        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            return Some(TowerAction::Move(0, 1));
        }
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            return Some(TowerAction::Move(-1, 0));
        }
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            return Some(TowerAction::Move(1, 0));
        }
        if is_key_pressed(KeyCode::R) {
            return Some(TowerAction::ReturnToTown);
        }
    }

    if ui::button_clicked(town_button_rect(), true) {
        return if state.tower_run.is_some() {
            Some(TowerAction::ReturnToTown)
        } else {
            Some(TowerAction::ToTown)
        };
    }

    if let Some(run) = &state.tower_run {
        if let Some(action) = map_view::world_tap_action(run) {
            return Some(action);
        }
        if ui::button_clicked(explore_button_rect(), true) {
            return Some(TowerAction::Explore);
        }
        if ui::button_clicked(survey_button_rect(), run_has_survey(state)) {
            return Some(TowerAction::Survey);
        }
        if ui::button_clicked(camp_button_rect(), true) {
            return Some(TowerAction::Camp);
        }
        if ui::button_clicked(retreat_button_rect(), true) {
            return Some(TowerAction::ReturnToTown);
        }
        if ui::button_clicked(return_button_rect(), true) {
            return Some(TowerAction::ReturnToTown);
        }
    }

    None
}

pub fn draw(
    state: &GameState,
    data: &GameData,
    status_message: &str,
    guide_open: bool,
    guide_page: usize,
) {
    if let Some(run) = &state.tower_run {
        map_view::draw_map_world(state, data, run);
        draw_run_overlay(state, data, run);
        draw_party_rail(state);
        draw_action_dock(run);
        context_drawer::draw(data, run);
        journal::draw(data, run);
        journal::draw_anomaly(data, run);
        event_prompt::draw(state, data, run);
        ui::draw_button(guide_button_rect(true), "FIELD GUIDE", true);
        ui::draw_status_at(status_message, Rect::new(250.0, 60.0, 780.0, 28.0));
    } else {
        draw_backdrop();
        draw_header(state);
        draw_empty_run();
        draw_floor_reference(state, data);
        ui::draw_button(guide_button_rect(false), "OPEN FIELD GUIDE", true);
    }

    if state.tower_run.is_none() {
        ui::draw_status(status_message);
    }
    if guide_open {
        field_guide::draw(state, data, guide_page);
    }
}

fn draw_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(12, 16, 18, 255),
    );
    draw_rectangle(
        0.0,
        520.0,
        ui::VIEW_WIDTH,
        200.0,
        Color::from_rgba(26, 38, 34, 255),
    );

    for index in 0..8 {
        let x = 82.0 + index as f32 * 156.0;
        let height = 270.0 + (index % 4) as f32 * 54.0;
        draw_rectangle(
            x,
            520.0 - height,
            86.0,
            height,
            Color::from_rgba(34, 42, 49, 210),
        );
        draw_rectangle_lines(
            x,
            520.0 - height,
            86.0,
            height,
            2.0,
            Color::from_rgba(65, 80, 83, 190),
        );
    }
}

fn draw_header(state: &GameState) {
    ui::draw_panel(Rect::new(32.0, 24.0, ui::VIEW_WIDTH - 64.0, 78.0));
    draw_ui_text_ex(
        "Tower Map",
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
            "Best {}  Unlocked {}",
            state.tower_progress.best_floor, state.tower_progress.unlocked_floor
        ),
        760.0,
        70.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    let label = if state.tower_run.is_some() {
        "Return"
    } else {
        "Town"
    };
    ui::draw_button(town_button_rect(), label, true);
}

fn draw_run_overlay(state: &GameState, data: &GameData, run: &TowerRunState) {
    let floor = data.tower_floor(run.current_floor);
    let floor_name = floor
        .map(|floor| floor.name.as_str())
        .unwrap_or("Unknown Floor");
    let top = Rect::new(0.0, 0.0, ui::VIEW_WIDTH, 54.0);
    draw_overlay_panel(top);
    draw_ui_text_ex(
        "HATCHSPIRE",
        22.0,
        35.0,
        TextParams {
            font_size: 27,
            color: gold_bright(),
            ..Default::default()
        },
    );
    draw_line(205.0, 12.0, 205.0, 43.0, 1.0, gold_dim());
    draw_ui_text_ex(
        &format!(
            "FLOOR {}  ·  {}",
            run.current_floor,
            floor_name.to_uppercase()
        ),
        238.0,
        34.0,
        TextParams {
            font_size: 23,
            color: gold_bright(),
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!(
            "Eggs {}/{}     Supplies {}     Coins {}",
            state.egg_inventory.eggs.len() + run.found_eggs.len(),
            town_engine::egg_capacity(state),
            run.cargo_amount(),
            state.resources.amount("coins")
        ),
        755.0,
        34.0,
        TextParams {
            font_size: 19,
            color: gold_bright(),
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!("Pressure {}/{}", run.pressure, run.pressure_limit),
        555.0,
        34.0,
        TextParams {
            font_size: 18,
            color: if run.pressure + 2 >= run.pressure_limit {
                Color::from_rgba(232, 112, 82, 255)
            } else {
                gold_bright()
            },
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        "SET",
        1233.0,
        37.0,
        TextParams {
            font_size: 16,
            color: gold_bright(),
            ..Default::default()
        },
    );
}

fn draw_party_rail(state: &GameState) {
    let panel = Rect::new(10.0, 68.0, 152.0, 260.0);
    draw_overlay_panel(panel);
    for (index, slot) in state
        .monster_roster
        .party_slots
        .iter()
        .flatten()
        .take(3)
        .enumerate()
    {
        let Some(monster) = state.monster_roster.monster(*slot) else {
            continue;
        };
        let y = panel.y + 9.0 + index as f32 * 82.0;
        draw_rectangle(
            panel.x + 12.0,
            y,
            panel.w - 24.0,
            74.0,
            Color::from_rgba(8, 12, 14, 205),
        );
        assets::draw_party_portrait(&monster.species_id, panel.x + 8.0, y + 7.0, 61.0);
        draw_ui_text_ex(
            &monster.name,
            panel.x + 72.0,
            y + 27.0,
            TextParams {
                font_size: 17,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_rectangle(
            panel.x + 72.0,
            y + 38.0,
            68.0,
            7.0,
            Color::from_rgba(31, 47, 42, 255),
        );
        draw_rectangle(
            panel.x + 72.0,
            y + 38.0,
            68.0 * (monster.hp.max(0) as f32 / monster.max_hp.max(1) as f32),
            7.0,
            Color::from_rgba(112, 184, 108, 255),
        );
        draw_ui_text_ex(
            &format!("HP {}/{}", monster.hp, monster.max_hp),
            panel.x + 72.0,
            y + 62.0,
            TextParams {
                font_size: 13,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

fn draw_action_dock(run: &TowerRunState) {
    let explore_title = if run.route_target.is_some() {
        "Step now"
    } else {
        "Explore"
    };
    for (index, (icon, title)) in [
        ("+", explore_title),
        ("?", "Survey"),
        ("*", "Camp"),
        ("<", "Retreat"),
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(300.0 + index as f32 * 162.0, 650.0, 150.0, 58.0);
        draw_overlay_panel(rect);
        draw_ui_text_ex(
            icon,
            rect.x + 14.0,
            rect.y + 39.0,
            TextParams {
                font_size: 29,
                color: gold_bright(),
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            title,
            rect.x + 48.0,
            rect.y + 37.0,
            TextParams {
                font_size: 20,
                color: gold_bright(),
                ..Default::default()
            },
        );
    }
    draw_ui_text_ex(
        &format!("{} flare(s)", run.survey_charges),
        survey_button_rect().x + 36.0,
        survey_button_rect().y - 8.0,
        TextParams {
            font_size: 13,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    if run.camp_cooldown > 0 {
        draw_ui_text_ex(
            &format!("Ready in {} steps", run.camp_cooldown),
            camp_button_rect().x + 38.0,
            camp_button_rect().y - 8.0,
            TextParams {
                font_size: 13,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    } else if tower_engine::camp_sheltered(run) {
        draw_ui_text_ex(
            "SHELTER BONUS",
            camp_button_rect().x + 26.0,
            camp_button_rect().y - 8.0,
            TextParams {
                font_size: 13,
                color: Color::from_rgba(135, 211, 151, 255),
                ..Default::default()
            },
        );
    }
    if run.route_target.is_some() {
        draw_ui_text_ex(
            "ROUTE ACTIVE · tap a room to redirect",
            explore_button_rect().x - 2.0,
            explore_button_rect().y - 8.0,
            TextParams {
                font_size: 13,
                color: Color::from_rgba(232, 173, 82, 255),
                ..Default::default()
            },
        );
    }
}

fn draw_overlay_panel(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(8, 12, 13, 238),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::from_rgba(104, 76, 37, 235),
    );
    draw_rectangle_lines(
        rect.x + 4.0,
        rect.y + 4.0,
        rect.w - 8.0,
        rect.h - 8.0,
        1.0,
        Color::from_rgba(184, 133, 60, 100),
    );
}

fn gold_bright() -> Color {
    Color::from_rgba(227, 196, 139, 255)
}
fn gold_dim() -> Color {
    Color::from_rgba(112, 82, 42, 220)
}

fn draw_empty_run() {
    let rect = Rect::new(32.0, 124.0, 560.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("No Active Run", rect.x + 20.0, rect.y + 34.0);
    draw_ui_text_ex(
        "Enter the tower from town to begin a run.",
        rect.x + 20.0,
        rect.y + 88.0,
        TextParams {
            font_size: 25,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_wrapped_line(
        "Dungeon maps are generated each run with rooms, landmarks, event sites, hazards, caches, eggs, enemies, stairs, and exits.",
        rect.x + 20.0,
        rect.y + 132.0,
        58,
        ui::TEXT_DIM,
    );
}

fn draw_floor_reference(state: &GameState, data: &GameData) {
    let rect = Rect::new(620.0, 124.0, 628.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Known Floors", rect.x + 20.0, rect.y + 34.0);
    draw_ui_text_ex(
        &format!(
            "FIELD GUIDE  Enemies {}/{}  Landmarks {}/{}  Hazards {}/{}",
            state.tower_discoveries.enemy_ids.len(),
            data.enemies.len(),
            state.tower_discoveries.special_location_ids.len(),
            data.tower_special_locations.len(),
            state.tower_discoveries.hazard_ids.len(),
            data.tower_hazards.len()
        ),
        rect.x + 20.0,
        rect.y + 60.0,
        TextParams {
            font_size: 14,
            color: gold_bright(),
            ..Default::default()
        },
    );

    for (index, floor) in data.tower_floors.iter().take(10).enumerate() {
        let y = rect.y + 88.0 + index as f32 * 38.0;
        let color = if floor.floor <= state.tower_progress.unlocked_floor {
            ui::TEXT_BRIGHT
        } else {
            ui::TEXT_DIM
        };
        draw_ui_text_ex(
            &format!("{}  {}", floor.floor, floor.name),
            rect.x + 20.0,
            y,
            TextParams {
                font_size: 19,
                color,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!("{}  {}", floor.theme, floor.enemy_hint),
            rect.x + 300.0,
            y,
            TextParams {
                font_size: 16,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

fn draw_wrapped_line(text: &str, x: f32, y: f32, max_chars: usize, color: Color) {
    let mut line = String::new();
    let mut row = 0;

    for word in text.split_whitespace() {
        let next_len = if line.is_empty() {
            word.len()
        } else {
            line.len() + 1 + word.len()
        };
        if next_len > max_chars && !line.is_empty() {
            draw_ui_text_ex(
                &line,
                x,
                y + row as f32 * 20.0,
                TextParams {
                    font_size: 16,
                    color,
                    ..Default::default()
                },
            );
            line.clear();
            row += 1;
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }

    if !line.is_empty() {
        draw_ui_text_ex(
            &line,
            x,
            y + row as f32 * 20.0,
            TextParams {
                font_size: 16,
                color,
                ..Default::default()
            },
        );
    }
}

fn town_button_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH - 116.0, 18.0, 98.0, 40.0)
}

fn return_button_rect() -> Rect {
    town_button_rect()
}

pub(crate) fn explore_button_rect() -> Rect {
    Rect::new(300.0, 650.0, 150.0, 58.0)
}

pub(crate) fn survey_button_rect() -> Rect {
    Rect::new(462.0, 650.0, 150.0, 58.0)
}

fn camp_button_rect() -> Rect {
    Rect::new(624.0, 650.0, 150.0, 58.0)
}

pub(crate) fn retreat_button_rect() -> Rect {
    Rect::new(786.0, 650.0, 150.0, 58.0)
}

fn run_has_survey(state: &GameState) -> bool {
    state
        .tower_run
        .as_ref()
        .is_some_and(|run| run.survey_charges > 0 && run.pressure < run.pressure_limit)
}

fn guide_button_rect(active_run: bool) -> Rect {
    if active_run {
        Rect::new(18.0, 348.0, 218.0, 46.0)
    } else {
        Rect::new(998.0, 616.0, 250.0, 52.0)
    }
}
