use super::*;

#[test]
fn new_game_is_immediate_only_without_existing_progress() {
    assert!(!new_game_requires_confirmation(false, false));
    assert!(new_game_requires_confirmation(true, false));
    assert!(new_game_requires_confirmation(false, true));
    assert!(new_game_requires_confirmation(true, true));
}
