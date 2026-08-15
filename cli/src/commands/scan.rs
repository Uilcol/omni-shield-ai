use std::path::PathBuf;
use std::process;

use omniuil_core::runtime::executor::RuntimeExecutor;

pub fn run(path: PathBuf, _threshold: Option<String>, _fail_on: Option<String>) {
    let target = path.to_string_lossy().to_string();

    println!("===============================");
    println!(" OmniShield AI Security Scan");
    println!("===============================");
    println!();

    println!("Target: {}", target);
    println!();

    let findings = RuntimeExecutor::execute(&target);

    if findings.is_empty() {
        println!("No security issues found.");
    } else {
        println!("Security Findings:");
        println!();

        for finding in &findings {
            println!(
                "[{}] {}:{} - {} ({})",
                finding.severity, finding.file, finding.line, finding.title, finding.id
            );
        }
    }

    println!();
    println!("Scan complete.");
    println!("Total findings: {}", findings.len());

    if findings.iter().any(|_| false) {
        process::exit(1);
    }
}
