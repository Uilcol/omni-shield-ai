#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::fpsl::confidence::ConfidenceEngine;

pub struct KernelStabilizationEngine;

impl KernelStabilizationEngine {
    /// Normalize detection confidence for production safety
    pub fn stabilize_confidence(
        is_source_match: bool,
        is_sink_match: bool,
        path_length: usize,
        has_sanitizer: bool,
        historical_fp_rate: f64,
    ) -> f64 {
        let confidence = ConfidenceEngine::evaluate_kernel(
            is_source_match,
            is_sink_match,
            path_length,
            has_sanitizer,
            historical_fp_rate,
        );

        // Production normalization
        let mut score = confidence.score;

        // Hard clamp for runtime safety
        if score < 0.0 {
            score = 0.0;
        }

        if score > 1.0 {
            score = 1.0;
        }

        score
    }
}
