use serde::{Deserialize, Serialize};

const GAME_NAME: &str = "hatchspire";
const SETTINGS_KEY: &str = "settings";
const SETTINGS_PATH_ENV: &str = "HATCHSPIRE_SETTINGS_TEST_PATH";
const UI_SCALES: [u16; 4] = [90, 100, 110, 125];

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct AppSettings {
    pub master_volume: u8,
    pub music_volume: u8,
    pub sfx_volume: u8,
    pub muted: bool,
    pub fullscreen: bool,
    pub reduced_motion: bool,
    pub ui_scale_percent: u16,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            master_volume: 80,
            music_volume: 70,
            sfx_volume: 80,
            muted: false,
            fullscreen: false,
            reduced_motion: false,
            ui_scale_percent: 100,
        }
    }
}

impl AppSettings {
    pub fn load() -> Result<Self, String> {
        if !macroquad_toolkit::persistence::json_key_exists_configured(
            GAME_NAME,
            SETTINGS_KEY,
            Some(SETTINGS_PATH_ENV),
        ) {
            return Ok(Self::default());
        }
        let mut settings: Self = macroquad_toolkit::persistence::load_json_key_configured(
            GAME_NAME,
            SETTINGS_KEY,
            Some(SETTINGS_PATH_ENV),
        )?;
        settings.normalize();
        Ok(settings)
    }

    pub fn save(&self) -> Result<(), String> {
        macroquad_toolkit::persistence::save_json_key_configured(
            GAME_NAME,
            SETTINGS_KEY,
            self,
            Some(SETTINGS_PATH_ENV),
        )
    }

    pub fn adjust_master(&mut self, delta: i8) {
        self.master_volume = adjusted_volume(self.master_volume, delta);
    }

    pub fn adjust_music(&mut self, delta: i8) {
        self.music_volume = adjusted_volume(self.music_volume, delta);
    }

    pub fn adjust_sfx(&mut self, delta: i8) {
        self.sfx_volume = adjusted_volume(self.sfx_volume, delta);
    }

    pub fn effective_music_volume(&self) -> f32 {
        effective_volume(self.master_volume, self.music_volume, self.muted)
    }

    pub fn effective_sfx_volume(&self) -> f32 {
        effective_volume(self.master_volume, self.sfx_volume, self.muted)
    }

    pub fn cycle_ui_scale(&mut self) {
        let index = UI_SCALES
            .iter()
            .position(|scale| *scale == self.ui_scale_percent)
            .unwrap_or(1);
        self.ui_scale_percent = UI_SCALES[(index + 1) % UI_SCALES.len()];
    }

    pub fn window_dimensions(&self) -> (i32, i32) {
        let scale = i32::from(self.ui_scale_percent);
        (1280 * scale / 100, 720 * scale / 100)
    }

    fn normalize(&mut self) {
        self.master_volume = self.master_volume.min(100);
        self.music_volume = self.music_volume.min(100);
        self.sfx_volume = self.sfx_volume.min(100);
        self.ui_scale_percent = UI_SCALES
            .iter()
            .copied()
            .min_by_key(|scale| scale.abs_diff(self.ui_scale_percent))
            .unwrap_or(100);
    }
}

fn adjusted_volume(current: u8, delta: i8) -> u8 {
    i16::from(current)
        .saturating_add(i16::from(delta))
        .clamp(0, 100) as u8
}

fn effective_volume(master: u8, group: u8, muted: bool) -> f32 {
    if muted {
        0.0
    } else {
        f32::from(master) * f32::from(group) / 10_000.0
    }
}
