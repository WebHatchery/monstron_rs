use macroquad::prelude::*;
use macroquad_toolkit::input::GamepadFrame;
use macroquad_toolkit::ui::draw_ui_text_ex;
use std::cell::{Cell, RefCell};

pub const VIEW_WIDTH: f32 = 1280.0;
pub const VIEW_HEIGHT: f32 = 720.0;
pub const BACKGROUND: Color = Color::new(0.075, 0.086, 0.102, 1.0);
pub const PANEL: Color = Color::new(0.118, 0.137, 0.157, 0.92);
pub const PANEL_EDGE: Color = Color::new(0.275, 0.329, 0.376, 1.0);
pub const TEXT: Color = Color::new(0.827, 0.851, 0.847, 1.0);
pub const TEXT_BRIGHT: Color = Color::new(0.957, 0.965, 0.941, 1.0);
pub const TEXT_DIM: Color = Color::new(0.572, 0.627, 0.627, 1.0);
pub const ACCENT: Color = Color::new(0.604, 0.827, 0.608, 1.0);
pub const WARN: Color = Color::new(0.914, 0.612, 0.369, 1.0);
const BUTTON: Color = Color::new(0.173, 0.243, 0.275, 1.0);
const BUTTON_HOVER: Color = Color::new(0.224, 0.337, 0.365, 1.0);
const BUTTON_DISABLED: Color = Color::new(0.145, 0.157, 0.169, 1.0);

#[derive(Clone, Copy, Debug, Default)]
struct ControllerUiState {
    focused: Option<Rect>,
    frame: GamepadFrame,
    active: bool,
}

thread_local! {
    static CONTROLLER_TARGETS: RefCell<Vec<Rect>> = const { RefCell::new(Vec::new()) };
    static CONTROLLER_STATE: Cell<ControllerUiState> = const { Cell::new(ControllerUiState {
        focused: None,
        frame: GamepadFrame {
            connected: false,
            confirm: false,
            cancel: false,
            secondary: false,
            tertiary: false,
            menu: false,
            next: false,
            previous: false,
            up: false,
            down: false,
            left: false,
            right: false,
            held_up: false,
            held_down: false,
            held_left: false,
            held_right: false,
        },
        active: false,
    }) };
}

#[derive(Clone, Copy, Debug)]
pub struct Tooltip {
    pub rect: Rect,
    pub title: &'static str,
    pub detail: &'static str,
}

pub fn button_clicked(rect: Rect, enabled: bool) -> bool {
    enabled
        && (is_mouse_over(rect) && is_mouse_button_released(MouseButton::Left)
            || controller_activated(rect))
}

pub(crate) fn begin_controller_registry() {
    CONTROLLER_TARGETS.with(|targets| targets.borrow_mut().clear());
}

pub(crate) fn begin_controller_modal() {
    begin_controller_registry();
}

pub(crate) fn register_controller_target(rect: Rect, enabled: bool) {
    if !enabled {
        return;
    }
    CONTROLLER_TARGETS.with(|targets| {
        let mut targets = targets.borrow_mut();
        if !targets.contains(&rect) {
            targets.push(rect);
        }
    });
}

pub(crate) fn controller_targets() -> Vec<Rect> {
    CONTROLLER_TARGETS.with(|targets| targets.borrow().clone())
}

pub(crate) fn set_controller_input(
    focused: Option<Rect>,
    mut frame: GamepadFrame,
    active: bool,
    allow_cancel: bool,
) {
    if !allow_cancel {
        frame.cancel = false;
    }
    CONTROLLER_STATE.with(|state| {
        state.set(ControllerUiState {
            focused,
            frame,
            active,
        });
    });
}

pub(crate) fn controller_cancel_pressed() -> bool {
    CONTROLLER_STATE.with(|state| {
        let state = state.get();
        state.active && state.frame.cancel
    })
}

pub(crate) fn controller_menu_pressed() -> bool {
    CONTROLLER_STATE.with(|state| {
        let state = state.get();
        state.active && state.frame.menu
    })
}

pub(crate) fn controller_direction() -> Option<(i32, i32)> {
    CONTROLLER_STATE.with(|state| {
        let state = state.get();
        if !state.active {
            return None;
        }
        if state.frame.up {
            Some((0, -1))
        } else if state.frame.down {
            Some((0, 1))
        } else if state.frame.left {
            Some((-1, 0))
        } else if state.frame.right {
            Some((1, 0))
        } else {
            None
        }
    })
}

