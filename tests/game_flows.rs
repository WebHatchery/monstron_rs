use hatchspire::data::GameDataLoader;
use hatchspire::engine::{
    combat_engine::{self, CombatCommand},
    combat_replay::{CombatReplay, ReplayReport},
    egg_engine, monster_engine, tower_engine, town_engine,
};
use hatchspire::screens;
use hatchspire::state::{
    EggCareFocus, GameState, TowerMapObject, TowerMapObjectKind, TowerRunGoal,
};

fn data_and_state() -> (hatchspire::data::GameData, GameState) {
    let data = GameDataLoader::load_embedded().expect("embedded game data should load");
    let state = GameState::new(&data);
    (data, state)
}

#[test]
fn hatchery_care_and_hatching_flow_preserves_capacity_and_traits() {
    let (data, mut state) = data_and_state();
    state.town.set_building_level("hatchery", 1);
    state.town.set_building_level("stable", 1);
    state.resources.add("herbs", 4);
    state
        .egg_inventory
        .add_egg("mossy_egg".to_owned(), 1, 1, 0xCAFE);

    let care =
        egg_engine::care_for_egg(&mut state, &data, 1, hatchspire::state::EggCareFocus::Warm);
    assert!(care.summary.contains("warmed"));
    let hatch = egg_engine::hatch_egg(&mut state, &data, 1);
    assert!(hatch.summary.contains("hatched"));
    assert!(state.egg_inventory.eggs.is_empty());
    assert_eq!(state.monster_roster.monsters.len(), 2);
}

#[test]
fn explored_floor_one_eggs_can_become_a_three_monster_party() {
    let (data, mut state) = data_and_state();
    town_engine::scavenge_supplies(&mut state);
    town_engine::advance_building(&mut state, &data, "hatchery");
    town_engine::scavenge_supplies(&mut state);
    town_engine::advance_building(&mut state, &data, "stable");

    tower_engine::start_run_on_floor(&mut state, &data, TowerRunGoal::EggHunt, 1);
    let targets = state
        .tower_run
        .as_ref()
        .unwrap()
        .map
        .objects
        .iter()
        .filter(|object| object.kind == TowerMapObjectKind::Egg)
        .take(2)
        .map(|object| (object.x, object.y))
        .collect::<Vec<_>>();
    assert_eq!(
        targets.len(),
        2,
        "an Egg Hunt should generate two reachable nests"
    );
    {
        let run = state.tower_run.as_mut().unwrap();
        run.pressure_limit = 999;
        run.map
            .objects
            .retain(|object| object.kind == TowerMapObjectKind::Egg);
    }
    for target in targets {
        for _ in 0..128 {
            let before = state.tower_run.as_ref().unwrap().found_eggs.len();
            tower_engine::route_party_to(&mut state, &data, target);
            if state.tower_run.as_ref().unwrap().found_eggs.len() > before {
                break;
            }
        }
    }
    assert!(state.tower_run.as_ref().unwrap().found_eggs.len() >= 2);

    tower_engine::return_to_town(&mut state, &data);
    let egg_ids = state
        .egg_inventory
        .eggs
        .iter()
        .take(2)
        .map(|egg| egg.id)
        .collect::<Vec<_>>();
    assert_eq!(egg_ids.len(), 2);
    for egg_id in &egg_ids {
        let care = egg_engine::care_for_egg(&mut state, &data, *egg_id, EggCareFocus::Warm);
        assert!(care.summary.contains("warmed"));
    }
    town_engine::reduce(&mut state, &data, &town_engine::TownCommand::Sleep);
    for egg_id in egg_ids {
        let hatch = egg_engine::hatch_egg(&mut state, &data, egg_id);
        assert!(hatch.summary.contains("hatched"));
    }

    let recruits = state
        .monster_roster
        .monsters
        .iter()
        .skip(1)
        .map(|monster| monster.id)
        .collect::<Vec<_>>();
    for monster_id in recruits {
        let result = monster_engine::toggle_party_member(&mut state, &data, monster_id);
        assert!(result.summary.contains("Assigned"));
    }
    assert_eq!(tower_engine::battle_ready_party_count(&state), 3);
}

