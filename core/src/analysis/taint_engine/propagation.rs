#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;

#[derive(Default)]
pub struct TaintPropagation {

    tainted: HashSet<String>,

}

impl TaintPropagation {

    pub fn new() -> Self {

        Self {
            tainted: HashSet::new()
        }

    }

    pub fn taint(&mut self, variable: &str) {

        self.tainted.insert(variable.to_string());

    }

    pub fn is_tainted(&self, variable: &str) -> bool {

        self.tainted.contains(variable)

    }
}
