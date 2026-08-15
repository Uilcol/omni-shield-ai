#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use crate::rules::finding::Finding;
use serde::Serialize;

#[derive(Serialize)]
pub struct SarifReport {
    version: String,
    runs: Vec<SarifRun>,
}

#[derive(Serialize)]
pub struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Serialize)]
pub struct SarifTool {
    driver: SarifDriver,
}

#[derive(Serialize)]
pub struct SarifDriver {
    name: String,
}

#[derive(Serialize)]
pub struct SarifResult {
    ruleId: String,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
pub struct SarifMessage {
    text: String,
}

#[derive(Serialize)]
pub struct SarifLocation {
    physicalLocation: SarifPhysicalLocation,
}

#[derive(Serialize)]
pub struct SarifPhysicalLocation {
    artifactLocation: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Serialize)]
pub struct SarifArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
pub struct SarifRegion {
    startLine: usize,
}

pub fn generate(findings: &[Finding]) -> String {
    let results: Vec<SarifResult> = findings
        .iter()
        .map(|f| SarifResult {
            ruleId: f.rule_id.clone(),
            message: SarifMessage {
                text: f.message.clone(),
            },
            locations: vec![SarifLocation {
                physicalLocation: SarifPhysicalLocation {
                    artifactLocation: SarifArtifactLocation {
                        uri: f.file.clone(),
                    },
                    region: SarifRegion {
                        startLine: f.line,
                    },
                },
            }],
        })
        .collect();

    let report = SarifReport {
        version: "2.1.0".to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "OmniUil AI".to_string(),
                },
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
}