#[test]
fn stable_roster_and_recovery_actions_follow_daily_commitments() {
    let (data, mut state) = data_and_state();
    state.town.set_building_level("stable", 1);
    let rootling = data.species("rootling").expect("rootling data");
    let monster_id = state
        .monster_roster
        .add_monster("Rooty".to_owned(), rootling, 0x100);

    let joined = monster_engine::toggle_party_member(&mut state, &data, monster_id);
    assert!(joined.summary.contains("Assigned"));
    let slot = state
        .monster_roster
        .party_slots
        .iter()
        .position(|id| *id == Some(monster_id))
        .expect("monster should join the party");
    state
        .monster_roster
        .monster_mut(monster_id)
        .unwrap()
        .condition
        .fatigue = 4;
    state
        .monster_roster
        .monster_mut(monster_id)
        .unwrap()
        .condition
        .injury_days = 1;
    let recovery = monster_engine::recover_monsters(&mut state);
    assert!(recovery.fatigue_reduced > 0 || recovery.injuries_healed > 0);
    let benched = monster_engine::remove_party_slot(&mut state, slot);
    assert!(benched.summary.contains("Removed"));
}

#[test]
fn shop_trades_and_purchase_validation_are_atomic() {
    let (data, mut state) = data_and_state();
    state.town.set_building_level("shop", 1);
    state.resources.add("coins", 6);
    let coins_before = state.resources.amount("coins");
    let herbs_before = state.resources.amount("herbs");
    let bought = town_engine::trade_shop(&mut state, &data, town_engine::ShopTrade::BuyHerbs);
    assert!(bought.summary.contains("Completed"));
    assert_eq!(state.resources.amount("coins"), coins_before - 6);
    assert_eq!(state.resources.amount("herbs"), herbs_before + 3);

    state
        .resources
        .add("coins", -state.resources.amount("coins"));
    let rejected = town_engine::trade_shop(&mut state, &data, town_engine::ShopTrade::BuyStone);
    assert!(rejected.summary.contains("needs"));
    assert_eq!(state.resources.amount("stone"), 8);
}

#[test]
fn tower_preparation_movement_return_and_rewards_form_one_flow() {
    let (data, mut state) = data_and_state();
    let started = tower_engine::start_run(&mut state, &data, TowerRunGoal::Salvage);
    assert!(started.summary.contains("enters floor"));
    let (dx, dy, x, y) = {
        let run = state.tower_run.as_ref().expect("run should start");
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .find_map(|(dx, dy)| {
                let x = run.map.player_x as i32 + dx;
                let y = run.map.player_y as i32 + dy;
                (x >= 0 && y >= 0 && run.map.is_passable(x as u32, y as u32))
                    .then_some((dx, dy, x as u32, y as u32))
            })
            .expect("start room should have an adjacent tile")
    };
    state
        .tower_run
        .as_mut()
        .unwrap()
        .map
        .objects
        .push(TowerMapObject {
            kind: TowerMapObjectKind::Loot,
            x,
            y,
            resource_id: "wood".to_owned(),
            amount: 3,
            egg_type_id: String::new(),
            hatch_days: 0,
            palette_seed: 0,
            enemy_id: String::new(),
            special_location_id: String::new(),
            event_id: String::new(),
            hazard_id: String::new(),
            wandering: false,
            revealed: false,
        });
    let moved = tower_engine::move_party(&mut state, &data, dx, dy);
    assert!(moved.summary.contains("Found 3"));
    let wood_before = state.resources.amount("wood");
    let returned = tower_engine::return_to_town(&mut state, &data);
    assert!(returned.summary.contains("Returned"));
    assert!(state.tower_run.is_none());
    assert_eq!(state.resources.amount("wood"), wood_before + 3);
}

#[test]
fn combat_commands_resolution_and_replay_recover_after_victory() {
    let (data, mut state) = data_and_state();
    combat_engine::start_encounter(&mut state, &data, 1, false);
    for _ in 0..80 {
        if state
            .combat
            .as_ref()
            .is_some_and(|combat| combat.outcome.is_some())
        {
            break;
        }
        combat_engine::reduce_command(&mut state, &data, CombatCommand::Attack);
    }
    let replay = CombatReplay::from_combat(state.combat.as_ref().expect("combat remains"));
    assert_eq!(replay.run(&data), ReplayReport::Match);
    let finish = combat_engine::finish_combat(&mut state, &data);
    assert!(finish.summary.contains("Victory"));
    assert!(state.combat.is_none());
}

