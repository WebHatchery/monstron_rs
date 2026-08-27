//! Application-side action reducers and screen transitions.

use macroquad::prelude::*;

use super::Game;
use crate::engine::{
    combat_engine::{self, CombatDestination},
    tower_engine, town_engine,
};
use crate::playtest_report;
use crate::save::{compatibility, SaveCompatibility, SaveData, SaveRepository};
use crate::screens::{
    combat::CombatAction,
    help::HelpAction,
    menu::{MenuAction, NewGameConfirmationAction, SaveResetConfirmationAction, SettingsAction},
    placeholder::PlaceholderAction,
    save_recovery::SaveRecoveryAction,
    tower::TowerAction,
    town::TownAction,
    AppScreen,
};
use crate::state::{GameState, TowerRunGoal};

impl Game {
    pub(crate) fn apply_menu_action(&mut self, action: MenuAction) {
        match action {
            MenuAction::NewGame => {
                if crate::screens::menu::new_game_requires_confirmation(
                    SaveRepository::exists(),
                    self.state.is_some(),
                ) {
                    self.screen = AppScreen::ConfirmNewGame;
                    self.status_message =
                        "Choose whether to keep the old game or start over.".to_owned();
                } else {
                    self.start_new_game();
                }
            }
            MenuAction::LoadGame => self.load_game(),
            MenuAction::SaveOptions => {
                self.screen = AppScreen::ConfirmSaveReset;
                self.status_message = "Choose whether to keep or delete the save.".to_owned();
            }
            MenuAction::Settings => {
                self.screen = AppScreen::Settings;
            }
            MenuAction::ExitGame => {
                macroquad::miniquad::window::quit();
            }
        }
    }

    pub(crate) fn apply_save_reset_confirmation_action(
        &mut self,
        action: SaveResetConfirmationAction,
    ) {
        match action {
            SaveResetConfirmationAction::KeepSave => {
                self.screen = AppScreen::MainMenu;
                self.status_message = "Kept the existing save.".to_owned();
            }
            SaveResetConfirmationAction::DeleteSave => match SaveRepository::delete() {
                Ok(()) => {
                    self.state = None;
                    self.screen = AppScreen::MainMenu;
                    self.status_message =
                        "Saved camp deleted. Start New Game when ready.".to_owned();
                }
                Err(error) => {
                    self.screen = AppScreen::ConfirmSaveReset;
                    self.status_message = format!("Could not delete the save: {error}");
                }
            },
        }
    }

    pub(crate) fn apply_new_game_confirmation_action(&mut self, action: NewGameConfirmationAction) {
        match action {
            NewGameConfirmationAction::Cancel => {
                self.screen = AppScreen::MainMenu;
                self.status_message = "Kept the existing game.".to_owned();
            }
            NewGameConfirmationAction::StartOver => self.start_new_game(),
        }
    }

    pub(crate) fn apply_save_recovery_action(&mut self, action: SaveRecoveryAction) {
        match action {
            SaveRecoveryAction::Retry => self.load_game(),
            SaveRecoveryAction::RestoreBackup => match SaveRepository::restore_backup() {
                Ok(displaced_slot) => {
                    self.load_game();
                    let load_status = self.status_message.clone();
                    self.status_message = match displaced_slot {
                        Some(name) => format!(
                            "Backup restored; the replaced file is preserved as {name}. {load_status}"
                        ),
                        None => format!("Backup restored. {load_status}"),
                    };
                }
                Err(error) => {
                    self.screen = AppScreen::SaveRecovery;
                    self.status_message = format!("Could not restore the backup: {error}");
                    self.save_recovery_has_backup = SaveRepository::backup_exists();
                }
            },
            SaveRecoveryAction::PreserveAndStartNew => match SaveRepository::quarantine() {
                Ok(quarantine_name) => {
                    self.start_new_game();
                    self.status_message = format!(
                        "Unreadable save preserved as {quarantine_name}. New camp started safely."
                    );
                }
                Err(error) => {
                    self.screen = AppScreen::SaveRecovery;
                    self.status_message = format!("Could not preserve the save: {error}");
                }
            },
            SaveRecoveryAction::BackToTitle => {
                self.screen = AppScreen::MainMenu;
                self.status_message = "The existing save was left unchanged.".to_owned();
            }
        }
    }

