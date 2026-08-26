use super::*;

#[test]
fn volume_adjustments_clamp_and_effective_groups_respect_master_and_mute() {
    let mut settings = AppSettings::default();
    settings.adjust_master(50);
    settings.adjust_music(-100);
    settings.adjust_sfx(-30);
    assert_eq!(settings.master_volume, 100);
    assert_eq!(settings.music_volume, 0);
    assert_eq!(settings.sfx_volume, 50);
    assert_eq!(settings.effective_music_volume(), 0.0);
    assert_eq!(settings.effective_sfx_volume(), 0.5);

    settings.muted = true;
    assert_eq!(settings.effective_sfx_volume(), 0.0);
}

#[test]
fn missing_new_fields_use_safe_defaults_and_out_of_range_values_normalize() {
    let mut settings: AppSettings = serde_json::from_str(r#"{"master_volume":255}"#).unwrap();
    settings.normalize();
    assert_eq!(settings.master_volume, 100);
    assert_eq!(settings.music_volume, 70);
    assert!(!settings.fullscreen);
    assert!(!settings.reduced_motion);
    assert_eq!(settings.ui_scale_percent, 100);
}

#[test]
fn ui_scale_cycles_supported_whole_canvas_sizes_and_normalizes_old_values() {
    let mut settings = AppSettings::default();
    assert_eq!(settings.window_dimensions(), (1280, 720));
    settings.cycle_ui_scale();
    assert_eq!(settings.window_dimensions(), (1408, 792));
    settings.cycle_ui_scale();
    assert_eq!(settings.window_dimensions(), (1600, 900));
    settings.cycle_ui_scale();
    assert_eq!(settings.window_dimensions(), (1152, 648));

    settings.ui_scale_percent = 117;
    settings.normalize();
    assert_eq!(settings.ui_scale_percent, 110);
}
