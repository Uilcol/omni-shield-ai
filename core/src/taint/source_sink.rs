#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct TaintSource; pub struct TaintSink;

pub fn default_sources() -> Vec<TaintSource> {
    vec![
        TaintSource {
            name: "request.args".to_string(),
        },
        TaintSource {
            name: "request.form".to_string(),
        },
        TaintSource {
            name: "stdin".to_string(),
        },
    ]
}

pub fn default_sinks() -> Vec<TaintSink> {
    vec![
        TaintSink {
            name: "eval".to_string(),
        },
        TaintSink {
            name: "exec".to_string(),
        },
        TaintSink {
            name: "system".to_string(),
        },
        TaintSink {
            name: "execute".to_string(),
        },
    ]
}
