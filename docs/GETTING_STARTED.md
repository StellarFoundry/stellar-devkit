# Getting Started

## Prerequisites

- A recent stable Rust toolchain.

## Build

```bash
cargo build --release
```

The binary is written to `target/release/stellar-foundry`.

## First steps

```bash
stellar-foundry version
stellar-foundry doctor
```

`doctor` reports the tool version, its capabilities, the known network profiles,
and that **network access is disabled**. There is no network functionality yet.

## Classify an address

```bash
stellar-foundry strkey CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE
```

Output:

```json
{ "kind": "contract", "value": "CA3D..." }
```

## Decode XDR

Decode a base64 `ScVal`:

```bash
stellar-foundry scval --format json <BASE64_SCVAL>
```

Decode a transaction envelope or a contract event:

```bash
stellar-foundry envelope --format json <BASE64_ENVELOPE>
stellar-foundry event --format json <BASE64_EVENT>
```

Malformed input produces a structured error and exit code `3`; it never panics.

## Exit codes

| Code | Meaning |
| ---- | ------- |
| 0 | Success |
| 2 | Usage error |
| 3 | Runtime error (decode failure) |

## Next

- [CLI reference](CLI.md)
- [Architecture](ARCHITECTURE.md)
- [Ecosystem research](ECOSYSTEM_RESEARCH.md)
