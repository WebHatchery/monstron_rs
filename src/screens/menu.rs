use macroquad::prelude::*;

use crate::settings::AppSettings;
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    NewGame,
    LoadGame,
    SaveOptions,
    Settings,
    ExitGame,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingsAction {
    AdjustMaster(i8),
    AdjustMusic(i8),
    AdjustSfx(i8),
    ToggleMute,
    ToggleFullscreen,
    ToggleReducedMotion,
    OpenHelp,
    Back,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NewGameConfirmationAction {
    Cancel,
    StartOver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SaveResetConfirmationAction {
    KeepSave,
    DeleteSave,
}

pub fn new_game_requires_confirmation(has_saved_game: bool, has_active_game: bool) -> bool {
    has_saved_game || has_active_game
}

pub fn handle_input(has_save: bool) -> Option<MenuAction> {
    if is_key_pressed(KeyCode::Enter) {
        return Some(MenuAction::NewGame);
    }

    if has_save && is_key_pressed(KeyCode::L) {
        return Some(MenuAction::LoadGame);
    }
    if is_key_pressed(KeyCode::S) {
        return Some(MenuAction::Settings);
    }

    let new_game = new_game_rect();
    if ui::button_clicked(new_game, true) {
        return Some(MenuAction::NewGame);
    }

    let load_game = load_game_rect();
    if ui::button_clicked(load_game, has_save) {
        return Some(MenuAction::LoadGame);
    }

    let settings = settings_rect();
    if ui::button_clicked(settings, true) {
        return Some(MenuAction::Settings);
    }

    let save_options = save_options_rect();
    if ui::button_clicked(save_options, has_save) {
        return Some(MenuAction::SaveOptions);
    }

    let exit_game = exit_game_rect();
    if ui::button_clicked(exit_game, true) {
        return Some(MenuAction::ExitGame);
    }

    None
}

pub fn handle_settings_input() -> Option<SettingsAction> {
    if is_key_pressed(KeyCode::Escape) {
        return Some(SettingsAction::Back);
    }
    if is_key_pressed(KeyCode::Enter)
        || is_key_pressed(KeyCode::Space)
        || is_key_pressed(KeyCode::F)
        || ui::button_clicked(fullscreen_toggle_rect(), true)
    {
        return Some(SettingsAction::ToggleFullscreen);
    }
    for (action, rect) in volume_buttons() {
        if ui::button_clicked(rect, true) {
            return Some(action);
        }
    }
    if ui::button_clicked(mute_toggle_rect(), true) {
        return Some(SettingsAction::ToggleMute);
    }
    if ui::button_clicked(reduced_motion_toggle_rect(), true) {
        return Some(SettingsAction::ToggleReducedMotion);
    }
    if ui::button_clicked(settings_back_rect(), true) {
        return Some(SettingsAction::Back);
    }
    if ui::button_clicked(help_button_rect(), true) {
        return Some(SettingsAction::OpenHelp);
    }

    None
}

pub fn handle_new_game_confirmation_input() -> Option<NewGameConfirmationAction> {
    if is_key_pressed(KeyCode::Escape) || ui::button_clicked(keep_game_rect(), true) {
        return Some(NewGameConfirmationAction::Cancel);
    }
    if ui::button_clicked(start_over_rect(), true) {
        return Some(NewGameConfirmationAction::StartOver);
    }

    None
}

pub fn handle_save_reset_confirmation_input() -> Option<SaveResetConfirmationAction> {
    if is_key_pressed(KeyCode::Escape) || ui::button_clicked(keep_save_rect(), true) {
        return Some(SaveResetConfirmationAction::KeepSave);
    }
    if ui::button_clicked(delete_save_rect(), true) {
        return Some(SaveResetConfirmationAction::DeleteSave);
    }

    None
}

pub fn draw(title_texture: &Texture2D, has_save: bool) {
    draw_title_art(title_texture);
    ui::draw_title_button(new_game_rect(), "New Game", true);
    ui::draw_title_button(load_game_rect(), "Load Game", has_save);
    ui::draw_title_button(save_options_rect(), "Save Options", has_save);
    ui::draw_title_button(settings_rect(), "Settings", true);
    ui::draw_title_button(exit_game_rect(), "Exit Game", true);
    ui::draw_tooltip_target(ui::Tooltip {
        rect: new_game_rect(),
        title: "New Game",
        detail: "Start a fresh camp beside the tower.",
    });
    ui::draw_tooltip_target(ui::Tooltip {
        rect: load_game_rect(),
        title: "Load Game",
        detail: "Restore the last saved camp.",
    });
    ui::draw_tooltip_target(ui::Tooltip {
        rect: settings_rect(),
        title: "Settings",
        detail: "Change display mode or open Help & Support.",
    });
}

pub fn draw_save_reset_confirmation(title_texture: &Texture2D, has_save: bool) {
    draw(title_texture, has_save);
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(3, 7, 10, 205),
    );

    let panel = Rect::new(350.0, 190.0, 580.0, 340.0);
    ui::draw_panel(panel);
    ui::draw_centered_text(
        "Delete the saved camp?",
        ui::VIEW_WIDTH * 0.5,
        panel.y + 66.0,
        36,
        ui::TEXT_BRIGHT,
    );
    ui::draw_centered_text(
        "DELETE SAVE removes the save slot and current session.",
        ui::VIEW_WIDTH * 0.5,
        panel.y + 122.0,
        19,
        ui::WARN,
    );
    ui::draw_centered_text(
        "This cannot be undone. Tap KEEP SAVE to return safely.",
        ui::VIEW_WIDTH * 0.5,
        panel.y + 160.0,
        18,
        ui::TEXT,
    );
    ui::draw_title_button(keep_save_rect(), "KEEP SAVE", true);
    ui::draw_title_button(delete_save_rect(), "DELETE SAVE", true);
}

pub fn draw_settings(settings: &AppSettings, status_message: &str) {
    draw_rectangle(0.0, 0.0, ui::VIEW_WIDTH, ui::VIEW_HEIGHT, ui::BACKGROUND);
    draw_ui_text_ex(
        "Settings",
        72.0,
        96.0,
        TextParams {
            font_size: 44,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_volume_row("Master volume", settings.master_volume, 128.0, 0);
    draw_volume_row("Music volume", settings.music_volume, 198.0, 1);
    draw_volume_row("SFX volume", settings.sfx_volume, 268.0, 2);
    ui::draw_toggle(mute_toggle_rect(), "Mute all audio", settings.muted);
    ui::draw_toggle(fullscreen_toggle_rect(), "Fullscreen", settings.fullscreen);
    ui::draw_toggle(
        reduced_motion_toggle_rect(),
        "Reduced motion",
        settings.reduced_motion,
    );
    ui::draw_title_button(help_button_rect(), "Help & Support", true);
    ui::draw_title_button(settings_back_rect(), "Back", true);
    ui::draw_status(status_message);
}

fn draw_volume_row(label: &str, value: u8, y: f32, row: usize) {
    let rect = Rect::new(300.0, y, 680.0, 56.0);
    ui::draw_panel(rect);
    draw_ui_text_ex(
        label,
        rect.x + 18.0,
        rect.y + 35.0,
        TextParams {
            font_size: 22,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    ui::draw_centered_text(&format!("{value}%"), 750.0, rect.y + 36.0, 22, ui::TEXT);
    ui::draw_button(volume_button_rect(row, false), "−", true);
    ui::draw_button(volume_button_rect(row, true), "+", true);
}

pub fn draw_new_game_confirmation(title_texture: &Texture2D, has_save: bool) {
    draw(title_texture, has_save);
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(3, 7, 10, 205),
    );

    let panel = Rect::new(350.0, 190.0, 580.0, 340.0);
    ui::draw_panel(panel);
    ui::draw_centered_text(
        "Start a new camp?",
        ui::VIEW_WIDTH * 0.5,
        panel.y + 66.0,
        36,
        ui::TEXT_BRIGHT,
    );
    ui::draw_centered_text(
        "You already have progress in this session or in the save slot.",
        ui::VIEW_WIDTH * 0.5,
        panel.y + 116.0,
        20,
        ui::TEXT,
    );
    ui::draw_centered_text(
        "START OVER begins from day 1 and immediately replaces that save slot.",
        ui::VIEW_WIDTH * 0.5,
        panel.y + 150.0,
        18,
        ui::WARN,
    );
    ui::draw_centered_text(
        "Tap KEEP OLD GAME to return without changing anything.",
        ui::VIEW_WIDTH * 0.5,
        panel.y + 184.0,
        18,
        ui::TEXT_DIM,
    );
    ui::draw_title_button(keep_game_rect(), "KEEP OLD GAME", true);
    ui::draw_title_button(start_over_rect(), "START OVER", true);
}

fn draw_title_art(title_texture: &Texture2D) {
    let texture_size = title_texture.size();
    let scale = (ui::VIEW_WIDTH / texture_size.x).max(ui::VIEW_HEIGHT / texture_size.y);
    let width = texture_size.x * scale;
    let height = texture_size.y * scale;
    let x = (ui::VIEW_WIDTH - width) * 0.5;
    let y = (ui::VIEW_HEIGHT - height) * 0.5;

    draw_texture_ex(
        title_texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(width, height)),
            ..Default::default()
        },
    );
}

fn new_game_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH * 0.5 - 120.0, 388.0, 240.0, 44.0)
}

