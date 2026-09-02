use super::*;
use crate::data::GameDataLoader;

#[test]
fn crown_exit_reseals_until_the_guardian_falls() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    start_run(&mut state, &data, TowerRunGoal::PushDeeper);
    let run = state.tower_run.as_mut().unwrap();
    run.current_floor = 10;
    run.map.floor = 10;
    run.map.objects.clear();
    let exit = crown_exit(run.map.player_x, run.map.player_y);

    let result = resolve_map_object(&mut state, &data, exit);

    assert!(result.summary.contains("threshold is sealed"));
    assert!(!result.returned_to_town);
    let run = state.tower_run.as_ref().expect("sealed run should remain");
    assert_eq!(run.map.objects.len(), 1);
    assert_eq!(run.map.objects[0].kind, TowerMapObjectKind::Exit);
}

#[test]
fn mirror_checkpoint_names_its_guardian_and_seals_the_deeper_stair() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    let map = generate_map(&state, &data, 5, TowerRunGoal::PushDeeper, 505);
    let stairs = map
        .objects
        .iter()
        .find(|object| object.kind == TowerMapObjectKind::Stairs)
        .cloned()
        .expect("checkpoint should retain a deeper stair");
    assert!(map.objects.iter().any(|object| {
        object.kind == TowerMapObjectKind::Boss && object.enemy_id == "mirror_matriarch"
    }));
    state.tower_run = Some(TowerRunState::new(5, 13, TowerRunGoal::PushDeeper).with_map(map));

    let result = resolve_map_object(&mut state, &data, stairs.clone());
    assert!(result.summary.contains("guardian seals"));
    assert_eq!(state.tower_run.as_ref().unwrap().current_floor, 5);

    state.tower_run.as_mut().unwrap().boss_defeated = true;
    let result = resolve_map_object(&mut state, &data, stairs);
    assert!(result.summary.contains("Descended to floor 6"));
    assert_eq!(state.tower_run.as_ref().unwrap().current_floor, 6);
}

#[test]
fn opened_crown_returns_rewards_and_completes_the_campaign() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    start_run(&mut state, &data, TowerRunGoal::PushDeeper);
    let wood_before = state.resources.amount("wood");
    let run = state.tower_run.as_mut().unwrap();
    run.current_floor = 10;
    run.map.floor = 10;
    run.boss_defeated = true;
    run.rooms_explored = 73;
    run.stats.landmarks_resolved = 8;
    run.add_cargo("wood", 7);
    let exit = crown_exit(run.map.player_x, run.map.player_y);

    let result = resolve_map_object(&mut state, &data, exit);

    assert!(result.completed_tower);
    assert!(result.returned_to_town);
    assert!(result.summary.contains("Verdant Crown opens to daylight"));
    assert!(result.summary.contains("73 steps"));
    assert!(state.story_flags.has("verdant_crown_restored"));
    assert!(state.tower_run.is_none());
    assert_eq!(state.resources.amount("wood"), wood_before + 7);
}

fn crown_exit(x: u32, y: u32) -> TowerMapObject {
    TowerMapObject {
        kind: TowerMapObjectKind::Exit,
        x,
        y,
        resource_id: String::new(),
        amount: 0,
        egg_type_id: String::new(),
        hatch_days: 0,
        palette_seed: 0,
        enemy_id: String::new(),
        special_location_id: String::new(),
        event_id: String::new(),
        hazard_id: String::new(),
        wandering: false,
        revealed: false,
    }
}
