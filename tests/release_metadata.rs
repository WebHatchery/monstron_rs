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
