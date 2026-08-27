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

#[test]
fn toolkit_build_id_names_the_compiled_shared_revision() {
    let revision = TOOLKIT_BUILD_ID
        .trim_end_matches("-dirty")
        .strip_prefix('g')
        .expect("toolkit build id should start with g");
    assert!(revision == "unknown" || revision.len() == 12);
    assert!(
        revision
            .chars()
            .all(|character| character.is_ascii_hexdigit())
            || revision == "unknown"
    );
}
