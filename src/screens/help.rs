use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

use crate::ui;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HelpAction {
    ExportLocalSummary,
    Back,
}

pub fn handle_input(can_export: bool) -> Option<HelpAction> {
    if is_key_pressed(KeyCode::Escape) || ui::button_clicked(back_rect(), true) {
        Some(HelpAction::Back)
    } else if ui::button_clicked(export_rect(), can_export) {
        Some(HelpAction::ExportLocalSummary)
    } else {
        None
    }
}

pub fn draw(save_location: &str, summary_location: &str, can_export: bool, status_message: &str) {
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

    let panel = Rect::new(72.0, 112.0, 1136.0, 500.0);
    ui::draw_panel(panel);
    draw_label("Build", panel.x + 28.0, panel.y + 42.0);
    draw_value(
        &format!("Hatchspire {}", crate::build_info::BUILD_ID),
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
        "Recovery can restore the previous save while preserving the replaced file, or archive an unreadable save.",
        panel.x + 28.0,
        panel.y + 298.0,
    );
    draw_value(
        "If crash_log.txt exists beside the saved camp, attach it manually; the game never uploads it.",
        panel.x + 28.0,
        panel.y + 328.0,
    );

    draw_label("When reporting a problem", panel.x + 28.0, panel.y + 376.0);
    draw_value(
        "Include the build above, the exact on-screen message, and the saved-camp file when it is safe to share.",
        panel.x + 28.0,
        panel.y + 406.0,
    );

    draw_label("Local tester summary", panel.x + 28.0, panel.y + 450.0);
    let summary_text = if status_message.contains("tester summary") {
        status_message
    } else {
        summary_location
    };
    for (index, line) in wrapped_lines(summary_text, 92).iter().take(2).enumerate() {
        draw_value(line, panel.x + 28.0, panel.y + 478.0 + index as f32 * 22.0);
    }

    ui::draw_title_button(export_rect(), "EXPORT LOCAL SUMMARY", can_export);
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
    Rect::new(660.0, 636.0, 320.0, 48.0)
}

fn export_rect() -> Rect {
    Rect::new(300.0, 636.0, 320.0, 48.0)
}
