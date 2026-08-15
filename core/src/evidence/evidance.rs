#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::evidence::trace::TraceStep;

#[derive(Debug, Clone)]
pub struct Evidence {
    pub file: String,
    pub line: usize,
    pub snippet: String,
}

impl Evidence {
    pub fn new(
        rule_id: String,
        message: String,
        file: String,
        line: usize,
        trace: Vec<TraceStep>,
    ) -> Self {
        Self {
            rule_id,
            message,
            file,
            line,
            trace,
            snippet: None,
            explanation: None,
        }
    }

    pub fn with_snippet(mut self, snippet: String) -> Self {
        self.snippet = Some(snippet);
        self
    }

    pub fn with_explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation);
        self
    }
}
