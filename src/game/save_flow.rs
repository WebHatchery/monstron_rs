use super::Game;
use crate::save::{SaveData, SaveRepository};
use crate::state::PlaytestMetrics;

#[cfg(test)]
mod tests;

impl Game {
    pub(super) fn apply_progression(&mut self, apply: impl FnOnce(&mut Self)) {
        let before_state = self.state.clone();
        let before = self.progression_snapshot();
        apply(self);
        let after_before_metrics = self.progression_snapshot();
        if before != after_before_metrics {
            if let (Some(before_state), Some(after_state)) =
                (before_state.as_ref(), &mut self.state)
            {
                PlaytestMetrics::record_transition(before_state, after_state);
            }
        }
        self.autosave_if_changed(before);
    }

    pub(super) fn autosave_new_game(&mut self) {
        self.autosave_if_changed(None);
    }

    fn progression_snapshot(&self) -> Option<Vec<u8>> {
        self.state
            .as_ref()
            .and_then(|state| serde_json::to_vec(state).ok())
    }

    fn autosave_if_changed(&mut self, before: Option<Vec<u8>>) {
        if !self.autosave_enabled {
            return;
        }
        let after = self.progression_snapshot();
        if !should_autosave(before.as_deref(), after.as_deref()) {
            return;
        }

        let Some(state) = self.state.clone() else {
            return;
        };
        let save_data = SaveData {
            version: self.data.config.save_version,
            state,
        };
        let result = SaveRepository::save(&save_data);
        self.status_message = status_with_autosave(&self.status_message, result);
    }
}

fn should_autosave(before: Option<&[u8]>, after: Option<&[u8]>) -> bool {
    after.is_some() && before != after
}

fn status_with_autosave(summary: &str, result: Result<(), String>) -> String {
    match result {
        Ok(()) => format!("{summary}  [AUTOSAVED]"),
        Err(error) => format!("{summary}  [AUTOSAVE FAILED: {error}]"),
    }
}
