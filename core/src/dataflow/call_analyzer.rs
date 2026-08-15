#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct CallAnalyzer;

impl CallAnalyzer {

    pub fn extract_calls(line: &str) -> Vec<String> {

        // 🔥 simples: detecta "func(...)"
        let mut calls = Vec::new();

        let parts: Vec<&str> = line.split('(').collect();

        if parts.len() > 1 {
            let name = parts[0].trim();

            if !name.is_empty() {
                calls.push(name.to_string());
            }
        }

        calls
    }
}
