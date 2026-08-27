use serde_json::Value;
use std::fs;
use std::path::Path;

#[test]
fn catalog_metadata_is_explicitly_internal_and_has_no_inherited_repository_claim() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let raw =
        fs::read_to_string(root.join("game_page.json")).expect("game_page.json should be readable");
    let page: Value = serde_json::from_str(&raw).expect("game_page.json should be valid JSON");

    assert_eq!(page["status"]["text"], "Internal Preview");
    assert!(page.get("repository").is_none());
    assert!(!raw.contains("monstron_rs"));

    let details = page["details"]
        .as_array()
        .expect("catalog details should be an array");
    let platform = details
        .iter()
        .find(|detail| detail["label"] == "Platform")
        .expect("catalog details should name the platform");
    assert!(platform["value"]
        .as_str()
        .is_some_and(|value| value.contains("Windows 10/11 x64 target")));
    assert!(platform["value"]
        .as_str()
        .is_some_and(|value| value.contains("development-only")));

    let controls = page["controls"]
        .as_array()
        .expect("catalog controls should be an array");
    assert_eq!(controls[0]["key"], "Mouse / touch");
    assert!(controls
        .iter()
        .any(|control| control["key"] == "Tower actions"));
    assert!(controls.iter().any(|control| control["key"] == "Combat"));
}

#[test]
fn packaged_player_documents_match_current_local_data_and_known_issue_facts() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let read = |name: &str| {
        fs::read_to_string(root.join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
    };
    let player_readme = read("PLAYER_README.md");
    let support = read("SUPPORT.md");
    let known_issues = read("KNOWN_ISSUES.md");
    let privacy = read("PRIVACY.md");
    let packager = read("scripts/package_windows_preview.ps1");
    let normalized = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");

    for required in [
        "PLAYER_README.md",
        "SUPPORT.md",
        "KNOWN_ISSUES.md",
        "PRIVACY.md",
        "CREDITS.md",
        "THIRD_PARTY_NOTICES.md",
    ] {
        assert!(packager.contains(required), "packager omits {required}");
    }

    assert!(normalized(&player_readme).contains("EXPORT LOCAL SUMMARY"));
    assert!(support.contains("tester_summary.txt"));
    let privacy = normalized(&privacy);
    assert!(privacy.contains("saved local pacing/balance counters"));
    assert!(privacy.contains("only when the player taps"));
    assert!(privacy.contains("never uploaded automatically"));
    assert!(!known_issues.contains("chroma/magenta art defects"));
    assert!(known_issues.contains("final human visual review is pending"));
    assert!(normalized(&known_issues).contains("a custom icon derived from the title art"));
    assert!(normalized(&known_issues).contains("a code signature remain absent"));

    let notices = normalized(&read("THIRD_PARTY_NOTICES.md"));
    assert!(notices.contains("hash-verified copies from each exact upstream tag"));
    assert!(notices.contains("quad-rand 0.2.3"));
    assert!(packager.contains("gilrs 0.10.10"));
    assert!(packager.contains("gilrs-core 0.5.15"));
    assert!(packager.contains("supplementalLicenseHashes"));
}
