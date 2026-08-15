#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct AnalysisContext {
    pub file: String,
}

impl AnalysisContext {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}