fn controller_activated(rect: Rect) -> bool {
    CONTROLLER_STATE.with(|state| {
        let state = state.get();
        state.active && state.frame.confirm && state.focused == Some(rect)
    })
}

fn controller_focused(rect: Rect) -> bool {
    CONTROLLER_STATE.with(|state| {
        let state = state.get();
        state.active && state.focused == Some(rect)
    })
}

pub(crate) fn draw_controller_focus(rect: Rect, enabled: bool) {
    if enabled && controller_focused(rect) {
        draw_rectangle_lines(
            rect.x - 3.0,
            rect.y - 3.0,
            rect.w + 6.0,
            rect.h + 6.0,
            3.0,
            Color::from_rgba(246, 196, 83, 255),
        );
        draw_circle(
            rect.x + 4.0,
            rect.y + 4.0,
            8.0,
            Color::from_rgba(9, 19, 20, 245),
        );
        draw_centered_text("A", rect.x + 4.0, rect.y + 8.0, 11, ACCENT);
    }
}

pub fn virtual_camera() -> Camera2D {
    Camera2D {
        target: vec2(VIEW_WIDTH * 0.5, VIEW_HEIGHT * 0.5),
        zoom: vec2(2.0 / VIEW_WIDTH, 2.0 / VIEW_HEIGHT),
        ..Default::default()
    }
}

pub fn draw_button(rect: Rect, label: &str, enabled: bool) {
    register_controller_target(rect, enabled);
    let hovered = enabled && (is_mouse_over(rect) || controller_focused(rect));
    let color = if !enabled {
        BUTTON_DISABLED
    } else if hovered {
        BUTTON_HOVER
    } else {
        BUTTON
    };
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(color).with_border(2.0, PANEL_EDGE);
    macroquad_toolkit::ui::draw_surface(rect, &surface);

    let text_color = if enabled { TEXT_BRIGHT } else { TEXT_DIM };
    let font_size = if rect.h < 30.0 || rect.w < 80.0 {
        18.0
    } else if rect.h < 38.0 {
        20.0
    } else {
        24.0
    };
    macroquad_toolkit::ui::draw_text_centered_in_box(
        label, rect.x, rect.y, rect.w, rect.h, font_size, text_color,
    );
    draw_controller_focus(rect, enabled);
}

pub fn draw_button_with_tooltip(
    rect: Rect,
    label: &str,
    enabled: bool,
    title: &'static str,
    detail: &'static str,
) {
    draw_button(rect, label, enabled);
    draw_tooltip_target(Tooltip {
        rect,
        title,
        detail,
    });
}

pub fn draw_tooltip_target(tooltip: Tooltip) {
    let hint_x = tooltip.rect.x + tooltip.rect.w - 10.0;
    let hint_y = tooltip.rect.y + 10.0;
    draw_circle(hint_x, hint_y, 7.0, Color::from_rgba(19, 32, 34, 245));
    draw_circle_lines(hint_x, hint_y, 7.0, 1.0, ACCENT);
    draw_centered_text("?", hint_x, hint_y + 4.0, 11, ACCENT);

    if is_mouse_over(tooltip.rect)
        || is_mouse_button_down(MouseButton::Left) && is_mouse_over(tooltip.rect)
    {
        let panel = Rect::new(
            (tooltip.rect.x + tooltip.rect.w - 276.0).max(12.0),
            (tooltip.rect.y - 82.0).max(112.0),
            264.0,
            70.0,
        );
        draw_panel(panel);
        draw_ui_text_ex(
            tooltip.title,
            panel.x + 12.0,
            panel.y + 23.0,
            TextParams {
                font_size: 16,
                color: TEXT_BRIGHT,
                ..Default::default()
            },
        );
        draw_ui_text_ex(
            tooltip.detail,
            panel.x + 12.0,
            panel.y + 48.0,
            TextParams {
                font_size: 14,
                color: TEXT_DIM,
                ..Default::default()
            },
        );
    }
}

