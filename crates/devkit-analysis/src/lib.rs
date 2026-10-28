//! Deterministic, evidence-backed structural checks for contract WASM.
//! Findings are indicators, not proof that a contract is vulnerable or malicious.

use devkit_contract::ContractReport;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub title: String,
    pub severity: Severity,
    pub message: String,
    pub evidence: String,
}

pub fn analyze(report: &ContractReport) -> Vec<Finding> {
    let mut findings = Vec::new();
    if !report.has_contract_spec {
        findings.push(Finding {
            rule_id: "DEVKIT-WASM-001".into(),
            title: "Soroban contract specification is absent".into(),
            severity: Severity::Warning,
            message: "No contractspecv0 custom section was found; callers may not be able to inspect typed contract interfaces from this artifact.".into(),
            evidence: format!("custom_sections={}", report.custom_sections.join(",")),
        });
    }
    for import in report.imports.iter().filter(|item| item.module != "env") {
        findings.push(Finding {
            rule_id: "DEVKIT-WASM-002".into(),
            title: "Import from a non-standard module".into(),
            severity: Severity::Warning,
            message: "The module imports a symbol from a module other than env. Verify this import is expected for the target runtime.".into(),
            evidence: format!("{}::{}", import.module, import.name),
        });
    }
    if report.wasm_bytes > 1024 * 1024 {
        findings.push(Finding {
            rule_id: "DEVKIT-WASM-003".into(),
            title: "Large WASM artifact".into(),
            severity: Severity::Note,
            message: "The WASM artifact exceeds 1 MiB. Review size and deployment/resource constraints; size alone is not a vulnerability.".into(),
            evidence: format!("wasm_bytes={}", report.wasm_bytes),
        });
    }
    findings
}

pub fn to_sarif(findings: &[Finding], artifact_uri: Option<&str>) -> Value {
    let rules = vec![
        json!({"id":"DEVKIT-WASM-001","name":"MissingContractSpec","shortDescription":{"text":"Soroban contract specification is absent"},"defaultConfiguration":{"level":"warning"}}),
        json!({"id":"DEVKIT-WASM-002","name":"NonStandardImport","shortDescription":{"text":"Import from a non-standard module"},"defaultConfiguration":{"level":"warning"}}),
        json!({"id":"DEVKIT-WASM-003","name":"LargeWasmArtifact","shortDescription":{"text":"Large WASM artifact"},"defaultConfiguration":{"level":"note"}}),
    ];
    let results: Vec<Value> = findings
        .iter()
        .map(|finding| {
            let level = match finding.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Note => "note",
            };
            let mut result = json!({
                "ruleId": finding.rule_id,
                "level": level,
                "message": {"text": finding.message},
                "properties": {"title": finding.title, "evidence": finding.evidence}
            });
            if let Some(uri) = artifact_uri {
                result["locations"] =
                    json!([{"physicalLocation":{"artifactLocation":{"uri":uri}}}]);
            }
            result
        })
        .collect();
    json!({
        "$schema":"https://json.schemastore.org/sarif-2.1.0.json",
        "version":"2.1.0",
        "runs":[{"tool":{"driver":{"name":"stellar-foundry","informationUri":"https://github.com/StellarFoundry/stellar-devkit","rules":rules}},"results":results}]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use devkit_contract::{ContractReport, ImportInfo};

    #[test]
    fn emits_evidence_for_nonstandard_imports() {
        let report = ContractReport {
            wasm_bytes: 100,
            imports: vec![ImportInfo {
                module: "host".into(),
                name: "call".into(),
            }],
            exports: vec![],
            custom_sections: vec![],
            has_contract_spec: false,
            contract_spec_bytes: 0,
        };
        let findings = analyze(&report);
        assert!(findings
            .iter()
            .any(|item| item.rule_id == "DEVKIT-WASM-002" && item.evidence == "host::call"));
        let sarif = to_sarif(&findings, Some("contract.wasm"));
        assert_eq!(sarif["version"], "2.1.0");
        assert!(sarif["runs"][0]["results"].as_array().unwrap().len() >= 2);
    }
}
