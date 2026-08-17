use std::fs;
use std::path::PathBuf;

use omniuil_core::report::sarif;
use omniuil_core::runtime::executor::RuntimeExecutor;

fn severity_rank(severity: &str) -> Option<u8> {
    match severity.to_ascii_lowercase().as_str() {
        "info" => Some(0),
        "low" => Some(1),
        "medium" => Some(2),
        "high" => Some(3),
        "critical" => Some(4),
        _ => None,
    }
}

fn parse_fail_on(value: Option<&str>) -> Result<Option<u8>, String> {
    match value {
        None => Ok(None),
        Some(value) => severity_rank(value).map(Some).ok_or_else(|| {
            format!(
                "Invalid --fail-on value '{}'. Use: info, low, medium, high, critical.",
                value
            )
        }),
    }
}

pub fn run(
    path: PathBuf,
    _threshold: Option<String>,
    fail_on: Option<String>,
    sarif_path: Option<PathBuf>,
) -> i32 {
    let target = path.to_string_lossy().to_string();

    println!("===============================");
    println!(" OmniShield AI Security Scan");
    println!("===============================");
    println!();

    println!("Target: {}", target);
    println!();

    let fail_threshold = match parse_fail_on(fail_on.as_deref()) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            return 2;
        }
    };

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

    if let Some(path) = sarif_path {
        match fs::write(&path, sarif::generate(&findings)) {
            Ok(()) => {
                println!();
                println!("SARIF report: {}", path.display());
            }
            Err(error) => {
                eprintln!("Error writing SARIF report '{}': {}", path.display(), error);
                return 2;
            }
        }
    }

    println!();
    println!("Scan complete.");
    println!("Total findings: {}", findings.len());

    if let Some(threshold) = fail_threshold {
        let blocking = findings.iter().any(|finding| {
            severity_rank(&finding.severity)
                .map(|severity| severity >= threshold)
                .unwrap_or(false)
        });

        if blocking {
            return 1;
        }
    }

    0
}
