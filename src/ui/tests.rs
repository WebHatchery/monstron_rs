use super::*;

#[test]
fn controller_registry_keeps_enabled_unique_targets_and_modal_scope() {
    let first = Rect::new(1.0, 2.0, 30.0, 40.0);
    let second = Rect::new(50.0, 2.0, 30.0, 40.0);
    begin_controller_registry();
    register_controller_target(first, true);
    register_controller_target(first, true);
    register_controller_target(second, false);
    assert_eq!(controller_targets(), vec![first]);

    begin_controller_modal();
    register_controller_target(second, true);
    assert_eq!(controller_targets(), vec![second]);
}

#[test]
fn controller_activation_respects_focus_and_cancel_suppression() {
    let focused = Rect::new(1.0, 2.0, 30.0, 40.0);
    let other = Rect::new(50.0, 2.0, 30.0, 40.0);
    let frame = GamepadFrame {
        connected: true,
        confirm: true,
        cancel: true,
        ..Default::default()
    };
    set_controller_input(Some(focused), frame, true, false);
    assert!(controller_activated(focused));
    assert!(!controller_activated(other));
    assert!(!controller_cancel_pressed());
}
