#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct Source {
    pub name: String,
}

pub struct Sink {
    pub name: String,
}

pub struct TaintConfig;

impl TaintConfig {

    pub fn default_sources() -> Vec<Source> {
        vec![
            Source { name: "input".into() },
            Source { name: "request".into() },
            Source { name: "params".into() },
        ]
    }

    pub fn default_sinks() -> Vec<Sink> {
        vec![
            Sink { name: "eval".into() },
            Sink { name: "exec".into() },
            Sink { name: "system".into() },
        ]
    }
}
