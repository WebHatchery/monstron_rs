use macroquad::prelude::*;

use crate::assets;
use crate::data::GameData;
use crate::engine::{job_engine, monster_engine};
use crate::state::{GameState, MonsterInstance, TownJobKind};
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

const JOBS: [TownJobKind; 4] = [
    TownJobKind::Forage,
    TownJobKind::Quarry,
    TownJobKind::Workshop,
    TownJobKind::HatcheryCare,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkshopAction {
    ToTown,
    Assign(u64, TownJobKind),
    Clear(u64),
}

pub fn handle_input(state: &GameState) -> Option<WorkshopAction> {
    if is_key_pressed(KeyCode::Escape) || ui::controller_cancel_pressed() {
        return Some(WorkshopAction::ToTown);
    }
    if ui::button_clicked(town_button_rect(), true) {
        return Some(WorkshopAction::ToTown);
    }

    for (index, monster) in state.monster_roster.monsters.iter().take(6).enumerate() {
        for (job_index, job) in JOBS.iter().enumerate() {
            if ui::button_clicked(
                job_button_rect(index, job_index),
                monster_engine::can_take_daily_action(state, monster).is_ok()
                    || state.town.monster_job(monster.id).is_some(),
            ) {
                return Some(WorkshopAction::Assign(monster.id, *job));
            }
        }
        if ui::button_clicked(rest_button_rect(index), true) {
            return Some(WorkshopAction::Clear(monster.id));
        }
    }

    None
}

pub fn draw(state: &GameState, data: &GameData, status_message: &str) {
    draw_backdrop();
    draw_header(state);
    draw_assignments(state, data);
    draw_reference();
    ui::draw_status(status_message);
}

fn draw_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(25, 27, 30, 255),
    );
    draw_rectangle(
        0.0,
        492.0,
        ui::VIEW_WIDTH,
        228.0,
        Color::from_rgba(54, 48, 38, 255),
    );
    draw_circle(1110.0, 130.0, 92.0, Color::from_rgba(224, 150, 83, 28));
}

fn draw_header(state: &GameState) {
    ui::draw_panel(Rect::new(32.0, 24.0, ui::VIEW_WIDTH - 64.0, 78.0));
    draw_ui_text_ex(
        "Workshop",
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
            "Day {}  Workers {}",
            state.day,
            state.town.assignments.len()
        ),
        806.0,
        70.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    ui::draw_button(town_button_rect(), "Town", true);
}

fn draw_assignments(state: &GameState, data: &GameData) {
    let rect = Rect::new(32.0, 124.0, 840.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Town Jobs", rect.x + 20.0, rect.y + 34.0);

    if state.monster_roster.monsters.is_empty() {
        draw_ui_text_ex(
            "No monsters are available for town work.",
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

    for (index, monster) in state.monster_roster.monsters.iter().take(6).enumerate() {
        draw_worker_row(rect, index, monster, state, data);
    }
}

fn draw_worker_row(
    rect: Rect,
    index: usize,
    monster: &MonsterInstance,
    state: &GameState,
    data: &GameData,
) {
    let y = rect.y + 76.0 + index as f32 * 62.0;
    let job = state.town.monster_job(monster.id);
    assets::draw_monster_badge(&monster.species_id, rect.x + 22.0, y - 32.0, 38.0);

    draw_ui_text_ex(
        &monster_label(monster, data),
        rect.x + 76.0,
        y,
        TextParams {
            font_size: 20,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!(
            "{}  Bond {}  {}  Plan: {}",
            monster.town_skill,
            monster.bond,
            monster_engine::condition_label(monster),
            monster_engine::daily_plan_label(state, monster)
        ),
        rect.x + 76.0,
        y + 23.0,
        TextParams {
            font_size: 15,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    let preview = job
        .map(|job_kind| job_engine::job_preview(monster, data, job_kind))
        .unwrap_or_else(|| "Choose overnight work.".to_owned());
    draw_ui_text_ex(
        &preview,
        rect.x + 76.0,
        y + 43.0,
        TextParams {
            font_size: 13,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );

    for (job_index, job_kind) in JOBS.iter().enumerate() {
        ui::draw_button(
            job_button_rect(index, job_index),
            job_engine::job_label(*job_kind),
            monster_engine::can_take_daily_action(state, monster).is_ok() || job.is_some(),
        );
    }
    ui::draw_button(rest_button_rect(index), "Rest", true);
}

fn draw_reference() {
    let rect = Rect::new(896.0, 124.0, ui::VIEW_WIDTH - 928.0, 476.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Job Output", rect.x + 20.0, rect.y + 34.0);

    for (index, job) in JOBS.iter().enumerate() {
        let y = rect.y + 78.0 + index as f32 * 82.0;
        draw_ui_text_ex(
            job_engine::job_label(*job),
            rect.x + 20.0,
            y,
            TextParams {
                font_size: 21,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            job_engine::job_detail(*job),
            rect.x + 20.0,
            y + 25.0,
            TextParams {
                font_size: 15,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

fn monster_label(monster: &MonsterInstance, data: &GameData) -> String {
    let species_name = data
        .species(&monster.species_id)
        .map(|species| species.name.as_str())
        .unwrap_or(monster.species_id.as_str());
    format!("{} the {}", monster.name, species_name)
}

fn town_button_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH - 148.0, 44.0, 86.0, 34.0)
}

fn job_button_rect(row: usize, column: usize) -> Rect {
    Rect::new(
        402.0 + column as f32 * 84.0,
        166.0 + row as f32 * 62.0,
        76.0,
        28.0,
    )
}

fn rest_button_rect(row: usize) -> Rect {
    Rect::new(738.0, 166.0 + row as f32 * 62.0, 66.0, 28.0)
}
