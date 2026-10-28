//! Bounded structural inspection for untrusted Soroban contract WASM.
//!
//! This crate validates the WASM container but never executes it. The
//! contractspecv0 custom section is detected and sized; decoding individual
//! XDR specification entries is intentionally reported as a limitation until
//! a versioned spec model is added.

use std::io::Cursor;

use devkit_core::DevkitError;
use serde::Serialize;
use serde_json::Value;
use stellar_xdr::{Limited, Limits, ReadXdr, ScSpecEntry};
use wasmparser::{Parser, Payload, Validator};

pub const MAX_WASM_BYTES: usize = 16 * 1024 * 1024;
const MAX_SPEC_ENTRIES: usize = 4096;
const MAX_XDR_DEPTH: u32 = 128;

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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ContractSpecEntry {
    /// Stable variant name from the official XDR model.
    pub kind: String,
    /// Best-effort declared name for functions and user-defined types.
    pub name: Option<String>,
    /// Complete XDR-JSON representation of the decoded entry.
    pub definition: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ContractReport {
    pub wasm_bytes: usize,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
    pub custom_sections: Vec<String>,
    pub has_contract_spec: bool,
    pub contract_spec_bytes: usize,
    pub contract_spec_entries: Vec<ContractSpecEntry>,
}

/// Decode the concatenated ScSpecEntry XDR stream with explicit resource
/// limits. The WASM module is already capped at 16 MiB; an additional entry
/// cap prevents a tiny malicious stream from producing an excessive report.
fn decode_contract_spec(bytes: &[u8]) -> Result<Vec<ContractSpecEntry>, DevkitError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let limits = Limits {
        depth: MAX_XDR_DEPTH,
        len: MAX_WASM_BYTES,
    };
    let cursor = Cursor::new(bytes);
    let mut reader = Limited::new(cursor, limits);
    let mut entries = Vec::new();
    for item in ScSpecEntry::read_xdr_iter(&mut reader) {
        let entry = item.map_err(|error| {
            DevkitError::decode(format!("invalid contractspecv0 XDR entry: {error}"))
        })?;
        if entries.len() >= MAX_SPEC_ENTRIES {
            return Err(DevkitError::invalid(format!(
                "contractspecv0 exceeds the {MAX_SPEC_ENTRIES}-entry limit"
            )));
        }
        let definition = serde_json::to_value(&entry).map_err(DevkitError::decode)?;
        let debug = format!("{entry:?}");
        let kind = debug
            .split(['(', ' ', '{'])
            .next()
            .unwrap_or("Unknown")
            .to_owned();
        let name = find_name(&definition);
        entries.push(ContractSpecEntry {
            kind,
            name,
            definition,
        });
    }
    Ok(entries)
}

fn find_name(value: &Value) -> Option<String> {
    match value {
        Value::Object(object) => {
            if let Some(Value::String(name)) = object.get("name") {
                return Some(name.clone());
            }
            object.values().find_map(find_name)
        }
        Value::Array(values) => values.iter().find_map(find_name),
        _ => None,
    }
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
        contract_spec_entries: Vec::new(),
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
                    if report.has_contract_spec {
                        return Err(DevkitError::decode(
                            "WASM module contains multiple contractspecv0 sections",
                        ));
                    }
                    report.has_contract_spec = true;
                    report.contract_spec_bytes = section.data().len();
                    report.contract_spec_entries = decode_contract_spec(section.data())?;
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
    #[test]
    fn rejects_malformed_contract_spec_xdr() {
        let name = b"contractspecv0";
        let payload_len = 1 + name.len() + 1;
        let mut wasm = b"\\0asm\\x01\\0\\0\\0".to_vec();
        wasm.extend_from_slice(&[0, payload_len as u8, name.len() as u8]);
        wasm.extend_from_slice(name);
        wasm.push(0xff);

        let error = inspect_wasm(&wasm).unwrap_err().to_string();
        assert!(error.contains("contractspecv0 XDR entry"), "{error}");
    }

}
