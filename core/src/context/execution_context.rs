#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::cache::runtime_cache::RuntimeKernelMemoryCache;
use crate::pipeline::metrics::PipelineMetrics;

pub struct GlobalExecutionContext {
    pub cache: RuntimeKernelMemoryCache,
    pub historical_fp_rate: f64,
    pub adaptive_factor: f64,
}

impl GlobalExecutionContext {
    pub fn new() -> Self {
        Self {
            cache: RuntimeKernelMemoryCache::new(),
            historical_fp_rate: 0.0,
            adaptive_factor: 1.0,
        }
    }

    /// Decide whether pipeline execution should continue
    pub fn should_continue_execution(&self, metrics: &PipelineMetrics) -> bool {
        if metrics.analysis_time_ms > 5000 {
            return false;
        }

        if self.historical_fp_rate > 0.8 {
            return false;
        }

        true
    }
}
