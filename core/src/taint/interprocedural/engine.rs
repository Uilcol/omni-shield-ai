#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::taint::intraprocedural::IntraProceduralTaint;
use crate::taint::sink::SinkDetector;
use crate::taint::source::SourceDetector;
use crate::taint::sanitizer::SanitizerDetector;

#[derive(Debug)]
pub struct TaintEngine {
    analyzer: IntraProceduralTaint,
}

impl TaintEngine {
    pub fn new() -> Self {
        Self {
            analyzer: IntraProceduralTaint::new(),
        }
    }

    pub fn process_assignment(&mut self, from: &str, to: &str) {
        self.analyzer.propagate(from, to);
    }

    pub fn process_call(&mut self, function: &str, argument: &str) -> Option<String> {
        if SourceDetector::is_source(function) {
            self.analyzer.mark_source(argument);
        }

        if SanitizerDetector::is_sanitizer(function) {
            return None;
        }

        if SinkDetector::is_sink(function) && self.analyzer.is_tainted(argument) {
            return Some(format!(
                "Tainted data '{}' reached sink '{}'",
                argument, function
            ));
        }

        None
    }
}
