//! Byte-order and cross-platform stability tests.
//!
//! XDR is big-endian on every host. These tests use explicit byte sequences so
//! they do not depend on the host's native endianness, and they assert the
//! stable JSON produced by the decoder.

use devkit_xdr::inspect_scval_base64;
use stellar_xdr::{Limits, ReadXdr, ScVal, WriteXdr};

#[test]
fn u32_uses_big_endian_discriminant_and_payload() {
    let value = ScVal::U32(0x0102_0304);
    let encoded = value.to_xdr(Limits::none()).expect("encode scval");

    // SCVal discriminant 3 (U32), then the value in network byte order.
    assert_eq!(encoded, vec![0, 0, 0, 3, 1, 2, 3, 4]);

    let decoded = ScVal::from_xdr(&encoded, Limits::none()).expect("decode scval");
    assert_eq!(decoded, value);
}

#[test]
fn i32_keeps_twos_complement_in_big_endian_order() {
    let bytes = [0, 0, 0, 4, 0xff, 0xff, 0xff, 0xfe];
    let value = ScVal::from_xdr(bytes, Limits::none()).expect("decode i32");
    assert_eq!(value, ScVal::I32(-2));
    assert_eq!(value.to_xdr(Limits::none()).unwrap(), bytes);
}

#[test]
fn i64_uses_big_endian_discriminant_and_payload() {
    let value = ScVal::I64(0x0102_0304_0506_0708);
    let encoded = value.to_xdr(Limits::none()).expect("encode scval");

    // SCVal discriminant 6 (I64), then eight big-endian bytes.
    assert_eq!(encoded, vec![0, 0, 0, 6, 1, 2, 3, 4, 5, 6, 7, 8]);

    let decoded = ScVal::from_xdr(&encoded, Limits::none()).expect("decode scval");
    assert_eq!(decoded, value);
}

#[test]
fn base64_and_json_output_are_identical_on_every_host() {
    // `00 00 00 03 01 02 03 04` is ScVal::U32(0x01020304).
    let json = inspect_scval_base64("AAAAAwECAwQ=").expect("decode scval document");
    assert_eq!(json["u32"], 0x0102_0304u32);
}
