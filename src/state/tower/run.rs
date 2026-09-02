//! Serializable expedition state and run-owned cargo/event operations.

use serde::{Deserialize, Serialize};

use crate::data::{ResourceAmount, TowerBlessing};
use crate::state::ResourceStack;

use super::goal::{survey_charges_for, TowerRunGoal};
use super::map::TowerMapState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TowerProgress {
    pub best_floor: u32,
    pub unlocked_floor: u32,
    #[serde(default)]
    pub defeated_guardian_floors: Vec<u32>,
}

impl TowerProgress {
    pub fn guardian_defeated(&self, floor: u32) -> bool {
        self.defeated_guardian_floors.contains(&floor)
    }

    pub fn record_guardian_defeat(&mut self, floor: u32) -> bool {
        if self.guardian_defeated(floor) {
            false
        } else {
            self.defeated_guardian_floors.push(floor);
            self.defeated_guardian_floors.sort_unstable();
            true
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TowerRunState {
    pub current_floor: u32,
    pub rooms_explored: u32,
    pub pressure: u32,
    pub pressure_limit: u32,
    #[serde(default)]
    pub pressure_stage: u8,
    #[serde(default)]
    pub camp_cooldown: u32,
    #[serde(default = "default_survey_charges")]
    pub survey_charges: u32,
    #[serde(default)]
    pub goal: TowerRunGoal,
    #[serde(default)]
    pub map: TowerMapState,
    pub cargo: Vec<ResourceStack>,
    pub found_eggs: Vec<TowerFoundEgg>,
    pub event_log: Vec<String>,
    #[serde(default)]
    pub pending_event: Option<TowerPendingEvent>,
    #[serde(default)]
    pub completed_landmarks: Vec<TowerCompletedLandmark>,
    #[serde(default)]
    pub contract_id: String,
    #[serde(default)]
    pub contract_complete: bool,
    #[serde(default)]
    pub stats: TowerRunStats,
    #[serde(default)]
    pub blessings: Vec<TowerBlessing>,
    #[serde(default)]
    pub boss_defeated: bool,
    #[serde(default)]
    pub anomaly_id: String,
    #[serde(default)]
    pub route_target: Option<(u32, u32)>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TowerFoundEgg {
    pub egg_type_id: String,
    pub hatch_days: u32,
    pub origin_floor: u32,
    pub palette_seed: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TowerPendingEvent {
    pub special_location_id: String,
    pub event_ids: Vec<String>,
    #[serde(default)]
    pub x: u32,
    #[serde(default)]
    pub y: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TowerCompletedLandmark {
    pub special_location_id: String,
    pub event_id: String,
    pub x: u32,
    pub y: u32,
    #[serde(default)]
    pub changed_room: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TowerRunStats {
    pub floors_descended: u32,
    #[serde(default)]
    pub landmarks_visited: u32,
    pub landmarks_resolved: u32,
    pub hazards_countered: u32,
}

impl TowerRunState {
    pub fn new(current_floor: u32, pressure_limit: u32, goal: TowerRunGoal) -> Self {
        Self {
            current_floor,
            rooms_explored: 0,
            pressure: 0,
            pressure_limit,
            pressure_stage: 0,
            camp_cooldown: 0,
            survey_charges: survey_charges_for(goal),
            goal,
            map: TowerMapState::empty(),
            cargo: Vec::new(),
            found_eggs: Vec::new(),
            event_log: vec![format!("Entered floor {current_floor} on a {goal} run.")],
            pending_event: None,
            completed_landmarks: Vec::new(),
            contract_id: goal.contract_id().to_owned(),
            contract_complete: false,
            stats: TowerRunStats::default(),
            blessings: Vec::new(),
            boss_defeated: false,
            anomaly_id: String::new(),
            route_target: None,
        }
    }

    pub fn with_map(mut self, map: TowerMapState) -> Self {
        self.map = map;
        self
    }

    pub fn add_cargo(&mut self, resource_id: &str, amount: i32) {
        if let Some(stack) = self
            .cargo
            .iter_mut()
            .find(|stack| stack.resource_id == resource_id)
        {
            stack.amount += amount;
            return;
        }

        self.cargo.push(ResourceStack {
            resource_id: resource_id.to_owned(),
            amount,
        });
    }

    pub fn cargo_amount(&self) -> i32 {
        self.cargo.iter().map(|stack| stack.amount.max(0)).sum()
    }

    pub fn cargo_amount_for(&self, resource_id: &str) -> i32 {
        self.cargo
            .iter()
            .find(|stack| stack.resource_id == resource_id)
            .map_or(0, |stack| stack.amount.max(0))
    }

    pub fn can_afford_cargo(&self, costs: &[ResourceAmount]) -> bool {
        costs
            .iter()
            .all(|cost| self.cargo_amount_for(&cost.resource_id) >= cost.amount)
    }

    pub fn spend_cargo(&mut self, costs: &[ResourceAmount]) -> bool {
        if !self.can_afford_cargo(costs) {
            return false;
        }
        for cost in costs {
            if let Some(stack) = self
                .cargo
                .iter_mut()
                .find(|stack| stack.resource_id == cost.resource_id)
            {
                stack.amount -= cost.amount;
            }
        }
        self.cargo.retain(|stack| stack.amount > 0);
        true
    }

    pub fn add_event(&mut self, message: String) {
        self.event_log.push(message);
        if self.event_log.len() > 7 {
            let overflow = self.event_log.len() - 7;
            self.event_log.drain(0..overflow);
        }
    }

    pub fn has_blessing(&self, blessing: TowerBlessing) -> bool {
        self.blessings.contains(&blessing)
    }

    pub fn add_blessing(&mut self, blessing: TowerBlessing) -> bool {
        if self.has_blessing(blessing) {
            false
        } else {
            self.blessings.push(blessing);
            true
        }
    }

    pub fn consume_blessing(&mut self, blessing: TowerBlessing) -> bool {
        let Some(index) = self.blessings.iter().position(|active| *active == blessing) else {
            return false;
        };
        self.blessings.remove(index);
        true
    }
}

fn default_survey_charges() -> u32 {
    2
}
