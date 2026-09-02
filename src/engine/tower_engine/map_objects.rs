use super::max_floor;
use crate::data::{GameData, MonsterRole, PassiveSkill, TowerFloorDefinition};
use crate::state::{
    GameState, TowerMapObject, TowerMapObjectKind, TowerMapRng, TowerMapState, TowerRoomKind,
    TowerRunGoal,
};
use std::collections::VecDeque;

pub(super) fn add_map_objects(
    map: &mut TowerMapState,
    state: &GameState,
    data: &GameData,
    floor_number: u32,
    goal: TowerRunGoal,
    rng: &mut TowerMapRng,
) {
    let Some(floor) = data.tower_floor(floor_number) else {
        return;
    };

    if floor.floor < max_floor(data) && !floor.is_boss_floor {
        place_object(map, TowerMapObject::stairs(0, 0), rng);
    }
    if floor.floor % 5 == 0 || floor.is_boss_floor {
        place_object(map, TowerMapObject::exit(0, 0), rng);
    }

    for object in special_location_objects(data, floor_number, goal, rng) {
        place_room_landmark(map, object, rng);
    }
    let secret_count = if goal == TowerRunGoal::Salvage { 2 } else { 1 };
    for _ in 0..secret_count {
        if let Some(object) = secret_cache_object(floor, rng) {
            place_object(map, object, rng);
        }
    }
    for object in loot_objects(floor, goal, rng) {
        place_object(map, object, rng);
    }
    for object in egg_objects(floor, data, goal, rng) {
        place_object(map, object, rng);
    }
    for object in hazard_objects(data, floor_number, goal, rng) {
        place_object(map, object, rng);
    }

    if !floor.guardian_enemy_id.is_empty() && !state.tower_progress.guardian_defeated(floor_number)
    {
        place_object(map, TowerMapObject::boss(&floor.guardian_enemy_id), rng);
    } else if floor.is_boss_floor && !state.tower_progress.guardian_defeated(floor_number) {
        if let Some(enemy) = eligible_enemies(data, floor_number, true).first() {
            place_object(map, TowerMapObject::boss(&enemy.id), rng);
        }
    } else {
        let enemies = eligible_enemies(data, floor_number, false);
        let count = enemy_count(map, state, floor_number, goal) as usize;
        for enemy in varied_enemy_roster(enemies, count, rng) {
            place_object(map, TowerMapObject::enemy(&enemy.id), rng);
        }
    }
}

fn varied_enemy_roster<'a>(
    mut enemies: Vec<&'a crate::data::EnemyDefinition>,
    count: usize,
    rng: &mut TowerMapRng,
) -> Vec<&'a crate::data::EnemyDefinition> {
    for upper in (1..enemies.len()).rev() {
        let swap = rng.range(0, upper as u32 + 1) as usize;
        enemies.swap(upper, swap);
    }
    enemies.into_iter().cycle().take(count).collect()
}

fn eligible_enemies(
    data: &GameData,
    floor_number: u32,
    is_boss: bool,
) -> Vec<&crate::data::EnemyDefinition> {
    data.enemies
        .iter()
        .filter(|enemy| {
            enemy.is_boss == is_boss
                && enemy.min_floor <= floor_number
                && enemy.max_floor >= floor_number
        })
        .collect()
}

fn special_location_objects(
    data: &GameData,
    floor_number: u32,
    goal: TowerRunGoal,
    rng: &mut TowerMapRng,
) -> Vec<TowerMapObject> {
    let eligible: Vec<_> = data
        .tower_special_locations
        .iter()
        .filter(|location| location.min_floor <= floor_number && location.max_floor >= floor_number)
        .collect();
    if eligible.is_empty() {
        return Vec::new();
    }

    let count = if goal == TowerRunGoal::Scout || (eligible.len() > 1 && rng.chance(1, 3)) {
        2
    } else {
        1
    };
    let start = rng.range(0, eligible.len() as u32) as usize;
    (0..count.min(eligible.len()))
        .filter_map(|offset| {
            let location = eligible[(start + offset) % eligible.len()];
            let event_id = location
                .event_ids
                .get(rng.range(0, location.event_ids.len() as u32) as usize)?;
            Some(TowerMapObject::special_location(&location.id, event_id))
        })
        .collect()
}

