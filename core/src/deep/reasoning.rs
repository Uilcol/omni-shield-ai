use crate::analysis::cfg::CFG;

pub struct ReasoningEngine;

impl ReasoningEngine {
    pub fn analyze(cfg: &CFG) -> Vec<String> {
        let mut findings = Vec::new();

        for node in &cfg.nodes {
            if node.code.contains("eval") {
                findings.push(format!("⚠️ Possible RCE via eval at line {}", node.id));
            }

            if node.code.contains("SELECT") && node.code.contains("+") {
                findings.push(format!("⚠️ Possible SQL Injection at line {}", node.id));
            }
        }

        findings
    }
}
