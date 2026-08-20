use omniuil_core::runtime::executor::RuntimeExecutor;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir(name: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before UNIX epoch")
        .as_nanos();

    let dir = std::env::temp_dir().join(format!("omnishield-{name}-{nonce}"));
    fs::create_dir_all(&dir).expect("failed to create temp directory");
    dir
}

#[test]
fn detects_eval_and_command_injection_with_real_lines() {
    let dir = temp_dir("runtime-lines");
    let file = dir.join("vulnerable.py");

    fs::write(
        &file,
        concat!(
            "user_input = input()\n",
            "print(user_input)\n",
            "print('safe')\n",
            "eval(user_input)\n",
            "print('still running')\n",
            "os.system(user_input)\n",
        ),
    )
    .expect("failed to write fixture");

    let findings = RuntimeExecutor::execute(dir.to_str().unwrap());

    let eval = findings
        .iter()
        .find(|f| f.id == "PY-EVAL-001")
        .expect("expected PY-EVAL-001");

    let command = findings
        .iter()
        .find(|f| f.id == "PY-CMD-001")
        .expect("expected PY-CMD-001");

    assert_eq!(eval.line, 4);
    assert_eq!(command.line, 6);

    fs::remove_dir_all(dir).expect("failed to remove temp directory");
}

#[test]
fn clean_project_has_no_findings() {
    let dir = temp_dir("runtime-clean");
    let file = dir.join("main.rs");

    fs::write(&file, "fn main() { println!(\"hello\"); }\n")
        .expect("failed to write fixture");

    let findings = RuntimeExecutor::execute(dir.to_str().unwrap());

    assert!(
        findings.is_empty(),
        "expected clean project, got {} findings",
        findings.len()
    );

    fs::remove_dir_all(dir).expect("failed to remove temp directory");
}
