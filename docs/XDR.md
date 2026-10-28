# XDR JSON schema

`devkit-xdr` emits a versioned document for every decoded value. The envelope is
the stable contract; the `value` member is the XDR-JSON mapping defined below.

Current schema version: **1.0** (`devkit_xdr::XDR_JSON_SCHEMA_VERSION`).

## Document envelope

```json
{
  "schema_version": "1.0",
  "kind": "scval",
  "value": { "u32": 42 }
}
```

| Field | Type | Presence | Meaning |
| ----- | ---- | -------- | ------- |
| `schema_version` | string | always | Schema version in `MAJOR.MINOR` form. |
| `kind` | string | always | `scval`, `transaction_envelope`, or `contract_event`. |
| `value` | any JSON | always | Decoded XDR-JSON payload (see below). |
| `raw_xdr` | string | opt-in | Canonical, trimmed base64 XDR input. |

`raw_xdr` is omitted by default so reports stay compact. Request it with
`--raw` on the CLI or `DecodeOptions { include_raw: true }` in the library.

```console
$ stellar-foundry --format json --raw scval AAAAAwAAACo=
{"kind":"scval","raw_xdr":"AAAAAwAAACo=","schema_version":"1.0","value":{"u32":42}}
```

The raw encoding is always recoverable: the input is base64 XDR and the decoder
never rewrites it. The flag only echoes it into the document.

## Kinds

### `scval`

`value` is the XDR-JSON mapping of a single `ScVal`. See
[`SCVAL.md`](SCVAL.md) for the variant table and examples.

### `transaction_envelope`

`value` is the XDR-JSON mapping of a `TransactionEnvelope`. The top-level key
identifies the envelope variant:

- `tx_v0` — legacy `TransactionV0Envelope`
- `tx` — `TransactionV1Envelope` (contains `tx` and `signatures`)
- `tx_fee_bump` — `FeeBumpTransactionEnvelope`

Example (`tx` variant, abbreviated):

```json
{
  "schema_version": "1.0",
  "kind": "transaction_envelope",
  "value": {
    "tx": {
      "tx": {
        "source_account": "G...",
        "fee": 100,
        "seq_num": "42",
        "cond": "none",
        "memo": "none",
        "operations": []
      },
      "signatures": [
        { "hint": "abababab", "signature": "cdcd..." }
      ]
    }
  }
}
```

### `contract_event`

`value` is the XDR-JSON mapping of a `ContractEvent`:

| Field | Meaning |
| ----- | ------- |
| `type` | `system`, `contract`, or `diagnostic` |
| `contract_id` | strkey contract id, or `null` for system events |
| `body.v0.topics` | array of `ScVal` values, each in the `scval` schema |
| `body.v0.data` | `ScVal` payload, in the `scval` schema |
| `ext` | extension point (`v0` in schema 1.0) |

## XDR-JSON payload rules

The `value` payload is the deterministic `stellar-xdr` serde mapping, frozen as
part of schema 1.0. Rules that consumers may rely on:

- Enums and union variants are tagged objects or strings using `snake_case`
  names, for example `{"u32":42}`, `{"symbol":"transfer"}`, `"none"`.
- 64-bit integers use JSON strings (`"42"`), matching `stellar-xdr`'s
  `NumberOrString` mapping, so no precision is lost.
- Byte arrays are lowercase hex strings; hashes and signature hints are hex.
- Account, muxed-account, and contract ids are strkeys.
- Field names use `snake_case` (`seq_num`, `source_account`).
- Maps and vectors preserve XDR order; the decoder does not reorder them.
- Undecoded or unknown future XDR variants are a decode error, not silent data
  loss.

## Versioning and migration

- `MAJOR` changes when an existing field is removed, renamed, retyped, or
  reordered in a way that breaks consumers. A major bump requires a migration
  section in this file and a new snapshot set.
- `MINOR` changes are additive: new optional fields, new `kind` values, or new
  documented XDR variants. Existing fields keep their names and meanings.
- Every successfully decoded document carries `schema_version`. Decode errors
  are returned through the existing structured error type, not as documents.
- Unknown fields must be ignored by consumers.

### Migrating from the unversioned output

Before schema 1.0, CLI output was `{ "kind": ..., "value": ... }` with no
version. Migration is additive:

1. Read `schema_version`; reject documents whose major version is not `1`.
2. Keep reading `kind` and `value` as before.
3. Treat `raw_xdr` as optional and only present when requested.

No field was removed or renamed, so existing consumers that ignored unknown
fields continue to work.

## Tests

- `crates/devkit-xdr/tests/schema_snapshots.rs` validates the envelope and
  compares all three kinds against committed golden files in
  `crates/devkit-xdr/tests/snapshots/`.
- Unit tests assert that `raw_xdr` is opt-in and preserved verbatim.
- Changing a snapshot requires updating this document and bumping the schema
  version when the change is not additive.
