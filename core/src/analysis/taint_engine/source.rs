#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct TaintSource {
    pub name: String,
}

impl TaintSource {

    pub fn is_source(function: &str) -> bool {

        matches!(
            function,
            "input"
            | "request"
            | "stdin"
            | "read_line"
            | "getenv"
        )

    }
}
