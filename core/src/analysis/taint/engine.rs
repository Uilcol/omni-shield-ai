#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::taint::models::TaintPath;

pub fn analyze_nodes() -> Vec<TaintPath> {
    let mut paths = Vec::new();

    // 🔥 MOCK compatível com nova struct
    let from_id = 1;
    let to_id = 2;

    paths.push(TaintPath {
        source: "user_input".into(),
        sink: "exec".into(),

        // ✅ agora String
        nodes: vec![
            from_id.to_string(),
            to_id.to_string(),
        ],

        sanitized: false,

        // ✅ obrigatório agora
            "input()".into(),
            "var x".into(),
            "exec(x)".into(),
        ],
    });

    paths
}
