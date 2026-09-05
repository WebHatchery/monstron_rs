use super::*;

#[test]
fn legacy_rng_keeps_seed_sequence_and_empty_range_behavior() {
    let mut rng = TowerMapRng::new(42);
    assert_eq!(rng.range(9, 9), 9);
    assert!(rng.chance(0, 0));
    // Neither operation above consumes a draw.
    assert_eq!(rng.next_u32(), 2_440_530_669);
    assert_eq!(rng.next_u32(), 968_358_053);
    assert_eq!(rng.range(0, 1), 0);
    assert_eq!(rng.next_u32(), 2_707_539_007);
    let mut zero = TowerMapRng::new(0);
    assert_eq!(zero.next_u32(), 1_817_669_548);
}
