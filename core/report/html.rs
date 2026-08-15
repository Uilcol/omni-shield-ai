#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use crate::finding::Finding;

pub fn generate(findings: &Vec<Finding>) -> String {

    let mut rows = String::new();

pub fn run() {
pub fn run() {
            for f in findings {
        rows.push_str(&format!(
            r#"<tr>
<td>{}</td>
<td>{}</td>
<td>{}</td>
<td>{}</td>
<td>{}</td>
</tr>"#,
            f.id,
            f.file,
            f.line,
            f.title,
            f.severity
        ));
    }

    format!(r#"
<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>OmniUil Report</title>

<style>
body {{
    background: #0d1117;
    color: #c9d1d9;
    font-family: Arial;
}}

h1 {{
    color: #58a6ff;
}}

table {{
    width: 100%;
    border-collapse: collapse;
    margin-top: 20px;
}}

th, td {{
    padding: 12px;
    border-bottom: 1px solid #30363d;
}}

th {{
    background: #161b22;
}}

tr:hover {{
    background: #1f2933;
}}
</style>
</head>

<body>

<h1>🚀 OmniUil Scan Report</h1>

<p>Total findings: {}</p>

<table>
<tr>
<th>ID</th>
<th>File</th>
<th>Line</th>
<th>Title</th>
<th>Severity</th>
</tr>

{}

</table>

</body>
</html>
"#,
        findings.len(),
        rows
    )
}


