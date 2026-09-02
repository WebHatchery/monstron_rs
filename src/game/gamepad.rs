//! Game-level controller polling and automatic visible-button focus.

use macroquad_toolkit::input::GamepadFrame;

use super::Game;
use crate::screens::{tutorial, AppScreen};
use crate::ui;

impl Game {
    pub(super) fn prepare_gamepad(&mut self) {
        let frame = self.gamepad.capture();
        if frame.connected {
            self.gamepad_active |= frame_has_activity(frame);
        } else {
            self.gamepad_active = false;
        }
        if self.gamepad_screen != self.screen {
            self.gamepad_screen = self.screen;
            self.gamepad_focus = 0;
        }

        let targets = ui::controller_targets();
        if targets.is_empty() {
            self.gamepad_focus = 0;
        } else {
            self.gamepad_focus = self.gamepad_focus.min(targets.len() - 1);
            let direct_movement = self.screen == AppScreen::Tower
                && !self.tower_guide_open
                && self.state.as_ref().is_some_and(|state| {
                    state
                        .tower_run
                        .as_ref()
                        .is_some_and(|run| run.pending_event.is_none())
                });
            self.gamepad_focus = moved_focus(
                self.gamepad_focus,
                targets.len(),
                navigation_delta(frame, direct_movement),
            );
        }

        let tutorial_step = self
            .stable_rehome_pending
            .is_none()
            .then(|| {
                self.state.as_ref().and_then(|state| {
                    tutorial::current_step(state, self.screen, self.town_menu_open)
                })
            })
            .flatten();
        ui::set_controller_input(
            targets.get(self.gamepad_focus).copied(),
            frame,
            self.gamepad_active,
            tutorial_allows_cancel(tutorial_step),
        );
    }
}

fn frame_has_activity(frame: GamepadFrame) -> bool {
    frame.confirm
        || frame.cancel
        || frame.secondary
        || frame.tertiary
        || frame.menu
        || frame.next
        || frame.previous
        || frame.up
        || frame.down
        || frame.left
        || frame.right
}

fn navigation_delta(frame: GamepadFrame, direct_movement: bool) -> i32 {
    let previous = frame.previous || (!direct_movement && (frame.up || frame.left));
    let next = frame.next || (!direct_movement && (frame.down || frame.right));
    (next as i32) - (previous as i32)
}

fn moved_focus(current: usize, len: usize, delta: i32) -> usize {
    if len == 0 || delta == 0 {
        return current.min(len.saturating_sub(1));
    }
    if delta < 0 {
        current.checked_sub(1).unwrap_or(len - 1)
    } else {
        (current + 1) % len
    }
}

fn tutorial_allows_cancel(step: Option<tutorial::TutorialStep>) -> bool {
    step.is_none_or(|step| {
        matches!(
            step,
            tutorial::TutorialStep::LeaveHatchery
                | tutorial::TutorialStep::Retreat
                | tutorial::TutorialStep::LeaveEggHatchery
        )
    })
}

#[cfg(test)]
mod tests;
