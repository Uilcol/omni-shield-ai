#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct FindingConfidence {
    pub score: f64,
    pub explanation: String,
}

pub struct ConfidenceEngine;

impl ConfidenceEngine {
    /// Base evaluation (legacy compatible)
    pub fn evaluate(
        is_source_match: bool,
        is_sink_match: bool,
        path_length: usize,
        has_sanitizer: bool,
    ) -> FindingConfidence {
        let mut score = 0.0;

        if is_source_match {
            score += 0.3;
        }

        if is_sink_match {
            score += 0.3;
        }

        // Path length contribution (performance aware)
        score += (path_length as f64) * 0.05;

        if has_sanitizer {
            score -= 0.4;
        }

        // Kernel safety normalization
        score = ConfidenceEngine::normalize_score(score);

        let explanation = ConfidenceEngine::generate_explanation(score);

        FindingConfidence { score, explanation }
    }

    /// ⭐ Performance Optimization Kernel Adaptive Evaluation
    pub fn evaluate_kernel(
        is_source_match: bool,
        is_sink_match: bool,
        path_length: usize,
        has_sanitizer: bool,
        historical_fp_rate: f64,
    ) -> FindingConfidence {
        let mut score = 0.0;

        if is_source_match {
            score += 0.35;
        }

        if is_sink_match {
            score += 0.35;
        }

        // Performance heuristic penalty for long paths
        let path_factor = if path_length > 20 { 0.03 } else { 0.05 };

        score += (path_length as f64) * path_factor;

        if has_sanitizer {
            score -= 0.5;
        }

        // Adaptive suppression intelligence
        if historical_fp_rate > 0.5 {
            score *= 0.7;
        }

        if historical_fp_rate < 0.2 {
            score *= 1.2;
        }

        score = ConfidenceEngine::normalize_score(score);

        let explanation = ConfidenceEngine::generate_explanation(score);

        FindingConfidence { score, explanation }
    }

    /// Kernel-safe normalization
    fn normalize_score(score: f64) -> f64 {
        if score < 0.0 {
            return 0.0;
        }

        if score > 1.0 {
            return 1.0;
        }

        score
    }

    /// Explanation generator
    fn generate_explanation(score: f64) -> String {
        if score > 0.8 {
            "High confidence vulnerability candidate".to_string()
        } else if score > 0.5 {
            "Medium confidence detection".to_string()
        } else {
            "Low confidence - possible false positive".to_string()
        }
    }
}
