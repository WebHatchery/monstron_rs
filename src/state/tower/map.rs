//! Generated tower map state and its deterministic random source.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum TowerTileKind {
    #[default]
    Wall,
    Floor,
    Corridor,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TowerMapObjectKind {
    Loot,
    SecretCache,
    Egg,
    Enemy,
    Boss,
    Hazard,
    SpecialLocation,
    Stairs,
    Exit,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum TowerRoomKind {
    #[default]
    Unknown,
    Camp,
    Nest,
    Cache,
    Encounter,
    Hazard,
    Landmark,
    Traversal,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TowerRoom {
    pub start_x: u32,
    pub start_y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TowerMapObject {
    pub kind: TowerMapObjectKind,
    pub x: u32,
    pub y: u32,
    #[serde(default)]
    pub resource_id: String,
    #[serde(default)]
    pub amount: i32,
    #[serde(default)]
    pub egg_type_id: String,
    #[serde(default)]
    pub hatch_days: u32,
    #[serde(default)]
    pub palette_seed: u64,
    #[serde(default)]
    pub enemy_id: String,
    #[serde(default)]
    pub special_location_id: String,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub hazard_id: String,
    #[serde(default)]
    pub wandering: bool,
    #[serde(default)]
    pub revealed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TowerMapState {
    pub floor: u32,
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub player_x: u32,
    pub player_y: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub tiles: Vec<TowerTileKind>,
    #[serde(default)]
    pub visibility: Vec<TowerTileVisibility>,
    pub rooms: Vec<TowerRoom>,
    #[serde(default)]
    pub room_kinds: Vec<TowerRoomKind>,
    #[serde(default)]
    pub room_art_variants: Vec<u8>,
    pub objects: Vec<TowerMapObject>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum TowerTileVisibility {
    #[default]
    Hidden,
    Explored,
    Visible,
}

impl TowerTileKind {
    pub fn is_passable(self) -> bool {
        matches!(self, Self::Floor | Self::Corridor)
    }
}

impl TowerRoom {
    pub fn center(self) -> (u32, u32) {
        (
            self.start_x + self.width / 2,
            self.start_y + self.height / 2,
        )
    }

    pub fn random_inner(self, rng: &mut TowerMapRng) -> (u32, u32) {
        let min_x = self.start_x + 1;
        let max_x = (self.start_x + self.width - 1).max(min_x + 1);
        let min_y = self.start_y + 1;
        let max_y = (self.start_y + self.height - 1).max(min_y + 1);
        (rng.range(min_x, max_x), rng.range(min_y, max_y))
    }

    pub fn intersects_padded(self, other: Self) -> bool {
        let left = self.start_x.saturating_sub(1);
        let right = self.start_x + self.width + 1;
        let top = self.start_y.saturating_sub(1);
        let bottom = self.start_y + self.height + 1;

        left <= other.start_x + other.width
            && right >= other.start_x
            && top <= other.start_y + other.height
            && bottom >= other.start_y
    }
}

impl TowerMapState {
    pub fn empty() -> Self {
        Self {
            floor: 0,
            width: 0,
            height: 0,
            seed: 0,
            player_x: 0,
            player_y: 0,
            start_x: 0,
            start_y: 0,
            tiles: Vec::new(),
            visibility: Vec::new(),
            rooms: Vec::new(),
            room_kinds: Vec::new(),
            room_art_variants: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn new(width: u32, height: u32, floor: u32, seed: u64) -> Self {
        Self {
            floor,
            width,
            height,
            seed,
            player_x: 0,
            player_y: 0,
            start_x: 0,
            start_y: 0,
            tiles: vec![TowerTileKind::Wall; (width * height) as usize],
            visibility: vec![TowerTileVisibility::Hidden; (width * height) as usize],
            rooms: Vec::new(),
            room_kinds: Vec::new(),
            room_art_variants: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0 || self.tiles.is_empty()
    }

    pub fn tile_at(&self, x: u32, y: u32) -> TowerTileKind {
        self.index(x, y)
            .and_then(|index| self.tiles.get(index).copied())
            .unwrap_or(TowerTileKind::Wall)
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: TowerTileKind) {
        if let Some(index) = self.index(x, y) {
            if let Some(slot) = self.tiles.get_mut(index) {
                *slot = tile;
            }
        }
    }

    pub fn is_passable(&self, x: u32, y: u32) -> bool {
        self.tile_at(x, y).is_passable()
    }

    pub fn ensure_visibility(&mut self) -> bool {
        let expected_len = (self.width * self.height) as usize;
        if expected_len == 0 || self.visibility.len() == expected_len {
            return false;
        }

        self.visibility = vec![TowerTileVisibility::Hidden; expected_len];
        true
    }

    pub fn ensure_room_kinds(&mut self) -> bool {
        if self.room_kinds.len() == self.rooms.len() {
            return false;
        }
        self.room_kinds
            .resize(self.rooms.len(), TowerRoomKind::Unknown);
        true
    }

    pub fn ensure_room_art_variants(&mut self) -> bool {
        if self.room_art_variants.len() == self.rooms.len() {
            return false;
        }
        self.room_art_variants.resize(self.rooms.len(), 0);
        true
    }

    pub fn room_art_variant(&self, index: usize) -> usize {
        self.room_art_variants.get(index).copied().unwrap_or(0) as usize
    }

    pub fn set_room_art_variant(&mut self, index: usize, variant: u8) {
        self.ensure_room_art_variants();
        if let Some(slot) = self.room_art_variants.get_mut(index) {
            *slot = variant % 3;
        }
    }

    pub fn room_kind(&self, index: usize) -> TowerRoomKind {
        self.room_kinds
            .get(index)
            .copied()
            .unwrap_or(TowerRoomKind::Unknown)
    }

    pub fn set_room_kind(&mut self, index: usize, kind: TowerRoomKind) {
        self.ensure_room_kinds();
        let Some(current) = self.room_kinds.get_mut(index) else {
            return;
        };
        if room_kind_priority(kind) >= room_kind_priority(*current) {
            *current = kind;
        }
    }

    pub fn visibility_at(&self, x: u32, y: u32) -> TowerTileVisibility {
        self.index(x, y)
            .and_then(|index| self.visibility.get(index).copied())
            .unwrap_or(TowerTileVisibility::Hidden)
    }

    pub fn set_visibility(&mut self, x: u32, y: u32, visibility: TowerTileVisibility) {
        if let Some(index) = self.index(x, y) {
            if let Some(slot) = self.visibility.get_mut(index) {
                *slot = visibility;
            }
        }
    }

    pub fn is_visible(&self, x: u32, y: u32) -> bool {
        self.visibility_at(x, y) == TowerTileVisibility::Visible
    }

    pub fn is_discovered(&self, x: u32, y: u32) -> bool {
        matches!(
            self.visibility_at(x, y),
            TowerTileVisibility::Explored | TowerTileVisibility::Visible
        )
    }

    pub fn object_index_at(&self, x: u32, y: u32) -> Option<usize> {
        self.objects
            .iter()
            .position(|object| object.x == x && object.y == y)
    }

    pub fn object_at(&self, x: u32, y: u32) -> Option<&TowerMapObject> {
        self.objects
            .iter()
            .find(|object| object.x == x && object.y == y)
    }

    fn index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }
}

impl Default for TowerMapState {
    fn default() -> Self {
        Self::empty()
    }
}

fn room_kind_priority(kind: TowerRoomKind) -> u8 {
    match kind {
        TowerRoomKind::Unknown => 0,
        TowerRoomKind::Traversal => 1,
        TowerRoomKind::Camp => 2,
        TowerRoomKind::Cache => 3,
        TowerRoomKind::Hazard => 4,
        TowerRoomKind::Nest => 5,
        TowerRoomKind::Encounter => 6,
        TowerRoomKind::Landmark => 7,
    }
}

#[derive(Clone, Debug)]
pub struct TowerMapRng {
    stream: macroquad_toolkit::rng::LegacyLcg64<1_442_695_040_888_963_407>,
}

impl TowerMapRng {
    pub fn new(seed: u64) -> Self {
        Self {
            stream: macroquad_toolkit::rng::LegacyLcg64::new(seed),
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.stream.next_u32()
    }

    pub fn range(&mut self, min: u32, max_exclusive: u32) -> u32 {
        if max_exclusive <= min {
            return min;
        }
        min + self.next_u32() % (max_exclusive - min)
    }

    pub fn chance(&mut self, numerator: u32, denominator: u32) -> bool {
        denominator == 0 || self.range(0, denominator) < numerator
    }
}

#[cfg(test)]
mod tests;
