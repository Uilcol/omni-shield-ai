#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashMap;

#[derive(Debug)]
pub struct AdaptiveModelState {
    pub false_positive_counts: HashMap<String, usize>,
    pub true_positive_counts: HashMap<String, usize>,
}

impl AdaptiveModelState {
    pub fn new() -> Self {
        Self {
            false_positive_counts: HashMap::new(),
            true_positive_counts: HashMap::new(),
        }
    }

    pub fn record_false_positive(&mut self, rule_id: &str) {
        *self
            .false_positive_counts
            .entry(rule_id.to_string())
            .or_insert(0) += 1;
    }

    pub fn record_true_positive(&mut self, rule_id: &str) {
        *self
            .true_positive_counts
            .entry(rule_id.to_string())
            .or_insert(0) += 1;
    }

    pub fn false_positive_rate(&self, rule_id: &str) -> f64 {
        let fp = *self.false_positive_counts.get(rule_id).unwrap_or(&0) as f64;
        let tp = *self.true_positive_counts.get(rule_id).unwrap_or(&0) as f64;

        if fp + tp == 0.0 {
            return 0.0;
        }

        fp / (fp + tp)
    }
}
