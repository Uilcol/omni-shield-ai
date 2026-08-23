use crate::deep::interprocedural::InterproceduralAnalyzer;
use crate::deep::taint_engine::TaintEngine;
use crate::findings::finding::Finding;
use std::fs;

fn find_sink_line(content: &str, sink: &str) -> Option<usize> {
    let needle = if sink.contains('.') {
        sink.rsplit('.').next().unwrap_or(sink)
    } else {
        sink
    };

    content
        .lines()
        .position(|line| line.contains(needle) || line.contains(sink))
        .map(|index| index + 1)
}

fn find_path_line(content: &str, path: &[String]) -> Option<usize> {
    path.iter()
        .rev()
        .find_map(|step| find_sink_line(content, step))
}

fn find_function_line(content: &str, function_name: &str) -> Option<usize> {
    let needle = format!("def {}", function_name);

    content
        .lines()
        .position(|line| line.trim_start().starts_with(&needle))
        .map(|index| index + 1)
}

pub fn scan(path: &str) -> Vec<Finding> {
    let content = fs::read_to_string(path).unwrap_or_default();

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let taint_flows = TaintEngine::analyze(lines.clone());
    let function_flows = InterproceduralAnalyzer::analyze(lines.clone());

    let mut findings = Vec::new();

    for flow in taint_flows {
        let line = find_sink_line(&content, &flow.sink)
            .or_else(|| find_path_line(&content, &flow.path))
            .unwrap_or(1);

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
            line,
            evidence: flow.path.join(" -> "),
            recommendation: format!(
                "Sanitize input '{}' before reaching '{}'",
                flow.source, flow.sink
            ),
        });
    }

    for func in function_flows {
        if func.returns_tainted {
            let line = find_function_line(&content, &func.function_name).unwrap_or(1);

            findings.push(Finding {
                id: "INTERPROC-001".to_string(),
                title: format!("Function '{}' returns tainted data", func.function_name),
                severity: "High".to_string(),
                cwe: "CWE-20".to_string(),
                owasp: "A03:2021 - Injection".to_string(),
                confidence: 0.96,
                file: path.to_string(),
                line,
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
