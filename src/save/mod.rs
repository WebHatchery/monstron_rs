use serde::{Deserialize, Serialize};

use crate::state::GameState;

const GAME_NAME: &str = "hatchspire";
const SAVE_SLOT: &str = "slot_1";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaveData {
    pub version: u32,
    pub state: GameState,
}

pub struct SaveRepository;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SaveCompatibility {
    Current,
    Older,
    Newer,
}

pub fn compatibility(save_version: u32, supported_version: u32) -> SaveCompatibility {
    match save_version.cmp(&supported_version) {
        std::cmp::Ordering::Less => SaveCompatibility::Older,
        std::cmp::Ordering::Equal => SaveCompatibility::Current,
        std::cmp::Ordering::Greater => SaveCompatibility::Newer,
    }
}

impl SaveRepository {
    pub fn save(save_data: &SaveData) -> Result<(), String> {
        macroquad_toolkit::persistence::save_to_slot_with_version_and_backup(
            GAME_NAME,
            SAVE_SLOT,
            save_data,
            env!("CARGO_PKG_VERSION"),
        )
    }

    pub fn rewrite_loaded(save_data: &SaveData) -> Result<(), String> {
        macroquad_toolkit::persistence::save_to_slot_with_version(
            GAME_NAME,
            SAVE_SLOT,
            save_data,
            env!("CARGO_PKG_VERSION"),
        )
    }

    pub fn load() -> Result<SaveData, String> {
        macroquad_toolkit::persistence::load_from_slot(GAME_NAME, SAVE_SLOT)
    }

    pub fn exists() -> bool {
        macroquad_toolkit::persistence::slot_exists(GAME_NAME, SAVE_SLOT)
    }

    pub fn quarantine() -> Result<String, String> {
        macroquad_toolkit::persistence::quarantine_slot(GAME_NAME, SAVE_SLOT)
    }

    pub fn backup_exists() -> bool {
        macroquad_toolkit::persistence::slot_backup_exists(GAME_NAME, SAVE_SLOT)
    }

    pub fn restore_backup() -> Result<Option<String>, String> {
        macroquad_toolkit::persistence::restore_slot_backup(GAME_NAME, SAVE_SLOT)
    }

    pub fn delete() -> Result<(), String> {
        for slot in [
            SAVE_SLOT.to_owned(),
            format!("{SAVE_SLOT}_backup"),
            format!("{SAVE_SLOT}_before_restore"),
        ] {
            macroquad_toolkit::persistence::delete_slot(GAME_NAME, &slot)?;
        }
        Ok(())
    }

    pub fn location_description() -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            macroquad_toolkit::persistence::get_app_data_path(GAME_NAME, "save_slot_1.json")
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "Windows could not determine the save-data folder.".to_owned())
        }

        #[cfg(target_arch = "wasm32")]
        {
            "Browser storage for this site (development build only).".to_owned()
        }
    }
}

#[cfg(test)]
mod tests;
