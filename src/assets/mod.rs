use std::cell::RefCell;
use std::collections::HashMap;

use macroquad::prelude::*;

use crate::data::{DungeonEnemyVisual, GameData, TowerHazardVisual, TowerLocationVisual};
use crate::state::{TowerMapObject, TowerMapObjectKind, TowerRunState};

const MONSTERS: &str = "monsters";
const EGGS: &str = "eggs";
const ENEMIES: &str = "enemies";
const LANDMARKS: &str = "landmarks";
const NPCS: &str = "npcs";
const ROOMS: &str = "rooms";
const VFX: &str = "vfx";
const DUNGEON_FEATURES: &str = "dungeon_features";
const DUNGEON_ENEMIES: &str = "dungeon_enemies";
const ROOM_MODULES: &str = "room_modules";
const BIOME_ROOM_VARIANTS: &str = "biome_room_variants";
const EXPEDITION_ROOM_VARIANTS: &str = "expedition_room_variants";
const PARTY_PORTRAITS: &str = "party_portraits";
const PURPOSE_ROOMS: &str = "purpose_rooms";
const MOSS_GATE_WORLD: &str = "moss_gate_world";
const SPECIAL_LOCATIONS: &str = "special_locations";
const HAZARDS: &str = "hazards";
const ESCALATION_LANDMARKS: &str = "escalation_landmarks";
const WANDERING_ENEMIES: &str = "wandering_enemies";
const ENEMY_INTENT_SILHOUETTES: &str = "enemy_intent_silhouettes";
const ESCAPE_CUES: &str = "escape_cues";
const ANOMALIES: &str = "anomalies";
const SECRETS: &str = "secrets";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DungeonBiome {
    Moss,
    Flooded,
    Ember,
    Frost,
    Root,
    Void,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DungeonRoomPurpose {
    Camp,
    Cache,
    Encounter,
    Nest,
    Traversal,
    Shrine,
}

impl DungeonBiome {
    pub fn for_floor(floor: u32) -> Self {
        match floor {
            4 | 7 => Self::Ember,
            5 | 8 => Self::Flooded,
            6 | 9 => Self::Frost,
            10 => Self::Void,
            3 => Self::Root,
            _ => Self::Moss,
        }
    }

    fn atlas_index(self) -> usize {
        match self {
            Self::Moss => 0,
            Self::Flooded => 1,
            Self::Ember => 2,
            Self::Frost => 3,
            Self::Root => 4,
            Self::Void => 5,
        }
    }
}

impl DungeonRoomPurpose {
    fn atlas_index(self) -> usize {
        match self {
            Self::Camp => 0,
            Self::Cache => 1,
            Self::Encounter => 2,
            Self::Nest => 3,
            Self::Traversal => 4,
            Self::Shrine => 5,
        }
    }
}

thread_local! {
    static TEXTURES: RefCell<HashMap<&'static str, Texture2D>> = RefCell::new(HashMap::new());
}

/// Draws a canonical companion portrait. The six slots are deliberately stable:
/// Slime, Rootling, Glowmoth, Pebblepup, Emberkit, and Rillfin.
pub fn draw_monster_badge(species_id: &str, x: f32, y: f32, size: f32) {
    let index = match species_id {
        "slime" => 0,
        "rillfin" => 1,
        "rootling" => 2,
        "emberkit" => 3,
        "pebblepup" => 4,
        "glowmoth" => 5,
        _ => 0,
    };
    draw_atlas(MONSTERS, 2, 3, index, x, y, size, size);
}

pub fn draw_monster_sprite(species_id: &str, x: f32, y: f32, size: f32) {
    draw_monster_badge(species_id, x, y, size);
}

/// Draws one of the twelve named egg designs, rather than a seed-coloured blob.
pub fn draw_egg_badge(egg_type_id: &str, x: f32, y: f32, size: f32) {
    let index = match egg_type_id {
        "mossy_egg" => 0,
        "glimmer_egg" => 1,
        "pebble_egg" => 2,
        "ember_egg" => 3,
        "ripple_egg" => 4,
        "rootbound_egg" => 5,
        "lantern_egg" => 6,
        "garden_egg" => 7,
        "ore_veined_egg" => 8,
        "boss_egg" => 9,
        "moonlit_egg" => 10,
        "sunken_egg" => 11,
        _ => 0,
    };
    draw_atlas(EGGS, 4, 3, index, x, y, size, size);
}

pub fn draw_enemy_badge_visual(visual: DungeonEnemyVisual, x: f32, y: f32, size: f32) {
    draw_atlas(ENEMIES, 2, 3, enemy_visual_index(visual), x, y, size, size);
}

pub fn draw_landmark(index: usize, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(LANDMARKS, 2, 3, index % 6, x, y, width, height);
}

pub fn draw_npc(npc_id: &str, x: f32, y: f32, size: f32) {
    let index = match npc_id {
        "mara" => 0,
        "bram" => 1,
        "lio" => 2,
        _ => 0,
    };
    draw_atlas(NPCS, 3, 2, index, x, y, size, size);
}

pub fn draw_room_vignette(floor: u32, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(
        ROOMS,
        2,
        3,
        (floor.saturating_sub(1) as usize / 2) % 6,
        x,
        y,
        width,
        height,
    );
}

pub fn draw_combat_vfx(index: usize, x: f32, y: f32, size: f32) {
    draw_atlas(VFX, 3, 2, index % 6, x, y, size, size);
}

/// Draws the tower's shared landmark vocabulary: nest, cache, stairs, crystal
/// threshold, recovery spring, and ancient machinery.
pub fn draw_dungeon_feature(index: usize, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(DUNGEON_FEATURES, 2, 3, index % 6, x, y, width, height);
}

pub fn draw_dungeon_enemy(index: usize, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(DUNGEON_ENEMIES, 3, 2, index % 6, x, y, width, height);
}

pub fn draw_dungeon_enemy_visual(
    visual: DungeonEnemyVisual,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    draw_dungeon_enemy(enemy_visual_index(visual), x, y, width, height);
}

pub fn draw_wandering_enemy_visual(
    visual: DungeonEnemyVisual,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let index = match visual {
        DungeonEnemyVisual::Crawler => 2,
        DungeonEnemyVisual::Winged => 3,
        DungeonEnemyVisual::Rooted => 0,
        DungeonEnemyVisual::Wisp => 1,
        DungeonEnemyVisual::Aquatic => 5,
        DungeonEnemyVisual::Armored => 4,
    };
    draw_atlas(WANDERING_ENEMIES, 2, 3, index, x, y, width, height);
}

pub fn draw_enemy_intent_silhouette(
    visual: DungeonEnemyVisual,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    draw_atlas(
        ENEMY_INTENT_SILHOUETTES,
        3,
        2,
        enemy_visual_index(visual),
        x,
        y,
        width,
        height,
    );
}

fn enemy_visual_index(visual: DungeonEnemyVisual) -> usize {
    match visual {
        DungeonEnemyVisual::Crawler => 0,
        DungeonEnemyVisual::Winged => 1,
        DungeonEnemyVisual::Rooted => 2,
        DungeonEnemyVisual::Wisp => 3,
        DungeonEnemyVisual::Aquatic => 4,
        DungeonEnemyVisual::Armored => 5,
    }
}

pub fn draw_special_location(visual: TowerLocationVisual, x: f32, y: f32, width: f32, height: f32) {
    draw_special_location_tinted(visual, x, y, width, height, WHITE);
}

pub fn draw_resolved_special_location(
    visual: TowerLocationVisual,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    draw_special_location_tinted(
        visual,
        x,
        y,
        width,
        height,
        Color::new(0.42, 0.60, 0.55, 0.62),
    );
}

fn draw_special_location_tinted(
    visual: TowerLocationVisual,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    tint: Color,
) {
    let index = special_location_index(visual);
    draw_atlas_tinted(SPECIAL_LOCATIONS, 3, 2, index, x, y, width, height, tint);
}

fn special_location_index(visual: TowerLocationVisual) -> usize {
    match visual {
        TowerLocationVisual::Shrine => 0,
        TowerLocationVisual::RecoverySpring => 1,
        TowerLocationVisual::AncientMachinery => 2,
        TowerLocationVisual::RelicArchive => 3,
        TowerLocationVisual::SecretClue => 4,
        TowerLocationVisual::Hazard => 5,
    }
}

/// Resolves every actionable map object to the same authored art used by both
/// the world map and the context drawer. Keeping this binding here prevents a
/// map icon and its inspection card from silently choosing different assets.
pub fn draw_tower_map_object(
    data: &GameData,
    run: &TowerRunState,
    object: &TowerMapObject,
    sealed: bool,
    rect: Rect,
) {
    let (x, y, width, height) = (rect.x, rect.y, rect.w, rect.h);
    match object.kind {
        TowerMapObjectKind::Loot => draw_dungeon_feature(1, x, y, width, height),
        TowerMapObjectKind::SecretCache => draw_secret_discovery(
            (run.map.seed as usize + object.x as usize + object.y as usize) % 6,
            x,
            y,
            width,
            height,
        ),
        TowerMapObjectKind::Egg => draw_egg_badge(
            &object.egg_type_id,
            x + width * 0.08,
            y,
            height.min(width * 0.84),
        ),
        TowerMapObjectKind::Enemy | TowerMapObjectKind::Boss => {
            if let Some(enemy) = data.enemy(&object.enemy_id) {
                if object.wandering {
                    draw_wandering_enemy_visual(enemy.visual, x, y, width, height);
                } else {
                    draw_dungeon_enemy_visual(enemy.visual, x, y, width, height);
                }
            } else {
                draw_dungeon_enemy(0, x, y, width, height);
            }
        }
        TowerMapObjectKind::SpecialLocation => {
            if let Some(location) = data.tower_special_location(&object.special_location_id) {
                draw_special_location(location.visual, x, y, width, height);
            } else {
                draw_dungeon_feature(4, x, y, width, height);
            }
        }
        TowerMapObjectKind::Hazard => {
            if let Some(hazard) = data.tower_hazard(&object.hazard_id) {
                draw_tower_hazard(hazard.visual, x, y, width, height);
            } else {
                draw_dungeon_feature(4, x, y, width, height);
            }
        }
        TowerMapObjectKind::Stairs => draw_escalation_landmark(
            DungeonBiome::for_floor(run.current_floor),
            x,
            y,
            width,
            height,
        ),
        TowerMapObjectKind::Exit if sealed => draw_escalation_landmark(
            DungeonBiome::for_floor(run.current_floor),
            x,
            y,
            width,
            height,
        ),
        TowerMapObjectKind::Exit => draw_escape_cue(
            DungeonBiome::for_floor(run.current_floor),
            x,
            y,
            width,
            height,
        ),
    }
}

pub fn draw_tower_hazard(visual: TowerHazardVisual, x: f32, y: f32, width: f32, height: f32) {
    let index = match visual {
        TowerHazardVisual::Spores => 0,
        TowerHazardVisual::Brambles => 1,
        TowerHazardVisual::CinderVent => 2,
        TowerHazardVisual::Flood => 3,
        TowerHazardVisual::Frostfall => 4,
        TowerHazardVisual::CrownPollen => 5,
    };
    draw_atlas(HAZARDS, 3, 2, index, x, y, width, height);
}

pub fn draw_escalation_landmark(biome: DungeonBiome, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(
        ESCALATION_LANDMARKS,
        2,
        3,
        biome.atlas_index(),
        x,
        y,
        width,
        height,
    );
}

pub fn draw_escape_cue(biome: DungeonBiome, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(ESCAPE_CUES, 2, 3, biome.atlas_index(), x, y, width, height);
}

pub fn draw_tower_anomaly(index: usize, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(ANOMALIES, 2, 3, index % 6, x, y, width, height);
}

pub fn draw_secret_discovery(index: usize, x: f32, y: f32, width: f32, height: f32) {
    draw_atlas(SECRETS, 3, 2, index % 6, x, y, width, height);
}

/// Draws a large illustrated room module. The atlas is arranged as a 3x2
/// family: moss, flooded, ember, frost, root, and void.
pub fn draw_dungeon_room(
    biome: DungeonBiome,
    purpose: DungeonRoomPurpose,
    variant: usize,
    rect: Rect,
    tint: Color,
) {
    let (x, y, width, height) = (rect.x, rect.y, rect.w, rect.h);
    let base_asset = match variant % 3 {
        0 => ROOM_MODULES,
        1 => BIOME_ROOM_VARIANTS,
        _ => EXPEDITION_ROOM_VARIANTS,
    };
    draw_atlas_tinted(
        base_asset,
        3,
        2,
        biome.atlas_index(),
        x,
        y,
        width,
        height,
        tint,
    );
    draw_atlas_tinted(
        PURPOSE_ROOMS,
        3,
        2,
        purpose.atlas_index(),
        x,
        y,
        width,
        height,
        Color::new(tint.r, tint.g, tint.b, tint.a * 0.25),
    );
}

pub fn draw_moss_gate_world(x: f32, y: f32, width: f32, height: f32) {
    let texture = texture(MOSS_GATE_WORLD);
    draw_texture_ex(
        &texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(width, height)),
            ..Default::default()
        },
    );
}

