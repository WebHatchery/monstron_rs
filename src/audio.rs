//! Original procedural audio and state-driven playback cues.

use macroquad::audio::PlaySoundParams;
use macroquad_toolkit::audio::SoundManager;
use macroquad_toolkit::synth::{render_wav, SynthConfig, Voice, Wave};

use crate::screens::AppScreen;
use crate::settings::AppSettings;
use crate::state::{CombatOutcome, GameState};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum AudioId {
    Ui,
    Step,
    Discover,
    Stair,
    Encounter,
    Strike,
    Victory,
    Defeat,
    Rest,
    Finale,
    TownLoop,
    TowerLoop,
}

impl AudioId {
    const ALL: [Self; 12] = [
        Self::Ui,
        Self::Step,
        Self::Discover,
        Self::Stair,
        Self::Encounter,
        Self::Strike,
        Self::Victory,
        Self::Defeat,
        Self::Rest,
        Self::Finale,
        Self::TownLoop,
        Self::TowerLoop,
    ];

    fn seed(self) -> u64 {
        0x4841_5443_4853_5000 | self as u64
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Music {
    Town,
    Tower,
}

impl Music {
    fn audio_id(self) -> AudioId {
        match self {
            Self::Town => AudioId::TownLoop,
            Self::Tower => AudioId::TowerLoop,
        }
    }

    fn for_screen(screen: AppScreen) -> Self {
        match screen {
            AppScreen::Tower | AppScreen::Combat => Self::Tower,
            _ => Self::Town,
        }
    }
}

pub(crate) struct GameAudio {
    sounds: SoundManager<AudioId>,
    current_music: Option<Music>,
}

impl GameAudio {
    pub(crate) async fn load() -> (Self, Option<String>) {
        let config = SynthConfig::default();
        let mut sounds = SoundManager::new();
        let mut failures = Vec::new();
        for id in AudioId::ALL {
            let bytes = render_wav(&voices(id), &config, id.seed());
            if let Err(error) = sounds.load_sound_bytes(id, &bytes).await {
                failures.push(format!("{id:?}: {error}"));
            }
        }
        let error = (!failures.is_empty()).then(|| failures.join("; "));
        (
            Self {
                sounds,
                current_music: None,
            },
            error,
        )
    }

    pub(crate) fn respond(
        &mut self,
        before: &AudioSnapshot,
        after: &AudioSnapshot,
        settings: &AppSettings,
    ) {
        self.sounds.sfx_volume = settings.effective_sfx_volume();
        if before != after {
            self.sync_music(
                Music::for_screen(after.screen),
                settings.effective_music_volume(),
            );
            self.sounds.play_sfx(cue_after(before, after), 0.72);
        } else if let Some(music) = self.current_music {
            self.sounds
                .set_raw_volume(music.audio_id(), settings.effective_music_volume() * 0.34);
        }
    }

    fn sync_music(&mut self, wanted: Music, volume: f32) {
        if self.current_music == Some(wanted) {
            self.sounds.set_raw_volume(wanted.audio_id(), volume * 0.34);
            return;
        }
        if let Some(current) = self.current_music {
            self.sounds.stop_raw(current.audio_id());
        }
        self.sounds.play_raw(
            wanted.audio_id(),
            PlaySoundParams {
                looped: true,
                volume: volume * 0.34,
            },
        );
        self.current_music = Some(wanted);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AudioSnapshot {
    screen: AppScreen,
    status_signature: u64,
    day: Option<u32>,
    tower_floor: Option<u32>,
    tower_position: Option<(u32, u32)>,
    rooms_explored: Option<u32>,
    combat: Option<CombatMoment>,
    crown_restored: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CombatMoment {
    round: u32,
    turn_index: usize,
    log_len: usize,
    outcome: Option<CombatOutcome>,
}

impl AudioSnapshot {
    pub(crate) fn capture(screen: AppScreen, status: &str, state: Option<&GameState>) -> Self {
        let run = state.and_then(|state| state.tower_run.as_ref());
        let combat = state.and_then(|state| state.combat.as_ref());
        Self {
            screen,
            status_signature: text_signature(status),
            day: state.map(|state| state.day),
            tower_floor: run.map(|run| run.current_floor),
            tower_position: run.map(|run| (run.map.player_x, run.map.player_y)),
            rooms_explored: run.map(|run| run.rooms_explored),
            combat: combat.map(|combat| CombatMoment {
                round: combat.round,
                turn_index: combat.turn_index,
                log_len: combat.log.len(),
                outcome: combat.outcome,
            }),
            crown_restored: state
                .is_some_and(|state| state.story_flags.has("verdant_crown_restored")),
        }
    }
}

fn text_signature(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3)
    })
}

fn cue_after(before: &AudioSnapshot, after: &AudioSnapshot) -> AudioId {
    if (!before.crown_restored && after.crown_restored) || after.screen == AppScreen::Finale {
        return AudioId::Finale;
    }
    let old_outcome = before.combat.and_then(|combat| combat.outcome);
    let new_outcome = after.combat.and_then(|combat| combat.outcome);
    if old_outcome != new_outcome {
        return match new_outcome {
            Some(CombatOutcome::Victory) => AudioId::Victory,
            Some(CombatOutcome::Defeat | CombatOutcome::Fled) => AudioId::Defeat,
            None => AudioId::Ui,
        };
    }
    if before.screen != AppScreen::Combat && after.screen == AppScreen::Combat {
        return AudioId::Encounter;
    }
    if before.combat != after.combat && after.combat.is_some() {
        return AudioId::Strike;
    }
    if before.tower_floor != after.tower_floor && after.tower_floor.is_some() {
        return AudioId::Stair;
    }
    if before.rooms_explored < after.rooms_explored {
        return AudioId::Discover;
    }
    if before.tower_position != after.tower_position && after.tower_position.is_some() {
        return AudioId::Step;
    }
    if before.day < after.day {
        return AudioId::Rest;
    }
    AudioId::Ui
}

fn voices(id: AudioId) -> Vec<Voice> {
    match id {
        AudioId::Ui => vec![Voice::tone(0.0, 0.08, 760.0, 0.34)
            .glide(980.0)
            .wave(Wave::Triangle)],
        AudioId::Step => vec![
            Voice::tone(0.0, 0.055, 105.0, 0.3)
                .glide(72.0)
                .wave(Wave::Triangle),
            Voice::tone(0.0, 0.035, 900.0, 0.11).wave(Wave::Noise),
        ],
        AudioId::Discover => vec![
            Voice::tone(0.0, 0.16, 523.25, 0.34).wave(Wave::Triangle),
            Voice::tone(0.07, 0.22, 783.99, 0.3).wave(Wave::Triangle),
        ],
        AudioId::Stair => vec![
            Voice::tone(0.0, 0.18, 392.0, 0.32).wave(Wave::Triangle),
            Voice::tone(0.1, 0.24, 587.33, 0.32).wave(Wave::Triangle),
            Voice::tone(0.2, 0.3, 783.99, 0.28).wave(Wave::Triangle),
        ],
        AudioId::Encounter => vec![
            Voice::tone(0.0, 0.3, 170.0, 0.44)
                .glide(78.0)
                .wave(Wave::Square),
            Voice::tone(0.0, 0.12, 420.0, 0.2).wave(Wave::Noise),
        ],
        AudioId::Strike => vec![
            Voice::tone(0.0, 0.09, 260.0, 0.38)
                .glide(82.0)
                .wave(Wave::Triangle),
            Voice::tone(0.0, 0.055, 1100.0, 0.24).wave(Wave::Noise),
        ],
        AudioId::Victory => arpeggio(&[523.25, 659.25, 783.99, 1046.5], 0.11, 0.3),
        AudioId::Defeat => arpeggio(&[392.0, 311.13, 233.08], 0.14, 0.34),
        AudioId::Rest => vec![
            Voice::tone(0.0, 0.55, 261.63, 0.24)
                .wave(Wave::Sine)
                .attack(0.18),
            Voice::tone(0.04, 0.6, 392.0, 0.2)
                .wave(Wave::Sine)
                .attack(0.2),
        ],
        AudioId::Finale => arpeggio(&[261.63, 329.63, 392.0, 523.25, 659.25], 0.14, 0.55),
        AudioId::TownLoop => town_loop(),
        AudioId::TowerLoop => tower_loop(),
    }
}

fn arpeggio(frequencies: &[f32], spacing: f32, duration: f32) -> Vec<Voice> {
    frequencies
        .iter()
        .enumerate()
        .map(|(index, frequency)| {
            Voice::tone(index as f32 * spacing, duration, *frequency, 0.28)
                .wave(Wave::Triangle)
                .attack(0.08)
        })
        .collect()
}

fn town_loop() -> Vec<Voice> {
    let mut voices = Vec::new();
    for (index, frequency) in [261.63, 329.63, 392.0, 329.63, 293.66, 392.0]
        .iter()
        .enumerate()
    {
        voices.push(
            Voice::tone(index as f32 * 2.0, 1.65, *frequency, 0.16)
                .wave(Wave::Sine)
                .attack(0.28),
        );
    }
    voices.push(
        Voice::tone(10.0, 2.0, 130.81, 0.13)
            .wave(Wave::Triangle)
            .attack(0.3),
    );
    voices
}

fn tower_loop() -> Vec<Voice> {
    let mut voices = Vec::new();
    for (index, frequency) in [110.0, 130.81, 98.0, 146.83, 110.0, 82.41]
        .iter()
        .enumerate()
    {
        voices.push(
            Voice::tone(index as f32 * 2.0, 1.8, *frequency, 0.18)
                .wave(Wave::Triangle)
                .attack(0.34),
        );
    }
    voices.push(
        Voice::tone(10.0, 2.0, 329.63, 0.08)
            .wave(Wave::Sine)
            .attack(0.42),
    );
    voices
}

#[cfg(test)]
mod tests;
