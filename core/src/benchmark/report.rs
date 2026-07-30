#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use serde::Serialize;

#[derive(Serialize)]
pub struct BenchmarkReport {
    pub total_samples: usize,
    pub true_positive: usize,
    pub false_positive: usize,
    pub true_negative: usize,
    pub false_negative: usize,
    pub fp_rate: f64,
    pub precision: f64,
    pub recall: f64,
    pub latency_ms: u128,
}