    pub(crate) fn apply_settings_action(&mut self, action: SettingsAction) {
        match action {
            SettingsAction::AdjustMaster(delta) => self.settings.adjust_master(delta),
            SettingsAction::AdjustMusic(delta) => self.settings.adjust_music(delta),
            SettingsAction::AdjustSfx(delta) => self.settings.adjust_sfx(delta),
            SettingsAction::ToggleMute => self.settings.muted = !self.settings.muted,
            SettingsAction::ToggleFullscreen => {
                self.settings.fullscreen = !self.settings.fullscreen;
                set_fullscreen(self.settings.fullscreen);
                if !self.settings.fullscreen {
                    let (width, height) = self.settings.window_dimensions();
                    request_new_screen_size(width as f32, height as f32);
                }
            }
            SettingsAction::ToggleReducedMotion => {
                self.settings.reduced_motion = !self.settings.reduced_motion;
            }
            SettingsAction::CycleUiScale => {
                self.settings.cycle_ui_scale();
                if !self.settings.fullscreen {
                    let (width, height) = self.settings.window_dimensions();
                    request_new_screen_size(width as f32, height as f32);
                }
            }
            SettingsAction::OpenHelp => {
                self.screen = AppScreen::Help;
                self.status_message =
                    "Tester summaries are created locally only when you tap EXPORT LOCAL SUMMARY."
                        .to_owned();
                return;
            }
            SettingsAction::Back => {
                self.screen = AppScreen::MainMenu;
                return;
            }
        }
        self.status_message = match self.settings.save() {
            Ok(()) => "Settings saved independently from game progress.".to_owned(),
            Err(error) => format!("Settings could not be saved: {error}"),
        };
    }

    pub(crate) fn apply_help_action(&mut self, action: HelpAction) {
        match action {
            HelpAction::ExportLocalSummary => self.export_tester_summary(),
            HelpAction::Back => self.screen = AppScreen::Settings,
        }
    }

    fn export_tester_summary(&mut self) {
        let saved_state;
        let state = if let Some(state) = self.state.as_ref() {
            state
        } else {
            saved_state = match SaveRepository::load() {
                Ok(save)
                    if compatibility(save.version, self.data.config.save_version)
                        != SaveCompatibility::Newer =>
                {
                    save.state
                }
                Ok(save) => {
                    self.status_message = format!(
                        "Could not export tester summary: save version {} is newer than supported version {}.",
                        save.version, self.data.config.save_version
                    );
                    return;
                }
                Err(error) => {
                    self.status_message = format!("Could not export tester summary: {error}");
                    return;
                }
            };
            &saved_state
        };

        self.status_message = match playtest_report::export(state, &self.data) {
            Ok(path) => format!("Local tester summary saved: {}", path.display()),
            Err(error) => format!("Could not export tester summary: {error}"),
        };
    }

    pub(crate) fn apply_town_action(&mut self, action: TownAction) {
        match action {
            TownAction::Sleep => {
                self.town_menu_open = false;
                if let Some(state) = &mut self.state {
                    let result = town_engine::reduce(state, &self.data, &TownAction::Sleep);
                    self.status_message = result.summary;
                    self.screen = AppScreen::EndOfDay;
                }
            }
            TownAction::DungeonPrep => {
                self.town_menu_open = false;
                self.screen = AppScreen::DungeonPrep;
                self.status_message = "Choose a party before entering the tower.".to_owned();
            }
            TownAction::OpenMenu => {
                self.town_menu_open = true;
            }
            TownAction::CloseMenu => {
                self.town_menu_open = false;
            }
            TownAction::OpenHatchery => self.open_facility("hatchery", AppScreen::Hatchery),
            TownAction::OpenStable => self.open_facility("stable", AppScreen::Stable),
            TownAction::OpenBreeding => {
                self.open_facility("breeding_grove", AppScreen::Breeding);
            }
            TownAction::OpenWorkshop => self.open_facility("workshop", AppScreen::Workshop),
            TownAction::OpenShop => self.open_facility("shop", AppScreen::Shop),
            TownAction::Scavenge => {
                self.town_menu_open = false;
                if let Some(state) = &mut self.state {
                    self.status_message =
                        town_engine::reduce(state, &self.data, &TownAction::Scavenge).summary;
                }
            }
            TownAction::AdvanceBuilding(building_id) => {
                self.town_menu_open = false;
                if let Some(state) = &mut self.state {
                    self.status_message = town_engine::reduce(
                        state,
                        &self.data,
                        &TownAction::AdvanceBuilding(building_id),
                    )
                    .summary;
                }
            }
            TownAction::Trade(trade) => {
                self.town_menu_open = false;
                if let Some(state) = &mut self.state {
                    self.status_message =
                        town_engine::reduce(state, &self.data, &TownAction::Trade(trade)).summary;
                }
            }
            TownAction::GreetNpc(npc_id) => {
                self.town_menu_open = false;
                if let Some(state) = &mut self.state {
                    self.status_message =
                        town_engine::reduce(state, &self.data, &TownAction::GreetNpc(npc_id))
                            .summary;
                }
            }
            TownAction::Save => self.save_game(),
            TownAction::Load => self.load_game(),
            TownAction::BackToMenu => {
                self.town_menu_open = false;
                self.screen = AppScreen::MainMenu;
                self.status_message = "Returned to title.".to_owned();
            }
        }
    }

