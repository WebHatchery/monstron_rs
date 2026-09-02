use super::*;

#[test]
fn guardian_defeats_are_sorted_and_recorded_once() {
    let mut progress = TowerProgress {
        best_floor: 1,
        unlocked_floor: 1,
        defeated_guardian_floors: Vec::new(),
    };

    assert!(progress.record_guardian_defeat(10));
    assert!(progress.record_guardian_defeat(5));
    assert!(!progress.record_guardian_defeat(10));
    assert_eq!(progress.defeated_guardian_floors, vec![5, 10]);
    assert!(progress.guardian_defeated(5));
    assert!(!progress.guardian_defeated(4));
}

#[test]
fn field_guide_records_unlock_two_survey_preparation_ranks() {
    let mut discoveries = TowerDiscoveryState::default();
    assert_eq!(discoveries.record_count(), 0);
    assert_eq!(discoveries.survey_bonus(), 0);

    for index in 0..12 {
        discoveries.discover_enemy(&format!("enemy_{index}"));
    }
    assert_eq!(discoveries.record_count(), 12);
    assert_eq!(discoveries.survey_bonus(), 1);

    for index in 0..12 {
        discoveries.discover_special_location(&format!("landmark_{index}"));
    }
    for index in 0..6 {
        discoveries.discover_hazard(&format!("hazard_{index}"));
    }
    assert_eq!(discoveries.record_count(), 30);
    assert_eq!(discoveries.survey_bonus(), 2);

    assert!(!discoveries.discover_enemy("enemy_0"));
    assert_eq!(discoveries.record_count(), 30);

    assert!(discoveries.discover_event("keepers_blessing"));
    assert!(!discoveries.discover_event("keepers_blessing"));
    assert_eq!(discoveries.record_count(), 30);
}

#[test]
fn room_art_variants_are_bounded_and_restore_for_legacy_maps() {
    let mut map = TowerMapState::new(12, 8, 4, 77);
    map.rooms.push(TowerRoom {
        start_x: 1,
        start_y: 1,
        width: 5,
        height: 4,
    });
    map.rooms.push(TowerRoom {
        start_x: 7,
        start_y: 2,
        width: 4,
        height: 4,
    });

    assert!(map.ensure_room_art_variants());
    map.set_room_art_variant(0, 8);
    map.set_room_art_variant(1, 4);

    assert_eq!(map.room_art_variant(0), 2);
    assert_eq!(map.room_art_variant(1), 1);
    assert!(!map.ensure_room_art_variants());

    map.room_art_variants.clear();
    assert!(map.ensure_room_art_variants());
    assert_eq!(map.room_art_variant(1), 0);
}
