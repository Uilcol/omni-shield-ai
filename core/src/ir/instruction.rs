#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub enum Expression {
    Variable(String),
    Literal(String),
    Call {
        function: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Assign {
        target: String,
        value: Expression,
    },

    Call {
        function: String,
        args: Vec<Expression>,
    },

    Return(Option<Expression>),
}
