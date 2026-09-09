use crate::deep::querydsl;
use crate::deep::smt_guard::SmtGuard;
use crate::findings::dedup::DedupEngine;
use crate::findings::finding::Finding;
use crate::languages::python::PythonParser;
use std::fs;
use std::path::Path;

pub struct RuntimeExecutor;

impl RuntimeExecutor {
    pub fn execute(path: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        Self::scan_recursive(path, &mut findings);

        let validated = SmtGuard::filter_false_positives(findings);
        DedupEngine::deduplicate(validated)
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

            if file_path.ends_with(".py") {
                for line in PythonParser::find_calls(&content, "eval") {
                    findings.push(Self::build_finding(
                        "PY-EVAL-001",
                        "Use of eval() detected",
                        "High",
                        &file_path,
                        line,
                        "eval(...)",
                    ));
                }
            }

            if file_path.ends_with(".py") {
                for line in PythonParser::find_calls(&content, "os.system") {
                    findings.push(Self::build_finding(
                        "PY-CMD-001",
                        "Possible Command Injection",
                        "Critical",
                        &file_path,
                        line,
                        "os.system(...)",
                    ));
                }
            }

            if file_path.ends_with(".py") {
                for line in PythonParser::find_sql_injection_lines(&content) {
                    findings.push(Self::build_finding(
                        "PY-SQLI-001",
                        "Possible SQL Injection",
                        "Critical",
                        &file_path,
                        line,
                        "query construction with user-controlled input",
                    ));
                }
            }

            if let Some(line) = Self::find_line(&content, "password =")
                .or_else(|| Self::find_line(&content, "api_key ="))
            {
                findings.push(Self::build_finding(
                    "PY-SECRET-001",
                    "Possible hardcoded secret detected",
                    "High",
                    &file_path,
                    line,
                    "hardcoded password",
                ));
            }

            if let Some(line) = Self::find_line(&content, "hashlib.md5") {
                findings.push(Self::build_finding(
                    "PY-CRYPTO-001",
                    "Weak cryptography (MD5) detected",
                    "Medium",
                    &file_path,
                    line,
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

    fn find_line(content: &str, needle: &str) -> Option<usize> {
        content
            .lines()
            .position(|line| line.contains(needle))
            .map(|index| index + 1)
    }

    fn build_finding(
        id: &str,
        title: &str,
        severity: &str,
        file: &str,
        line: usize,
        evidence: &str,
    ) -> Finding {
        Finding {
            id: id.to_string(),
            title: title.to_string(),
            severity: severity.to_string(),
            cwe: "CWE".to_string(),
            owasp: "OWASP".to_string(),
            confidence: 0.95,
            file: file.to_string(),
            line,
            evidence: evidence.to_string(),
            recommendation: "Review and remediate securely.".to_string(),
            security_path: None,
        }
    }
}
