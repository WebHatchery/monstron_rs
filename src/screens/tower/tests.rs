use super::*;

#[test]
fn expedition_status_wraps_to_two_bounded_lines() {
    let text = "The party enters floor 4 under a crystal bloom and follows a winding route toward a distant landmark before something moves in the dark beyond it.";
    let lines = wrap_status_lines(text, 42);

    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(|line| line.chars().count() <= 42));
    assert!(lines[1].ends_with('…'));
}

#[test]
fn short_expedition_status_stays_on_one_line() {
    assert_eq!(
        wrap_status_lines("The party reaches the focused room.", 84),
        ["The party reaches the focused room."]
    );
}
