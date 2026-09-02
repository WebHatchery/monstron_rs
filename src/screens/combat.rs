use macroquad::prelude::*;

use crate::assets;
use crate::data::{EnemyBehavior, GameData, MonsterRole};
use crate::engine::combat_engine::{
    player_attack_target, player_skill_target, queued_enemy_intent, CombatCommand, CombatTarget,
    EnemyIntentKind,
};
use crate::state::{CombatOutcome, CombatSide, CombatState, Combatant, GameState};
use crate::ui;
use macroquad_toolkit::ui::draw_ui_text_ex;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CombatAction {
    Command(CombatCommand),
    Continue,
}

pub fn handle_input(state: &GameState) -> Option<CombatAction> {
    let combat = state.combat.as_ref()?;

    if combat.outcome.is_some() {
        if is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Space)
            || ui::button_clicked(continue_rect(), true)
        {
            return Some(CombatAction::Continue);
        }
        return None;
    }

    if !combat.is_player_turn() {
        return None;
    }

    if is_key_pressed(KeyCode::A) {
        return Some(CombatAction::Command(CombatCommand::Attack));
    }
    if is_key_pressed(KeyCode::S) {
        return Some(CombatAction::Command(CombatCommand::Skill));
    }
    if is_key_pressed(KeyCode::D) {
        return Some(CombatAction::Command(CombatCommand::Defend));
    }
    if is_key_pressed(KeyCode::I) {
        return Some(CombatAction::Command(CombatCommand::Item));
    }
    if is_key_pressed(KeyCode::F) || is_key_pressed(KeyCode::Escape) {
        return Some(CombatAction::Command(CombatCommand::Flee));
    }

    for (command, rect) in command_buttons() {
        if ui::button_clicked(rect, true) {
            return Some(CombatAction::Command(command));
        }
    }

    None
}

pub fn draw(state: &GameState, data: &GameData, status_message: &str) {
    draw_backdrop();
    if let Some(combat) = &state.combat {
        draw_header(combat);
        draw_formation(combat, data);
        draw_actions(combat);
        if combat.outcome.is_some() {
            draw_rewards(combat, data);
        } else {
            draw_tactics(combat);
        }
        draw_log(combat);
    } else {
        draw_empty();
    }
    ui::draw_status(status_message);
}

fn draw_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        ui::VIEW_WIDTH,
        ui::VIEW_HEIGHT,
        Color::from_rgba(19, 21, 25, 255),
    );
    draw_rectangle(
        0.0,
        440.0,
        ui::VIEW_WIDTH,
        280.0,
        Color::from_rgba(35, 39, 42, 255),
    );
    draw_circle(980.0, 160.0, 130.0, Color::from_rgba(155, 92, 72, 26));
    assets::draw_room_vignette(1, 270.0, 112.0, 710.0, 390.0);
    draw_rectangle(270.0, 112.0, 710.0, 390.0, Color::from_rgba(10, 15, 17, 84));
}