fn hazard_objects(
    data: &GameData,
    floor_number: u32,
    goal: TowerRunGoal,
    rng: &mut TowerMapRng,
) -> Vec<TowerMapObject> {
    if goal == TowerRunGoal::SafeRun {
        return Vec::new();
    }
    let eligible: Vec<_> = data
        .tower_hazards
        .iter()
        .filter(|hazard| hazard.min_floor <= floor_number && hazard.max_floor >= floor_number)
        .collect();
    if eligible.is_empty() {
        return Vec::new();
    }
    let count = if goal == TowerRunGoal::PushDeeper || floor_number >= 8 {
        2
    } else {
        1
    };
    (0..count)
        .filter_map(|_| {
            let hazard = eligible.get(rng.range(0, eligible.len() as u32) as usize)?;
            Some(TowerMapObject::hazard(&hazard.id))
        })
        .collect()
}

fn loot_objects(
    floor: &TowerFloorDefinition,
    goal: TowerRunGoal,
    rng: &mut TowerMapRng,
) -> Vec<TowerMapObject> {
    if floor.loot.is_empty() {
        return Vec::new();
    }

    let mut count = 3 + (floor.floor / 3).min(3);
    match goal {
        TowerRunGoal::Salvage => count += 3,
        TowerRunGoal::EggHunt | TowerRunGoal::SafeRun => count = count.saturating_sub(1),
        _ => {}
    }

    (0..count)
        .filter_map(|_| {
            let base = floor
                .loot
                .get(rng.range(0, floor.loot.len() as u32) as usize)?;
            let goal_bonus = if goal == TowerRunGoal::Salvage { 2 } else { 0 };
            Some(TowerMapObject {
                kind: TowerMapObjectKind::Loot,
                x: 0,
                y: 0,
                resource_id: base.resource_id.clone(),
                amount: base.amount + goal_bonus + rng.range(0, 3) as i32,
                egg_type_id: String::new(),
                hatch_days: 0,
                palette_seed: 0,
                enemy_id: String::new(),
                special_location_id: String::new(),
                event_id: String::new(),
                hazard_id: String::new(),
                wandering: false,
                revealed: false,
            })
        })
        .collect()
}

fn secret_cache_object(
    floor: &TowerFloorDefinition,
    rng: &mut TowerMapRng,
) -> Option<TowerMapObject> {
    let loot = floor
        .loot
        .get(rng.range(0, floor.loot.len() as u32) as usize)?;
    Some(TowerMapObject {
        kind: TowerMapObjectKind::SecretCache,
        x: 0,
        y: 0,
        resource_id: loot.resource_id.clone(),
        amount: loot.amount + 3 + (floor.floor / 3) as i32,
        egg_type_id: String::new(),
        hatch_days: 0,
        palette_seed: 0,
        enemy_id: String::new(),
        special_location_id: String::new(),
        event_id: String::new(),
        hazard_id: String::new(),
        wandering: false,
        revealed: false,
    })
}

fn egg_objects(
    floor: &TowerFloorDefinition,
    data: &GameData,
    goal: TowerRunGoal,
    rng: &mut TowerMapRng,
) -> Vec<TowerMapObject> {
    if floor.egg_types.is_empty() {
        return Vec::new();
    }

    let count = match goal {
        TowerRunGoal::EggHunt => 2 + rng.range(0, 2),
        TowerRunGoal::Salvage => rng.range(0, 2),
        _ if rng.chance(2, 3) => 1,
        _ => 0,
    };

    (0..count)
        .filter_map(|_| {
            let egg_id = floor
                .egg_types
                .get(rng.range(0, floor.egg_types.len() as u32) as usize)?;
            let egg = data.egg_type(egg_id)?;
            Some(TowerMapObject {
                kind: TowerMapObjectKind::Egg,
                x: 0,
                y: 0,
                resource_id: String::new(),
                amount: 0,
                egg_type_id: egg.id.clone(),
                hatch_days: egg.hatch_days,
                palette_seed: 0xE66_0000 ^ u64::from(floor.floor) << 24 ^ u64::from(rng.next_u32()),
                enemy_id: String::new(),
                special_location_id: String::new(),
                event_id: String::new(),
                hazard_id: String::new(),
                wandering: false,
                revealed: false,
            })
        })
        .collect()
}

