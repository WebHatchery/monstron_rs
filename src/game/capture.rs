//! Deterministic scene fixtures used by the screenshot verification harness.

use super::Game;
use crate::engine::{tower_engine, town_engine};
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
            "stable_full" => {
                self.begin_capture_fixture(AppScreen::Stable);
                self.seed_capture_full_roster();
            }
            "stable_page_two" => {
                self.begin_capture_fixture(AppScreen::Stable);
                self.seed_capture_full_roster();
                self.stable_roster_page = 1;
            }
            "stable_rehome_warning" => {
                self.begin_capture_fixture(AppScreen::Stable);
                self.seed_capture_full_roster();
                self.stable_rehome_pending = Some(4);
            }
            "breeding" => self.begin_capture_fixture(AppScreen::Breeding),
            "workshop" => self.begin_capture_fixture(AppScreen::Workshop),
            "shop" => self.begin_capture_fixture(AppScreen::Shop),
            "tower" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.enter_tower(TowerRunGoal::Balanced);
            }
            "tower_revisit" => {
                self.begin_capture_fixture(AppScreen::Town);
                if let Some(state) = &mut self.state {
                    state.tower_progress.best_floor = 6;
                    state.tower_progress.unlocked_floor = 7;
                }
                self.tower_prep_floor = 4;
                self.enter_tower(TowerRunGoal::Scout);
            }
            "tower_floor_unlocked" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.enter_tower(TowerRunGoal::PushDeeper);
                let stair = self.state.as_ref().and_then(|state| {
                    state
                        .tower_run
                        .as_ref()?
                        .map
                        .objects
                        .iter()
                        .find_map(|object| {
                            (object.kind == crate::state::TowerMapObjectKind::Stairs)
                                .then_some((object.x, object.y))
                        })
                });
                if let (Some(state), Some(stair)) = (&mut self.state, stair) {
                    let run = state.tower_run.as_mut().unwrap();
                    run.pressure_limit = 999;
                    run.map
                        .objects
                        .retain(|object| object.kind == crate::state::TowerMapObjectKind::Stairs);
                    for _ in 0..128 {
                        let result = tower_engine::route_party_to(state, &self.data, stair);
                        self.status_message = result.summary;
                        if state
                            .tower_run
                            .as_ref()
                            .is_some_and(|run| run.current_floor == 2)
                        {
                            break;
                        }
                    }
                    if let Some(warning) = self.status_message.find(" The tower stirs") {
                        self.status_message.truncate(warning);
                    }
                    if let Some(run) = state.tower_run.as_mut() {
                        run.pressure = 0;
                        run.pressure_stage = 0;
                        run.event_log = vec![self.status_message.clone()];
                    }
                }
            }
            "dungeon_prep_unlocked" => {
                self.begin_capture_fixture(AppScreen::DungeonPrep);
                if let Some(state) = &mut self.state {
                    state.tower_progress.best_floor = 6;
                    state.tower_progress.unlocked_floor = 7;
                }
                self.tower_prep_floor = 4;
                self.status_message = "Floor 4 selected. Choose an expedition goal.".to_owned();
            }
            "combat" => self.begin_capture_combat(None),
            "combat_status" => self.begin_capture_combat_status(),
            "combat_victory" => self.begin_capture_combat(Some(CombatOutcome::Victory)),
            "combat_defeat" => self.begin_capture_combat(Some(CombatOutcome::Defeat)),
            "tutorial_welcome" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
            }
            "tutorial_town" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::WELCOME]);
            }
            "tutorial_build_hatchery" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
                if let Some(state) = &mut self.state {
                    state.town.set_building_level("hatchery", 0);
                }
                self.set_capture_tutorial_flags(&[
                    crate::screens::tutorial::WELCOME,
                    crate::screens::tutorial::SCAVENGED,
                ]);
            }
            "tutorial_open_hatchery" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
                self.set_capture_tutorial_flags(&[
                    crate::screens::tutorial::WELCOME,
                    crate::screens::tutorial::SCAVENGED,
                ]);
            }
            "tutorial_hatchery" => {
                self.begin_capture_fixture(AppScreen::Hatchery);
                self.reset_capture_tutorial();
                if let Some(state) = &mut self.state {
                    state.egg_inventory.eggs.clear();
                }
                self.set_capture_tutorial_flags(&[
                    crate::screens::tutorial::WELCOME,
                    crate::screens::tutorial::SCAVENGED,
                    crate::screens::tutorial::HATCHERY_OPENED,
                ]);
            }
            "tutorial_prep" => {
                self.begin_capture_fixture(AppScreen::DungeonPrep);
                self.reset_capture_tutorial();
                let mut flags = pre_tower_tutorial_flags();
                flags.push(crate::screens::tutorial::PREP_OPENED);
                self.set_capture_tutorial_flags(&flags);
            }
            "tutorial_tower" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.enter_tower(TowerRunGoal::SafeRun);
                self.reset_capture_tutorial();
                self.set_capture_tutorial_flags(&first_tower_tutorial_flags());
            }
            "tutorial_survey" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.enter_tower(TowerRunGoal::SafeRun);
                self.reset_capture_tutorial();
                let mut flags = first_tower_tutorial_flags();
                flags.push(crate::screens::tutorial::EXPLORED);
                self.set_capture_tutorial_flags(&flags);
            }
            "tutorial_retreat" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.enter_tower(TowerRunGoal::Balanced);
                self.reset_capture_tutorial();
                let mut flags = first_tower_tutorial_flags();
                flags.extend([
                    crate::screens::tutorial::EXPLORED,
                    crate::screens::tutorial::SURVEYED,
                ]);
                self.set_capture_tutorial_flags(&flags);
            }
            "tutorial_combat" => {
                self.begin_capture_combat(None);
                self.reset_capture_tutorial();
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::COMPLETE]);
            }
            "tutorial_combat_action" => {
                self.begin_capture_combat(None);
                self.reset_capture_tutorial();
                self.set_capture_tutorial_flags(&[
                    crate::screens::tutorial::COMPLETE,
                    crate::screens::tutorial::COMBAT_INTRO,
                ]);
            }
            "tutorial_finished" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
                let mut flags = first_tower_tutorial_flags();
                flags.extend([
                    crate::screens::tutorial::EXPLORED,
                    crate::screens::tutorial::SURVEYED,
                    crate::screens::tutorial::RETURNED,
                    crate::screens::tutorial::RECOVERED,
                    crate::screens::tutorial::SAVED,
                ]);
                self.set_capture_tutorial_flags(&flags);
            }
            "tutorial_recovery" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
                self.set_capture_tutorial_flags(&returned_tutorial_flags());
            }
            "tutorial_end_day" => {
                self.begin_capture_fixture(AppScreen::EndOfDay);
                self.reset_capture_tutorial();
                let mut flags = returned_tutorial_flags();
                flags.push(crate::screens::tutorial::RECOVERED);
                self.set_capture_tutorial_flags(&flags);
            }
            "tutorial_menu" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
                let mut flags = returned_tutorial_flags();
                flags.push(crate::screens::tutorial::RECOVERED);
                self.set_capture_tutorial_flags(&flags);
            }
            "tutorial_save" => {
                self.begin_capture_fixture(AppScreen::Town);
                self.reset_capture_tutorial();
                let mut flags = returned_tutorial_flags();
                flags.push(crate::screens::tutorial::RECOVERED);
                self.set_capture_tutorial_flags(&flags);
                self.town_menu_open = true;
            }
            "tutorial_egg_intro" => {
                self.begin_capture_egg_tutorial(AppScreen::Town, 2, false);
            }
            "tutorial_egg_open" => {
                self.begin_capture_egg_tutorial(AppScreen::Town, 2, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
            }
            "tutorial_egg_care" => {
                self.begin_capture_egg_tutorial(AppScreen::Hatchery, 2, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
            }
            "tutorial_egg_leave" => {
                self.begin_capture_egg_tutorial(AppScreen::Hatchery, 1, true);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
            }
            "tutorial_egg_sleep" => {
                self.begin_capture_egg_tutorial(AppScreen::Town, 1, true);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
            }
            "tutorial_egg_end_day" => {
                self.begin_capture_egg_tutorial(AppScreen::EndOfDay, 1, true);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
            }
            "tutorial_egg_hatch" => {
                self.begin_capture_egg_tutorial(AppScreen::Hatchery, 0, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
            }
            "tutorial_egg_build_stable" => {
                self.begin_capture_egg_tutorial(AppScreen::Town, 0, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
                if let Some(state) = &mut self.state {
                    state.town.set_building_level("stable", 0);
                }
            }
            "tutorial_egg_upgrade_stable" => {
                self.begin_capture_egg_tutorial(AppScreen::Town, 0, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
                if let Some(state) = &mut self.state {
                    let template = state.monster_roster.monsters[0].clone();
                    while state.monster_roster.monsters.len() < 6 {
                        let mut monster = template.clone();
                        monster.id = state.monster_roster.monsters.len() as u64 + 1;
                        state.monster_roster.monsters.push(monster);
                    }
                }
            }
            "tutorial_egg_open_stable" => {
                self.begin_capture_egg_tutorial(AppScreen::Town, 0, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
                self.seed_capture_full_roster();
            }
            "tutorial_egg_rehome" => {
                self.begin_capture_egg_tutorial(AppScreen::Stable, 0, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
                self.seed_capture_full_roster();
            }
            "tutorial_egg_rehome_warning" => {
                self.begin_capture_egg_tutorial(AppScreen::Stable, 0, false);
                self.set_capture_tutorial_flags(&[crate::screens::tutorial::EGG_INTRO]);
                self.seed_capture_full_roster();
                self.stable_rehome_pending = Some(4);
            }
            "tutorial_egg_finished" => {
                self.begin_capture_egg_tutorial(AppScreen::Hatchery, 0, false);
                if let Some(state) = &mut self.state {
                    state.egg_inventory.eggs.clear();
                }
                self.set_capture_tutorial_flags(&[
                    crate::screens::tutorial::EGG_INTRO,
                    crate::screens::tutorial::EGG_HATCHED,
                ]);
            }
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
        state.story_flags.add(crate::screens::tutorial::SKIPPED);
        self.screen = screen;
        self.town_menu_open = false;
        self.stable_roster_page = 0;
        self.stable_rehome_pending = None;
        self.tower_prep_floor = 1;
        self.status_message =
            "Seeded verification scene. Tap a visible control to continue.".to_owned();
    }

    fn reset_capture_tutorial(&mut self) {
        if let Some(state) = &mut self.state {
            state
                .story_flags
                .flags
                .retain(|flag| !flag.starts_with("tutorial_"));
        }
    }

    fn set_capture_tutorial_flags(&mut self, flags: &[&str]) {
        if let Some(state) = &mut self.state {
            for flag in flags {
                state.story_flags.add(flag);
            }
        }
    }

    fn begin_capture_egg_tutorial(
        &mut self,
        screen: AppScreen,
        days_remaining: u32,
        cared_today: bool,
    ) {
        self.begin_capture_fixture(screen);
        self.reset_capture_tutorial();
        self.set_capture_tutorial_flags(&[crate::screens::tutorial::COMPLETE]);
        if let Some(state) = &mut self.state {
            if let Some(egg) = state.egg_inventory.eggs.first_mut() {
                egg.days_remaining = days_remaining;
                egg.last_care_day = if cared_today { state.day } else { 0 };
            }
        }
    }

    fn seed_capture_full_roster(&mut self) {
        let Some(state) = &mut self.state else {
            return;
        };
        state.town.set_building_level("stable", 3);
        let names = [
            "Fern", "Cinder", "Brook", "Pebble", "Glimmer", "Mallow", "Ash", "Reed", "Dew",
        ];
        let species_ids = ["rootling", "emberkit", "rillfin", "pebblepup", "glowmoth"];
        while state.monster_roster.monsters.len() < town_engine::MAX_MONSTER_CAPACITY {
            let index = state.monster_roster.monsters.len() - 3;
            let species_id = species_ids[index % species_ids.len()];
            if let Some(species) = self.data.species(species_id) {
                state.monster_roster.add_monster(
                    names[index].to_owned(),
                    species,
                    0x5100 + index as u64,
                );
            } else {
                break;
            }
        }
    }
}

fn first_tower_tutorial_flags() -> Vec<&'static str> {
    let mut flags = pre_tower_tutorial_flags();
    flags.extend([
        crate::screens::tutorial::PREP_OPENED,
        crate::screens::tutorial::TOWER_ENTERED,
    ]);
    flags
}

fn pre_tower_tutorial_flags() -> Vec<&'static str> {
    vec![
        crate::screens::tutorial::WELCOME,
        crate::screens::tutorial::SCAVENGED,
        crate::screens::tutorial::HATCHERY_OPENED,
        crate::screens::tutorial::HATCHERY_VISITED,
    ]
}

fn returned_tutorial_flags() -> Vec<&'static str> {
    let mut flags = first_tower_tutorial_flags();
    flags.extend([
        crate::screens::tutorial::EXPLORED,
        crate::screens::tutorial::SURVEYED,
        crate::screens::tutorial::RETURNED,
    ]);
    flags
}
