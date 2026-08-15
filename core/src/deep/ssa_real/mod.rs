use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SSAVariable {
    pub original: String,
    pub version: usize,
}

pub struct SSAReal;

impl SSAReal {
    pub fn transform(code: &str) -> Vec<String> {
        let mut versions: HashMap<String, usize> =
            HashMap::new();

        let mut output = Vec::new();

        for line in code.lines() {
            let l = line.trim();

            if l.contains("=") {
                let parts: Vec<&str> =
                    l.split('=').collect();

                if parts.len() >= 2 {
                    let var =
                        parts[0].trim().replace("let ", "");

                    let expr = parts[1].trim();

                    let version =
                        versions.entry(var.clone()).or_insert(0);

                    *version += 1;

                    output.push(format!(
                        "{}_{} = {}",
                        var,
                        version,
                        expr
                    ));
                }
            }
        }

        output
    }
}
