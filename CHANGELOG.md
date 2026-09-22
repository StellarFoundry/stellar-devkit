# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Workspace with `devkit-core`, `devkit-xdr`, and `devkit-cli`.
- `devkit-core`: `DevkitError`, `Network` profiles with canonical passphrases,
  `OutputFormat`.
- `devkit-xdr`: decode base64 `ScVal`, `TransactionEnvelope`, and
  `ContractEvent` to JSON with bounded reading; classify Stellar strkeys.
- `stellar-foundry` CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`,
  `event`, with terminal and JSON output and stable exit codes.
- `devkit-rpc`: typed Stellar RPC protocol layer with an injectable `Transport`,
  a deterministic `MockTransport`, validated `Endpoint` (https only; loopback
  `http`; no embedded credentials), typed `Health` / `NetworkInfo` /
  `LatestLedger`, `getEvents` and `getFeeStats` passthrough, and a retry policy
  that retries only transport failures.
- CLI: `endpoint <URL>` validates an RPC endpoint against the security policy.
- Documentation: ecosystem research, architecture, getting started, CLI
  reference, project status, roadmap.
- CI: format, clippy, and tests on Linux, Windows, and macOS.
- Tooling: `tools/changelog.py` generates reviewable release sections from
  Conventional Commits; a dry-run workflow runs on demand and on release.
- Benchmark: `cargo run --release -p devkit-xdr --example mem_profile` reports
  peak and current memory per decode input size (see `docs/MEMORY.md`).
- Fuzzing: `fuzz/` cargo-fuzz targets for XDR/SCVal decoding, exercised by a
  weekly bounded scheduled workflow (see `docs/TESTING.md`).
- Governance: README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT, LICENSE,
  pull-request and issue templates.