    pub(crate) fn apply_placeholder_action(&mut self, action: PlaceholderAction) {
        match action {
            PlaceholderAction::ToTown => {
                self.screen = AppScreen::Town;
                self.status_message = "Returned to tower camp.".to_owned();
            }
            PlaceholderAction::ToTower(goal) => {
                self.enter_tower(goal);
            }
        }
    }

    pub(crate) fn apply_tower_action(&mut self, action: TowerAction) {
        match action {
            TowerAction::Move(dx, dy) => {
                let result = self
                    .state
                    .as_mut()
                    .map(|state| tower_engine::move_party(state, &self.data, dx, dy));
                if let Some(result) = result {
                    self.apply_tower_result(result);
                }
            }
            TowerAction::RouteTo(x, y) => {
                let result = self
                    .state
                    .as_mut()
                    .map(|state| tower_engine::route_party_to(state, &self.data, (x, y)));
                if let Some(result) = result {
                    self.apply_tower_result(result);
                }
            }
            TowerAction::Explore => {
                let result = self
                    .state
                    .as_mut()
                    .map(|state| tower_engine::explore_party(state, &self.data));
                if let Some(result) = result {
                    self.apply_tower_result(result);
                }
            }
            TowerAction::Survey => {
                if let Some(state) = &mut self.state {
                    self.status_message = tower_engine::survey_floor(state, &self.data).summary;
                }
            }
            TowerAction::Camp => {
                if let Some(state) = &mut self.state {
                    self.status_message = tower_engine::camp_party(state, &self.data).summary;
                }
            }
            TowerAction::ChooseEvent(event_id) => {
                let result = self
                    .state
                    .as_mut()
                    .map(|state| tower_engine::choose_special_event(state, &self.data, &event_id));
                if let Some(result) = result {
                    self.apply_tower_result(result);
                }
            }
            TowerAction::LeaveEvent => {
                if let Some(state) = &mut self.state {
                    self.status_message =
                        tower_engine::leave_special_event(state, &self.data).summary;
                }
            }
            TowerAction::ReturnToTown => {
                if let Some(state) = &mut self.state {
                    self.status_message = tower_engine::return_to_town(state, &self.data).summary;
                }
                self.screen = AppScreen::Town;
                self.tower_guide_open = false;
            }
            TowerAction::ToTown => {
                self.screen = AppScreen::Town;
                self.tower_guide_open = false;
                self.status_message = "Returned to tower camp.".to_owned();
            }
            TowerAction::OpenGuide => {
                self.tower_guide_open = true;
                self.tower_guide_page = 0;
            }
            TowerAction::CloseGuide => self.tower_guide_open = false,
            TowerAction::GuidePage(delta) => {
                self.tower_guide_page = if delta < 0 {
                    self.tower_guide_page
                        .saturating_sub(delta.unsigned_abs() as usize)
                } else {
                    self.tower_guide_page.saturating_add(delta as usize)
                };
            }
        }
    }

    fn apply_tower_result(&mut self, result: tower_engine::TowerResult) {
        self.status_message = result.summary;
        if result.returned_to_town {
            self.screen = AppScreen::Town;
        }

        let Some(encounter) = result.encounter else {
            return;
        };
        let Some(state) = &mut self.state else {
            return;
        };
        let combat_result = combat_engine::start_named_encounter(
            state,
            &self.data,
            encounter.floor,
            encounter.is_boss,
            encounter.enemy_id.as_deref(),
        );
        self.status_message = combat_result.summary;
        if state.combat.is_some() {
            self.screen = AppScreen::Combat;
        }
    }