fn load_game_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH * 0.5 - 120.0, 442.0, 240.0, 44.0)
}

fn save_options_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH * 0.5 - 120.0, 496.0, 240.0, 44.0)
}

fn settings_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH * 0.5 - 120.0, 550.0, 240.0, 44.0)
}

fn exit_game_rect() -> Rect {
    Rect::new(ui::VIEW_WIDTH * 0.5 - 120.0, 604.0, 240.0, 44.0)
}

fn fullscreen_toggle_rect() -> Rect {
    Rect::new(300.0, 408.0, 680.0, 56.0)
}

fn settings_back_rect() -> Rect {
    Rect::new(665.0, 620.0, 240.0, 44.0)
}

fn help_button_rect() -> Rect {
    Rect::new(375.0, 620.0, 240.0, 44.0)
}

fn mute_toggle_rect() -> Rect {
    Rect::new(300.0, 338.0, 680.0, 56.0)
}

fn reduced_motion_toggle_rect() -> Rect {
    Rect::new(300.0, 478.0, 680.0, 56.0)
}

fn volume_buttons() -> [(SettingsAction, Rect); 6] {
    [
        (
            SettingsAction::AdjustMaster(-10),
            volume_button_rect(0, false),
        ),
        (
            SettingsAction::AdjustMaster(10),
            volume_button_rect(0, true),
        ),
        (
            SettingsAction::AdjustMusic(-10),
            volume_button_rect(1, false),
        ),
        (SettingsAction::AdjustMusic(10), volume_button_rect(1, true)),
        (SettingsAction::AdjustSfx(-10), volume_button_rect(2, false)),
        (SettingsAction::AdjustSfx(10), volume_button_rect(2, true)),
    ]
}

fn volume_button_rect(row: usize, increase: bool) -> Rect {
    let y = 133.0 + row as f32 * 70.0;
    let x = if increase { 910.0 } else { 840.0 };
    Rect::new(x, y, 52.0, 46.0)
}

fn keep_game_rect() -> Rect {
    Rect::new(390.0, 454.0, 230.0, 48.0)
}

fn start_over_rect() -> Rect {
    Rect::new(660.0, 454.0, 230.0, 48.0)
}

fn keep_save_rect() -> Rect {
    Rect::new(390.0, 454.0, 230.0, 48.0)
}

fn delete_save_rect() -> Rect {
    Rect::new(660.0, 454.0, 230.0, 48.0)
}
