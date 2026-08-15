#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct TaintSink {
    pub name: String,
}

impl TaintSink {

    pub fn is_sink(function: &str) -> bool {

        matches!(
            function,
            "eval"
            | "exec"
            | "system"
            | "popen"
            | "sql_query"
        )

    }
}
