use super::*;
use crate::data::GameDataLoader;
use crate::state::GameState;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;

#[cfg(not(target_arch = "wasm32"))]
static REPORT_ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn summary_is_explicitly_local_and_contains_agreed_pacing_and_balance_fields() {
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let mut state = GameState::new(&data);
    state.day = 7;
    state.playtest_metrics.progression_actions = 42;
    state.playtest_metrics.expeditions_started = 4;
    state.playtest_metrics.expeditions_returned = 3;
    state.playtest_metrics.combat_victories = 2;
    state.playtest_metrics.combat_defeats = 1;

    let summary = render(&state, &data, "0.1.0+gtest");

    assert!(summary.contains("HATCHSPIRE LOCAL TESTER SUMMARY"));
    assert!(summary.contains("did not transmit or upload it"));
    assert!(summary.contains("Build: 0.1.0+gtest"));
    assert!(summary.contains("Current day: 7"));
    assert!(summary.contains("Recorded progression actions: 42"));
    assert!(summary.contains("Expedition return rate: 75%"));
    assert!(summary.contains("Combat win rate: 66%"));
    assert!(summary.contains("CURRENT RESOURCES"));
    assert!(summary.contains("CURRENT FACILITIES"));
}

#[test]
fn rates_are_not_invented_before_any_recorded_attempts() {
    assert_eq!(percent(0, 0), "not available");
    assert_eq!(percent(1, 3), "33%");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn export_writes_only_to_the_explicit_local_test_path() {
    let _guard = REPORT_ENV_LOCK.lock().expect("report env lock");
    let data = GameDataLoader::load_embedded().expect("embedded data should load");
    let state = GameState::new(&data);
    let unique = format!(
        "hatchspire_tester_summary_{}_{}.txt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    );
    let path = std::env::temp_dir().join(unique);
    std::env::set_var(TEST_PATH_ENV, &path);

    let exported = export(&state, &data).expect("summary should export");
    let content = std::fs::read_to_string(&exported).expect("summary should be readable");

    std::env::remove_var(TEST_PATH_ENV);
    std::fs::remove_file(&path).expect("summary should be removable");
    assert_eq!(exported, path);
    assert!(content.contains("created only because the player tapped"));
}
