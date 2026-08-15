#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct Sink;

impl Sink {
    pub fn is_sink(node: &str) -> bool {
        let sinks = [
            "eval",
            "exec",
            "query",
            "system",
        ];

        sinks.iter().any(|s| node.contains(s))
    }
}

  pub struct SinkDetector;

impl SinkDetector {
    pub fn is_sink(_name: &str) -> bool {
        false
    }
}
