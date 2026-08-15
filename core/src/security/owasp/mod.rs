pub struct OwaspMapper;

impl OwaspMapper {
    pub fn map(issue: &str) -> &'static str {
        if issue.contains("SQL") {
            "OWASP A03:2021 - Injection"
        } else if issue.contains("Command") {
            "OWASP A03:2021 - Injection"
        } else {
            "OWASP A04:2021 - Insecure Design"
        }
    }
}