    pub(crate) fn apply_combat_action(&mut self, action: CombatAction) {
        match action {
            CombatAction::Command(command) => {
                if let Some(state) = &mut self.state {
                    self.status_message =
                        combat_engine::reduce_command(state, &self.data, command).summary;
                }
            }
            CombatAction::Continue => {
                if let Some(state) = &mut self.state {
                    let finish = combat_engine::finish_combat(state, &self.data);
                    self.status_message = finish.summary;
                    self.screen = match finish.destination {
                        CombatDestination::Combat => AppScreen::Combat,
                        CombatDestination::Tower => AppScreen::Tower,
                        CombatDestination::Town => AppScreen::Town,
                    };
                }
            }
        }
    }

    pub(crate) fn open_facility(&mut self, building_id: &str, screen: AppScreen) {
        let Some(state) = &self.state else {
            self.status_message = "No active save. Start a new game.".to_owned();
            return;
        };

        if state.town.building_level(building_id) == 0 {
            let building_name = self
                .data
                .building(building_id)
                .map(|building| building.name.as_str())
                .unwrap_or(building_id);
            self.status_message =
                format!("Build the {building_name} first. Tap its Upgrade button on the town map.");
            return;
        }

        self.screen = screen;
        self.town_menu_open = false;
        self.status_message = "Facility opened.".to_owned();
    }

    pub(crate) fn enter_tower(&mut self, goal: TowerRunGoal) {
        let Some(state) = &mut self.state else {
            self.screen = AppScreen::MainMenu;
            self.status_message = "No active save. Start a new game.".to_owned();
            return;
        };

        let result = tower_engine::start_run(state, &self.data, goal);
        let run_started = state.tower_run.is_some();
        self.status_message = result.summary;
        if run_started {
            self.screen = AppScreen::Tower;
        }
    }

    pub(crate) fn start_new_game(&mut self) {
        let state = GameState::new(&self.data);
        self.state = Some(state);
        self.screen = AppScreen::Town;
        self.town_menu_open = false;
        self.status_message = "New save started beside the ruined tower.".to_owned();
        self.autosave_new_game();
    }

    pub(crate) fn save_game(&mut self) {
        let Some(state) = &self.state else {
            self.status_message = "Nothing to save yet.".to_owned();
            return;
        };

        let save_data = SaveData {
            version: self.data.config.save_version,
            state: state.clone(),
        };

        match SaveRepository::save(&save_data) {
            Ok(()) => {
                self.status_message = format!("Saved day {}.", state.day);
            }
            Err(error) => {
                self.status_message = format!("Save failed: {error}");
            }
        }
    }

    pub(crate) fn load_game(&mut self) {
        match SaveRepository::load() {
            Ok(save_data) => {
                let save_compatibility =
                    compatibility(save_data.version, self.data.config.save_version);
                if save_compatibility == SaveCompatibility::Newer {
                    self.status_message = format!(
                        "Save version {} is newer than supported version {}.",
                        save_data.version, self.data.config.save_version
                    );
                    self.save_recovery_can_preserve = false;
                    self.save_recovery_has_backup = SaveRepository::backup_exists();
                    self.screen = AppScreen::SaveRecovery;
                    return;
                }

                let loaded_version = save_data.version;
                let loaded_day = save_data.state.day;
                let mut loaded_state = save_data.state;
                let before_repairs = serde_json::to_vec(&loaded_state).ok();
                loaded_state.monster_roster.ensure_art_profiles(&self.data);
                tower_engine::ensure_map(&mut loaded_state, &self.data);
                let after_repairs = serde_json::to_vec(&loaded_state).ok();
                let repairs_applied = before_repairs != after_repairs;
                self.state = Some(loaded_state);
                self.screen = AppScreen::Town;
                self.town_menu_open = false;

                let migrated = SaveData {
                    version: self.data.config.save_version,
                    state: self.state.clone().expect("loaded state was just installed"),
                };
                self.status_message = match SaveRepository::rewrite_loaded(&migrated) {
                    Ok(()) if save_compatibility == SaveCompatibility::Older => format!(
                        "Loaded day {loaded_day} and upgraded save version {loaded_version} to {}.",
                        self.data.config.save_version
                    ),
                    Ok(()) if repairs_applied => {
                        format!("Loaded day {loaded_day} and repaired missing save fields.")
                    }
                    Ok(()) => format!("Loaded save on day {loaded_day}."),
                    Err(error) => format!(
                        "Loaded day {loaded_day}, but the normalized save could not be written: {error}"
                    ),
                };
            }
            Err(error) => {
                self.status_message = format!("Load failed: {error}");
                self.save_recovery_can_preserve = true;
                self.save_recovery_has_backup = SaveRepository::backup_exists();
                self.screen = AppScreen::SaveRecovery;
            }
        }
    }
}