pub fn draw_party_portrait(species_id: &str, x: f32, y: f32, size: f32) {
    let index = match species_id {
        "slime" => 0,
        "rillfin" => 1,
        "emberkit" => 2,
        "rootling" => 3,
        "glowmoth" => 4,
        "pebblepup" => 5,
        _ => 0,
    };
    draw_atlas(PARTY_PORTRAITS, 3, 2, index, x, y, size, size);
}

#[allow(clippy::too_many_arguments)]
fn draw_atlas(
    asset: &'static str,
    columns: usize,
    rows: usize,
    index: usize,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    draw_atlas_tinted(asset, columns, rows, index, x, y, width, height, WHITE);
}

#[allow(clippy::too_many_arguments)]
fn draw_atlas_tinted(
    asset: &'static str,
    columns: usize,
    rows: usize,
    index: usize,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    tint: Color,
) {
    let texture = texture(asset);
    let cell_w = texture.width() / columns as f32;
    let cell_h = texture.height() / rows as f32;
    let column = index % columns;
    let row = index / columns;
    draw_texture_ex(
        &texture,
        x,
        y,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(width, height)),
            source: Some(Rect::new(
                column as f32 * cell_w,
                row as f32 * cell_h,
                cell_w,
                cell_h,
            )),
            ..Default::default()
        },
    );
}

