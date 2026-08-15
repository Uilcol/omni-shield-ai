#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug)]
pub struct Query {
    pub source: String,
    pub sink: String,
    pub require_unsanitized: bool,
}

impl Query {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            sink: String::new(),
            require_unsanitized: false,
        }
    }
}
