#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::pipeline::metrics::PipelineMetrics;

pub struct KernelOptimizationEngine;

impl KernelOptimizationEngine {
    /// Decide whether pipeline execution is worth it
    pub fn should_execute(metrics: &PipelineMetrics) -> bool {
        // Low analysis time → allow execution
        if metrics.analysis_time_ms < 2000 {
            return true;
        }

        // High findings count may justify deeper scan
        if metrics.findings_count > 0 {
            return true;
        }

        false
    }
}