pub fn draw_title_button(rect: Rect, label: &str, enabled: bool) {
    register_controller_target(rect, enabled);
    let hovered = enabled && (is_mouse_over(rect) || controller_focused(rect));
    let fill = if !enabled {
        Color::from_rgba(18, 24, 29, 210)
    } else if hovered {
        Color::from_rgba(30, 59, 55, 235)
    } else {
        Color::from_rgba(16, 31, 38, 225)
    };
    let outer_edge = if hovered {
        Color::from_rgba(190, 232, 166, 255)
    } else if enabled {
        Color::from_rgba(107, 138, 119, 235)
    } else {
        Color::from_rgba(63, 73, 75, 210)
    };
    let warm_edge = if hovered {
        Color::from_rgba(234, 181, 91, 255)
    } else {
        Color::from_rgba(143, 103, 61, 225)
    };

    draw_rectangle(
        rect.x + 5.0,
        rect.y + 6.0,
        rect.w,
        rect.h,
        Color::from_rgba(2, 6, 9, 170),
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, outer_edge);
    draw_line(
        rect.x + 10.0,
        rect.y + 4.0,
        rect.x + rect.w - 10.0,
        rect.y + 4.0,
        1.0,
        Color::from_rgba(214, 244, 191, 70),
    );
    draw_line(
        rect.x + 12.0,
        rect.y + rect.h - 4.0,
        rect.x + rect.w - 12.0,
        rect.y + rect.h - 4.0,
        1.5,
        warm_edge,
    );

    let text_color = if enabled {
        Color::from_rgba(230, 248, 218, 255)
    } else {
        Color::from_rgba(127, 143, 137, 255)
    };
    macroquad_toolkit::ui::draw_text_centered_in_box(
        label, rect.x, rect.y, rect.w, rect.h, 22.0, text_color,
    );
    draw_controller_focus(rect, enabled);
}

pub fn draw_toggle(rect: Rect, label: &str, enabled: bool) {
    register_controller_target(rect, true);
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(PANEL).with_border(1.5, PANEL_EDGE);
    macroquad_toolkit::ui::draw_surface(rect, &surface);

    draw_ui_text_ex(
        label,
        rect.x + 20.0,
        rect.y + 36.0,
        TextParams {
            font_size: 24,
            color: TEXT_BRIGHT,
            ..Default::default()
        },
    );

    let track_h = 30.0;
    let track_w = 76.0;
    let track_x = rect.x + rect.w - track_w - 20.0;
    let track_y = rect.y + rect.h * 0.5 - track_h * 0.5;
    let track_color = if enabled { ACCENT } else { BUTTON_DISABLED };
    let knob_x = if enabled {
        track_x + track_w - track_h * 0.5
    } else {
        track_x + track_h * 0.5
    };

    draw_rectangle(
        track_x + track_h * 0.5,
        track_y,
        track_w - track_h,
        track_h,
        track_color,
    );
    draw_circle(
        track_x + track_h * 0.5,
        track_y + track_h * 0.5,
        track_h * 0.5,
        track_color,
    );
    draw_circle(
        track_x + track_w - track_h * 0.5,
        track_y + track_h * 0.5,
        track_h * 0.5,
        track_color,
    );
    draw_circle(knob_x, track_y + track_h * 0.5, track_h * 0.38, TEXT_BRIGHT);
    draw_controller_focus(rect, true);
}

pub fn draw_panel(rect: Rect) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(PANEL).with_border(1.5, PANEL_EDGE);
    macroquad_toolkit::ui::draw_surface(rect, &surface);
}

pub fn draw_section_title(title: &str, x: f32, y: f32) {
    draw_ui_text_ex(
        title,
        x,
        y,
        TextParams {
            font_size: 28,
            color: TEXT_BRIGHT,
            ..Default::default()
        },
    );
}

pub fn draw_centered_text(text: &str, center_x: f32, y: f32, font_size: u16, color: Color) {
    macroquad_toolkit::ui::draw_text_centered(
        text,
        center_x,
        y,
        macroquad_toolkit::ui::TextStyle::new(font_size as f32, color),
    );
}

pub fn draw_status(status_message: &str) {
    draw_status_at(
        status_message,
        Rect::new(24.0, VIEW_HEIGHT - 48.0, VIEW_WIDTH - 48.0, 28.0),
    );
}

pub fn draw_status_at(status_message: &str, rect: Rect) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(Color::new(0.063, 0.071, 0.082, 0.86));
    macroquad_toolkit::ui::draw_surface(rect, &surface);
    draw_ui_text_ex(
        status_message,
        rect.x + 12.0,
        rect.y + 20.0,
        TextParams {
            font_size: 20,
            color: TEXT_DIM,
            ..Default::default()
        },
    );
}

fn is_mouse_over(rect: Rect) -> bool {
    let mouse = macroquad_toolkit::ui::virtual_mouse_position(VIEW_WIDTH, VIEW_HEIGHT);
    macroquad_toolkit::input::rect_contains_point(rect, mouse)
}

#[cfg(test)]
mod tests;
