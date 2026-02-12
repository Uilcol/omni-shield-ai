#[derive(Debug)]
pub enum Decision {
    Allow,
    Reject(&'static str),
}
