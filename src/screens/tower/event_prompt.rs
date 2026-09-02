use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

use super::TowerAction;
use crate::assets;
use crate::data::{GameData, TowerEventDefinition};
use crate::engine::tower_engine;
use crate::state::{GameState, TowerRunState};
use crate::ui;

pub(super) fn handle_input(
    state: &GameState,
    data: &GameData,
    run: &TowerRunState,
) -> Option<TowerAction> {
    let pending = run.pending_event.as_ref()?;
    for (index, event_id) in pending.event_ids.iter().take(2).enumerate() {
        let enabled = tower_engine::event_choice_available(state, data, event_id);
        if ui::button_clicked(choice_rect(index), enabled) {
            return Some(TowerAction::ChooseEvent(event_id.clone()));
        }
    }
    if ui::button_clicked(leave_rect(), true) {
        return Some(TowerAction::LeaveEvent);
    }
    None
}

pub(super) fn draw(state: &GameState, data: &GameData, run: &TowerRunState) {
    let Some(pending) = &run.pending_event else {
        return;
    };
    ui::begin_controller_modal();
    let location = data.tower_special_location(&pending.special_location_id);
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(1, 5, 7, 190),
    );
    let panel = Rect::new(248.0, 98.0, 784.0, 512.0);
    super::draw_overlay_panel(panel);
    draw_ui_text_ex(
        location.map_or("Tower Landmark", |entry| entry.name.as_str()),
        panel.x + 34.0,
        panel.y + 48.0,
        TextParams {
            font_size: 32,
            color: super::gold_bright(),
            ..Default::default()
        },
    );
    super::draw_wrapped_line(
        location.map_or(
            "The party has found something the expedition journal cannot identify.",
            |entry| entry.description.as_str(),
        ),
        panel.x + 146.0,
        panel.y + 124.0,
        54,
        ui::TEXT_DIM,
    );
    if let Some(location) = location {
        assets::draw_special_location(location.visual, panel.x + 34.0, panel.y + 88.0, 94.0, 94.0);
        draw_ui_text_ex(
            "EVENT SITE",
            panel.x + 150.0,
            panel.y + 98.0,
            TextParams {
                font_size: 13,
                color: ui::ACCENT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            &format!("VISIT {}", run.stats.landmarks_visited),
            panel.x + 150.0,
            panel.y + 154.0,
            TextParams {
                font_size: 15,
                color: super::gold_bright(),
                ..Default::default()
            },
        );
    }

    for (index, event_id) in pending.event_ids.iter().take(2).enumerate() {
        draw_choice(
            data,
            state,
            run,
            data.tower_event(event_id),
            choice_rect(index),
            index,
        );
    }
    ui::draw_button(leave_rect(), "LEAVE UNDISTURBED", true);
    draw_ui_text_ex(
        "Choose one approach. The expedition pauses until you tap a choice or LEAVE.",
        panel.x + 78.0,
        panel.y + panel.h - 24.0,
        TextParams {
            font_size: 14,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
}

fn draw_choice(
    data: &GameData,
    state: &GameState,
    run: &TowerRunState,
    event: Option<&TowerEventDefinition>,
    rect: Rect,
    index: usize,
) {
    let enabled =
        event.is_some_and(|event| tower_engine::event_choice_available(state, data, &event.id));
    ui::register_controller_target(rect, enabled);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(13, 23, 24, 248),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        if enabled {
            super::gold_dim()
        } else {
            ui::TEXT_DIM
        },
    );
    ui::draw_controller_focus(rect, enabled);
    let tried = event.is_some_and(|entry| state.tower_discoveries.event_ids.contains(&entry.id));
    let label = choice_label(index, event, tried);
    draw_ui_text_ex(
        &label,
        rect.x + 20.0,
        rect.y + 32.0,
        TextParams {
            font_size: 22,
            color: if enabled {
                ui::TEXT_BRIGHT
            } else {
                ui::TEXT_DIM
            },
            ..Default::default()
        },
    );
    if let Some(event) = event {
        super::draw_wrapped_line(
            &event.narrative,
            rect.x + 20.0,
            rect.y + 56.0,
            78,
            ui::TEXT_DIM,
        );
        let mut effects = effect_summary(data, event);
        if !tried {
            effects.push_str("  ·  NEW: +1 survey (max 5)");
        }
        draw_ui_text_ex(
            &effects,
            rect.x + 438.0,
            rect.y + 32.0,
            TextParams {
                font_size: 15,
                color: super::gold_bright(),
                ..Default::default()
            },
        );
        let mut requirements = Vec::new();
        if !event.cargo_costs.is_empty() {
            requirements.push(
                event
                    .cargo_costs
                    .iter()
                    .map(|cost| {
                        format!(
                            "{} {}/{}",
                            data.resource_name(&cost.resource_id),
                            run.cargo_amount_for(&cost.resource_id),
                            cost.amount
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("  ·  "),
            );
        }
        if let Some(passive) = event.required_passive {
            requirements.push(format!("Party: {passive}"));
        }
        if let Some(element) = event.required_element {
            requirements.push(format!("Party: {element} affinity"));
        }
        if !requirements.is_empty() {
            draw_ui_text_ex(
                &requirements.join("  ·  "),
                rect.x + 438.0,
                rect.y + 78.0,
                TextParams {
                    font_size: 14,
                    color: if enabled {
                        super::gold_bright()
                    } else {
                        Color::from_rgba(220, 112, 91, 255)
                    },
                    ..Default::default()
                },
            );
        }
    }
}

fn effect_summary(data: &GameData, event: &TowerEventDefinition) -> String {
    let mut effects = event
        .rewards
        .iter()
        .map(|reward| {
            format!(
                "+{} {}",
                reward.amount,
                data.resource_name(&reward.resource_id)
            )
        })
        .collect::<Vec<_>>();
    if event.party_healing > 0 {
        effects.push(format!("Heal {} each", event.party_healing));
    }
    if event.pressure_delta != 0 {
        effects.push(format!("Pressure {:+}", event.pressure_delta));
    }
    if !event.enemy_id.is_empty() {
        effects.push("Ambush".to_owned());
    }
    if !event.egg_type_id.is_empty() {
        let egg_name = data
            .egg_type(&event.egg_type_id)
            .map(|egg| egg.name.as_str())
            .unwrap_or(&event.egg_type_id);
        effects.push(format!("Gain {egg_name}"));
    }
    if event.reveal_map {
        effects.push("Reveal routes".to_owned());
    }
    if event.reveal_secrets > 0 {
        effects.push(format!("Reveal {} secret(s)", event.reveal_secrets));
    }
    if event.refresh_camp {
        effects.push("Camp ready".to_owned());
    }
    if event.creates_shelter {
        effects.push("Create shelter".to_owned());
    }
    if event.survey_charges > 0 {
        effects.push(format!("+{} survey", event.survey_charges));
    }
    if event.repel_wanderers {
        effects.push("Repel hunters".to_owned());
    }
    if let Some(blessing) = event.blessing {
        effects.push(format!("Gain {}", blessing.label()));
    }
    if effects.is_empty() {
        "Unknown consequence".to_owned()
    } else {
        effects.join("  ·  ")
    }
}

fn choice_label(index: usize, event: Option<&TowerEventDefinition>, tried: bool) -> String {
    let name = event.map_or("Unknown Approach", |entry| entry.name.as_str());
    if tried {
        format!("{}  {}  ·  TRIED", index + 1, name)
    } else {
        format!("{}  {}  ·  NEW", index + 1, name)
    }
}

fn choice_rect(index: usize) -> Rect {
    Rect::new(282.0, 222.0 + index as f32 * 122.0, 716.0, 104.0)
}

fn leave_rect() -> Rect {
    Rect::new(525.0, 484.0, 230.0, 52.0)
}

#[cfg(test)]
mod tests;
