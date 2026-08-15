use crate::findings::finding::Finding;

pub struct SmtGuard;

impl SmtGuard {
    pub fn filter_false_positives(findings: Vec<Finding>) -> Vec<Finding> {
        let mut validated = Vec::new();

        for finding in findings {
            let evidence = finding.evidence.to_lowercase();

            let appears_sanitized = evidence.contains("sanitize(")
                || evidence.contains("escape(")
                || evidence.contains("prepared statement")
                || evidence.contains("parameterized")
                || evidence.contains("validator")
                || evidence.contains("schema.validate");

            if !appears_sanitized {
                validated.push(finding);
            }
        }

        validated
    }
}
