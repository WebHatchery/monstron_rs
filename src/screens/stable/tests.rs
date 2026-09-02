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
