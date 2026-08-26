use super::*;

#[test]
fn summary_reports_average_p95_and_max_without_losing_outliers() {
    let mut timing = FrameTiming::default();
    for micros in 1..=100 {
        timing.record(Duration::from_micros(micros));
    }

    let sample = timing.summarize("tower", 1280, 720);
    assert_eq!(sample.frames, 100);
    assert_eq!(sample.average_cpu_micros, 50);
    assert_eq!(sample.p95_cpu_micros, 95);
    assert_eq!(sample.max_cpu_micros, 100);
    assert_eq!((sample.width, sample.height), (1280, 720));
}

#[test]
fn empty_summary_is_safe_and_zeroed() {
    let sample = FrameTiming::default().summarize("empty", 1, 1);
    assert_eq!(sample.frames, 0);
    assert_eq!(sample.average_cpu_micros, 0);
    assert_eq!(sample.p95_cpu_micros, 0);
    assert_eq!(sample.max_cpu_micros, 0);
}
