#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct SymbolicVariable {

    pub name: String,
    pub constraint: Option<String>,

}

impl SymbolicVariable {

    pub fn new(name: &str) -> Self {

        Self {

            name: name.to_string(),
            constraint: None,

        }

    }

    pub fn with_constraint(name: &str, constraint: &str) -> Self {

        Self {

            name: name.to_string(),
            constraint: Some(constraint.to_string()),

        }

    }
}
