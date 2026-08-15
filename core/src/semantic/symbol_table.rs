use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: HashMap<String, String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, symbol_type: &str) {
        self.symbols
            .insert(name.to_string(), symbol_type.to_string());
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.symbols.get(name)
    }
}
