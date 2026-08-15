use crate::deep::interprocedural::InterproceduralAnalyzer;
use crate::deep::taint_engine::TaintEngine;
use crate::findings::finding::Finding;
use std::fs;

pub fn scan(path: &str) -> Vec<Finding> {
    let content = fs::read_to_string(path).unwrap_or_default();

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let taint_flows = TaintEngine::analyze(lines.clone());
    let function_flows = InterproceduralAnalyzer::analyze(lines.clone());

    let mut findings = Vec::new();

    for flow in taint_flows {
        findings.push(Finding {
            id: "TAINT-001".to_string(),
            title: format!(
                "Unsafe tainted data flow detected: {} → {}",
                flow.source, flow.sink
            ),
            severity: "Critical".to_string(),
            cwe: "CWE-20".to_string(),
            owasp: "A03:2021 - Injection".to_string(),
            confidence: 0.99,
            file: path.to_string(),
            line: 1,
            evidence: flow.path.join(" -> "),
            recommendation: format!(
                "Sanitize input '{}' before reaching '{}'",
                flow.source, flow.sink
            ),
        });
    }

    for func in function_flows {
        if func.returns_tainted {
            findings.push(Finding {
                id: "INTERPROC-001".to_string(),
                title: format!("Function '{}' returns tainted data", func.function_name),
                severity: "High".to_string(),
                cwe: "CWE-20".to_string(),
                owasp: "A03:2021 - Injection".to_string(),
                confidence: 0.96,
                file: path.to_string(),
                line: 1,
                evidence: format!(
                    "Function '{}' propagates user-controlled input",
                    func.function_name
                ),
                recommendation: format!(
                    "Validate or sanitize return values from '{}'",
                    func.function_name
                ),
            });
        }
    }

    findings
}
