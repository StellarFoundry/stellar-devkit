# Getting Started

## Prerequisites

- Rust 1.91 or newer.
- No network access is required for decoding, strkey, or WASM inspection.

## Build and install

```bash
cargo build --release
```

The binary is written to `target/release/stellar-foundry`. To install it from
the workspace:

```bash
cargo install --path crates/devkit-cli
```

## First steps

```bash
stellar-foundry version
stellar-foundry doctor
```

`doctor` reports the tool version, its capabilities, the known network profiles,
and that network access is explicit and limited to `rpc-health`.

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

Pass `-` to read one value from piped stdin:

```bash
printf 'AAAAAwAAACo=' | stellar-foundry --format json scval -
```

Malformed input produces a structured error and exit code `3`; it never panics.

## Inspect a contract artifact

```bash
stellar-foundry inspect-wasm ./contract.wasm
stellar-foundry analyze-wasm --format sarif ./contract.wasm
```

Inspection is read-only and never executes the contract.

## Check an RPC endpoint

```bash
stellar-foundry endpoint https://soroban-testnet.stellar.org
stellar-foundry rpc-health https://soroban-testnet.stellar.org
```

`rpc-health` is the only command that makes a network request. It accepts an
endpoint argument, `STELLAR_FOUNDRY_RPC_URL`, or `rpc_url` in a TOML config file.
See [CLI.md](CLI.md) for configuration and precedence.

## Exit codes

| Code | Meaning |
| ---- | ------- |
| 0 | Success |
| 2 | Usage error |
| 3 | Runtime error (decode failure, I/O) |

## Next

- [CLI reference](CLI.md)
- [Why DevKit](WHY_DEVKIT.md)
- [Architecture](ARCHITECTURE.md)
- [Implemented capabilities](IMPLEMENTED_CAPABILITIES.md)
