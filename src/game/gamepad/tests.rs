use super::*;

#[test]
fn focus_wraps_and_dungeon_movement_reserves_directions() {
    assert_eq!(moved_focus(0, 3, -1), 2);
    assert_eq!(moved_focus(2, 3, 1), 0);
    assert_eq!(moved_focus(1, 3, 0), 1);

    let frame = GamepadFrame {
        right: true,
        next: true,
        ..Default::default()
    };
    assert_eq!(navigation_delta(frame, true), 1);
    assert_eq!(navigation_delta(frame, false), 1);
    assert_eq!(
        navigation_delta(
            GamepadFrame {
                right: true,
                ..Default::default()
            },
            true
        ),
        0
    );
}

#[test]
fn tutorial_cancel_is_available_only_for_visible_back_actions() {
    assert!(tutorial_allows_cancel(None));
    assert!(tutorial_allows_cancel(Some(
        tutorial::TutorialStep::Retreat
    )));
    assert!(!tutorial_allows_cancel(Some(
        tutorial::TutorialStep::Scavenge
    )));
}
