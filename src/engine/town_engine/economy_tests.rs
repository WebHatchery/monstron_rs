use super::*;
use crate::data::GameDataLoader;

#[test]
fn two_gathers_fund_the_required_hatchery_and_stable_foundation() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);

    scavenge_supplies(&mut state);
    let hatchery = advance_building(&mut state, &data, "hatchery");
    assert!(hatchery.summary.starts_with("Built Hatchery"));
    scavenge_supplies(&mut state);
    let stable = advance_building(&mut state, &data, "stable");
    assert!(stable.summary.starts_with("Built Stable"));

    assert_eq!(state.town.building_level("hatchery"), 1);
    assert_eq!(state.town.building_level("stable"), 1);
    assert!(egg_capacity(&state) >= 3);
    assert!(monster_capacity(&state) >= 6);
}

#[test]
fn repeatable_town_income_can_complete_every_facility_without_a_dead_end() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    let mut gathers = 0;

    while data
        .buildings
        .iter()
        .any(|building| state.town.building_level(&building.id) < building.max_level)
    {
        let mut upgraded = false;
        for building in &data.buildings {
            let before = state.town.building_level(&building.id);
            if before < building.max_level {
                advance_building(&mut state, &data, &building.id);
                upgraded |= state.town.building_level(&building.id) > before;
            }
        }
        if !upgraded {
            scavenge_supplies(&mut state);
            gathers += 1;
            assert!(
                gathers <= 55,
                "facility economy requires excessive gathering"
            );
        }
    }

    assert!(data
        .buildings
        .iter()
        .all(|building| { state.town.building_level(&building.id) == building.max_level }));
}
