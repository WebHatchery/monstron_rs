use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

use crate::ui;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HelpAction {
    Back,
}

pub fn handle_input() -> Option<HelpAction> {
    if is_key_pressed(KeyCode::Escape) || ui::button_clicked(back_rect(), true) {
        Some(HelpAction::Back)
    } else {
        None
    }
}

pub fn draw(save_location: &str) {
    draw_rectangle(0.0, 0.0, ui::VIEW_WIDTH, ui::VIEW_HEIGHT, ui::BACKGROUND);
    draw_ui_text_ex(
        "Help & Support",
        72.0,
        82.0,
        TextParams {
            font_size: 42,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );

    let panel = Rect::new(72.0, 112.0, 1136.0, 490.0);
    ui::draw_panel(panel);
    draw_label("Build", panel.x + 28.0, panel.y + 42.0);
    draw_value(
        &format!("Hatchspire {}", env!("CARGO_PKG_VERSION")),
        panel.x + 28.0,
        panel.y + 72.0,
    );

    draw_label("Saved camp location", panel.x + 28.0, panel.y + 124.0);
    for (index, line) in wrapped_lines(save_location, 92).iter().enumerate() {
        draw_value(line, panel.x + 28.0, panel.y + 154.0 + index as f32 * 25.0);
    }

    draw_label("Save safety", panel.x + 28.0, panel.y + 238.0);
    draw_value(
        "Progress autosaves after successful actions. [AUTOSAVE FAILED] means the latest change may not be on disk.",
        panel.x + 28.0,
        panel.y + 268.0,
    );
    draw_value(
        "An unreadable save can be preserved from the recovery screen before starting a new camp.",
        panel.x + 28.0,
        panel.y + 298.0,
    );

    draw_label("When reporting a problem", panel.x + 28.0, panel.y + 354.0);
    draw_value(
        "Include the build above, the exact on-screen message, and the saved-camp file when it is safe to share.",
        panel.x + 28.0,
        panel.y + 384.0,
    );
    ui::draw_title_button(back_rect(), "BACK TO SETTINGS", true);
}

fn wrapped_lines(text: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(1);
    let characters = text.chars().collect::<Vec<_>>();
    if characters.is_empty() {
        return vec![String::new()];
    }
    characters
        .chunks(max_chars)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn draw_label(text: &str, x: f32, y: f32) {
    draw_ui_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size: 21,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
}

fn draw_value(text: &str, x: f32, y: f32) {
    draw_ui_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size: 18,
            color: ui::TEXT,
            ..Default::default()
        },
    );
}

fn back_rect() -> Rect {
    Rect::new(500.0, 632.0, 280.0, 48.0)
}
