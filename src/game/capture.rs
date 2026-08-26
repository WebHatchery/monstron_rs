//! Deterministic scene fixtures used by the screenshot verification harness.

use super::Game;
use crate::screens::AppScreen;
use crate::state::{TowerRunGoal, TownJobKind};

impl Game {
    /// Seed a specific scene for the screenshot harness. Bypasses normal
    /// facility-unlock gating so a fresh save can still reach these screens.
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.autosave_enabled = false;
        match scene {
            "town" => self.begin_capture_fixture(AppScreen::Town),
            "hatchery" => self.begin_capture_fixture(AppScreen::Hatchery),
            "stable" => self.begin_capture_fixture(AppScreen::Stable),
            "breeding" => self.begin_capture_fixture(AppScreen::Breeding),
            "workshop" => self.begin_capture_fixture(AppScreen::Workshop),
            "shop" => self.begin_capture_fixture(AppScreen::Shop),
            "tower" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.enter_tower(TowerRunGoal::Balanced);
            }
            "combat" => {
                self.begin_capture_fixture(AppScreen::Town);
                if let Some(state) = &mut self.state {
                    let result =
                        crate::engine::combat_engine::start_encounter(state, &self.data, 1, false);
                    self.status_message = result.summary;
                    self.screen = AppScreen::Combat;
                }
            }
            "mainmenu" => {
                self.state = None;
                self.screen = AppScreen::MainMenu;
                self.status_message = "Ready.".to_owned();
            }
            "new_game_warning" => {
                self.begin_capture_fixture(AppScreen::ConfirmNewGame);
                self.status_message =
                    "Choose whether to keep the old game or start over.".to_owned();
            }
            "save_recovery" => {
                self.state = None;
                self.screen = AppScreen::SaveRecovery;
                self.save_recovery_can_preserve = true;
                self.status_message =
                    "Load failed: the save data is incomplete or damaged.".to_owned();
            }
            "autosave_notice" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.status_message = "Built Hatchery at level 1.  [AUTOSAVED]".to_owned();
            }
            "save_migration_notice" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.status_message = "Loaded day 7 and upgraded save version 0 to 1.".to_owned();
            }
            "save_reset_warning" => {
                self.begin_capture_fixture(AppScreen::ConfirmSaveReset);
                self.status_message = "Choose whether to keep or delete the save.".to_owned();
            }
            "help" => {
                self.state = None;
                self.screen = AppScreen::Help;
                self.status_message = "Help verification scene.".to_owned();
            }
            "settings" => {
                self.state = None;
                self.settings = crate::settings::AppSettings {
                    master_volume: 80,
                    music_volume: 60,
                    sfx_volume: 70,
                    muted: false,
                    fullscreen: false,
                    reduced_motion: true,
                    ui_scale_percent: 110,
                };
                self.screen = AppScreen::Settings;
                self.status_message = "Settings saved independently from game progress.".to_owned();
            }
            _ => {
                // Default: boot state is the main menu.
            }
        }
    }

    fn begin_capture_fixture(&mut self, screen: AppScreen) {
        self.start_new_game();
        let Some(state) = &mut self.state else {
            return;
        };
        for building_id in ["hatchery", "stable", "breeding_grove", "workshop", "shop"] {
            state.town.set_building_level(building_id, 1);
        }
        for (resource_id, amount) in [
            ("coins", 80),
            ("wood", 60),
            ("stone", 50),
            ("ore", 8),
            ("herbs", 30),
            ("crystal", 4),
        ] {
            state.resources.add(resource_id, amount);
        }
        if let Some(species) = self.data.species("rillfin") {
            state
                .monster_roster
                .add_monster("Ripple".to_owned(), species, 0xBEE5_7001);
        }
        if let Some(species) = self.data.species("emberkit") {
            state
                .monster_roster
                .add_monster("Ember".to_owned(), species, 0xF17E_2002);
        }
        let _ = state.monster_roster.assign_to_party(2);
        let _ = state.monster_roster.assign_to_party(3);
        if screen == AppScreen::Stable {
            if let Some(monster) = state.monster_roster.monster_mut(2) {
                monster.condition.fatigue = 3;
            }
            if let Some(monster) = state.monster_roster.monster_mut(3) {
                monster.condition.injury_days = 1;
            }
        }
        if screen == AppScreen::Workshop {
            state.town.set_monster_job(1, TownJobKind::Forage);
        }
        state
            .egg_inventory
            .add_egg("mossy_egg".to_owned(), 0, 3, 0xA7C4_0001);
        self.screen = screen;
        self.town_menu_open = false;
        self.status_message =
            "Seeded verification scene. Tap a visible control to continue.".to_owned();
    }
}
