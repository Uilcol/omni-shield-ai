use crate::deep::dataflow::DataFlow;

pub struct SQLiDetector;

impl SQLiDetector {
    pub fn detect(code: &str) -> Vec<String> {
        let mut df = DataFlow::new();
        let mut vulns = vec![];

        for line in code.lines() {
            if line.contains("input(") {
                df.taint("user_input", "input");
            }

            if line.contains("query") && df.is_tainted("user_input") {
                vulns.push("SQL Injection detected".to_string());
            }
        }

        vulns
    }
}
