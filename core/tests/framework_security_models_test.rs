use omniuil_core::security::database::SecurityDatabase;
use omniuil_core::security_graph::{SecurityEdgeKind, SecurityNodeKind};
use omniuil_core::security_graph_builder::SecurityGraphBuilder;

#[test]
fn python_framework_sources_are_recognized() {
    let db = SecurityDatabase::new();

    assert!(db.is_source("request.args"));
    assert!(db.is_source("request.form"));
    assert!(db.is_source("request.json"));
    assert!(db.is_source("request.GET"));
    assert!(db.is_source("request.POST"));
}

#[test]
fn python_and_database_sinks_are_recognized() {
    let db = SecurityDatabase::new();

    assert!(db.is_sink("cursor.execute(query)"));
    assert!(db.is_sink("subprocess.run(command)"));
    assert!(db.is_sink("eval(value)"));
}

#[test]
fn sanitizers_are_recognized() {
    let db = SecurityDatabase::new();

    assert!(db.is_sanitizer("html.escape(value)"));
    assert!(db.is_sanitizer("bleach.clean(value)"));
    assert!(db.is_sanitizer("shlex.quote(value)"));
}

#[test]
fn framework_source_reaches_sink_in_security_graph() {
    let code = concat!("user_input = request.args\n", "eval(user_input)\n",);

    let graph = SecurityGraphBuilder::from_python(code, "flask_app.py");

    assert!(graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Source));

    assert!(graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Sink));

    assert!(graph
        .edges()
        .iter()
        .any(|edge| { edge.kind == SecurityEdgeKind::FlowsTo }));
}

#[test]
fn sanitizer_is_present_in_security_graph() {
    let code = concat!(
        "user_input = request.args\n",
        "safe = html.escape(user_input)\n",
    );

    let graph = SecurityGraphBuilder::from_python(code, "flask_safe.py");

    assert!(graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Sanitizer));

    assert!(graph
        .edges()
        .iter()
        .any(|edge| edge.kind == SecurityEdgeKind::Sanitizes));
}
