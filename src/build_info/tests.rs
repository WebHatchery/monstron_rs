use super::*;

#[test]
fn build_id_keeps_the_version_and_git_identity_together() {
    assert!(BUILD_ID.starts_with(&format!("{VERSION}+g")));
    let revision = BUILD_ID
        .trim_end_matches("-dirty")
        .strip_prefix(&format!("{VERSION}+g"))
        .expect("build id should start with the package version");
    assert!(revision == "unknown" || revision.len() == 12);
    assert!(
        revision
            .chars()
            .all(|character| character.is_ascii_hexdigit())
            || revision == "unknown"
    );
}
