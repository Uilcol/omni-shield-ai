pub struct PatternExtractor;

impl PatternExtractor {
    pub fn extract(lines: &[String]) -> Vec<String> {
        let mut patterns = Vec::new();

        for line in lines {
            if line.contains("input") && line.contains("eval") {
                patterns.push("input -> eval".to_string());
            }

            if line.contains("request") && line.contains("exec") {
                patterns.push("request -> exec".to_string());
            }
        }

        patterns
    }
}
