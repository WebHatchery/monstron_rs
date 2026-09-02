//! Top-level application coordinator: screen input, drawing, and shared session state.

mod actions;
mod capture;
mod save_flow;

use macroquad::prelude::*;

use crate::data::{GameData, GameDataLoader};
use crate::playtest_report;
use crate::save::SaveRepository;
use crate::screens::placeholder::PlaceholderKind;
use crate::screens::{
    breeding, combat, finale, hatchery, help, menu, placeholder, save_recovery, shop, stable,
    tower, town, workshop, AppScreen,
};
use crate::settings::AppSettings;
use crate::state::GameState;
use crate::ui;

pub struct Game {
    pub(crate) data: GameData,
    pub(crate) state: Option<GameState>,
    pub(crate) screen: AppScreen,
    pub(crate) status_message: String,
    pub(crate) town_menu_open: bool,
    tower_guide_open: bool,
    tower_guide_page: usize,
    pub(crate) tower_route_step_ready_at: f64,
    title_texture: Texture2D,
    settings: AppSettings,
    save_recovery_can_preserve: bool,
    save_recovery_has_backup: bool,
    autosave_enabled: bool,
}

impl Game {
    pub async fn new() -> Self {
        let (data, status_message) = match GameDataLoader::load_embedded() {
            Ok(data) => (data, "Ready.".to_owned()),
            Err(error) => {
                eprintln!("Failed to load embedded data: {error}");
                (
                    GameData::fallback(),
                    format!("Loaded fallback data after content error: {error}"),
                )
            }
        };

        let (settings, settings_error) = match AppSettings::load() {
            Ok(settings) => (settings, None),
            Err(error) => (AppSettings::default(), Some(error)),
        };
        let status_message = match settings_error {
            Some(error) => format!("{status_message} Settings reset to defaults: {error}"),
            None => status_message,
        };

        let title_texture =
            Texture2D::from_file_with_format(include_bytes!("../hatchspire_title.png"), None);
        title_texture.set_filter(FilterMode::Linear);

        Self {
            data,
            state: None,
            screen: AppScreen::MainMenu,
            status_message,
            town_menu_open: false,
            tower_guide_open: false,
            tower_guide_page: 0,
            tower_route_step_ready_at: 0.0,
            title_texture,
            settings,
            save_recovery_can_preserve: false,
            save_recovery_has_backup: false,
            autosave_enabled: true,
        }
    }

    pub fn update(&mut self) {
        match self.screen {
            AppScreen::MainMenu => {
                let has_save = SaveRepository::exists();
                if let Some(action) = menu::handle_input(has_save) {
                    self.apply_menu_action(action);
                }
            }
            AppScreen::ConfirmNewGame => {
                if let Some(action) = menu::handle_new_game_confirmation_input() {
                    self.apply_new_game_confirmation_action(action);
                }
            }
            AppScreen::ConfirmSaveReset => {
                if let Some(action) = menu::handle_save_reset_confirmation_input() {
                    self.apply_save_reset_confirmation_action(action);
                }
            }
            AppScreen::SaveRecovery => {
                if let Some(action) = save_recovery::handle_input(
                    self.save_recovery_can_preserve,
                    self.save_recovery_has_backup,
                ) {
                    self.apply_save_recovery_action(action);
                }
            }
            AppScreen::Settings => {
                if let Some(action) = menu::handle_settings_input() {
                    self.apply_settings_action(action);
                }
            }
            AppScreen::Help => {
                let can_export = playtest_report::is_supported()
                    && (self.state.is_some() || SaveRepository::exists());
                if let Some(action) = help::handle_input(can_export) {
                    self.apply_help_action(action);
                }
            }
            AppScreen::Town => {
                if let Some(state) = &self.state {
                    if let Some(action) = town::handle_input(state, &self.data, self.town_menu_open)
                    {
                        if matches!(&action, town::TownAction::Save | town::TownAction::Load) {
                            self.apply_town_action(action);
                        } else {
                            self.apply_progression(|game| game.apply_town_action(action));
                        }
                    }
                } else {
                    self.screen = AppScreen::MainMenu;
                    self.status_message = "No active save. Start a new game.".to_owned();
                }
            }
            AppScreen::Hatchery => {
                if let Some(state) = &self.state {
                    if let Some(action) = hatchery::handle_input(state) {
                        self.apply_progression(|game| game.apply_hatchery_action(action));
                    }
                } else {
                    self.screen = AppScreen::MainMenu;
                    self.status_message = "No active save. Start a new game.".to_owned();
                }
            }
            AppScreen::Stable => {
                if let Some(state) = &self.state {
                    if let Some(action) = stable::handle_input(state) {
                        self.apply_progression(|game| game.apply_stable_action(action));
                    }
                } else {
                    self.screen = AppScreen::MainMenu;
                    self.status_message = "No active save. Start a new game.".to_owned();
                }
            }
            AppScreen::Breeding => {
                if let Some(state) = &self.state {
                    if let Some(action) = breeding::handle_input(state) {
                        self.apply_progression(|game| game.apply_breeding_action(action));
                    }
                } else {
                    self.screen = AppScreen::MainMenu;
                    self.status_message = "No active save. Start a new game.".to_owned();
                }
            }
            AppScreen::Workshop => {
                if let Some(state) = &self.state {
                    if let Some(action) = workshop::handle_input(state) {
                        self.apply_progression(|game| game.apply_workshop_action(action));
                    }
                } else {
                    self.screen = AppScreen::MainMenu;
                    self.status_message = "No active save. Start a new game.".to_owned();
                }
            }
            AppScreen::Shop => {
                if let Some(action) = shop::handle_input() {
                    self.apply_progression(|game| game.apply_shop_action(action));
                }
            }
            AppScreen::DungeonPrep => {
                if let Some(action) = placeholder::handle_input(PlaceholderKind::DungeonPrep) {
                    self.apply_progression(|game| game.apply_placeholder_action(action));
                }
            }
            AppScreen::Tower => {
                if let Some(state) = &self.state {
                    let action = tower::handle_input(
                        state,
                        &self.data,
                        self.tower_guide_open,
                        self.tower_guide_page,
                    );
                    if let Some(action) = action {
                        self.apply_progression(|game| game.apply_tower_action(action));
                    } else {
                        self.advance_tower_route();
                    }
                } else {
                    self.screen = AppScreen::MainMenu;
                    self.status_message = "No active save. Start a new game.".to_owned();
                }
            }
            AppScreen::Combat => {
                if let Some(state) = &self.state {
                    if let Some(action) = combat::handle_input(state) {
                        self.apply_progression(|game| game.apply_combat_action(action));
                    }
                } else {
                    self.screen = AppScreen::MainMenu;
                    self.status_message = "No active save. Start a new game.".to_owned();
                }
            }
            AppScreen::Finale => {
                if let Some(action) = finale::handle_input() {
                    self.apply_finale_action(action);
                }
            }
            AppScreen::EndOfDay => {
                if let Some(action) = placeholder::handle_input(PlaceholderKind::EndOfDay) {
                    self.apply_progression(|game| game.apply_placeholder_action(action));
                }
            }
        }
    }

