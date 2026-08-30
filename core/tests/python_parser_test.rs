use omniuil_core::languages::python::PythonParser;

#[test]
fn finds_real_eval_call_line() {
    let code = concat!(
        "# eval(user_input)\n",
        "message = \"eval(user_input)\"\n",
        "\n",
        "user_input = input()\n",
        "eval(user_input)\n",
    );

    let lines = PythonParser::find_calls(code, "eval");

    assert_eq!(lines, vec![5]);
}

#[test]
fn finds_multiple_eval_calls() {
    let code = concat!(
        "eval(first)\n",
        "print('safe')\n",
        "eval(second)\n",
    );

    let lines = PythonParser::find_calls(code, "eval");

    assert_eq!(lines, vec![1, 3]);
}

#[test]
fn ignores_text_containing_eval() {
    let code = concat!(
        "text = \"eval(user_input)\"\n",
        "# eval(user_input)\n",
        "value = \"safe\"\n",
    );

    let lines = PythonParser::find_calls(code, "eval");

    assert!(lines.is_empty());
}
