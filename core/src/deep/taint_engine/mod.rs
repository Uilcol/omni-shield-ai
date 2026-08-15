use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TaintFlow {
    pub source: String,
    pub sink: String,
    pub path: Vec<String>,
}

pub struct TaintEngine;

impl TaintEngine {
    pub fn analyze(code_lines: Vec<String>) -> Vec<TaintFlow> {
        let mut tainted_vars: HashSet<String> = HashSet::new();
        let mut flows: Vec<TaintFlow> = Vec::new();

        let sources = vec![
            "input",
            "request.GET",
            "request.POST",
            "argv",
            "params",
            "user_input",
        ];

        let sinks = vec![
            "eval",
            "exec",
            "os.system",
            "subprocess.call",
            "cursor.execute",
        ];

        for line in code_lines {
            for source in &sources {
                if line.contains(source) {
                    tainted_vars.insert(source.to_string());
                }
            }

            for sink in &sinks {
                if line.contains(sink) {
                    for tainted in &tainted_vars {
                        if line.contains(tainted) {
                            flows.push(TaintFlow {
                                source: tainted.clone(),
                                sink: sink.to_string(),
                                path: vec![line.clone()],
                            });
                        }
                    }
                }
            }
        }

        flows
    }
}
