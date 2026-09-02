use super::*;

#[test]
fn floor_selection_stays_inside_the_unlocked_range() {
    assert_eq!(normalize_floor_selection(0, 6), 1);
    assert_eq!(normalize_floor_selection(4, 6), 4);
    assert_eq!(normalize_floor_selection(9, 6), 6);
    assert_eq!(normalize_floor_selection(3, 0), 1);
}
