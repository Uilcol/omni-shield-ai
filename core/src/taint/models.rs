#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct TaintSource {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct TaintSink {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct TaintPath {
    pub is_sanitized: bool,
}
#[derive(Debug, Clone)]
pub struct TaintResult;
