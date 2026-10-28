//! Golden snapshots and structural validation for the versioned XDR schema.
//!
//! The snapshot files under `tests/snapshots/` are the reviewable contract for
//! schema version 1.0. If a change intentionally alters them, bump
//! `XDR_JSON_SCHEMA_VERSION` and document the migration in `docs/XDR.md`.

use std::path::{Path, PathBuf};

use devkit_xdr::{
    inspect_envelope_base64_with_options, inspect_event_base64_with_options,
    inspect_scval_base64_with_options, DecodeOptions, XDR_JSON_SCHEMA_VERSION,
};
use stellar_xdr::{
    BytesM, ContractEvent, ContractEventBody, ContractEventType, ContractEventV0, ContractId,
    DecoratedSignature, ExtensionPoint, Hash, Limits, Memo, MuxedAccount, Preconditions, ScVal,
    SequenceNumber, Signature, SignatureHint, Transaction, TransactionEnvelope, TransactionExt,
    TransactionV1Envelope, Uint256, VecM, WriteXdr,
};

fn snapshot_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join(format!("{name}.json"))
}

fn assert_snapshot(name: &str, value: &serde_json::Value) {
    let actual = format!("{}\n", serde_json::to_string_pretty(value).unwrap());
    let path = snapshot_path(name);
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &actual).unwrap();
        panic!(
            "created missing snapshot {}; re-run the test to compare",
            path.display()
        );
    }
    let expected = std::fs::read_to_string(&path).unwrap();
    assert_eq!(actual, expected, "snapshot {} drifted", path.display());
}

fn assert_document(document: &serde_json::Value, kind: &str, raw_xdr: Option<&str>) {
    let object = document.as_object().expect("document is an object");
    assert_eq!(object["schema_version"], XDR_JSON_SCHEMA_VERSION);
    assert_eq!(object["kind"], kind);
    assert!(object.contains_key("value"), "document has no value");

    let allowed = if raw_xdr.is_some() {
        vec!["schema_version", "kind", "value", "raw_xdr"]
    } else {
        vec!["schema_version", "kind", "value"]
    };
    for key in object.keys() {
        assert!(
            allowed.contains(&key.as_str()),
            "unexpected document field `{key}`"
        );
    }
    assert_eq!(object.get("raw_xdr").and_then(|v| v.as_str()), raw_xdr);
}

fn sample_envelope() -> TransactionEnvelope {
    TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0x22; 32])),
            fee: 100,
            seq_num: SequenceNumber(42),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: VecM::try_from(Vec::new()).unwrap(),
            ext: TransactionExt::V0,
        },
        signatures: VecM::try_from(vec![DecoratedSignature {
            hint: SignatureHint([0xab; 4]),
            signature: Signature(BytesM::try_from(vec![0xcd; 64]).unwrap()),
        }])
        .unwrap(),
    })
}

fn sample_event() -> ContractEvent {
    ContractEvent {
        ext: ExtensionPoint::V0,
        contract_id: Some(ContractId(Hash([0x11; 32]))),
        type_: ContractEventType::Contract,
        body: ContractEventBody::V0(ContractEventV0 {
            topics: VecM::try_from(vec![ScVal::Symbol(
                "transfer".try_into().expect("valid symbol"),
            )])
            .unwrap(),
            data: ScVal::U32(7),
        }),
    }
}

#[test]
fn scval_document_matches_golden_snapshot_and_schema() {
    let encoded = ScVal::U32(42).to_xdr_base64(Limits::none()).unwrap();
    let document = inspect_scval_base64_with_options(&encoded, DecodeOptions::default()).unwrap();
    assert_document(&document, "scval", None);
    assert_snapshot("scval", &document);
}

#[test]
fn envelope_document_matches_golden_snapshot_and_schema() {
    let encoded = sample_envelope().to_xdr_base64(Limits::none()).unwrap();
    let document =
        inspect_envelope_base64_with_options(&encoded, DecodeOptions::default()).unwrap();
    assert_document(&document, "transaction_envelope", None);
    assert_snapshot("envelope", &document);
}

#[test]
fn event_document_matches_golden_snapshot_and_schema() {
    let encoded = sample_event().to_xdr_base64(Limits::none()).unwrap();
    let document = inspect_event_base64_with_options(&encoded, DecodeOptions::default()).unwrap();
    assert_document(&document, "contract_event", None);
    assert_snapshot("event", &document);
}

#[test]
fn raw_xdr_is_only_present_when_requested() {
    let encoded = ScVal::Bool(true).to_xdr_base64(Limits::none()).unwrap();
    let plain = inspect_scval_base64_with_options(&encoded, DecodeOptions::default()).unwrap();
    assert_document(&plain, "scval", None);

    let raw = inspect_scval_base64_with_options(
        &format!("{encoded}\n"),
        DecodeOptions { include_raw: true },
    )
    .unwrap();
    assert_document(&raw, "scval", Some(&encoded));
}
