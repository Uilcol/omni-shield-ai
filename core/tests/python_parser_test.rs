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
    let code = concat!("eval(first)\n", "print('safe')\n", "eval(second)\n",);

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

#[test]
fn finds_real_os_system_call_lines() {
    let code = concat!(
        "# os.system(user_input)\n",
        "message = \"os.system(user_input)\"\n",
        "\n",
        "user_input = input()\n",
        "os.system(user_input)\n",
    );

    let lines = PythonParser::find_calls(code, "os.system");

    assert_eq!(lines, vec![5]);
}

#[test]
fn ignores_text_containing_os_system() {
    let code = concat!(
        "text = \"os.system(user_input)\"\n",
        "# os.system(user_input)\n",
        "value = \"safe\"\n",
    );

    let lines = PythonParser::find_calls(code, "os.system");

    assert!(lines.is_empty());
}

#[test]
fn finds_sql_injection_from_string_concatenation() {
    let code = concat!(
        "query = \"SELECT * FROM users WHERE id=\" + user_input\n",
        "safe = \"SELECT * FROM users\"\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert_eq!(lines, vec![1]);
}

#[test]
fn finds_sql_injection_from_f_string() {
    let code = concat!(
        "user_input = input()\n",
        "query = f\"SELECT * FROM users WHERE id={user_input}\"\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert_eq!(lines, vec![2]);
}

#[test]
fn ignores_static_sql_and_sql_text() {
    let code = concat!(
        "query = \"SELECT * FROM users\"\n",
        "text = \"SELECT ... + user_input\"\n",
        "# SELECT * FROM users + user_input\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert!(lines.is_empty());
}

#[test]
fn finds_sql_injection_through_local_tainted_variable() {
    let code = concat!(
        "user_id = input()\n",
        "query = \"SELECT * FROM users WHERE id=\" + user_id\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert_eq!(lines, vec![2]);
}

#[test]
fn ignores_taint_words_inside_static_sql_strings() {
    let code = concat!(
        "text = \"SELECT ... + user_input\"\n",
        "safe = \"SELECT * FROM users WHERE id=1\"\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert!(lines.is_empty());
}

#[test]
fn finds_sql_injection_through_chained_taint() {
    let code = concat!(
        "raw = input()\n",
        "user_id = raw\n",
        "query = \"SELECT * FROM users WHERE id=\" + user_id\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert_eq!(lines, vec![3]);
}

#[test]
fn ignores_unrelated_local_variables() {
    let code = concat!(
        "raw = input()\n",
        "safe_id = 42\n",
        "query = \"SELECT * FROM users WHERE id=\" + safe_id\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert!(lines.is_empty());
}

#[test]
fn finds_sql_injection_through_tainted_function_return() {
    let code = concat!(
        "def get_user_id():\n",
        "    return input()\n",
        "\n",
        "user_id = get_user_id()\n",
        "query = \"SELECT * FROM users WHERE id=\" + user_id\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert_eq!(lines, vec![5]);
}

#[test]
fn ignores_clean_function_return_before_sql_sink() {
    let code = concat!(
        "def get_user_id():\n",
        "    return 42\n",
        "\n",
        "user_id = get_user_id()\n",
        "query = \"SELECT * FROM users WHERE id=\" + user_id\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert!(lines.is_empty());
}

#[test]
fn finds_sql_injection_through_function_argument_return() {
    let code = concat!(
        "def passthrough(value):\n",
        "    return value\n",
        "\n",
        "raw = input()\n",
        "user_id = passthrough(raw)\n",
        "query = \"SELECT * FROM users WHERE id=\" + user_id\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert_eq!(lines, vec![6]);
}

#[test]
fn ignores_clean_function_argument_return() {
    let code = concat!(
        "def passthrough(value):\n",
        "    return value\n",
        "\n",
        "safe_id = 42\n",
        "user_id = passthrough(safe_id)\n",
        "query = \"SELECT * FROM users WHERE id=\" + user_id\n",
    );

    let lines = PythonParser::find_sql_injection_lines(code);

    assert!(lines.is_empty());
}
