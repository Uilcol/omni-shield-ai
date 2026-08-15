use crate::deep::querydsl;
use crate::deep::smt_guard::SmtGuard;
use crate::findings::finding::Finding;
use std::fs;
use std::path::Path;

pub struct RuntimeExecutor;

impl RuntimeExecutor {
    pub fn execute(path: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        Self::scan_recursive(path, &mut findings);

        let validated = SmtGuard::filter_false_positives(findings);

        validated
    }

    fn scan_recursive(path: &str, findings: &mut Vec<Finding>) {
        let dir = Path::new(path);

        if !dir.exists() {
            return;
        }

        if dir.is_file() {
            let file_path = path.to_string();

            let deep_findings = querydsl::scan(&file_path);
            findings.extend(deep_findings);

            let content = fs::read_to_string(&file_path).unwrap_or_default();

            if content.contains("eval(") {
                findings.push(Self::build_finding(
                    "PY-EVAL-001",
                    "Use of eval() detected",
                    "High",
                    &file_path,
                    "eval(user_input)",
                ));
            }

            if content.contains("os.system(") {
                findings.push(Self::build_finding(
                    "PY-CMD-001",
                    "Possible Command Injection",
                    "Critical",
                    &file_path,
                    "os.system(user_input)",
                ));
            }

            if content.contains("SELECT ") && content.contains("+ user_input") {
                findings.push(Self::build_finding(
                    "PY-SQLI-001",
                    "Possible SQL Injection",
                    "Critical",
                    &file_path,
                    "query concatenation with user input",
                ));
            }

            if content.contains("password =") || content.contains("api_key =") {
                findings.push(Self::build_finding(
                    "PY-SECRET-001",
                    "Possible hardcoded secret detected",
                    "High",
                    &file_path,
                    "hardcoded password",
                ));
            }

            if content.contains("hashlib.md5") {
                findings.push(Self::build_finding(
                    "PY-CRYPTO-001",
                    "Weak cryptography (MD5) detected",
                    "Medium",
                    &file_path,
                    "hashlib.md5(...)",
                ));
            }

            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(p) = entry.path().to_str() {
                    Self::scan_recursive(p, findings);
                }
            }
        }
    }

    fn build_finding(id: &str, title: &str, severity: &str, file: &str, evidence: &str) -> Finding {
        Finding {
            id: id.to_string(),
            title: title.to_string(),
            severity: severity.to_string(),
            cwe: "CWE".to_string(),
            owasp: "OWASP".to_string(),
            confidence: 0.95,
            file: file.to_string(),
            line: 1,
            evidence: evidence.to_string(),
            recommendation: "Review and remediate securely.".to_string(),
        }
    }
}
