use crate::findings::finding::Finding;
use crate::policy::config::PolicyConfigLoader;
use crate::policy::policy_engine::PolicyEngine;

pub struct SecurityGate;

impl SecurityGate {
    pub fn should_fail_pipeline(findings: &[Finding]) -> bool {
        let config = PolicyConfigLoader::load("omniuil-policy.yml");
        let result = PolicyEngine::evaluate(findings);

        for finding in findings {
            if config.block_severity.contains(&finding.severity) {
                return true;
            }
        }

        result.should_block_pipeline
    }

    pub fn print_gate_result(findings: &[Finding]) {
        println!("\n======================================");
        println!("ENTERPRISE SECURITY GATE");
        println!("======================================");

        if Self::should_fail_pipeline(findings) {
            println!("STATUS: FAILED");
            println!("Reason: Policy violation detected.");
            println!("Pipeline should be blocked.");
        } else {
            println!("STATUS: PASSED");
            println!("No blocking policy violations found.");
        }

        println!("======================================\n");
    }
}
