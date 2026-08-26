//! Opt-in CPU timing for deterministic release-capture scenes.

use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PerformanceSample {
    pub scene: String,
    pub frames: usize,
    pub width: u32,
    pub height: u32,
    pub average_cpu_micros: u64,
    pub p95_cpu_micros: u64,
    pub max_cpu_micros: u64,
}

#[derive(Default)]
pub struct FrameTiming {
    cpu_micros: Vec<u64>,
}

impl FrameTiming {
    pub fn record(&mut self, duration: Duration) {
        self.cpu_micros
            .push(duration.as_micros().min(u128::from(u64::MAX)) as u64);
    }

    pub fn summarize(&self, scene: &str, width: u32, height: u32) -> PerformanceSample {
        let mut sorted = self.cpu_micros.clone();
        sorted.sort_unstable();
        let frames = sorted.len();
        let total = sorted.iter().map(|value| u128::from(*value)).sum::<u128>();
        let average = if frames == 0 {
            0
        } else {
            (total / frames as u128).min(u128::from(u64::MAX)) as u64
        };
        let p95_index = frames.saturating_mul(95).div_ceil(100).saturating_sub(1);

        PerformanceSample {
            scene: scene.to_owned(),
            frames,
            width,
            height,
            average_cpu_micros: average,
            p95_cpu_micros: sorted.get(p95_index).copied().unwrap_or(0),
            max_cpu_micros: sorted.last().copied().unwrap_or(0),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn append_json_line(path: &str, sample: &PerformanceSample) -> Result<(), String> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| format!("performance report open failed: {error}"))?;
    let json = serde_json::to_string(sample)
        .map_err(|error| format!("performance report serialization failed: {error}"))?;
    writeln!(file, "{json}").map_err(|error| format!("performance report write failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
pub fn append_json_line(_path: &str, _sample: &PerformanceSample) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests;
