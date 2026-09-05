use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

use super::{draw_overlay_panel, draw_wrapped_line, gold_bright};
use crate::assets;
use crate::data::GameData;
use crate::state::{TowerMapObject, TowerMapObjectKind, TowerRunState};
use crate::ui;

pub(super) fn draw(data: &GameData, run: &TowerRunState) {
    let panel = Rect::new(1060.0, 104.0, 208.0, 366.0);
    draw_overlay_panel(panel);
    let focus = nearest_visible_object(run);
    let floor_name = data
        .tower_floor(run.current_floor)
        .map(|floor| floor.name.as_str())
        .unwrap_or("Dungeon");
    let name = focus
        .and_then(|object| object_name(data, object))
        .unwrap_or(floor_name);
    draw_ui_text_ex(
        name,
        panel.x + 16.0,
        panel.y + 35.0,
        TextParams {
            font_size: 24,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        focus
            .map(object_kind_label)
            .unwrap_or("EXPEDITION  ·  ROUTE"),
        panel.x + 16.0,
        panel.y + 65.0,
        TextParams {
            font_size: 15,
            color: gold_bright(),
            ..Default::default()
        },
    );
    if let Some(object) = focus {
        draw_context_art(
            data,
            run,
            object,
            Rect::new(panel.x + 20.0, panel.y + 78.0, 168.0, 150.0),
        );
    } else {
        assets::draw_room_vignette(
            run.current_floor,
            panel.x + 20.0,
            panel.y + 78.0,
            168.0,
            150.0,
        );
    }
    let detail = focus
        .and_then(|object| object_detail(data, run, object))
        .unwrap_or_else(|| "Move through lit rooms to reveal what the tower is hiding.".to_owned());
    draw_wrapped_line(
        &detail,
        panel.x + 16.0,
        panel.y + 258.0,
        panel.w - 32.0,
        72.0,
        ui::TEXT_DIM,
    );
    draw_ui_text_ex(
        if focus.is_some() {
            "Approach to interact"
        } else {
            "Explore the map"
        },
        panel.x + 48.0,
        panel.y + 348.0,
        TextParams {
            font_size: 14,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
}

fn nearest_visible_object(run: &TowerRunState) -> Option<&TowerMapObject> {
    run.map
        .objects
        .iter()
        .filter(|object| run.map.is_visible(object.x, object.y))
        .filter(|object| object.kind != TowerMapObjectKind::SecretCache || object.revealed)
        .min_by_key(|object| {
            run.map.player_x.abs_diff(object.x) + run.map.player_y.abs_diff(object.y)
        })
}

fn object_name<'a>(data: &'a GameData, object: &'a TowerMapObject) -> Option<&'a str> {
    match object.kind {
        TowerMapObjectKind::Loot => Some(data.resource_name(&object.resource_id)),
        TowerMapObjectKind::SecretCache => Some("Surveyed Secret"),
        TowerMapObjectKind::Egg => data
            .egg_type(&object.egg_type_id)
            .map(|egg| egg.name.as_str()),
        TowerMapObjectKind::Enemy | TowerMapObjectKind::Boss => data
            .enemy(&object.enemy_id)
            .map(|enemy| enemy.name.as_str()),
        TowerMapObjectKind::Hazard => data
            .tower_hazard(&object.hazard_id)
            .map(|hazard| hazard.name.as_str()),
        TowerMapObjectKind::SpecialLocation => data
            .tower_special_location(&object.special_location_id)
            .map(|location| location.name.as_str()),
        TowerMapObjectKind::Stairs => Some("Deeper Stair"),
        TowerMapObjectKind::Exit => Some("Return Threshold"),
    }
}

fn object_detail(data: &GameData, run: &TowerRunState, object: &TowerMapObject) -> Option<String> {
    match object.kind {
        TowerMapObjectKind::Enemy | TowerMapObjectKind::Boss => data
            .enemy(&object.enemy_id)
            .map(|enemy| enemy.description.clone()),
        TowerMapObjectKind::SpecialLocation => data
            .tower_special_location(&object.special_location_id)
            .map(|location| location.description.clone()),
        TowerMapObjectKind::Hazard => data.tower_hazard(&object.hazard_id).map(|hazard| {
            let counter = hazard
                .counter_passive
                .map(|passive| passive.to_string())
                .or_else(|| {
                    hazard
                        .counter_element
                        .map(|element| format!("{element} monster"))
                })
                .unwrap_or_else(|| "none".to_owned());
            format!("{} Counter: {counter}.", hazard.description)
        }),
        TowerMapObjectKind::Egg => Some("A living egg waits in a tower nest.".to_owned()),
        TowerMapObjectKind::Loot => Some("Supplies can be carried safely back to town.".to_owned()),
        TowerMapObjectKind::SecretCache => {
            Some("A survey flare exposed a concealed cache behind the room facade.".to_owned())
        }
        TowerMapObjectKind::Stairs if boss_gate_is_sealed(data, run) => {
            Some("The floor guardian seals the deeper stair. Defeat it or tap RETREAT.".to_owned())
        }
        TowerMapObjectKind::Stairs => Some("This route leads to the next floor.".to_owned()),
        TowerMapObjectKind::Exit if boss_gate_is_sealed(data, run) => {
            Some("The guardian seals this threshold. Defeat it or tap RETREAT.".to_owned())
        }
        TowerMapObjectKind::Exit => {
            Some("The open threshold returns the party and its cargo to town.".to_owned())
        }
    }
}

fn object_kind_label(object: &TowerMapObject) -> &'static str {
    match object.kind {
        TowerMapObjectKind::Loot => "CACHE  ·  SUPPLIES",
        TowerMapObjectKind::SecretCache => "SECRET  ·  SURVEYED",
        TowerMapObjectKind::Egg => "NEST  ·  EGG",
        TowerMapObjectKind::Enemy if object.wandering => "ENCOUNTER  ·  HUNTER",
        TowerMapObjectKind::Enemy => "ENCOUNTER  ·  DENIZEN",
        TowerMapObjectKind::Boss => "ENCOUNTER  ·  BOSS",
        TowerMapObjectKind::Hazard => "HAZARD  ·  TELEGRAPHED",
        TowerMapObjectKind::SpecialLocation => "LANDMARK  ·  EVENT",
        TowerMapObjectKind::Stairs => "ROUTE  ·  DESCENT",
        TowerMapObjectKind::Exit => "ROUTE  ·  RETURN",
    }
}

fn draw_context_art(data: &GameData, run: &TowerRunState, object: &TowerMapObject, rect: Rect) {
    assets::draw_tower_map_object(data, run, object, boss_gate_is_sealed(data, run), rect);
}

fn boss_gate_is_sealed(data: &GameData, run: &TowerRunState) -> bool {
    !run.boss_defeated
        && data
            .tower_floor(run.current_floor)
            .is_some_and(|floor| floor.is_boss_floor || !floor.guardian_enemy_id.is_empty())
}
