use omniuil_core::security_graph::SecurityNodeKind;
use omniuil_core::security_graph_builder::SecurityGraphBuilder;

#[test]
fn builds_python_source_variable_function_and_sink_graph() {
    let code = concat!(
        "def run(value):\n",
        "    query = \"SELECT * FROM users WHERE id=\" + value\n",
        "    cursor.execute(query)\n",
        "\n",
        "user_input = input()\n",
        "run(user_input)\n",
    );

    let graph = SecurityGraphBuilder::from_python(code, "fixture.py");

    assert!(graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Function && node.label == "run"));

    assert!(graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Source));

    assert!(graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Sink && node.label.contains("cursor.execute")));
}

#[test]
fn builds_clean_graph_without_security_sink() {
    let code = concat!(
        "def add(a, b):\n",
        "    return a + b\n",
        "\n",
        "result = add(1, 2)\n",
    );

    let graph = SecurityGraphBuilder::from_python(code, "clean.py");

    assert!(!graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Sink));
}

#[test]
fn connects_function_to_call_and_call_to_sink() {
    let code = concat!("def run(value):\n", "    eval(value)\n",);

    let graph = SecurityGraphBuilder::from_python(code, "eval.py");

    let paths = graph.source_to_sink_paths("eval.py:source:assignment:1:1", "eval.py:sink:2", 8);

    let _ = paths;

    assert!(graph
        .nodes()
        .any(|node| node.kind == SecurityNodeKind::Call && node.label.starts_with("eval(")));

    assert!(graph
        .edges()
        .iter()
        .any(|edge| edge.kind == omniuil_core::security_graph::SecurityEdgeKind::Reaches));
}

#[test]
fn finds_source_to_sink_path_in_python_code() {
    let code = concat!("user_input = input()\n", "eval(user_input)\n",);

    let paths = SecurityGraphBuilder::find_python_source_sink_paths(code, "fixture.py", 8);

    assert!(
        !paths.is_empty(),
        "expected at least one source-to-sink path, got: {paths:?}"
    );

    assert!(
        paths.iter().any(|path| {
            path.iter().any(|id| id.contains(":source:"))
                && path.iter().any(|id| id.contains(":variable:user_input:1"))
                && path.iter().any(|id| id == "fixture.py:call:2")
                && path.iter().any(|id| id == "fixture.py:sink:2")
        }),
        "expected Source -> Variable -> Call -> Sink path, got: {paths:?}"
    );
}
