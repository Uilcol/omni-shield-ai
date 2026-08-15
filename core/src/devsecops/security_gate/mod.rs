use crate::risk::scoring::RiskScore;
use crate::devsecops::policy_engine::PolicyEngine;

pub struct SecurityGate;

impl SecurityGate {
    pub fn should_block(items: &[RiskScore]) -> bool {
        let policy = PolicyEngine::default_policy();

        for item in items {
            if item.final_score >= policy.minimum_score_block {
                println!(
                    "[BLOCKED] {} score={}",
                    item.issue,
                    item.final_score
                );
                return true;
            }
        }

        println!("[PASSED] Security gate approved");
        false
    }
}
