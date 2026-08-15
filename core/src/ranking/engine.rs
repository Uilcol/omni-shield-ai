#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::rules::rule::Rule;

pub struct RankingEngine;

impl RankingEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn score(&self, _rule: &Rule) -> f64 {
        0.8
    }
}
