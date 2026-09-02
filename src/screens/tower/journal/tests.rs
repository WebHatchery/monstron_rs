use super::*;

#[test]
fn journal_excerpt_preserves_short_messages() {
    assert_eq!(journal_excerpt("Found a cache.", 28), "Found a cache.");
}

#[test]
fn journal_excerpt_stays_inside_its_character_budget() {
    let excerpt = journal_excerpt(
        "Floor 2 unlocked. Descended to Lantern Hollows under Veil Mist.",
        28,
    );

    assert_eq!(excerpt.chars().count(), 28);
    assert!(excerpt.ends_with('…'));
}
