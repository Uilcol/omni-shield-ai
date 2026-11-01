use crate::deep::querydsl;
use crate::deep::smt_guard::SmtGuard;
use crate::findings::dedup::DedupEngine;
use crate::findings::finding::Finding;
use crate::languages::python::PythonParser;
use crate::security_graph::SecurityPath;
use crate::security_graph_builder::SecurityGraphBuilder;
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

            let mut deep_findings = querydsl::scan(&file_path);
            let content = fs::read_to_string(&file_path).unwrap_or_default();

            if file_path.ends_with(".py") {
                Self::attach_security_paths(&file_path, &content, &mut deep_findings);
            }

            findings.extend(deep_findings);

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

                for line in PythonParser::find_user_controlled_file_open_lines(&content) {
                    findings.push(Self::build_finding(
                        "PY-PATH-001",
                        "User-controlled file path reaches open()",
                        "High",
                        &file_path,
                        line,
                        "open() receives a path derived from an untrusted request source",
                    ));
                }

                for line in PythonParser::find_calls(&content, "pickle.loads") {
                    findings.push(Self::build_finding(
                        "PY-DESER-001",
                        "Unsafe Deserialization detected",
                        "High",
                        &file_path,
                        line,
                        "pickle.loads(...)",
                    ));
                }

                for line in PythonParser::find_calls(&content, "yaml.load") {
                    findings.push(Self::build_finding(
                        "PY-YAML-001",
                        "Unsafe YAML deserialization detected",
                        "High",
                        &file_path,
                        line,
                        "yaml.load(...)",
                    ));
                }

                for line in PythonParser::find_calls(&content, "subprocess.call") {
                    let source_line = content.lines().nth(line.saturating_sub(1)).unwrap_or("");

                    if source_line.contains("shell=True") {
                        findings.push(Self::build_finding(
                            "PY-CMD-002",
                            "Shell command execution with shell=True",
                            "Critical",
                            &file_path,
                            line,
                            "subprocess.call(..., shell=True)",
                        ));
                    }
                }
            }

            if let Some(line) = Self::find_hardcoded_secret_line(&content) {
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

    fn attach_security_paths(file: &str, content: &str, findings: &mut Vec<Finding>) {
        let graph = SecurityGraphBuilder::from_python(content, file);

        let sources: Vec<String> = graph
            .nodes()
            .filter(|node| node.kind == crate::security_graph::SecurityNodeKind::Source)
            .map(|node| node.id.clone())
            .collect();

        let sinks: Vec<String> = graph
            .nodes()
            .filter(|node| node.kind == crate::security_graph::SecurityNodeKind::Sink)
            .map(|node| node.id.clone())
            .collect();

        if sources.is_empty() || sinks.is_empty() {
            return;
        }

        let mut paths: Vec<SecurityPath> = Vec::new();

        for source in &sources {
            for sink in &sinks {
                paths.extend(graph.security_paths(source, sink, 16));
            }
        }

        for finding in findings.iter_mut() {
            if finding.file != file {
                continue;
            }

            if !matches!(
                finding.id.as_str(),
                "TAINT-001" | "PY-EVAL-001" | "PY-CMD-001"
            ) {
                continue;
            }

            let best_path = paths
                .iter()
                .filter(|path| {
                    path.sink()
                        .map(|step| step.line == finding.line)
                        .unwrap_or(false)
                        || path.steps.iter().any(|step| step.line == finding.line)
                })
                .min_by_key(|path| path.len());

            if let Some(path) = best_path {
                finding.security_path = Some(path.clone());

                if finding.evidence.trim().is_empty() {
                    finding.evidence = path.labels().join(" -> ");
                }
            }
        }
    }

    fn find_hardcoded_secret_line(content: &str) -> Option<usize> {
        for (index, line) in content.lines().enumerate() {
            let lower = line.to_ascii_lowercase();

            if !lower.contains("password") && !lower.contains("api_key") {
                continue;
            }

            let Some(eq) = line.find('=') else {
                continue;
            };

            let rhs = line[eq + 1..].trim();

            if rhs.starts_with('"') || rhs.starts_with('\'') {
                return Some(index + 1);
            }
        }

        None
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