fn enemy_count(
    map: &TowerMapState,
    state: &GameState,
    floor_number: u32,
    goal: TowerRunGoal,
) -> u32 {
    let profile = party_profile(state);
    let mut count = (map.rooms.len() as u32 / 2 + floor_number / 3).clamp(2, 9);
    match goal {
        TowerRunGoal::PushDeeper => count += 2,
        TowerRunGoal::SafeRun | TowerRunGoal::Scout => count = count.saturating_sub(2).max(1),
        _ => {}
    }
    if profile.safety >= 3 {
        count = count.saturating_sub(1).max(1);
    }
    count
}

#[derive(Default)]
struct TowerPartyProfile {
    safety: u32,
}

fn party_profile(state: &GameState) -> TowerPartyProfile {
    let mut profile = TowerPartyProfile::default();
    for monster_id in state.monster_roster.party_slots.iter().flatten() {
        let Some(monster) = state.monster_roster.monster(*monster_id) else {
            continue;
        };
        match monster.role {
            MonsterRole::Tank | MonsterRole::Support => profile.safety += 1,
            MonsterRole::Scout | MonsterRole::Striker => {}
        }
        match monster.passive {
            PassiveSkill::ResistsPoison | PassiveSkill::SoothesInjuries => profile.safety += 2,
            PassiveSkill::FindsSmallLoot
            | PassiveSkill::DetectsEggs
            | PassiveSkill::FindsStone
            | PassiveSkill::BurnsBrambles => {}
        }
    }
    profile
}

fn place_object(map: &mut TowerMapState, mut object: TowerMapObject, rng: &mut TowerMapRng) {
    if map.rooms.is_empty() {
        return;
    }

    for _ in 0..80 {
        let room_index = if map.rooms.len() > 1 {
            rng.range(1, map.rooms.len() as u32) as usize
        } else {
            0
        };
        let (x, y) = map.rooms[room_index].random_inner(rng);
        if x == map.player_x && y == map.player_y {
            continue;
        }
        if !map.is_passable(x, y) || map.object_at(x, y).is_some() {
            continue;
        }

        let room_kind = room_kind_for_object(object.kind);
        object.x = x;
        object.y = y;
        map.objects.push(object);
        map.set_room_kind(room_index, room_kind);
        return;
    }
}

pub(super) fn spawn_pressure_enemy(
    map: &mut TowerMapState,
    data: &GameData,
    seed: u64,
) -> Option<String> {
    let eligible = eligible_enemies(data, map.floor, false);
    if eligible.is_empty() {
        return None;
    }
    let mut rng = TowerMapRng::new(seed);
    let fresh = eligible
        .iter()
        .copied()
        .filter(|enemy| {
            !map.objects.iter().any(|object| {
                matches!(
                    object.kind,
                    TowerMapObjectKind::Enemy | TowerMapObjectKind::Boss
                ) && object.enemy_id == enemy.id
            })
        })
        .collect::<Vec<_>>();
    let pool = if fresh.is_empty() { &eligible } else { &fresh };
    let enemy = pool[rng.range(0, pool.len() as u32) as usize];
    let before = map.objects.len();
    let mut object = TowerMapObject::enemy(&enemy.id);
    object.wandering = true;
    place_object(map, object, &mut rng);
    (map.objects.len() > before).then(|| enemy.id.clone())
}

pub(super) enum WanderingAdvance {
    Moved,
    Encounter(TowerMapObject),
}

pub(super) fn advance_wandering_enemy(map: &mut TowerMapState) -> Option<WanderingAdvance> {
    let hunter_indices = map
        .objects
        .iter()
        .enumerate()
        .filter_map(|(index, object)| object.wandering.then_some(index))
        .collect::<Vec<_>>();
    if hunter_indices.is_empty() {
        return None;
    }
    let hunter_index =
        hunter_indices[(map.seed as usize + map.player_x as usize + map.player_y as usize)
            % hunter_indices.len()];
    let hunter = &map.objects[hunter_index];
    let (next_x, next_y) = chase_step(map, hunter_index, hunter.x, hunter.y)?;
    if next_x == map.player_x && next_y == map.player_y {
        return Some(WanderingAdvance::Encounter(
            map.objects.remove(hunter_index),
        ));
    }
    map.objects[hunter_index].x = next_x;
    map.objects[hunter_index].y = next_y;
    Some(WanderingAdvance::Moved)
}