fn texture(asset: &'static str) -> Texture2D {
    TEXTURES.with(|textures| {
        let mut textures = textures.borrow_mut();
        textures
            .entry(asset)
            .or_insert_with(|| Texture2D::from_file_with_format(asset_bytes(asset), None))
            .clone()
    })
}

fn asset_bytes(asset: &str) -> &'static [u8] {
    match asset {
        MONSTERS => {
            include_bytes!("../../assets/generated/monster_art/monster_sprite_atlas_v1.png")
        }
        EGGS => include_bytes!("../../assets/generated/monster_art/canonical_egg_atlas_v1.png"),
        ENEMIES => {
            include_bytes!("../../assets/generated/monster_art/enemy_boss_sprite_v2_atlas.png")
        }
        LANDMARKS => {
            include_bytes!("../../assets/generated/town/town_facility_landmarks_v4_atlas.png")
        }
        NPCS => include_bytes!("../../assets/generated/town/town_service_npcs_v5_atlas.png"),
        ROOMS => include_bytes!(
            "../../assets/generated/dungeon/dungeon_deep_room_vignettes_v5_atlas.png"
        ),
        VFX => include_bytes!("../../assets/generated/combat/combat_vfx_atlas_v1.png"),
        DUNGEON_FEATURES => {
            include_bytes!("../../assets/generated/dungeon/dungeon_sprite_atlas_v1.png")
        }
        DUNGEON_ENEMIES => {
            include_bytes!("../../assets/generated/dungeon/dungeon_enemy_atlas_v1.png")
        }
        ROOM_MODULES => include_bytes!(
            "../../assets/generated/dungeon/dungeon_biome_room_module_v2_atlas_v1.png"
        ),
        BIOME_ROOM_VARIANTS => {
            include_bytes!("../../assets/generated/dungeon/dungeon_biome_room_atlas_v1.png")
        }
        EXPEDITION_ROOM_VARIANTS => include_bytes!(
            "../../assets/generated/dungeon/dungeon_room_module_expedition_space_v2_atlas_v1.png"
        ),
        PARTY_PORTRAITS => include_bytes!(
            "../../assets/generated/monster_art/monster_party_context_portrait_v3_atlas_v1.png"
        ),
        PURPOSE_ROOMS => {
            include_bytes!("../../assets/generated/dungeon/dungeon_room_module_atlas_v1.png")
        }
        MOSS_GATE_WORLD => {
            include_bytes!("../../assets/generated/dungeon/moss_gate_world_plate_v1.png")
        }
        SPECIAL_LOCATIONS => include_bytes!(
            "../../assets/generated/dungeon/dungeon_interaction_landmarks_v5_atlas.png"
        ),
        HAZARDS => {
            include_bytes!("../../assets/generated/dungeon/dungeon_hazard_telegraphs_v5_atlas.png")
        }
        ESCALATION_LANDMARKS => include_bytes!(
            "../../assets/generated/dungeon/dungeon_escalation_landmark_v2_atlas.png"
        ),
        WANDERING_ENEMIES => include_bytes!(
            "../../assets/generated/dungeon/dungeon_enemy_intent_wandering_atlas_v1.png"
        ),
        ENEMY_INTENT_SILHOUETTES => include_bytes!(
            "../../assets/generated/dungeon/dungeon_enemy_intent_silhouette_v2_atlas_v1.png"
        ),
        ESCAPE_CUES => include_bytes!(
            "../../assets/generated/dungeon/dungeon_escalation_escape_cue_atlas_v1.png"
        ),
        ANOMALIES => include_bytes!(
            "../../assets/generated/dungeon/dungeon_weather_discovery_v3_overlay.png"
        ),
        SECRETS => include_bytes!(
            "../../assets/generated/dungeon/dungeon_secret_discovery_v2_atlas_v1.png"
        ),
        _ => unreachable!("unknown visual asset"),
    }
}

#[cfg(test)]
mod tests;
