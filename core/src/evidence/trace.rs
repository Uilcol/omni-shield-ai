#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct TraceStep {
    pub file: String,
    pub line: usize,
    pub variable: String,
    pub kind: TraceKind,
}

#[derive(Debug, Clone)]
pub enum TraceKind {
    Source,
    Propagation,
    Sink,
}
