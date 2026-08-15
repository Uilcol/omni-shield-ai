#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct Sanitizer;

impl Sanitizer {
    pub fn is_sanitizer(node: &str) -> bool {
        let sanitizers = [
            "sanitize",
            "escape",
            "clean",
            "validate",
        ];

        sanitizers.iter().any(|s| node.contains(s))
    }
}

pub struct SanitizerDetector;

impl SanitizerDetector {
    pub fn is_sanitizer(_name: &str) -> bool {
        false
    }
}
