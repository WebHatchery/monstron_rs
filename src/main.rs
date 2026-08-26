#![allow(clippy::too_many_arguments)]

use macroquad::prelude::*;
use macroquad_toolkit::{capture, crash};
use std::time::Instant;

use hatchspire::game::Game;
use hatchspire::performance::{append_json_line, FrameTiming};
use hatchspire::settings::AppSettings;

fn window_conf() -> Conf {
    crash::install_crash_log("hatchspire");

    // Hand-built Conf means no automatic arming: without this the capture run
    // puts a full game window on the desktop for its whole duration.
    capture::headless::arm("HATCHSPIRE");
    let capture_mode = capture::capture_requested("HATCHSPIRE");
    let settings = AppSettings::load().unwrap_or_default();
    let (preferred_width, preferred_height) = settings.window_dimensions();

    // Built by hand (not capture::capture_window_conf) to keep sample_count: 0
    // and high_dpi: false, which the game already relies on; still honors
    // HATCHSPIRE_WINDOW_WIDTH/HEIGHT overrides for the capture harness.
    Conf {
        window_title: "Hatchspire".to_owned(),
        window_width: capture::env_i32("HATCHSPIRE_WINDOW_WIDTH", preferred_width),
        window_height: capture::env_i32("HATCHSPIRE_WINDOW_HEIGHT", preferred_height),
        window_resizable: true,
        fullscreen: if capture_mode {
            capture::env_bool("HATCHSPIRE_CAPTURE_FULLSCREEN", false)
        } else {
            settings.fullscreen
        },
        high_dpi: false,
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // Screenshot harness: when HATCHSPIRE_CAPTURE_PATH is set, seed a scene,
    // simulate deterministic frames, write a PNG, and exit.
    if let Some(configs) = capture::CaptureConfig::all_from_env("HATCHSPIRE") {
        let performance_report = capture::env_string("HATCHSPIRE_PERF_REPORT");
        for config in configs {
            game.begin_capture_scene(&config.scene);
            let mut timing = FrameTiming::default();
            capture::run_capture_once(&config, |_dt| {
                let started = Instant::now();
                game.update();
                game.draw();
                timing.record(started.elapsed());
            })
            .await;
            if let Some(path) = &performance_report {
                let sample = timing.summarize(
                    &config.scene,
                    screen_width().round().max(0.0) as u32,
                    screen_height().round().max(0.0) as u32,
                );
                append_json_line(path, &sample)
                    .unwrap_or_else(|error| panic!("could not write capture performance: {error}"));
            }
        }
        return;
    }

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
