use omniuil_core::deep::smt_guard::SmtGuard;
use omniuil_core::findings::finding::Finding;
use omniuil_core::security_graph::{SecurityNodeKind, SecurityPath, SecurityPathStep};
use omniuil_core::smt::path_validator::PathFeasibilityValidator;

fn step(id: &str, kind: SecurityNodeKind, line: usize) -> SecurityPathStep {
    SecurityPathStep {
        node_id: id.to_string(),
        kind,
        label: id.to_string(),
        file: "fixture.py".to_string(),
        line,
    }
}

fn unsanitized_path() -> SecurityPath {
    SecurityPath::new(vec![
        step("source", SecurityNodeKind::Source, 1),
        step("variable", SecurityNodeKind::Variable, 1),
        step("sink", SecurityNodeKind::Sink, 2),
    ])
}

fn sanitized_path() -> SecurityPath {
    SecurityPath::new(vec![
        step("source", SecurityNodeKind::Source, 1),
        step("variable", SecurityNodeKind::Variable, 1),
        step("sanitizer", SecurityNodeKind::Sanitizer, 2),
        step("sink", SecurityNodeKind::Sink, 3),
    ])
}

#[test]
fn proves_unsanitized_source_to_sink_path_satisfiable() {
    let result = PathFeasibilityValidator::validate(&unsanitized_path());

    assert!(result.satisfiable);
    assert!(result.validated);
    assert!(!result.sanitized);
    assert!(result.confidence_boost > 0.0);
}

#[test]
fn proves_sanitized_path_not_exploitable_under_current_model() {
    let result = PathFeasibilityValidator::validate(&sanitized_path());

    assert!(!result.satisfiable);
    assert!(result.validated);
    assert!(result.sanitized);
    assert!(result.confidence_boost < 0.0);
}

#[test]
fn smt_guard_removes_structurally_sanitized_finding() {
    let mut finding = Finding::new(
        "TAINT-001",
        "Unsafe tainted flow",
        "Critical",
        "CWE-20",
        "A03:2021 - Injection",
        0.99,
        "fixture.py",
        3,
        "source -> sanitizer -> sink",
        "Review flow",
    );

    finding.security_path = Some(sanitized_path());

    let filtered = SmtGuard::filter_false_positives(vec![finding]);

    assert!(filtered.is_empty());
}

#[test]
fn smt_guard_keeps_unsanitized_security_path() {
    let mut finding = Finding::new(
        "TAINT-001",
        "Unsafe tainted flow",
        "Critical",
        "CWE-20",
        "A03:2021 - Injection",
        0.99,
        "fixture.py",
        2,
        "source -> sink",
        "Review flow",
    );

    finding.security_path = Some(unsanitized_path());

    let filtered = SmtGuard::filter_false_positives(vec![finding]);

    assert_eq!(filtered.len(), 1);
}
