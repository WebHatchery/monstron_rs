use std::collections::{HashSet, VecDeque};

use super::*;
use crate::data::GameDataLoader;

#[test]
fn every_authored_floor_goal_and_seed_is_fully_connected() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let state = GameState::new(&data);
    let goals = [
        TowerRunGoal::Balanced,
        TowerRunGoal::EggHunt,
        TowerRunGoal::Salvage,
        TowerRunGoal::Scout,
        TowerRunGoal::PushDeeper,
        TowerRunGoal::SafeRun,
    ];

    for floor in 1..=10 {
        for goal in goals {
            for seed in 1..=32 {
                let map = generate_map(&state, &data, floor, goal, seed);
                let reachable = reachable_tiles(&map);
                let context = format!("floor {floor}, goal {goal}, seed {seed}");

                assert!(map.is_passable(map.start_x, map.start_y), "{context}");
                assert!(
                    map.tiles.iter().enumerate().all(|(index, tile)| {
                        !tile.is_passable()
                            || reachable
                                .contains(&(index as u32 % map.width, index as u32 / map.width))
                    }),
                    "disconnected passable tile on {context}"
                );
                assert!(
                    map.rooms
                        .iter()
                        .all(|room| reachable.contains(&room.center())),
                    "unreachable room center on {context}"
                );
                assert!(
                    map.objects
                        .iter()
                        .all(|object| reachable.contains(&(object.x, object.y))),
                    "unreachable map object on {context}"
                );

                let required_exit = if floor == 10 {
                    TowerMapObjectKind::Exit
                } else {
                    TowerMapObjectKind::Stairs
                };
                assert!(
                    map.objects
                        .iter()
                        .any(|object| object.kind == required_exit),
                    "missing floor exit on {context}"
                );
            }
        }
    }
}

fn reachable_tiles(map: &crate::state::TowerMapState) -> HashSet<(u32, u32)> {
    let mut reached = HashSet::from([(map.start_x, map.start_y)]);
    let mut queue = VecDeque::from([(map.start_x, map.start_y)]);
    while let Some((x, y)) = queue.pop_front() {
        for (next_x, next_y) in [
            (x.saturating_sub(1), y),
            (x.saturating_add(1), y),
            (x, y.saturating_sub(1)),
            (x, y.saturating_add(1)),
        ] {
            if next_x >= map.width
                || next_y >= map.height
                || !map.is_passable(next_x, next_y)
                || !reached.insert((next_x, next_y))
            {
                continue;
            }
            queue.push_back((next_x, next_y));
        }
    }
    reached
}
