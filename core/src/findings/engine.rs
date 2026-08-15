use crate::findings::finding::Finding;

pub struct FindingEngine;

impl FindingEngine {
    pub fn run(source_code: &str, file: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        if source_code.contains("eval(") {
            findings.push(Finding::new(
                "PY-EVAL-001",
                "Use of eval() detected",
                "High",
                "CWE-95",
                "A03:2021 - Injection",
                0.98,
                file,
                1,
                "eval(user_input)",
                "Avoid eval(); use safe parsing/validation instead.",
            ));
        }

        if source_code.contains("os.system(") {
            findings.push(Finding::new(
                "PY-CMD-001",
                "Possible Command Injection",
                "Critical",
                "CWE-78",
                "A03:2021 - Injection",
                0.99,
                file,
                1,
                "os.system(user_input)",
                "Avoid shell execution with untrusted input. Use safe APIs.",
            ));
        }

        if source_code.contains("SELECT * FROM") && source_code.contains("+ user_input") {
            findings.push(Finding::new(
                "PY-SQLI-001",
                "Possible SQL Injection",
                "Critical",
                "CWE-89",
                "A03:2021 - Injection",
                0.97,
                file,
                1,
                "query concatenation with user input",
                "Use parameterized queries / prepared statements.",
            ));
        }

        if source_code.contains("password =") {
            findings.push(Finding::new(
                "PY-SECRET-001",
                "Possible hardcoded secret detected",
                "High",
                "CWE-798",
                "A02:2021 - Cryptographic Failures",
                0.96,
                file,
                1,
                "hardcoded password",
                "Move secrets to environment variables or secret manager.",
            ));
        }

        if source_code.contains("md5(") {
            findings.push(Finding::new(
                "PY-CRYPTO-001",
                "Weak cryptography (MD5) detected",
                "Medium",
                "CWE-327",
                "A02:2021 - Cryptographic Failures",
                0.94,
                file,
                1,
                "hashlib.md5(...)",
                "Use SHA-256, Argon2, bcrypt, or stronger algorithms.",
            ));
        }

        findings
    }
}
