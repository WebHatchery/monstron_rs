use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

use super::{draw_overlay_panel, gold_bright};
use crate::assets;
use crate::data::GameData;
use crate::engine::tower_engine;
use crate::state::TowerRunState;
use crate::ui;

pub(super) fn draw(data: &GameData, run: &TowerRunState) {
    let panel = Rect::new(18.0, 538.0, 218.0, 148.0);
    draw_overlay_panel(panel);
    draw_ui_text_ex(
        "EXPEDITION JOURNAL",
        panel.x + 16.0,
        panel.y + 24.0,
        TextParams {
            font_size: 16,
            color: gold_bright(),
            ..Default::default()
        },
    );
    if let Some((progress, contract)) = tower_engine::contract_progress(run, data) {
        let marker = if run.contract_complete {
            " COMPLETE"
        } else {
            ""
        };
        draw_ui_text_ex(
            &format!(
                "{}  {}/{}{}",
                contract.name,
                progress.min(contract.target_amount),
                contract.target_amount,
                marker
            ),
            panel.x + 16.0,
            panel.y + 49.0,
            TextParams {
                font_size: 12,
                color: if run.contract_complete {
                    gold_bright()
                } else {
                    ui::TEXT_DIM
                },
                ..Default::default()
            },
        );
    }
    if !run.blessings.is_empty() {
        let labels = run
            .blessings
            .iter()
            .map(|blessing| blessing.label())
            .collect::<Vec<_>>()
            .join(" · ");
        draw_ui_text_ex(
            &format!("Blessings: {labels}"),
            panel.x + 16.0,
            panel.y + 69.0,
            TextParams {
                font_size: 11,
                color: gold_bright(),
                ..Default::default()
            },
        );
    }
    draw_ui_text_ex(
        &format!(
            "Landmarks visited {}  ·  resolved {}",
            run.stats.landmarks_visited, run.stats.landmarks_resolved
        ),
        panel.x + 16.0,
        panel.y + 88.0,
        TextParams {
            font_size: 11,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    for (index, message) in run.event_log.iter().rev().take(2).enumerate() {
        draw_ui_text_ex(
            &macroquad_toolkit::ui::truncate_text_to_width(
                &format!("• {message}"),
                panel.w - 32.0,
                12.0,
            ),
            panel.x + 16.0,
            panel.y + 105.0 + index as f32 * 22.0,
            TextParams {
                font_size: 12,
                color: ui::TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

pub(super) fn draw_anomaly(data: &GameData, run: &TowerRunState) {
    let Some(anomaly) = data.tower_anomaly(&run.anomaly_id) else {
        return;
    };
    let panel = Rect::new(18.0, 408.0, 218.0, 116.0);
    draw_overlay_panel(panel);
    assets::draw_tower_anomaly(
        anomaly.visual_index,
        panel.x + 7.0,
        panel.y + 22.0,
        82.0,
        82.0,
    );
    draw_ui_text_ex(
        "FLOOR ANOMALY",
        panel.x + 88.0,
        panel.y + 25.0,
        TextParams {
            font_size: 11,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &anomaly.name,
        panel.x + 88.0,
        panel.y + 48.0,
        TextParams {
            font_size: 17,
            color: gold_bright(),
            ..Default::default()
        },
    );
    draw_anomaly_detail(
        &anomaly.description,
        panel.x + 88.0,
        panel.y + 68.0,
        panel.w - 96.0,
        panel.h - 78.0,
    );
}

fn draw_anomaly_detail(text: &str, x: f32, y: f32, width: f32, height: f32) {
    macroquad_toolkit::ui::draw_text_block_ex(
        text,
        x,
        y - 10.0,
        width,
        height,
        macroquad_toolkit::ui::TextStyle::new(10.0, ui::TEXT_DIM).with_line_gap(4.0),
        10.0,
    );
}
