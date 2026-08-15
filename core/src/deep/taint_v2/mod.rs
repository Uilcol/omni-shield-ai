
#[derive(Debug, Clone)]
pub struct TaintFlow {
    pub source: String,
    pub sink: String,
    pub path: Vec<String>,
}

pub struct TaintEngineV2;

impl TaintEngineV2 {
    pub fn analyze(code: &str) -> Vec<TaintFlow> {
        let mut findings = Vec::new();

        let dangerous_sources = vec![
            "request.GET",
            "request.POST",
            "input(",
            "stdin",
            "argv",
            "params[",
        ];

        let dangerous_sinks = vec![
            "eval(",
            "exec(",
            "system(",
            "Runtime.getRuntime",
            "query(",
            "execute(",
        ];

        for src in &dangerous_sources {
            for sink in &dangerous_sinks {
                if code.contains(src) && code.contains(sink) {
                    findings.push(TaintFlow {
                        source: src.to_string(),
                        sink: sink.to_string(),
                        path: vec![
                            src.to_string(),
                            "propagation".to_string(),
                            sink.to_string(),
                        ],
                    });
                }
            }
        }

        findings
    }
}
