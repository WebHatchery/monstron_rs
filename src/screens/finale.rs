use macroquad::prelude::*;

use crate::assets;
use crate::state::GameState;
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub const EPILOGUE_SEEN: &str = "verdant_crown_epilogue_seen";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FinaleAction {
    ContinueInTown,
    ReturnToTitle,
}

pub fn handle_input() -> Option<FinaleAction> {
    if is_key_pressed(KeyCode::Enter) || ui::button_clicked(continue_rect(), true) {
        return Some(FinaleAction::ContinueInTown);
    }
    if is_key_pressed(KeyCode::Escape) || ui::button_clicked(title_rect(), true) {
        return Some(FinaleAction::ReturnToTitle);
    }
    None
}

pub fn draw(state: &GameState) {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        color(5, 13, 14, 255),
    );
    assets::draw_room_vignette(10, 0.0, 0.0, ui::VIEW_WIDTH, ui::VIEW_HEIGHT);
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        color(3, 9, 10, 175),
    );

    for ring in 0..7 {
        draw_circle_lines(
            ui::VIEW_WIDTH * 0.5,
            272.0,
            92.0 + ring as f32 * 20.0,
            2.0,
            color(174, 212, 118, 70_u8.saturating_sub(ring * 8)),
        );
    }

    let panel = Rect::new(176.0, 54.0, 928.0, 612.0);
    draw_rectangle(panel.x, panel.y, panel.w, panel.h, color(6, 13, 14, 228));
    draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        2.0,
        color(199, 154, 72, 230),
    );
    draw_rectangle_lines(
        panel.x + 7.0,
        panel.y + 7.0,
        panel.w - 14.0,
        panel.h - 14.0,
        1.0,
        color(109, 144, 81, 190),
    );

    ui::draw_centered_text(
        "THE CROWN BREATHES",
        640.0,
        126.0,
        42,
        color(232, 207, 145, 255),
    );
    ui::draw_centered_text(
        "Hatchspire is restored",
        640.0,
        164.0,
        22,
        color(148, 205, 137, 255),
    );
    draw_wrapped_centered(
        "The Verdant Crown parts above the tower. Daylight reaches the old halls, and every egg, companion, and rebuilt hearth below answers with new life.",
        640.0,
        218.0,
        70,
    );

    draw_party(state, 640.0, 355.0);

    let records = state.tower_discoveries.record_count();
    draw_ui_text_ex(
        &format!(
            "Floor {} reached  ·  {} Field Guide records  ·  Day {}",
            state.tower_progress.best_floor, records, state.day
        ),
        390.0,
        468.0,
        TextParams {
            font_size: 18,
            color: color(198, 187, 153, 255),
            ..Default::default()
        },
    );
    ui::draw_centered_text(
        "The story is complete. You can keep raising companions and explore the tower again.",
        640.0,
        515.0,
        18,
        color(166, 180, 166, 255),
    );

    ui::draw_button(continue_rect(), "CONTINUE IN TOWN", true);
    ui::draw_button(title_rect(), "RETURN TO TITLE", true);
}

fn draw_party(state: &GameState, center_x: f32, y: f32) {
    let members = state
        .monster_roster
        .party_slots
        .iter()
        .flatten()
        .filter_map(|id| state.monster_roster.monster(*id))
        .take(3)
        .collect::<Vec<_>>();
    let width = members.len().saturating_sub(1) as f32 * 116.0;
    for (index, monster) in members.iter().enumerate() {
        let x = center_x - width * 0.5 + index as f32 * 116.0;
        draw_circle(x, y, 52.0, color(205, 170, 83, 30));
        assets::draw_monster_sprite(&monster.species_id, x - 45.0, y - 50.0, 90.0);
        let name_width = measure_text(&monster.name, None, 18, 1.0).width;
        draw_ui_text_ex(
            &monster.name,
            x - name_width * 0.5,
            y + 70.0,
            TextParams {
                font_size: 18,
                color: ui::TEXT_BRIGHT,
                ..Default::default()
            },
        );
    }
}

fn draw_wrapped_centered(text: &str, center_x: f32, y: f32, max_chars: usize) {
    let mut lines = vec![String::new()];
    for word in text.split_whitespace() {
        let line = lines.last_mut().expect("one line is always present");
        if !line.is_empty() && line.len() + word.len() + 1 > max_chars {
            lines.push(word.to_owned());
        } else {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
    }
    for (index, line) in lines.iter().enumerate() {
        ui::draw_centered_text(line, center_x, y + index as f32 * 25.0, 19, ui::TEXT);
    }
}

fn continue_rect() -> Rect {
    Rect::new(374.0, 568.0, 250.0, 54.0)
}

fn title_rect() -> Rect {
    Rect::new(656.0, 568.0, 250.0, 54.0)
}

fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}
