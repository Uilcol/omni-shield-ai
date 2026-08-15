#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, f: String) {
        self.functions.push(f);
    }
}
