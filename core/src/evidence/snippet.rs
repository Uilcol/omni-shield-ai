#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::fs;

pub fn extract_snippet(file: &str, line: usize) -> Option<String> {
    let content = fs::read_to_string(file).ok()?;

    let lines: Vec<&str> = content.lines().collect();

    if line == 0 || line > lines.len() {
        return None;
    }

    let start = if line > 3 { line - 3 } else { 0 };
    let end = usize::min(line + 2, lines.len());

    let snippet = lines[start..end].join("\n");

    Some(snippet)
}
