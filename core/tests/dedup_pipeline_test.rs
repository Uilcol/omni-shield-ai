use omniuil_core::findings::dedup::DedupEngine;
use omniuil_core::findings::finding::Finding;

fn finding(id: &str, file: &str, line: usize, evidence: &str) -> Finding {
    Finding {
        id: id.to_string(),
        title: "test".to_string(),
        severity: "High".to_string(),
        cwe: "CWE".to_string(),
        owasp: "OWASP".to_string(),
        confidence: 0.95,
        file: file.to_string(),
        line,
        evidence: evidence.to_string(),
        recommendation: "test".to_string(),
    }
}

#[test]
fn removes_exact_duplicates() {
    let a = finding("TAINT-001", "test.py", 10, "input -> eval");
    let b = a.clone();

    let result = DedupEngine::deduplicate(vec![a, b]);

    assert_eq!(result.len(), 1);
}

#[test]
fn preserves_distinct_findings_with_different_evidence() {
    let eval = finding("TAINT-001", "test.py", 10, "input -> eval");
    let command = finding("TAINT-001", "test.py", 10, "input -> os.system");

    let result = DedupEngine::deduplicate(vec![eval, command]);

    assert_eq!(result.len(), 2);
}