fn draw_header(combat: &CombatState) {
    ui::draw_panel(Rect::new(32.0, 24.0, ui::VIEW_WIDTH - 64.0, 78.0));
    draw_ui_text_ex(
        &format!("Combat - Floor {}", combat.floor),
        58.0,
        72.0,
        TextParams {
            font_size: 34,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!("Round {}", combat.round),
        520.0,
        70.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
    let turn_text = if let Some(outcome) = combat.outcome {
        match outcome {
            CombatOutcome::Victory => "Victory",
            CombatOutcome::Defeat => "Defeat",
            CombatOutcome::Fled => "Fled",
        }
    } else if combat
        .current_turn()
        .is_some_and(|turn| turn.side == CombatSide::Ally)
    {
        "Party turn"
    } else {
        "Enemy turn"
    };
    draw_ui_text_ex(
        turn_text,
        ui::VIEW_WIDTH - 230.0,
        70.0,
        TextParams {
            font_size: 24,
            color: ui::ACCENT,
            ..Default::default()
        },
    );
}

fn draw_formation(combat: &CombatState, data: &GameData) {
    let rect = Rect::new(32.0, 124.0, 780.0, 330.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Formation", rect.x + 20.0, rect.y + 34.0);
    draw_ui_text_ex(
        "Allies",
        rect.x + 58.0,
        rect.y + 78.0,
        TextParams {
            font_size: 22,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        "Enemies",
        rect.x + 472.0,
        rect.y + 78.0,
        TextParams {
            font_size: 22,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );

    if combat.outcome.is_none() {
        assets::draw_combat_vfx(combat.round as usize, 422.0, 276.0, 32.0);
    }
    for combatant in &combat.allies {
        draw_combatant(data, combatant, true, ally_slot_rect(combatant.slot));
    }
    for combatant in &combat.enemies {
        draw_combatant(data, combatant, false, enemy_slot_rect(combatant.slot));
    }
}

fn draw_combatant(data: &GameData, combatant: &Combatant, is_ally: bool, rect: Rect) {
    let fill = if combatant.is_alive() {
        Color::from_rgba(29, 38, 43, 230)
    } else {
        Color::from_rgba(29, 31, 33, 180)
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.5, ui::PANEL_EDGE);

    if is_ally {
        assets::draw_monster_badge(&combatant.source_id, rect.x + 8.0, rect.y + 8.0, 30.0);
    } else {
        if let Some(enemy) = data.enemy(&combatant.source_id) {
            assets::draw_enemy_badge_visual(enemy.visual, rect.x + 8.0, rect.y + 8.0, 30.0);
        }
    }

    let name_color = if combatant.is_alive() {
        ui::TEXT_BRIGHT
    } else {
        ui::TEXT_DIM
    };
    draw_ui_text_ex(
        &combatant.name,
        rect.x + 43.0,
        rect.y + 22.0,
        TextParams {
            font_size: 15,
            color: name_color,
            ..Default::default()
        },
    );
    draw_hp_bar(combatant, rect.x + 43.0, rect.y + 29.0, rect.w - 50.0);
    draw_ui_text_ex(
        &format!("HP {}/{}", combatant.hp.max(0), combatant.max_hp),
        rect.x + 43.0,
        rect.y + 51.0,
        TextParams {
            font_size: 11,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    let row = if combatant.slot < 3 { "F" } else { "B" };
    draw_ui_text_ex(
        &format!(
            "{}{} {}{}",
            row,
            combatant.slot + 1,
            role_label(combatant, is_ally),
            status_label(combatant)
        ),
        rect.x + 8.0,
        rect.y + 67.0,
        TextParams {
            font_size: 12,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!("ATK {}  DEF {}", combatant.attack, combatant.defense),
        rect.x + 8.0,
        rect.y + 84.0,
        TextParams {
            font_size: 12,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    draw_ui_text_ex(
        &format!("SPD {}  MOR {}", combatant.speed, combatant.morale),
        rect.x + 8.0,
        rect.y + 100.0,
        TextParams {
            font_size: 12,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
}

fn draw_hp_bar(combatant: &Combatant, x: f32, y: f32, width: f32) {
    draw_rectangle(x, y, width, 12.0, Color::from_rgba(20, 24, 28, 255));
    let ratio = combatant.hp.max(0) as f32 / combatant.max_hp.max(1) as f32;
    let color = if ratio < 0.3 {
        Color::from_rgba(210, 86, 72, 255)
    } else {
        ui::ACCENT
    };
    draw_rectangle(x, y, width * ratio.clamp(0.0, 1.0), 12.0, color);
    draw_rectangle_lines(x, y, width, 12.0, 1.0, ui::PANEL_EDGE);
}

fn draw_actions(combat: &CombatState) {
    let rect = Rect::new(32.0, 474.0, 780.0, 126.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Actions", rect.x + 20.0, rect.y + 32.0);

    if combat.outcome.is_some() {
        ui::draw_button(continue_rect(), "Continue", true);
        return;
    }

    let enabled = combat.is_player_turn();
    for (command, button_rect) in command_buttons() {
        let label = match command {
            CombatCommand::Attack => "Attack",
            CombatCommand::Skill => current_skill_label(combat),
            CombatCommand::Defend => "Defend",
            CombatCommand::Item => "Herbs",
            CombatCommand::Flee => "Flee",
        };
        ui::draw_button(button_rect, label, enabled);
    }
}

fn draw_rewards(combat: &CombatState, data: &GameData) {
    let rect = Rect::new(836.0, 124.0, 412.0, 150.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Rewards", rect.x + 20.0, rect.y + 32.0);
    draw_ui_text_ex(
        &format!("XP: {}", combat.xp_reward),
        rect.x + 20.0,
        rect.y + 68.0,
        TextParams {
            font_size: 20,
            color: ui::TEXT_BRIGHT,
            ..Default::default()
        },
    );
    let reward_text = combat
        .rewards
        .iter()
        .map(|reward| {
            format!(
                "{} {}",
                reward.amount,
                data.resource_name(&reward.resource_id)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    draw_ui_text_ex(
        if reward_text.is_empty() {
            "Materials: none"
        } else {
            &reward_text
        },
        rect.x + 20.0,
        rect.y + 102.0,
        TextParams {
            font_size: 17,
            color: ui::TEXT_DIM,
            ..Default::default()
        },
    );
    if combat.enemies.iter().any(|enemy| enemy.is_marked) {
        draw_ui_text_ex(
            "Marked target bonus active",
            rect.x + 20.0,
            rect.y + 126.0,
            TextParams {
                font_size: 15,
                color: ui::ACCENT,
                ..Default::default()
            },
        );
    }
}

fn draw_tactics(combat: &CombatState) {
    let rect = Rect::new(836.0, 124.0, 412.0, 150.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Targeting & Intent", rect.x + 20.0, rect.y + 32.0);

    let lines = tactics_lines(combat);
    for (index, line) in lines.iter().enumerate() {
        draw_ui_text_ex(
            line,
            rect.x + 20.0,
            rect.y + 62.0 + index as f32 * 22.0,
            TextParams {
                font_size: 15,
                color: if index == 2 { ui::ACCENT } else { ui::TEXT },
                ..Default::default()
            },
        );
    }
}

fn tactics_lines(combat: &CombatState) -> [String; 4] {
    let attack = player_attack_target(combat)
        .map(|target| format!("Attack -> {} (auto)", target_label(combat, target)))
        .unwrap_or_else(|| "Attack -> no living target".to_owned());
    let skill = player_skill_target(combat)
        .map(|target| {
            format!(
                "{} -> {} (auto)",
                current_skill_label(combat),
                skill_target_label(combat, target)
            )
        })
        .unwrap_or_else(|| format!("{} -> no living target", current_skill_label(combat)));
    let threat = queued_enemy_intent(combat)
        .map(|intent| enemy_intent_label(combat, intent))
        .unwrap_or_else(|| "Next threat: none before round end".to_owned());
    let statuses = active_statuses(combat);
    [attack, skill, threat, statuses]
}

fn skill_target_label(combat: &CombatState, target: CombatTarget) -> String {
    let is_guard = combat.current_turn().is_some_and(|turn| {
        turn.side == CombatSide::Ally
            && combat
                .allies
                .get(turn.slot)
                .is_some_and(|ally| ally.role == Some(MonsterRole::Tank))
    });
    if is_guard {
        "back row (self braces)".to_owned()
    } else {
        target_label(combat, target)
    }
}

fn enemy_intent_label(
    combat: &CombatState,
    intent: crate::engine::combat_engine::EnemyIntent,
) -> String {
    let actor = combat
        .enemies
        .get(intent.actor_index)
        .map_or("Enemy", |enemy| enemy.name.as_str());
    let target = target_label(combat, intent.target);
    match intent.kind {
        EnemyIntentKind::Attack => {
            let verb = combat
                .enemies
                .get(intent.actor_index)
                .map_or("hit", |enemy| enemy_attack_verb(combat, enemy));
            let guard = if intent.intercepted { " [GUARDED]" } else { "" };
            format!("Next: {actor} {verb} -> {target}{guard}")
        }
        EnemyIntentKind::Brace => format!("Next: {actor} -> BRACE (self)"),
        EnemyIntentKind::Rally => format!("Next: {actor} -> RALLY {target}"),
        EnemyIntentKind::Ward => format!("Next: {actor} -> WARD {target}"),
    }
}

fn enemy_attack_verb(combat: &CombatState, enemy: &Combatant) -> &'static str {
    match enemy.enemy_behavior.unwrap_or_default() {
        EnemyBehavior::Bruiser if combat.round.is_multiple_of(3) => "heavy hit",
        EnemyBehavior::Harrier => "harry",
        EnemyBehavior::Hexer if combat.round.is_multiple_of(2) => "hex",
        EnemyBehavior::Swarm => "swarm hit",
        EnemyBehavior::Ambusher if combat.round == 1 => "ambush",
        EnemyBehavior::Regenerator if combat.round.is_multiple_of(2) && enemy.hp < enemy.max_hp => {
            "heal + hit"
        }
        EnemyBehavior::Sapper if combat.round.is_multiple_of(2) => "break stance + hit",
        EnemyBehavior::Leech if enemy.hp < enemy.max_hp => "leech hit",
        _ => "hit",
    }
}

fn target_label(combat: &CombatState, target: CombatTarget) -> String {
    let combatant = match target.side {
        CombatSide::Ally => combat.allies.get(target.index),
        CombatSide::Enemy => combat.enemies.get(target.index),
    };
    combatant.map_or_else(
        || "missing target".to_owned(),
        |unit| {
            let row = if unit.slot < 3 { 'F' } else { 'B' };
            format!("{} {row}{}", unit.name, unit.slot + 1)
        },
    )
}

fn active_statuses(combat: &CombatState) -> String {
    let guarding = combat
        .allies
        .iter()
        .any(|unit| unit.is_alive() && unit.is_guarding);
    let defending = combat
        .allies
        .iter()
        .chain(&combat.enemies)
        .any(|unit| unit.is_alive() && unit.is_defending && !unit.is_guarding);
    let marked = combat
        .enemies
        .iter()
        .any(|unit| unit.is_alive() && unit.is_marked);
    let mut labels = Vec::new();
    if guarding {
        labels.push("GUARDING");
    }
    if defending {
        labels.push("DEFENDING");
    }
    if marked {
        labels.push("MARKED");
    }
    if labels.is_empty() {
        "Active status: none".to_owned()
    } else {
        format!("Active: {}", labels.join(" | "))
    }
}

fn draw_log(combat: &CombatState) {
    let rect = Rect::new(836.0, 298.0, 412.0, 302.0);
    ui::draw_panel(rect);
    ui::draw_section_title("Battle Log", rect.x + 20.0, rect.y + 32.0);
    for (index, message) in combat.log.iter().rev().take(6).enumerate() {
        draw_ui_text_ex(
            message,
            rect.x + 20.0,
            rect.y + 68.0 + index as f32 * 34.0,
            TextParams {
                font_size: 16,
                color: ui::TEXT,
                ..Default::default()
            },
        );
    }
}

fn draw_empty() {
    let rect = Rect::new(250.0, 170.0, 780.0, 300.0);
    ui::draw_panel(rect);
    ui::draw_centered_text(
        "No Combat Active",
        ui::VIEW_WIDTH * 0.5,
        rect.y + 120.0,
        36,
        ui::TEXT_BRIGHT,
    );
}

fn ally_slot_rect(slot: usize) -> Rect {
    let column = slot % 3;
    let row = slot / 3;
    Rect::new(
        52.0 + column as f32 * 124.0,
        220.0 + row as f32 * 110.0,
        116.0,
        104.0,
    )
}

fn enemy_slot_rect(slot: usize) -> Rect {
    let column = slot % 3;
    let row = slot / 3;
    Rect::new(
        460.0 + column as f32 * 124.0,
        220.0 + row as f32 * 110.0,
        116.0,
        104.0,
    )
}

pub(crate) fn command_buttons() -> [(CombatCommand, Rect); 5] {
    [
        (CombatCommand::Attack, Rect::new(210.0, 538.0, 100.0, 36.0)),
        (CombatCommand::Skill, Rect::new(322.0, 538.0, 100.0, 36.0)),
        (CombatCommand::Defend, Rect::new(434.0, 538.0, 100.0, 36.0)),
        (CombatCommand::Item, Rect::new(546.0, 538.0, 100.0, 36.0)),
        (CombatCommand::Flee, Rect::new(658.0, 538.0, 100.0, 36.0)),
    ]
}

fn continue_rect() -> Rect {
    Rect::new(582.0, 538.0, 176.0, 36.0)
}

fn current_skill_label(combat: &CombatState) -> &'static str {
    let Some(turn) = combat.current_turn() else {
        return "Skill";
    };
    if turn.side != CombatSide::Ally {
        return "Skill";
    }
    let Some(ally) = combat.allies.get(turn.slot) else {
        return "Skill";
    };
    match ally.role {
        Some(MonsterRole::Scout) => "Mark",
        Some(MonsterRole::Tank) => "Guard",
        Some(MonsterRole::Support) => "Soothe",
        Some(MonsterRole::Striker) => "Burst",
        None => "Skill",
    }
}

fn role_label(combatant: &Combatant, is_ally: bool) -> &'static str {
    if !is_ally {
        return match combatant.enemy_behavior.unwrap_or_default() {
            EnemyBehavior::Standard => "Enemy",
            EnemyBehavior::Bruiser => "Bruise",
            EnemyBehavior::Bulwark => "Guard",
            EnemyBehavior::Harrier => "Harrier",
            EnemyBehavior::Hexer => "Hexer",
            EnemyBehavior::Swarm => "Swarm",
            EnemyBehavior::Ambusher => "Ambusher",
            EnemyBehavior::Regenerator => "Regenerator",
            EnemyBehavior::Packleader => "Packleader",
            EnemyBehavior::Sapper => "Sapper",
            EnemyBehavior::Leech => "Leech",
            EnemyBehavior::Warden => "Warden",
        };
    }
    match combatant.role {
        Some(MonsterRole::Scout) => "Scout",
        Some(MonsterRole::Tank) => "Tank",
        Some(MonsterRole::Support) => "Sup",
        Some(MonsterRole::Striker) => "Str",
        None => "Ally",
    }
}

fn status_label(combatant: &Combatant) -> &'static str {
    if combatant.is_guarding {
        " Gd"
    } else if combatant.is_defending {
        " Def"
    } else if combatant.is_marked {
        " Mark"
    } else {
        ""
    }
}
