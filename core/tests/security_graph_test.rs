use omniuil_core::security_graph::{
    SecurityEdge, SecurityEdgeKind, SecurityGraph, SecurityNode, SecurityNodeKind, SecurityPath,
};

fn node(id: &str, kind: SecurityNodeKind, label: &str, line: usize) -> SecurityNode {
    SecurityNode {
        id: id.to_string(),
        kind,
        label: label.to_string(),
        file: "fixture.py".to_string(),
        line,
    }
}

#[test]
fn builds_source_to_sink_security_path() {
    let mut graph = SecurityGraph::new();

    assert!(graph.add_node(node("source", SecurityNodeKind::Source, "input()", 1,)));

    assert!(graph.add_node(node("raw", SecurityNodeKind::Variable, "raw", 1,)));

    assert!(graph.add_node(node(
        "function",
        SecurityNodeKind::Function,
        "build_query",
        2,
    )));

    assert!(graph.add_node(node("sink", SecurityNodeKind::Sink, "cursor.execute", 3,)));

    assert!(graph.add_edge(SecurityEdge {
        from: "source".to_string(),
        to: "raw".to_string(),
        kind: SecurityEdgeKind::FlowsTo,
    }));

    assert!(graph.add_edge(SecurityEdge {
        from: "raw".to_string(),
        to: "function".to_string(),
        kind: SecurityEdgeKind::Calls,
    }));

    assert!(graph.add_edge(SecurityEdge {
        from: "function".to_string(),
        to: "sink".to_string(),
        kind: SecurityEdgeKind::Reaches,
    }));

    assert!(graph.reaches_kind("source", SecurityNodeKind::Sink));

    let paths = graph.source_to_sink_paths("source", "sink", 8);

    assert_eq!(
        paths,
        vec![vec![
            "source".to_string(),
            "raw".to_string(),
            "function".to_string(),
            "sink".to_string()
        ]]
    );
}

#[test]
fn rejects_duplicate_nodes_and_edges() {
    let mut graph = SecurityGraph::new();

    let source = node("source", SecurityNodeKind::Source, "input()", 1);

    assert!(graph.add_node(source.clone()));
    assert!(!graph.add_node(source));

    assert!(graph.add_node(node("sink", SecurityNodeKind::Sink, "execute()", 2,)));

    let edge = SecurityEdge {
        from: "source".to_string(),
        to: "sink".to_string(),
        kind: SecurityEdgeKind::Reaches,
    };

    assert!(graph.add_edge(edge.clone()));
    assert!(!graph.add_edge(edge));
}

#[test]
fn does_not_add_edges_to_unknown_nodes() {
    let mut graph = SecurityGraph::new();

    assert!(graph.add_node(node("source", SecurityNodeKind::Source, "input()", 1,)));

    assert!(!graph.add_edge(SecurityEdge {
        from: "source".to_string(),
        to: "missing".to_string(),
        kind: SecurityEdgeKind::FlowsTo,
    }));

    assert!(graph.edges().is_empty());
}

#[test]
fn builds_structured_security_path_evidence() {
    let mut graph = SecurityGraph::new();

    graph.add_node(node("source", SecurityNodeKind::Source, "input()", 1));

    graph.add_node(node(
        "variable",
        SecurityNodeKind::Variable,
        "user_input",
        1,
    ));

    graph.add_node(node("sink", SecurityNodeKind::Sink, "eval(user_input)", 2));

    graph.add_edge(SecurityEdge {
        from: "source".to_string(),
        to: "variable".to_string(),
        kind: SecurityEdgeKind::FlowsTo,
    });

    graph.add_edge(SecurityEdge {
        from: "variable".to_string(),
        to: "sink".to_string(),
        kind: SecurityEdgeKind::FlowsTo,
    });

    let paths = graph.security_paths("source", "sink", 8);

    assert_eq!(paths.len(), 1);

    let path = &paths[0];

    assert_eq!(path.len(), 3);
    assert_eq!(
        path.source().map(|step| step.label.as_str()),
        Some("input()")
    );
    assert_eq!(
        path.sink().map(|step| step.label.as_str()),
        Some("eval(user_input)")
    );
    assert_eq!(path.line_span(), Some((1, 2)));
    assert_eq!(
        path.labels(),
        vec![
            "input()".to_string(),
            "user_input".to_string(),
            "eval(user_input)".to_string(),
        ]
    );
}

#[test]
fn security_path_serializes_as_structured_finding_evidence() {
    use omniuil_core::findings::finding::Finding;
    use omniuil_core::security_graph::SecurityPathStep;

    let path = SecurityPath::new(vec![
        SecurityPathStep {
            node_id: "source".to_string(),
            kind: SecurityNodeKind::Source,
            label: "input()".to_string(),
            file: "fixture.py".to_string(),
            line: 1,
        },
        SecurityPathStep {
            node_id: "sink".to_string(),
            kind: SecurityNodeKind::Sink,
            label: "eval(user_input)".to_string(),
            file: "fixture.py".to_string(),
            line: 2,
        },
    ]);

    let finding = Finding {
        id: "TEST-001".to_string(),
        title: "Test finding".to_string(),
        severity: "High".to_string(),
        cwe: "CWE-20".to_string(),
        owasp: "A03".to_string(),
        confidence: 0.99,
        file: "fixture.py".to_string(),
        line: 2,
        evidence: "input() -> eval()".to_string(),
        recommendation: "Remediate input validation.".to_string(),
        security_path: Some(path.clone()),
    };

    let json = serde_json::to_value(&finding).expect("failed to serialize finding");

    assert!(json.get("security_path").is_some());

    let decoded: Finding = serde_json::from_value(json).expect("failed to deserialize finding");

    assert_eq!(decoded.security_path, Some(path));
}

#[test]
fn finding_without_security_path_remains_backward_compatible() {
    use omniuil_core::findings::finding::Finding;

    let json = serde_json::json!({
        "id": "TEST-002",
        "title": "Legacy finding",
        "severity": "High",
        "cwe": "CWE-20",
        "owasp": "A03",
        "confidence": 0.90,
        "file": "legacy.py",
        "line": 7,
        "evidence": "legacy evidence",
        "recommendation": "Review securely."
    });

    let finding: Finding = serde_json::from_value(json).expect("legacy finding must deserialize");

    assert!(finding.security_path.is_none());
}
