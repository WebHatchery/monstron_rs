use super::*;

#[test]
fn combatant_cards_stay_below_the_side_headings_and_inside_formation() {
    for slot in 0..6 {
        for rect in [ally_slot_rect(slot), enemy_slot_rect(slot)] {
            assert!(rect.y >= 220.0);
            assert!(rect.y + rect.h <= 454.0);
        }
    }
}

#[test]
fn combatant_cards_do_not_overlap_within_either_side() {
    for slots in [
        (0..6).map(ally_slot_rect).collect::<Vec<_>>(),
        (0..6).map(enemy_slot_rect).collect::<Vec<_>>(),
    ] {
        for left in 0..slots.len() {
            for right in (left + 1)..slots.len() {
                assert!(!slots[left].overlaps(&slots[right]));
            }
        }
    }
}
