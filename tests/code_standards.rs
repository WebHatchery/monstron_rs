// The shared file-size gate from CODE_STANDARDS §2.2 — the 800-total-line hard
// limit on every Rust source file, including tests — enforced under `cargo test`.

#[test]
fn source_files_stay_under_the_limit() {
    macroquad_toolkit::source_gate::assert_source_files_within_limit(
        env!("CARGO_MANIFEST_DIR"),
        &[],
    );
}

#[test]
fn testing_guidance_has_no_fixed_per_feature_case_quota() {
    let standards = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("CODE_STANDARDS.md"),
    )
    .expect("CODE_STANDARDS.md should be readable");
    let testing_guidance = standards
        .split("## 11. Testing Guidelines")
        .nth(1)
        .expect("testing guidelines section should exist");

    assert!(testing_guidance.contains("no fixed case count per feature"));
    assert!(!testing_guidance.contains("more than five"));
}
