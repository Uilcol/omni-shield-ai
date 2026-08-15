#[derive(Debug, Clone)]
pub struct ASTPattern {
    pub pattern: String,
}

pub struct ASTMatcher;

impl ASTMatcher {
    pub fn find(code: &str, pattern: ASTPattern) -> Vec<String> {
        let mut results = Vec::new();

        for line in code.lines() {
            let l = line.trim();

            if l.contains(&pattern.pattern) {
                results.push(l.to_string());
            }
        }

        results
    }
}
