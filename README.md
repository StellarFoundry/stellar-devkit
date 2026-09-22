# StellarFoundry DevKit

Developer infrastructure toolkit for the Stellar and Soroban ecosystem.

`stellar-devkit` composes with official tooling — it does **not** reimplement
XDR, strkey, or JSON-RPC. It provides a coherent workflow layer: decoding and
inspection with structured output, deterministic fixtures and mocks, analysis,
and a CLI.

The core works fully **offline** and never requires AI.

> Status: early. See [docs/PROJECT_STATUS.md](docs/PROJECT_STATUS.md) for exactly
> what works today. Only documented features exist.

## What works today

- `devkit-core` — shared errors, network profiles, output format.
- `devkit-xdr` — decode `ScVal`, `TransactionEnvelope`, and `ContractEvent` from
  base64 XDR into JSON; classify Stellar strkeys.
- `stellar-foundry` CLI — `version`, `doctor`, `strkey`, `scval`, `envelope`,
  `event`.

Everything else (RPC, fixtures, contract inspection, security analysis) is
planned and tracked as issues; it is not implemented.

## Install and build

```bash
cargo build --release
./target/release/stellar-foundry version
```

## Usage

```bash
stellar-foundry doctor
stellar-foundry strkey CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE
stellar-foundry scval --format json <BASE64_XDR>
stellar-foundry envelope --format json <BASE64_XDR>
stellar-foundry event --format json <BASE64_XDR>
```

Exit codes: `0` success, `2` usage error, `3` runtime/decode error.

## Design principles

- Compose with official crates; never duplicate them.
- Deterministic, offline by default, safe on untrusted input.
- Structured errors; no panics on malformed input.
- Machine-readable and human-readable output.
- Core works without AI; AI is never required.

## Documentation

- [Ecosystem research](docs/ECOSYSTEM_RESEARCH.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Getting started](docs/GETTING_STARTED.md)
- [CLI reference](docs/CLI.md)
- [Memory measurement](docs/MEMORY.md)
- [Project status](docs/PROJECT_STATUS.md)
- [Roadmap](docs/ROADMAP.md)
- [Contributing through Drips Wave](docs/WAVE.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md).

## License

Apache-2.0 © 2026 StellarFoundry
