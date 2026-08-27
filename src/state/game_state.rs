use serde::{Deserialize, Serialize};

use crate::data::GameData;
use crate::state::{
    ActivityLog, CombatState, EggInventory, MonsterInstance, MonsterRoster, PlaytestMetrics,
    ResourceInventory, TowerDiscoveryState, TowerProgress, TowerRunState, TownState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameState {
    pub day: u32,
    pub resources: ResourceInventory,
    pub town: TownState,
    pub monster_roster: MonsterRoster,
    pub egg_inventory: EggInventory,
    pub tower_progress: TowerProgress,
    #[serde(default)]
    pub tower_discoveries: TowerDiscoveryState,
    #[serde(default)]
    pub tower_run: Option<TowerRunState>,
    #[serde(default)]
    pub combat: Option<CombatState>,
    pub npc_relationships: Vec<NpcRelationship>,
    pub story_flags: StoryFlags,
    pub activity_log: ActivityLog,
    #[serde(default)]
    pub playtest_metrics: PlaytestMetrics,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpcRelationship {
    pub npc_id: String,
    pub friendship: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoryFlags {
    pub flags: Vec<String>,
}

impl StoryFlags {
    pub fn has(&self, flag: &str) -> bool {
        self.flags.iter().any(|existing| existing == flag)
    }

    pub fn add(&mut self, flag: &str) -> bool {
        if self.has(flag) {
            false
        } else {
            self.flags.push(flag.to_owned());
            true
        }
    }
}

impl GameState {
    pub fn new(data: &GameData) -> Self {
        let mut activity_log = ActivityLog::new();
        for message in &data.config.starting_log {
            activity_log.add(data.config.starting_day, message.clone());
        }

        let starter_species = data
            .species(&data.config.starter_species_id)
            .expect("validated game data should contain the starter species");
        let starter = MonsterInstance::from_species(
            1,
            data.config.starter_name.clone(),
            starter_species,
            0x51A1_5EED,
        );

        let mut party_slots = vec![None; 6];
        party_slots[0] = Some(starter.id);

        Self {
            day: data.config.starting_day,
            resources: ResourceInventory::from_data(data),
            town: TownState::from_data(data),
            monster_roster: MonsterRoster {
                next_id: 2,
                monsters: vec![starter],
                party_slots,
            },
            egg_inventory: EggInventory {
                next_id: 1,
                eggs: Vec::new(),
            },
            tower_progress: TowerProgress {
                best_floor: 0,
                unlocked_floor: 1,
            },
            tower_discoveries: TowerDiscoveryState::default(),
            tower_run: None,
            combat: None,
            npc_relationships: data
                .npcs
                .iter()
                .map(|npc| NpcRelationship {
                    npc_id: npc.id.clone(),
                    friendship: 0,
                })
                .collect(),
            story_flags: StoryFlags {
                flags: vec!["arrived_at_tower_camp".to_owned()],
            },
            activity_log,
            playtest_metrics: PlaytestMetrics::default(),
        }
    }

    pub fn npc_friendship(&self, npc_id: &str) -> i32 {
        self.npc_relationships
            .iter()
            .find(|relationship| relationship.npc_id == npc_id)
            .map(|relationship| relationship.friendship)
            .unwrap_or(0)
    }

    pub fn npc_friendship_mut(&mut self, npc_id: &str) -> &mut i32 {
        if let Some(index) = self
            .npc_relationships
            .iter()
            .position(|relationship| relationship.npc_id == npc_id)
        {
            return &mut self.npc_relationships[index].friendship;
        }

        self.npc_relationships.push(NpcRelationship {
            npc_id: npc_id.to_owned(),
            friendship: 0,
        });
        let index = self.npc_relationships.len() - 1;
        &mut self.npc_relationships[index].friendship
    }
}
