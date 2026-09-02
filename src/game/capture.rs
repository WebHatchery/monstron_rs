//! Deterministic scene fixtures used by the screenshot verification harness.

use super::Game;
use crate::screens::AppScreen;
use crate::state::{CombatOutcome, TowerRunGoal, TownJobKind};

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
            "combat" => self.begin_capture_combat(None),
            "combat_status" => self.begin_capture_combat_status(),
            "combat_victory" => self.begin_capture_combat(Some(CombatOutcome::Victory)),
            "combat_defeat" => self.begin_capture_combat(Some(CombatOutcome::Defeat)),
            "finale" => {
                self.begin_capture_fixture(AppScreen::Finale);
                if let Some(state) = &mut self.state {
                    state.tower_progress.best_floor = 10;
                    state.tower_progress.unlocked_floor = 10;
                    state.story_flags.add("verdant_crown_restored");
                    for enemy in &self.data.enemies {
                        state.tower_discoveries.discover_enemy(&enemy.id);
                    }
                }
                self.status_message = "The Verdant Crown opens to daylight.".to_owned();
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
                self.save_recovery_has_backup = true;
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
                self.begin_capture_fixture(AppScreen::Help);
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

    fn begin_capture_combat(&mut self, outcome: Option<CombatOutcome>) {
        self.begin_capture_fixture(AppScreen::Town);
        let Some(state) = &mut self.state else {
            return;
        };
        if let Some(species) = self.data.species("rootling") {
            let tank_id =
                state
                    .monster_roster
                    .add_monster("Bramble".to_owned(), species, 0xB4A6_1E04);
            let _ = state.monster_roster.assign_to_party(tank_id);
            state.monster_roster.party_slots.swap(0, 3);
        }
        let result = crate::engine::combat_engine::start_encounter(state, &self.data, 1, false);
        self.status_message = result.summary;
        self.screen = AppScreen::Combat;

        let Some(outcome) = outcome else {
            return;
        };
        let Some(combat) = &mut state.combat else {
            return;
        };
        combat.outcome = Some(outcome);
        match outcome {
            CombatOutcome::Victory => {
                for enemy in &mut combat.enemies {
                    enemy.hp = 0;
                }
                combat.add_log("The Moss Mite falls. The route is clear.".to_owned());
                self.status_message = "Victory. Tap CONTINUE to return to the tower.".to_owned();
            }
            CombatOutcome::Defeat => {
                for ally in &mut combat.allies {
                    ally.hp = 0;
                }
                combat.add_log("The party can no longer fight.".to_owned());
                self.status_message = "Defeat. Tap CONTINUE to recover in town.".to_owned();
            }
            CombatOutcome::Fled => unreachable!("the capture registry has no fled outcome scene"),
        }
    }

    fn begin_capture_combat_status(&mut self) {
        self.begin_capture_combat(None);
        let Some(combat) = self.state.as_mut().and_then(|state| state.combat.as_mut()) else {
            return;
        };
        if let Some(tank) = combat
            .allies
            .iter_mut()
            .find(|ally| ally.role == Some(crate::data::MonsterRole::Tank))
        {
            tank.hp = (tank.max_hp - 7).max(1);
            tank.is_defending = true;
            tank.is_guarding = true;
        }
        if let Some(support) = combat
            .allies
            .iter_mut()
            .find(|ally| ally.role == Some(crate::data::MonsterRole::Support))
        {
            support.hp = (support.max_hp - 5).max(1);
            support.is_defending = true;
        }
        if let Some(enemy) = combat.enemies.first_mut() {
            enemy.hp = (enemy.max_hp - 8).max(1);
            enemy.is_marked = true;
        }
        combat.add_log("Ember bursts into Moss Mite for 10 damage.".to_owned());
        combat.add_log("Bramble: GUARDING redirects back-row hits.".to_owned());
        combat.add_log("Ripple: DEFENDING halves incoming damage.".to_owned());
        combat.add_log("Moss Mite: MARKED takes +2 damage.".to_owned());
        self.status_message =
            "GUARDING redirects; DEFENDING halves damage; MARKED adds +2 damage.".to_owned();
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
