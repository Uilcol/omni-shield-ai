#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug)]
pub struct Query {
    pub source: String,
    pub sink: String,
}

pub fn parse_query(input: &str) -> Result<Query, String> {
    let input = input.to_lowercase();

    let source = if input.contains("user_input") {
        "user_input".to_string()
    } else {
        return Err("Missing source".into());
    };

    let sink = if input.contains("exec") {
        "exec".to_string()
    } else {
        return Err("Missing sink".into());
    };

    Ok(Query { source, sink })
}
