use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

use crate::ui;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SaveRecoveryAction {
    Retry,
    RestoreBackup,
    PreserveAndStartNew,
    BackToTitle,
}

pub fn handle_input(can_preserve: bool, has_backup: bool) -> Option<SaveRecoveryAction> {
    if ui::button_clicked(retry_rect(), true) {
        return Some(SaveRecoveryAction::Retry);
    }
    if can_preserve && ui::button_clicked(preserve_rect(), true) {
        return Some(SaveRecoveryAction::PreserveAndStartNew);
    }
    if has_backup && ui::button_clicked(restore_rect(), true) {
        return Some(SaveRecoveryAction::RestoreBackup);
    }
    if is_key_pressed(KeyCode::Escape) || ui::button_clicked(back_rect(), true) {
        return Some(SaveRecoveryAction::BackToTitle);
    }

    None
}

pub fn draw(problem: &str, can_preserve: bool, has_backup: bool) {
    draw_rectangle(0.0, 0.0, ui::VIEW_WIDTH, ui::VIEW_HEIGHT, ui::BACKGROUND);
    let panel = Rect::new(250.0, 110.0, 780.0, 500.0);
    ui::draw_panel(panel);

    ui::draw_centered_text(
        if can_preserve {
            "This save could not be read"
        } else {
            "This save comes from a newer version"
        },
        ui::VIEW_WIDTH * 0.5,
        panel.y + 68.0,
        34,
        ui::TEXT_BRIGHT,
    );
    draw_ui_text_ex(
        problem,
        panel.x + 48.0,
        panel.y + 126.0,
        TextParams {
            font_size: 18,
            color: ui::WARN,
            ..Default::default()
        },
    );

    let explanation = if has_backup {
        "Tap RESTORE BACKUP to preserve this file and reopen the previous known-good save."
    } else if can_preserve {
        "Tap RETRY LOAD after repairing the file, or preserve its exact contents before starting a new camp."
    } else {
        "Install a newer Hatchspire build to load it. This version will not alter the save."
    };
    ui::draw_centered_text(
        explanation,
        ui::VIEW_WIDTH * 0.5,
        panel.y + 190.0,
        18,
        ui::TEXT,
    );

    ui::draw_title_button(retry_rect(), "RETRY LOAD", true);
    if can_preserve {
        ui::draw_title_button(preserve_rect(), "PRESERVE & START NEW", true);
    }
    if has_backup {
        ui::draw_title_button(restore_rect(), "RESTORE BACKUP", true);
    }
    ui::draw_title_button(back_rect(), "BACK TO TITLE", true);
}

fn retry_rect() -> Rect {
    Rect::new(320.0, 388.0, 280.0, 50.0)
}

fn preserve_rect() -> Rect {
    Rect::new(680.0, 388.0, 280.0, 50.0)
}

fn restore_rect() -> Rect {
    Rect::new(500.0, 458.0, 280.0, 50.0)
}

fn back_rect() -> Rect {
    Rect::new(500.0, 530.0, 280.0, 50.0)
}
