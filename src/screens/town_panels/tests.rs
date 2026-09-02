use super::*;
use crate::data::GameDataLoader;

#[test]
fn campaign_objective_guides_every_story_threshold() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);

    assert_eq!(
        campaign_objective(&state, &data),
        CampaignObjective {
            heading: "NEXT · CLIMB THE TOWER".to_owned(),
            action: "Reach the open floor's stairs.".to_owned(),
        }
    );

    state.tower_progress.unlocked_floor = 5;
    assert_eq!(
        campaign_objective(&state, &data),
        CampaignObjective {
            heading: "NEXT · MIRROR MATRIARCH".to_owned(),
            action: "Defeat it, then reach the stairs.".to_owned(),
        }
    );

    state.tower_progress.unlocked_floor = 10;
    assert_eq!(
        campaign_objective(&state, &data),
        CampaignObjective {
            heading: "NEXT · VERDANT CROWN".to_owned(),
            action: "Defeat it and cross the threshold.".to_owned(),
        }
    );

    state.story_flags.add("verdant_crown_restored");
    assert_eq!(
        campaign_objective(&state, &data),
        CampaignObjective {
            heading: "CROWN RESTORED".to_owned(),
            action: "Free expeditions remain open.".to_owned(),
        }
    );
}
