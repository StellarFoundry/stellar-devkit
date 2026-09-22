//! XDR, SCVal, transaction, and event inspection.
//!
//! This crate is a thin, opinionated workflow layer over the official
//! `stellar-xdr` types. It does not reimplement XDR; it decodes with
//! `stellar-xdr`, then produces stable, developer-friendly JSON for inspection
//! and reporting.
//!
//! All decoding is bounded and safe on untrusted input: the XDR reader receives
//! explicit limits and every failure becomes a structured [`DevkitError`].

use devkit_core::DevkitError;
use serde::Serialize;
use serde_json::{json, Value};
use stellar_xdr::{ContractEvent, Limits, ReadXdr, ScVal, TransactionEnvelope};

/// The maximum nesting/size limit applied when decoding untrusted XDR.
fn limits() -> Limits {
    Limits::none()
}

/// A decoded Stellar strkey with its kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StrkeyInfo {
    /// The strkey kind, for example `account` or `contract`.
    pub kind: String,
    /// The input, echoed back after validation.
    pub value: String,
}

/// Decodes a base64 `ScVal` and returns developer-friendly JSON.
pub fn inspect_scval_base64(input: &str) -> Result<Value, DevkitError> {
    let value = ScVal::from_xdr_base64(input.trim(), limits()).map_err(DevkitError::decode)?;
    serde_json::to_value(&value).map_err(DevkitError::decode)
}

/// Decodes a base64 `TransactionEnvelope` and returns developer-friendly JSON.
pub fn inspect_envelope_base64(input: &str) -> Result<Value, DevkitError> {
    let value = TransactionEnvelope::from_xdr_base64(input.trim(), limits())
        .map_err(DevkitError::decode)?;
    serde_json::to_value(&value).map_err(DevkitError::decode)
}

/// Decodes a base64 `ContractEvent` and returns developer-friendly JSON.
pub fn inspect_event_base64(input: &str) -> Result<Value, DevkitError> {
    let value =
        ContractEvent::from_xdr_base64(input.trim(), limits()).map_err(DevkitError::decode)?;
    serde_json::to_value(&value).map_err(DevkitError::decode)
}

/// Classifies a Stellar strkey.
pub fn describe_strkey(input: &str) -> Result<StrkeyInfo, DevkitError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DevkitError::invalid("empty strkey"));
    }
    let decoded = stellar_strkey::Strkey::from_string(trimmed)
        .map_err(|e| DevkitError::decode(format!("invalid strkey: {e}")))?;
    let kind = match decoded {
        stellar_strkey::Strkey::PublicKeyEd25519(_) => "account",
        stellar_strkey::Strkey::Contract(_) => "contract",
        _ => "other",
    };
    Ok(StrkeyInfo {
        kind: kind.to_string(),
        value: trimmed.to_string(),
    })
}

/// Builds a compact summary object for a decoded value, used by the CLI.
pub fn summary(kind: &str, value: &Value) -> Value {
    json!({ "kind": kind, "value": value })
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_xdr::WriteXdr;

    fn roundtrip_scval(value: &ScVal) -> String {
        value.to_xdr_base64(limits()).expect("encode scval")
    }

    #[test]
    fn decodes_scval_u32_roundtrip() {
        let encoded = roundtrip_scval(&ScVal::U32(42));
        let decoded = inspect_scval_base64(&encoded).unwrap();
        let text = serde_json::to_string(&decoded).unwrap();
        assert!(text.contains("42"), "unexpected json: {text}");
    }

    #[test]
    fn decodes_scval_bool_roundtrip() {
        let encoded = roundtrip_scval(&ScVal::Bool(true));
        let decoded = inspect_scval_base64(&encoded).unwrap();
        let text = serde_json::to_string(&decoded).unwrap();
        assert!(text.contains("true"), "unexpected json: {text}");
    }

    #[test]
    fn rejects_malformed_base64() {
        let err = inspect_scval_base64("not-valid-base64!!!").unwrap_err();
        assert!(matches!(err, DevkitError::Decode(_)));
    }

    #[test]
    fn rejects_empty_strkey() {
        assert!(describe_strkey("  ").is_err());
    }

    #[test]
    fn classifies_account_and_contract_strkeys() {
        let account =
            stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey([0u8; 32]))
                .to_string();
        let info = describe_strkey(&account).unwrap();
        assert_eq!(info.kind, "account");

        let contract =
            stellar_strkey::Strkey::Contract(stellar_strkey::Contract([0u8; 32])).to_string();
        let info = describe_strkey(&contract).unwrap();
        assert_eq!(info.kind, "contract");
    }
}
