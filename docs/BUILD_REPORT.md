# Build Report

## Repository

<https://github.com/StellarFoundry/stellar-devkit> — default branch `main`,
commit `aec759c` (2026-10-28).

## Architecture

Seven crates with a one-way dependency direction:

```
devkit-cli ──▶ devkit-rpc ──▶ devkit-core
     ├──────▶ devkit-xdr ──▶ devkit-core
     ├──────▶ devkit-contract ──▶ devkit-core
     ├──────▶ devkit-analysis ──▶ devkit-core
     └──────▶ devkit-config
```

See [ARCHITECTURE.md](ARCHITECTURE.md).

## Implemented components

- `devkit-core`: `DevkitError`, `Network` profiles with canonical passphrases,
  `OutputFormat`.
- `devkit-xdr`: bounded base64 decoding of `ScVal`, `TransactionEnvelope`, and
  `ContractEvent` to JSON; strkey classification; malformed-input corpus.
- `devkit-rpc`: `Transport` trait, deterministic `MockTransport`, bounded
  `HttpTransport`, validated `Endpoint`, typed `Health`/`NetworkInfo`/
  `LatestLedger`, `getEvents`/`getFeeStats` passthrough, retry policy.
- `devkit-contract`: non-executing WASM validation, imports/exports/custom
  sections, and bounded `contractspecv0` decoding.
- `devkit-analysis`: deterministic structural findings and SARIF 2.1.0 output.
- `devkit-config`: strict TOML parsing, timeout validation, environment
  overrides, and static diagnostics.
- `devkit-cli`: `stellar-foundry version|doctor|strkey|scval|envelope|event|endpoint|inspect-wasm|analyze-wasm|rpc-health`.

## Tests

43 tests across unit, integration, malformed-corpus, and CLI tests. Run with
`cargo test --locked --workspace --all-features`. CI runs the same suite on
Linux, Windows, and macOS, plus an MSRV job on Rust 1.91.0.

## CI

`CI` workflow: format, Clippy (`-D warnings`), and tests on three operating
systems. Additional workflows: RustSec dependency audit, scheduled fuzz smoke,
release validation on version tags, changelog generation, and manual SARIF
upload.

## Documentation

`README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`,
`CHANGELOG.md`, `docs/ECOSYSTEM_RESEARCH.md`, `docs/WHY_DEVKIT.md`,
`docs/ARCHITECTURE.md`, `docs/GETTING_STARTED.md`, `docs/CLI.md`,
`docs/ROADMAP.md`, `docs/PROJECT_STATUS.md`, `docs/IMPLEMENTED_CAPABILITIES.md`,
`docs/ISSUE_BACKLOG.md`, `docs/WAVE_BOARD.md`, `docs/DRIPS_*`, `docs/APPEAL.md`,
and this report.

## Security

No network access by default; the only networked command is `rpc-health` with an
explicit endpoint. Decoding is bounded; errors are structured; endpoint policy
rejects plaintext remote HTTP, embedded credentials, and unknown schemes. The
HTTP transport disables redirects and caps response bytes before parsing. Fuzz
targets cover `ScVal`, `TransactionEnvelope`, and `ContractEvent`. No secrets are
read or logged.

## Release status

`v0.1.0` is tagged and published with Linux x86_64, macOS arm64, and Windows
x86_64 archives plus SHA256 checksums:

<https://github.com/StellarFoundry/stellar-devkit/releases/tag/v0.1.0>

crates.io publication is prepared but still pending; see
[PUBLISHING.md](PUBLISHING.md).

## Issue backlog

- **Total open issues:** 110
- **Trivial:** 6
- **Medium:** 78
- **High:** 26

Issue complexity is assigned from the scope of each individual issue rather than
an aggregate reward target. Complexity was not inflated. Most of the backlog is
**not implemented**.

## Drips readiness

See [DRIPS_READINESS.md](DRIPS_READINESS.md),
[DRIPS_APPLICATION_DRAFT.md](DRIPS_APPLICATION_DRAFT.md), and
[APPEAL.md](APPEAL.md). Approval, points, and payout are not guaranteed.

## Known limitations

- crates.io publication is still pending; use the GitHub Release or build from
  source.
- `contractspecv0` output is XDR-JSON with best-effort names, not a normalized
  ABI.
- Analysis rules are structural heuristics, not a vulnerability proof.
- RPC configuration applies to `rpc-health`; other commands do not make network
  requests.
- JSON output shape is unstable until the versioned schema lands.
- No external contributor has merged a PR yet.

## Recommended next steps

1. Tag and publish `v0.1.0` with release binaries and checksums.
2. Publish the workspace crates to crates.io.
3. Add typed RPC models for `getLedgerEntries`, `getTransaction(s)`, and
   `getFeeStats`.
4. Define the versioned JSON schema.
5. Grow external usage and contributors before the next appeal.
