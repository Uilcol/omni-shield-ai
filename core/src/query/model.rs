#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub enum QueryType {
    Taint,
    Ast,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
    Contains,
    Matches,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub field: String,
    pub op: Operator,
    pub value: String,
    pub negated: bool,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub name: String,
    pub query_type: QueryType,
    pub source: String,
    pub sink: String,
    pub conditions: Vec<Vec<Condition>>,
}
