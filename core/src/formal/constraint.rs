#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct Constraint {

    pub variable: String,
    pub condition: String,
    pub expression: String,

}

impl Constraint {

    pub fn new(variable: &str, condition: &str) -> Self {

        Self {

            variable: variable.to_string(),
            condition: condition.to_string(),
             expression: "".to_string(),

        }

    }
}
