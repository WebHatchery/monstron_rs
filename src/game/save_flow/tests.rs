use super::*;

#[test]
fn autosave_notice_preserves_the_action_result_and_names_failures() {
    assert_eq!(
        status_with_autosave("Built Hatchery.", Ok(())),
        "Built Hatchery.  [AUTOSAVED]"
    );
    assert_eq!(
        status_with_autosave("Returned safely.", Err("disk full".to_owned())),
        "Returned safely.  [AUTOSAVE FAILED: disk full]"
    );
}

#[test]
fn autosave_runs_only_when_live_progress_changed() {
    assert!(!should_autosave(None, None));
    assert!(!should_autosave(Some(b"same"), Some(b"same")));
    assert!(should_autosave(None, Some(b"new camp")));
    assert!(should_autosave(Some(b"before"), Some(b"after")));
}
