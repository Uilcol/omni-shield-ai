use crate::findings::finding::Finding;

pub struct CheckRunReporter;

impl CheckRunReporter {
    pub fn generate(findings: &[Finding]) -> String {
        let conclusion = if findings.is_empty() {
            "success"
        } else {
            "failure"
        };

        let mut output = String::new();

        output.push_str("{\n");
        output.push_str(&format!(
            r#"  "name": "OmniUil Security Check",
  "status": "completed",
  "conclusion": "{}",
  "output": {{
    "title": "Security Scan Results",
    "summary": "{} findings detected",
    "text": "#,
            conclusion,
            findings.len()
        ));

        if findings.is_empty() {
            output.push_str(r#""No vulnerabilities detected.""#);
        } else {
            let mut text = String::new();

            for finding in findings {
                text.push_str(&format!(
                    "- {} [{}] in {}\n",
                    finding.title, finding.severity, finding.file
                ));
            }

            output.push_str(&format!(r#""{}""#, text));
        }

        output.push_str(
            r#"
  }
}
"#,
        );

        output
    }
}
