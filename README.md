# StellarFoundry DevKit

Developer infrastructure toolkit for the Stellar and Soroban ecosystem.

`stellar-devkit` composes with official tooling — it does **not** reimplement
XDR, strkey, or JSON-RPC. It provides a coherent workflow layer: decoding and
inspection with structured output, deterministic fixtures and mocks, analysis,
and a CLI.

The core works fully **offline** and never requires AI.

> Status: early, `0.1.0`. The implementation described below is merged on
> `main`, and `v0.1.0` has prebuilt Linux, macOS, and Windows binaries on the
> [Releases page](https://github.com/StellarFoundry/stellar-devkit/releases).
> crates.io publication is still pending. See
> [docs/PROJECT_STATUS.md](docs/PROJECT_STATUS.md) for exactly what works
> today. Only documented features exist.

## What works today

- `devkit-core` — shared errors, network profiles, output format.
- `devkit-xdr` — decode `ScVal`, `TransactionEnvelope`, and `ContractEvent` from
  base64 XDR into JSON; classify Stellar strkeys; bounded malformed-input
  handling.
- `devkit-rpc` — endpoint validation, typed protocol layer, deterministic mock
  transport, retry policy, and a bounded HTTP JSON-RPC transport.
- `devkit-contract` — non-executing WASM validation, import/export/custom-section
  metadata, and bounded `contractspecv0` decoding.
- `devkit-analysis` — deterministic structural findings with stable rule IDs and
  SARIF 2.1.0 serialization.
- `devkit-config` — strict TOML parsing, timeout validation, environment
  overrides, and secret-safe diagnostics.
- `stellar-foundry` CLI — `version`, `doctor`, `strkey`, `scval`, `envelope`,
  `event`, `endpoint`, `inspect-wasm`, `analyze-wasm`, and `rpc-health`.
  Terminal and JSON output, SARIF output for analysis, and exit codes `0`/`2`/`3`.

See [docs/IMPLEMENTED_CAPABILITIES.md](docs/IMPLEMENTED_CAPABILITIES.md) for
exact scope and known limitations.

## Install and build

Download a prebuilt binary from the
[Releases page](https://github.com/StellarFoundry/stellar-devkit/releases), or
build from source:

```bash
cargo build --release
./target/release/stellar-foundry version
```

Or install the CLI from the workspace:

```bash
cargo install --path crates/devkit-cli
```

## Usage

```bash
stellar-foundry doctor
stellar-foundry strkey CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE
stellar-foundry scval --format json <BASE64_XDR>
stellar-foundry envelope --format json <BASE64_XDR>
stellar-foundry event --format json <BASE64_XDR>
stellar-foundry endpoint https://soroban-testnet.stellar.org
stellar-foundry inspect-wasm ./contract.wasm
stellar-foundry analyze-wasm --format sarif ./contract.wasm
stellar-foundry rpc-health https://soroban-testnet.stellar.org
```

Decode commands accept `-` to read one bounded value from stdin. Exit codes:
`0` success, `2` usage error, `3` runtime/decode error.

## Demo

Run a short tour with captured Soroban testnet data:

```bash
bash demo/run-demo.sh
```

On Windows:

```powershell
pwsh demo/run-demo.ps1
```

The demo decodes a real testnet transaction and event value, inspects a minimal
WASM module, emits SARIF, and optionally checks testnet RPC health. See
[demo/README.md](demo/README.md).

## Scope and limitations

- `contractspecv0` entries are decoded into XDR-JSON definitions with a variant
  kind and best-effort name; a normalized ABI schema is future work.
- Analysis rules are structural heuristics, not proof of a vulnerability.
- SARIF upload is a manual workflow and requires a WASM artifact plus
  code-scanning permissions.
- RPC configuration is currently consumed by `rpc-health`; other commands do not
  make network requests.
- crates.io publication is still pending; binaries are on the Releases page.

## Relationship to other Stellar tooling

- **Official crates and CLI:** DevKit depends on `stellar-xdr` and
  `stellar-strkey` and complements `stellar-cli`; it does not replace them.
- **`soroban-security-scanner`:** DevKit is a developer workflow and inspection
  toolkit (XDR, transactions, events, RPC, contract metadata). It is not a
  source-level vulnerability scanner. See
  [docs/WHY_DEVKIT.md](docs/WHY_DEVKIT.md) for the boundary.

## Design principles

- Compose with official crates; never duplicate them.
- Deterministic, offline by default, safe on untrusted input.
- Structured errors; no panics on malformed input.
- Machine-readable and human-readable output.
- Core works without AI; AI is never required.

## Documentation

- [Why DevKit](docs/WHY_DEVKIT.md)
- [Ecosystem research](docs/ECOSYSTEM_RESEARCH.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Getting started](docs/GETTING_STARTED.md)
- [CLI reference](docs/CLI.md)
- [Configuration](docs/CONFIGURATION.md)
- [Implemented capabilities](docs/IMPLEMENTED_CAPABILITIES.md)
- [Testnet demo](demo/README.md)
- [Publishing to crates.io](docs/PUBLISHING.md)
- [Memory measurement](docs/MEMORY.md)
- [Testing guide](docs/TESTING.md)
- [Project status](docs/PROJECT_STATUS.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing through Drips Wave](docs/WAVE.md)
- [Drips readiness checklist](docs/DRIPS_READINESS.md)
- [Final audit](docs/FINAL_AUDIT.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md).

## License

Apache-2.0 © 2026 StellarFoundry
