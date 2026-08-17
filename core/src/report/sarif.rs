use crate::findings::finding::Finding;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SarifReport {
    #[serde(rename = "$schema")]
    schema: String,
    version: String,
    runs: Vec<SarifRun>,
}

#[derive(Debug, Serialize)]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Debug, Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Debug, Serialize)]
struct SarifDriver {
    name: String,
    information_uri: String,
}

#[derive(Debug, Serialize)]
struct SarifResult {
    #[serde(rename = "ruleId")]
    rule_id: String,
    level: String,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

#[derive(Debug, Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Debug, Serialize)]
struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: SarifPhysicalLocation,
}

#[derive(Debug, Serialize)]
struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Debug, Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Debug, Serialize)]
struct SarifRegion {
    #[serde(rename = "startLine")]
    start_line: usize,
}

fn sarif_level(severity: &str) -> &'static str {
    match severity.to_ascii_lowercase().as_str() {
        "critical" | "high" => "error",
        "medium" => "warning",
        "low" | "info" => "note",
        _ => "warning",
    }
}

pub fn generate(findings: &[Finding]) -> String {
    let results = findings
        .iter()
        .map(|finding| SarifResult {
            rule_id: finding.id.clone(),
            level: sarif_level(&finding.severity).to_string(),
            message: SarifMessage {
                text: format!(
                    "{}. Evidence: {}. Recommendation: {}",
                    finding.title, finding.evidence, finding.recommendation
                ),
            },
            locations: vec![SarifLocation {
                physical_location: SarifPhysicalLocation {
                    artifact_location: SarifArtifactLocation {
                        uri: finding.file.clone(),
                    },
                    region: SarifRegion {
                        start_line: finding.line,
                    },
                },
            }],
        })
        .collect();

    let report = SarifReport {
        schema: "https://json.schemastore.org/sarif-2.1.0.json".to_string(),
        version: "2.1.0".to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "OmniShield AI".to_string(),
                    information_uri: "https://github.com/Uilcol/omni-shield-ai".to_string(),
                },
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&report)
        .unwrap_or_else(|_| "{\"version\":\"2.1.0\",\"runs\":[]}".to_string())
}
