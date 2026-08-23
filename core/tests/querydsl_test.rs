use omniuil_core::deep::querydsl;
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
fn interprocedural_finding_uses_function_declaration_line() {
    let dir = temp_dir("interproc-line");
    let file = dir.join("interproc.py");

    fs::write(
        &file,
        concat!(
            "\n",
            "\n",
            "# offset\n",
            "# offset\n",
            "\n",
            "def get_user_value():\n",
            "    user_input = input(\"value: \")\n",
            "    return user_input\n",
        ),
    )
    .expect("failed to write fixture");

    let findings = querydsl::scan(file.to_str().unwrap());

    let finding = findings
        .iter()
        .find(|f| f.id == "INTERPROC-001")
        .expect("expected INTERPROC-001");

    assert_eq!(finding.line, 6);

    fs::remove_dir_all(dir).expect("failed to remove temp directory");
}

#[test]
fn interprocedural_scan_does_not_emit_finding_for_clean_function() {
    let dir = temp_dir("interproc-clean");
    let file = dir.join("clean.py");

    fs::write(
        &file,
        concat!(
            "def get_constant():\n",
            "    value = 42\n",
            "    return value\n",
        ),
    )
    .expect("failed to write fixture");

    let findings = querydsl::scan(file.to_str().unwrap());

    assert!(
        findings.iter().all(|f| f.id != "INTERPROC-001"),
        "unexpected INTERPROC-001 finding"
    );

    fs::remove_dir_all(dir).expect("failed to remove temp directory");
}