    pub fn draw(&self) {
        clear_background(ui::BACKGROUND);
        set_camera(&ui::virtual_camera());

        match self.screen {
            AppScreen::MainMenu => {
                menu::draw(&self.title_texture, SaveRepository::exists());
            }
            AppScreen::ConfirmNewGame => {
                menu::draw_new_game_confirmation(&self.title_texture, SaveRepository::exists());
            }
            AppScreen::ConfirmSaveReset => {
                menu::draw_save_reset_confirmation(&self.title_texture, true);
            }
            AppScreen::SaveRecovery => {
                save_recovery::draw(
                    &self.status_message,
                    self.save_recovery_can_preserve,
                    self.save_recovery_has_backup,
                );
            }
            AppScreen::Settings => {
                menu::draw_settings(&self.settings, &self.status_message);
            }
            AppScreen::Help => {
                help::draw(
                    &SaveRepository::location_description(),
                    &playtest_report::location_description(),
                    playtest_report::is_supported()
                        && (self.state.is_some() || SaveRepository::exists()),
                    &self.status_message,
                );
            }
            AppScreen::Town => {
                if let Some(state) = &self.state {
                    town::draw(state, &self.data, &self.status_message, self.town_menu_open);
                }
            }
            AppScreen::Hatchery => {
                if let Some(state) = &self.state {
                    hatchery::draw(state, &self.data, &self.status_message);
                }
            }
            AppScreen::Stable => {
                if let Some(state) = &self.state {
                    stable::draw(state, &self.data, &self.status_message);
                }
            }
            AppScreen::Breeding => {
                if let Some(state) = &self.state {
                    breeding::draw(state, &self.data, &self.status_message);
                }
            }
            AppScreen::Workshop => {
                if let Some(state) = &self.state {
                    workshop::draw(state, &self.data, &self.status_message);
                }
            }
            AppScreen::Shop => {
                if let Some(state) = &self.state {
                    shop::draw(state, &self.data, &self.status_message);
                }
            }
            AppScreen::DungeonPrep => {
                placeholder::draw(PlaceholderKind::DungeonPrep, &self.status_message);
            }
            AppScreen::Tower => {
                if let Some(state) = &self.state {
                    tower::draw(
                        state,
                        &self.data,
                        &self.status_message,
                        self.tower_guide_open,
                        self.tower_guide_page,
                    );
                }
            }
            AppScreen::Combat => {
                if let Some(state) = &self.state {
                    combat::draw(state, &self.data, &self.status_message);
                }
            }
            AppScreen::Finale => {
                if let Some(state) = &self.state {
                    finale::draw(state);
                }
            }
            AppScreen::EndOfDay => {
                placeholder::draw(PlaceholderKind::EndOfDay, &self.status_message);
            }
        }

        set_default_camera();
    }
}
