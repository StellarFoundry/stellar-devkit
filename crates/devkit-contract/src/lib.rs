//! Bounded structural inspection for untrusted Soroban contract WASM.
//!
//! This crate validates the WASM container but never executes it. The
//! contractspecv0 custom section is detected and sized; decoding individual
//! XDR specification entries is intentionally reported as a limitation until
//! a versioned spec model is added.

use devkit_core::DevkitError;
use serde::Serialize;
use wasmparser::{Parser, Payload, Validator};

pub const MAX_WASM_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportInfo {
    pub module: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExportInfo {
    pub name: String,
    pub kind: String,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContractReport {
    pub wasm_bytes: usize,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
    pub custom_sections: Vec<String>,
    pub has_contract_spec: bool,
    pub contract_spec_bytes: usize,
}

pub fn inspect_wasm(bytes: &[u8]) -> Result<ContractReport, DevkitError> {
    if bytes.is_empty() {
        return Err(DevkitError::invalid("WASM input is empty"));
    }
    if bytes.len() > MAX_WASM_BYTES {
        return Err(DevkitError::invalid(format!(
            "WASM exceeds the {MAX_WASM_BYTES}-byte limit"
        )));
    }
    Validator::new()
        .validate_all(bytes)
        .map_err(|error| DevkitError::decode(format!("invalid WASM module: {error}")))?;
    let mut report = ContractReport {
        wasm_bytes: bytes.len(),
        imports: Vec::new(),
        exports: Vec::new(),
        custom_sections: Vec::new(),
        has_contract_spec: false,
        contract_spec_bytes: 0,
    };
    for payload in Parser::new(0).parse_all(bytes) {
        match payload.map_err(DevkitError::decode)? {
            Payload::ImportSection(reader) => {
                for import in reader {
                    let import = import.map_err(DevkitError::decode)?;
                    report.imports.push(ImportInfo {
                        module: import.module.to_owned(),
                        name: import.name.to_owned(),
                    });
                }
            }
            Payload::ExportSection(reader) => {
                for export in reader {
                    let export = export.map_err(DevkitError::decode)?;
                    report.exports.push(ExportInfo {
                        name: export.name.to_owned(),
                        kind: format!("{:?}", export.kind),
                        index: export.index,
                    });
                }
            }
            Payload::CustomSection(section) => {
                let name = section.name().to_owned();
                if name == "contractspecv0" {
                    report.has_contract_spec = true;
                    report.contract_spec_bytes = section.data().len();
                }
                report.custom_sections.push(name);
            }
            _ => {}
        }
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_and_non_wasm_inputs() {
        assert!(inspect_wasm(&[]).is_err());
        assert!(inspect_wasm(b"not wasm").is_err());
    }

    #[test]
    fn rejects_oversized_input_before_parsing() {
        let input = vec![0; MAX_WASM_BYTES + 1];
        assert!(inspect_wasm(&input)
            .unwrap_err()
            .to_string()
            .contains("limit"));
    }

    #[test]
    fn accepts_a_minimal_valid_wasm_module() {
        let report = inspect_wasm(b"\0asm\x01\0\0\0").unwrap();
        assert_eq!(report.wasm_bytes, 8);
        assert!(report.imports.is_empty());
        assert!(report.exports.is_empty());
        assert!(!report.has_contract_spec);
    }
}
