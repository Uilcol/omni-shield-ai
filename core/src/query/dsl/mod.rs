#[derive(Clone)]
pub struct Query {
    pub name: String,
    pub source: String,
    pub sink: String,
    pub pattern: String,
}
