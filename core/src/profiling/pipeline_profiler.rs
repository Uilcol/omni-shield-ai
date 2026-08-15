#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::time::Instant;

pub struct PipelineProfileReport {
    pub analysis_time_ms: u128,
    pub findings_count: usize,
}

pub struct PipelineProfiler;

impl PipelineProfiler {
    pub fn profile<F, T>(analysis_fn: F) -> (T, PipelineProfileReport)
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = analysis_fn();
        let duration = start.elapsed();
        (
            result,
            PipelineProfileReport {
                analysis_time_ms: duration.as_millis(),
                findings_count: 0,
            },
        )
    }
}
