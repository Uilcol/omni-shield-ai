#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct BenchmarkMetrics {
    pub total_samples: usize,
    pub detected_vulnerabilities: usize,
    pub false_positives: usize,
    pub execution_time_ms: u128,
}

pub fn precision(metrics: &BenchmarkMetrics) -> f64 {
    if metrics.detected_vulnerabilities == 0 {
        return 0.0;
    }

    let tp = metrics.detected_vulnerabilities - metrics.false_positives;

    tp as f64 / metrics.detected_vulnerabilities as f64
}

pub fn false_positive_rate(metrics: &BenchmarkMetrics) -> f64 {
    if metrics.detected_vulnerabilities == 0 {
        return 0.0;
    }

    metrics.false_positives as f64
        / metrics.detected_vulnerabilities as f64
}
