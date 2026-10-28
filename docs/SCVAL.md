# SCVal JSON schema

This page defines the `scval` document emitted by `devkit-xdr` and the
`stellar-foundry scval` command. The shared envelope is specified in
[`XDR.md`](XDR.md); this page covers the `value` payload.

Current schema version: **1.0**.

## Schema

```json
{
  "schema_version": "1.0",
  "kind": "scval",
  "value": { "u32": 42 }
}
```

- `schema_version` — `1.0`.
- `kind` — always `scval`.
- `value` — exactly one tagged `ScVal` object or unit string.
- `raw_xdr` — optional base64 input, enabled with `--raw`.

A decoder must reject a document whose `kind` is not `scval` or whose major
schema version is not `1`.

## Variant mapping

| SCVal variant | JSON shape | Example |
| ------------- | ---------- | ------- |
| `Bool` | `{"bool": true}` | `{"bool":false}` |
| `Void` | `"void"` | `"void"` |
| `Error` | `{"error": {...}}` | `{"error":{"contract":1}}` |
| `U32` | `{"u32": 42}` | number |
| `I32` | `{"i32": -1}` | number |
| `U64` | `{"u64": "42"}` | string, no precision loss |
| `I64` | `{"i64": "-1"}` | string |
| `Timepoint` | `{"timepoint": "1700000000"}` | string |
| `Duration` | `{"duration": "60"}` | string |
| `U128` | `{"u128": {"hi": "...", "lo": "..."}}` | parts |
| `I128` | `{"i128": {"hi": "...", "lo": "..."}}` | parts |
| `U256` | `{"u256": {"hi_hi": "...", "hi_lo": "...", "lo_hi": "...", "lo_lo": "..."}}` | parts |
| `I256` | `{"i256": {...}}` | parts |
| `Bytes` | `{"bytes": "<hex>"}` | lowercase hex |
| `String` | `{"string": "..."}` | UTF-8 |
| `Symbol` | `{"symbol": "transfer"}` | UTF-8 |
| `Vec` | `{"vec": [ ... ]}` or `{"vec": null}` | recursive `ScVal` |
| `Map` | `{"map": [{"key": ..., "value": ...}]}` or `null` | recursive, ordered |
| `Address` | `{"address": "G..."}` / `"C..."` | strkey |
| `ContractInstance` | `{"contract_instance": {...}}` | XDR-JSON |
| `LedgerKeyContractInstance` | `"ledger_key_contract_instance"` | unit string |
| `LedgerKeyNonce` | `{"ledger_key_nonce": {...}}` | XDR-JSON |
| `ExecutableTag` | `{"executable_tag": "..."}` | string |

The table reflects the `stellar-xdr` 28 serde mapping frozen by schema 1.0. If a
future `stellar-xdr` release adds a variant, the decoder reports a decode error
until the variant is documented here and the schema minor version is bumped.

## Examples

### Integer

```console
$ stellar-foundry --format json scval AAAAAwAAACo=
{"kind":"scval","schema_version":"1.0","value":{"u32":42}}
```

### Symbol and vector

```json
{
  "schema_version": "1.0",
  "kind": "scval",
  "value": {
    "vec": [
      { "symbol": "transfer" },
      { "u32": 7 }
    ]
  }
}
```

### Map

Maps are arrays of `key`/`value` pairs so arbitrary SCVal keys survive JSON
round-trips without string coercion:

```json
{
  "schema_version": "1.0",
  "kind": "scval",
  "value": {
    "map": [
      { "key": { "symbol": "admin" }, "value": { "address": "G..." } }
    ]
  }
}
```

## Recursion and limits

- Nested `Vec`, `Map`, and contract-instance values use the same schema
  recursively.
- Decoding is bounded to depth 128 and 16 MiB of XDR. Exceeding a limit returns
  a structured decode error; no partial document is emitted.
- The `value` payload never contains raw binary; bytes and hashes are hex and
  ids are strkeys.

## Stability

Within schema 1.x, existing variant tags and shapes do not change. Additive
changes (new documented variants or optional fields) bump the minor version.
Removing or renaming a tag requires schema 2.0 and a migration note in
`XDR.md`. Golden SCVal output is covered by
`crates/devkit-xdr/tests/snapshots/scval.json`.