fn chase_step(
    map: &TowerMapState,
    hunter_index: usize,
    start_x: u32,
    start_y: u32,
) -> Option<(u32, u32)> {
    let start = map_index(map, start_x, start_y)?;
    let target = map_index(map, map.player_x, map.player_y)?;
    let mut queue = VecDeque::from([start]);
    let mut previous = vec![None; (map.width * map.height) as usize];
    previous[start] = Some(start);

    while let Some(index) = queue.pop_front() {
        if index == target {
            break;
        }
        let (x, y) = coordinates(map, index);
        for (next_x, next_y) in chase_neighbors(map, x, y) {
            let Some(next) = map_index(map, next_x, next_y) else {
                continue;
            };
            let occupied = map.objects.iter().enumerate().any(|(index, object)| {
                index != hunter_index && object.x == next_x && object.y == next_y
            });
            if previous[next].is_some()
                || !map.is_passable(next_x, next_y)
                || (occupied && next != target)
            {
                continue;
            }
            previous[next] = Some(index);
            queue.push_back(next);
        }
    }
    previous[target]?;
    let mut step = target;
    while previous[step]? != start {
        step = previous[step]?;
    }
    Some(coordinates(map, step))
}

fn chase_neighbors(map: &TowerMapState, x: u32, y: u32) -> Vec<(u32, u32)> {
    [
        (x, y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        (x.saturating_add(1), y),
        (x, y.saturating_add(1)),
    ]
    .into_iter()
    .filter(|(next_x, next_y)| *next_x < map.width && *next_y < map.height)
    .collect()
}

fn map_index(map: &TowerMapState, x: u32, y: u32) -> Option<usize> {
    (x < map.width && y < map.height).then_some((y * map.width + x) as usize)
}

fn coordinates(map: &TowerMapState, index: usize) -> (u32, u32) {
    (index as u32 % map.width, index as u32 / map.width)
}

fn place_room_landmark(map: &mut TowerMapState, mut object: TowerMapObject, rng: &mut TowerMapRng) {
    if map.rooms.len() <= 1 {
        return;
    }
    for _ in 0..80 {
        let room_index = rng.range(1, map.rooms.len() as u32) as usize;
        let room = map.rooms[room_index];
        if map.objects.iter().any(|existing| {
            existing.x >= room.start_x
                && existing.x < room.start_x + room.width
                && existing.y >= room.start_y
                && existing.y < room.start_y + room.height
        }) {
            continue;
        }
        let (x, y) = room.center();
        object.x = x;
        object.y = y;
        map.objects.push(object);
        map.set_room_kind(room_index, TowerRoomKind::Landmark);
        return;
    }
}

fn room_kind_for_object(kind: TowerMapObjectKind) -> TowerRoomKind {
    match kind {
        TowerMapObjectKind::Loot | TowerMapObjectKind::SecretCache => TowerRoomKind::Cache,
        TowerMapObjectKind::Egg => TowerRoomKind::Nest,
        TowerMapObjectKind::Enemy | TowerMapObjectKind::Boss => TowerRoomKind::Encounter,
        TowerMapObjectKind::Hazard => TowerRoomKind::Hazard,
        TowerMapObjectKind::SpecialLocation => TowerRoomKind::Landmark,
        TowerMapObjectKind::Stairs | TowerMapObjectKind::Exit => TowerRoomKind::Traversal,
    }
}

impl TowerMapObject {
    fn empty(kind: TowerMapObjectKind) -> Self {
        Self {
            kind,
            x: 0,
            y: 0,
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

    fn stairs(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            ..Self::empty(TowerMapObjectKind::Stairs)
        }
    }

    fn exit(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            ..Self::empty(TowerMapObjectKind::Exit)
        }
    }

    fn enemy(enemy_id: &str) -> Self {
        Self {
            enemy_id: enemy_id.to_owned(),
            ..Self::empty(TowerMapObjectKind::Enemy)
        }
    }

    fn boss(enemy_id: &str) -> Self {
        Self {
            enemy_id: enemy_id.to_owned(),
            ..Self::empty(TowerMapObjectKind::Boss)
        }
    }

    fn special_location(location_id: &str, event_id: &str) -> Self {
        Self {
            special_location_id: location_id.to_owned(),
            event_id: event_id.to_owned(),
            ..Self::empty(TowerMapObjectKind::SpecialLocation)
        }
    }

    fn hazard(hazard_id: &str) -> Self {
        Self {
            hazard_id: hazard_id.to_owned(),
            ..Self::empty(TowerMapObjectKind::Hazard)
        }
    }
}

#[cfg(test)]
mod tests;
