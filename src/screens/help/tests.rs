use super::*;

#[test]
fn save_paths_wrap_without_losing_unicode_characters() {
    let path = "C:\\Users\\Kalai\\AppData\\Local\\hatchspire\\存档\\save_slot_1.json";
    let lines = wrapped_lines(path, 12);
    assert!(lines.iter().all(|line| line.chars().count() <= 12));
    assert_eq!(lines.concat(), path);
}

#[test]
fn summary_paths_wrap_without_losing_spaces() {
    let path = "C:\\Users\\Tester Name\\AppData\\Local\\hatchspire\\tester_summary.txt";
    let lines = wrapped_lines(path, 18);
    assert!(lines.iter().all(|line| line.chars().count() <= 18));
    assert_eq!(lines.concat(), path);
}
