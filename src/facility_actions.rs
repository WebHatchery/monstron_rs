use crate::engine::{breeding_engine, egg_engine, job_engine, monster_engine, town_engine};
use crate::game::Game;
use crate::screens::{
    breeding::BreedingAction, hatchery::HatcheryAction, shop::ShopAction, stable::StableAction,
    tutorial, workshop::WorkshopAction, AppScreen,
};

impl Game {
    pub(crate) fn apply_hatchery_action(&mut self, action: HatcheryAction) {
        match action {
            HatcheryAction::ToTown => {
                if let Some(state) = &mut self.state {
                    tutorial::mark(state, tutorial::HATCHERY_VISITED);
                }
                self.screen = AppScreen::Town;
                self.status_message = "Returned to tower camp.".to_owned();
            }
            HatcheryAction::CareEgg(egg_id, care_focus) => {
                if let Some(state) = &mut self.state {
                    self.status_message =
                        egg_engine::care_for_egg(state, &self.data, egg_id, care_focus).summary;
                }
            }
            HatcheryAction::HatchEgg(egg_id) => {
                if let Some(state) = &mut self.state {
                    let eggs_before = state.egg_inventory.eggs.len();
                    self.status_message = egg_engine::hatch_egg(state, &self.data, egg_id).summary;
                    mark_tutorial_hatch(state, eggs_before);
                }
            }
        }
    }

    pub(crate) fn apply_stable_action(&mut self, action: StableAction) {
        match action {
            StableAction::ToTown => {
                self.screen = AppScreen::Town;
                self.status_message = "Returned to tower camp.".to_owned();
            }
            StableAction::ToggleParty(monster_id) => {
                if let Some(state) = &mut self.state {
                    self.status_message =
                        monster_engine::toggle_party_member(state, &self.data, monster_id).summary;
                }
            }
            StableAction::RemoveSlot(slot_index) => {
                if let Some(state) = &mut self.state {
                    self.status_message =
                        monster_engine::remove_party_slot(state, slot_index).summary;
                }
            }
        }
    }

    pub(crate) fn apply_breeding_action(&mut self, action: BreedingAction) {
        match action {
            BreedingAction::ToTown => {
                self.screen = AppScreen::Town;
                self.status_message = "Returned to tower camp.".to_owned();
            }
            BreedingAction::Breed(first_id, second_id) => {
                if let Some(state) = &mut self.state {
                    self.status_message =
                        breeding_engine::breed_pair(state, &self.data, first_id, second_id).summary;
                }
            }
        }
    }

    pub(crate) fn apply_workshop_action(&mut self, action: WorkshopAction) {
        match action {
            WorkshopAction::ToTown => {
                self.screen = AppScreen::Town;
                self.status_message = "Returned to tower camp.".to_owned();
            }
            WorkshopAction::Assign(monster_id, job) => {
                if let Some(state) = &mut self.state {
                    self.status_message =
                        job_engine::assign_job(state, &self.data, monster_id, job).summary;
                }
            }
            WorkshopAction::Clear(monster_id) => {
                if let Some(state) = &mut self.state {
                    self.status_message = job_engine::clear_job(state, monster_id).summary;
                }
            }
        }
    }

    pub(crate) fn apply_shop_action(&mut self, action: ShopAction) {
        match action {
            ShopAction::ToTown => {
                self.screen = AppScreen::Town;
                self.status_message = "Returned to tower camp.".to_owned();
            }
            ShopAction::Trade(trade) => {
                if let Some(state) = &mut self.state {
                    self.status_message = town_engine::reduce(
                        state,
                        &self.data,
                        &town_engine::TownCommand::Trade(trade),
                    )
                    .summary;
                }
            }
        }
    }
}

fn mark_tutorial_hatch(state: &mut crate::state::GameState, eggs_before: usize) {
    if state.egg_inventory.eggs.len() < eggs_before {
        tutorial::mark(state, tutorial::EGG_HATCHED);
    }
}

#[cfg(test)]
mod tests;
