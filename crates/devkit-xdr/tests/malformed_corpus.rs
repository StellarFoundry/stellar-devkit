use devkit_core::DevkitError;
use devkit_xdr::{inspect_envelope_base64, inspect_event_base64, inspect_scval_base64};

type Decoder = fn(&str) -> Result<serde_json::Value, DevkitError>;

struct Fixture {
    name: &'static str,
    input: &'static str,
    decode: Decoder,
}

const FIXTURES: &[Fixture] = &[
    Fixture {
        name: "scval-empty",
        input: include_str!("../../../fixtures/malformed/scval-empty.txt"),
        decode: inspect_scval_base64,
    },
    Fixture {
        name: "scval-truncated-xdr",
        input: include_str!("../../../fixtures/malformed/scval-truncated-xdr.txt"),
        decode: inspect_scval_base64,
    },
    Fixture {
        name: "scval-invalid-discriminant",
        input: include_str!("../../../fixtures/malformed/scval-invalid-discriminant.txt"),
        decode: inspect_scval_base64,
    },
    Fixture {
        name: "scval-overlong-length",
        input: include_str!("../../../fixtures/malformed/scval-overlong-length.txt"),
        decode: inspect_scval_base64,
    },
    Fixture {
        name: "envelope-truncated-xdr",
        input: include_str!("../../../fixtures/malformed/envelope-truncated-xdr.txt"),
        decode: inspect_envelope_base64,
    },
    Fixture {
        name: "envelope-invalid-discriminant",
        input: include_str!("../../../fixtures/malformed/envelope-invalid-discriminant.txt"),
        decode: inspect_envelope_base64,
    },
    Fixture {
        name: "envelope-truncated-payload",
        input: include_str!("../../../fixtures/malformed/envelope-truncated-payload.txt"),
        decode: inspect_envelope_base64,
    },
    Fixture {
        name: "event-truncated-xdr",
        input: include_str!("../../../fixtures/malformed/event-truncated-xdr.txt"),
        decode: inspect_event_base64,
    },
    Fixture {
        name: "event-invalid-discriminant",
        input: include_str!("../../../fixtures/malformed/event-invalid-discriminant.txt"),
        decode: inspect_event_base64,
    },
    Fixture {
        name: "event-truncated-payload",
        input: include_str!("../../../fixtures/malformed/event-truncated-payload.txt"),
        decode: inspect_event_base64,
    },
];

#[test]
fn every_malformed_fixture_returns_a_structured_error_without_panicking() {
    for fixture in FIXTURES {
        let result = (fixture.decode)(fixture.input.trim());
        match result {
            Err(DevkitError::Decode(_)) => {}
            Err(other) => panic!(
                "fixture {} returned unexpected error category: {other}",
                fixture.name
            ),
            Ok(value) => panic!(
                "malformed fixture {} unexpectedly decoded as {value}",
                fixture.name
            ),
        }
    }
}

#[test]
fn corpus_contains_truncation_invalid_discriminants_and_overlong_length() {
    let names: Vec<_> = FIXTURES.iter().map(|fixture| fixture.name).collect();
    assert!(names.iter().any(|name| name.contains("truncated")));
    assert!(names
        .iter()
        .any(|name| name.contains("invalid-discriminant")));
    assert!(names.iter().any(|name| name.contains("overlong-length")));
}