#[test]
fn town_commands_cover_purchases_and_building_progression() {
    let (data, mut state) = data_and_state();
    state.resources.add("wood", 20);
    state.resources.add("herbs", 10);
    let command = town_engine::TownCommand::AdvanceBuilding("hatchery".to_owned());
    let built = town_engine::reduce(&mut state, &data, &command);
    assert!(built.summary.contains("Built Hatchery"));
    assert_eq!(state.town.building_level("hatchery"), 1);

    let trade = town_engine::reduce(
        &mut state,
        &data,
        &town_engine::TownCommand::GreetNpc("mara".to_owned()),
    );
    assert!(trade.summary.contains("Friendship"));
}

#[test]
fn balance_data_is_typed_and_every_reference_is_integrity_checked() {
    let (data, _) = data_and_state();
    assert_eq!(
        data.balance.monster_stat_curves.len(),
        data.monster_species.len()
    );
    assert_eq!(data.combat_cooldown("skill"), Some(0));
    assert!(data.shop_trade("buy_herbs").is_some());
    assert!(data.tower_reward(10).is_some());
    assert!(data.enemies.len() >= 80);
    assert_eq!(data.tower_hazards.len(), 6);
    assert_eq!(data.tower_contracts.len(), 6);
    assert_eq!(data.tower_anomalies.len(), 6);
    assert_eq!(data.tower_special_locations.len(), 22);
    assert_eq!(data.tower_events.len(), 44);
    for behavior in [
        hatchspire::data::EnemyBehavior::Bruiser,
        hatchspire::data::EnemyBehavior::Bulwark,
        hatchspire::data::EnemyBehavior::Harrier,
        hatchspire::data::EnemyBehavior::Hexer,
        hatchspire::data::EnemyBehavior::Swarm,
        hatchspire::data::EnemyBehavior::Ambusher,
        hatchspire::data::EnemyBehavior::Regenerator,
        hatchspire::data::EnemyBehavior::Packleader,
        hatchspire::data::EnemyBehavior::Sapper,
        hatchspire::data::EnemyBehavior::Leech,
        hatchspire::data::EnemyBehavior::Warden,
    ] {
        assert!(data.enemies.iter().any(|enemy| enemy.behavior == behavior));
    }
    for effect in [
        hatchspire::data::TowerAnomalyEffect::QuietVeil,
        hatchspire::data::TowerAnomalyEffect::EchoingRain,
        hatchspire::data::TowerAnomalyEffect::CacheBloom,
        hatchspire::data::TowerAnomalyEffect::MendingLights,
        hatchspire::data::TowerAnomalyEffect::NestingPulse,
        hatchspire::data::TowerAnomalyEffect::HunterTracks,
    ] {
        assert!(data
            .tower_anomalies
            .iter()
            .any(|anomaly| anomaly.effect == effect));
    }
}

#[test]
fn rendering_entry_points_cannot_mutate_progression_state() {
    let town_draw: fn(&GameState, &hatchspire::data::GameData, &str, bool) = screens::town::draw;
    let combat_draw: fn(&GameState, &hatchspire::data::GameData, &str) = screens::combat::draw;
    let _ = (town_draw, combat_draw);
}

#[test]
fn validation_failures_name_a_visible_recovery_action() {
    let (data, mut state) = data_and_state();
    state.town.set_building_level("hatchery", 1);
    state.resources.add("wood", -state.resources.amount("wood"));
    state
        .resources
        .add("herbs", -state.resources.amount("herbs"));
    let building = town_engine::advance_building(&mut state, &data, "hatchery");
    assert!(building.summary.contains("Tap Scavenge"));

    let egg = egg_engine::care_for_egg(
        &mut state,
        &data,
        999,
        hatchspire::state::EggCareFocus::Study,
    );
    assert!(egg.summary.contains("Tap Town"));

    let combat = combat_engine::player_action(&mut state, &data, CombatCommand::Item);
    assert!(combat.summary.contains("Tap Town"));

    state.monster_roster.party_slots.fill(None);
    let tower = tower_engine::start_run(&mut state, &data, TowerRunGoal::SafeRun);
    assert!(tower.summary.contains("Tap Stable"));
}
