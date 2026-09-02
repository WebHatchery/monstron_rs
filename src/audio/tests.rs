use super::*;
use macroquad_toolkit::synth::{audit, render_unclamped};

#[test]
fn every_procedural_sound_is_audible_bounded_and_unclipped() {
    let config = SynthConfig::default();
    for id in AudioId::ALL {
        let rendered = render_unclamped(&voices(id), &config, id.seed());
        let measured = audit::measure(&rendered, config.sample_rate);
        assert!(measured.peak > 0.01, "{id:?} is silent: {measured:?}");
        assert!(measured.rms > 0.002, "{id:?} is inaudible: {measured:?}");
        assert_eq!(measured.clipped, 0, "{id:?} clips: {measured:?}");
        assert!(measured.seconds <= 12.01, "{id:?} is too long");
    }
}

#[test]
fn gameplay_changes_choose_their_distinct_audio_cues() {
    let base = AudioSnapshot {
        screen: AppScreen::Tower,
        status_signature: text_signature("Ready"),
        day: Some(1),
        tower_floor: Some(1),
        tower_position: Some((1, 1)),
        rooms_explored: Some(1),
        combat: None,
        crown_restored: false,
    };

    let mut after = base;
    after.tower_position = Some((2, 1));
    assert_eq!(cue_after(&base, &after), AudioId::Step);

    after.rooms_explored = Some(2);
    assert_eq!(cue_after(&base, &after), AudioId::Discover);

    after.tower_floor = Some(2);
    assert_eq!(cue_after(&base, &after), AudioId::Stair);

    after = base;
    after.screen = AppScreen::Combat;
    after.combat = Some(CombatMoment {
        round: 1,
        turn_index: 0,
        log_len: 1,
        outcome: None,
    });
    assert_eq!(cue_after(&base, &after), AudioId::Encounter);

    let combat = after;
    after.combat.as_mut().unwrap().outcome = Some(CombatOutcome::Victory);
    assert_eq!(cue_after(&combat, &after), AudioId::Victory);

    after = base;
    after.crown_restored = true;
    after.screen = AppScreen::Finale;
    assert_eq!(cue_after(&base, &after), AudioId::Finale);
}
