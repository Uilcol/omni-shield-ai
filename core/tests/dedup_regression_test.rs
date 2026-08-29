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
fn runtime_pipeline_preserves_distinct_taint_findings() {
    let dir = temp_dir("dedup-regression");
    let file = dir.join("vulnerable.py");

    fs::write(
        &file,
        concat!(
            "user_input = input()\n",
            "eval(user_input)\n",
            "os.system(user_input)\n",
        ),
    )
    .expect("failed to write fixture");

    let findings = RuntimeExecutor::execute(dir.to_str().unwrap());

    let taint_findings: Vec<_> = findings
        .iter()
        .filter(|f| f.id == "TAINT-001")
        .collect();

    assert_eq!(
        taint_findings.len(),
        2,
        "expected distinct eval and os.system taint findings"
    );

    assert!(
        taint_findings
            .iter()
            .any(|f| f.line == 2 && f.evidence.contains("eval")),
        "expected eval taint finding on line 2"
    );

    assert!(
        taint_findings
            .iter()
            .any(|f| f.line == 3 && f.evidence.contains("os.system")),
        "expected os.system taint finding on line 3"
    );

    fs::remove_dir_all(dir).expect("failed to remove temporary directory");
}
