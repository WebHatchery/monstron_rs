use super::*;

#[test]
fn roster_pages_expose_every_stable_slot_and_clamp_stale_pages() {
    assert_eq!(roster_page_count(0), 1);
    assert_eq!(roster_page_count(8), 1);
    assert_eq!(roster_page_count(12), 2);
    assert_eq!(roster_page_bounds(12, 0), 0..8);
    assert_eq!(roster_page_bounds(12, 1), 8..12);
    assert_eq!(roster_page_bounds(3, 99), 0..3);
}

#[test]
fn level_progress_names_the_next_level_threshold() {
    assert_eq!(level_progress_label(1, 7), "Lv 1 · XP 7/20");
    assert_eq!(level_progress_label(6, 45), "Lv 6 · XP 45/120");
}
