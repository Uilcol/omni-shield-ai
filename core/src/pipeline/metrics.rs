#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct PipelineMetrics {
    pub analysis_time_ms: u64,
    pub findings_count: usize,
    pub false_positive_estimate: f64,
}
