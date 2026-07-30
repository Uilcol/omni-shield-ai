#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use omniuil_core::profiling::PipelineProfiler;

#[test]
fn latency_regression_guard() {
    let (_result, report) = PipelineProfiler::profile(|| {
        let mut total: usize = 0;
        for i in 0..10_000_usize {
            total += i;
        }

        total
    });

    let max_allowed_ms = if cfg!(debug_assertions) { 200 } else { 50 };

    assert!(
        report.analysis_time_ms <= max_allowed_ms,
        "Latency regression! Took {}ms (limit: {}ms)",
        report.analysis_time_ms,
        max_allowed_ms
    );
}
