use macroquad_toolkit::ui::wrap_literal_text_with_measure;

fn measured_width(text: &str) -> f32 {
    text.chars()
        .map(|ch| if ch.is_ascii() { 7.0 } else { 14.0 })
        .sum()
}

#[test]
fn save_paths_wrap_without_losing_unicode_characters() {
    let path = "C:\\Users\\Kalai\\AppData\\Local\\hatchspire\\存档\\save_slot_1.json";
    let lines = wrap_literal_text_with_measure(path, 84.0, measured_width);
    assert!(lines.iter().all(|line| measured_width(line) <= 84.0));
    assert_eq!(lines.concat(), path);
}

#[test]
fn summary_paths_wrap_without_losing_spaces() {
    let path = "C:\\Users\\Tester  Name\\AppData\\Local\\hatchspire\\tester_summary.txt";
    let lines = wrap_literal_text_with_measure(path, 126.0, measured_width);
    assert!(lines.iter().all(|line| measured_width(line) <= 126.0));
    assert_eq!(lines.concat(), path);
}
