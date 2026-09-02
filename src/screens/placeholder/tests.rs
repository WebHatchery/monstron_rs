use super::*;

#[test]
fn floor_selection_stays_inside_the_unlocked_range() {
    assert_eq!(normalize_floor_selection(0, 6), 1);
    assert_eq!(normalize_floor_selection(4, 6), 4);
    assert_eq!(normalize_floor_selection(9, 6), 6);
    assert_eq!(normalize_floor_selection(3, 0), 1);
}

#[test]
fn expedition_goals_are_disabled_without_a_ready_party() {
    let unavailable = buttons(PlaceholderKind::DungeonPrep, 4, 7, false);
    assert!(unavailable
        .iter()
        .filter_map(|(action, _, enabled)| {
            matches!(action, PlaceholderAction::ToTower(_)).then_some(enabled)
        })
        .all(|enabled| !enabled));
    assert!(unavailable
        .iter()
        .any(|(action, _, enabled)| { *action == PlaceholderAction::ToTown && *enabled }));

    let available = buttons(PlaceholderKind::DungeonPrep, 4, 7, true);
    assert!(available
        .iter()
        .filter_map(|(action, _, enabled)| {
            matches!(action, PlaceholderAction::ToTower(_)).then_some(enabled)
        })
        .all(|enabled| *enabled));
}
