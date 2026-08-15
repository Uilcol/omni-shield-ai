
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub caller: String,
    pub callee: String,
}

pub struct InterproceduralAnalyzerV3;

impl InterproceduralAnalyzerV3 {
    pub fn analyze(code: &str) -> Vec<FunctionCall> {
        let mut graph = Vec::new();

        let lines: Vec<&str> = code.lines().collect();

        for line in lines {
            if line.contains("call(") || line.contains("invoke(") {
                graph.push(FunctionCall {
                    caller: "unknown_fn".to_string(),
                    callee: line.trim().to_string(),
                });
            }
        }

        graph
    }
}
