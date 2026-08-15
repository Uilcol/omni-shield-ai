#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::metrics::PipelineMetrics;

pub struct PipelinePlanner;

impl PipelinePlanner {
    pub fn optimize_execution(metrics: &PipelineMetrics) -> bool {
        if metrics.findings_count > 0 && metrics.false_positive_estimate < 0.5 {
            return true;
        }

        metrics.analysis_time_ms < 2000
    }
}
