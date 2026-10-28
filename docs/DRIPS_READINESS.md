# Drips Readiness

This document records the repository's current state for a possible Drips Wave application. It does not guarantee approval, funding, payout, contributor activity, or points.

## Project summary

StellarFoundry DevKit is a Rust developer toolkit for Stellar and Soroban. It provides bounded XDR decoding, endpoint validation, a typed RPC protocol layer, WASM metadata inspection, deterministic structural findings, SARIF serialization, configuration parsing, and a CLI.

## Problem and ecosystem fit

Stellar developers routinely need to inspect XDR values, transactions, events, contract artifacts, and RPC behavior. DevKit aims to consolidate these workflows behind testable Rust APIs and command-line interfaces while composing with ecosystem crates such as `stellar-xdr` and `stellar-strkey`.

The project complements official Stellar tooling; it should not claim to replace the official CLI or RPC client.

## Implementation status on the current combined PR branch

The combined branch for PR #133 includes the feature work and the security-hardening changes from PR #127. This combined work is still under review and has **not** been merged to `main`.

- Bounded base64 decoding for `ScVal`, `TransactionEnvelope`, and `ContractEvent`, with structured errors and malformed-input regression fixtures.
- RPC endpoint policy validation and a typed RPC client with deterministic mock transport.
- HTTP JSON-RPC transport with timeout, redirects disabled, a response-size cap, response validation, and structured errors.
- Non-executing WASM validation, structural metadata, and bounded decoding of `contractspecv0` XDR entries into entry kind/name/definition records.
- Deterministic structural analysis findings with stable rule IDs and SARIF 2.1.0 serialization.
- Strict TOML configuration parsing, timeout validation, environment overrides, and static diagnostic events.
- CLI commands for WASM inspection, WASM analysis, and RPC health checks using optional TOML config, environment overrides, and a validated timeout.
- Pinned CI actions, locked dependency resolution, a dedicated Rust 1.91.0 MSRV job, RustSec dependency auditing, release-validation workflow, and release checklist.

See [IMPLEMENTED_CAPABILITIES.md](IMPLEMENTED_CAPABILITIES.md), [PROJECT_STATUS.md](PROJECT_STATUS.md), [CHANGELOG.md](../CHANGELOG.md), and [PR #133](https://github.com/StellarFoundry/stellar-devkit/pull/133) for exact scope.

## Known limitations (must be represented honestly)

- Contract-spec entries are decoded into XDR-JSON definitions with a variant kind and best-effort name; a normalized, fully typed ABI schema is still future work.
- Analysis rules are structural heuristics, not a complete vulnerability detector and not proof of malicious intent.
- SARIF upload is available through a manual workflow dispatch and requires a WASM artifact in the repository plus code-scanning permissions.
- RPC config is wired into `rpc-health`; other commands do not use RPC settings. Endpoint precedence is command-line, then environment, then config file.
- No claim is made here that the project has published crates or release binaries, has a large user base, or is approved for a Wave.

## Contributor workflow

- Issues are labeled by `area/*`, `type/*`, `difficulty/*`, and `phase/*`.
- Contributors request assignment before starting and follow each issue's scope and acceptance criteria.
- PRs reference the relevant issue, include tests and documentation, and report commands and results truthfully.
- The repository gate is `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and `cargo test --locked --workspace --all-features`. CI runs on Linux, Windows, and macOS and includes Rust 1.91.0 MSRV tests; release validation checks packaging.

## Issue quality and backlog

The backlog audit and issue labels are documented in [DRIPS_ISSUE_QUALITY_AUDIT.md](DRIPS_ISSUE_QUALITY_AUDIT.md) and [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md). Backlog issues are planned work, not completed features. Do not present issue count or labels as evidence that those features already exist.

## Security and release process

See [SECURITY.md](../SECURITY.md), [TESTING.md](TESTING.md), and [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md). Security claims must correspond to implementation and tests; secrets, endpoint values, and payloads should not be written to logs.

## Appeal readiness checklist

- [x] Both PR heads are included in a combined review branch; PR #127 remains open and unmerged.
- [x] The combined implementation passed CI on Linux, Windows, and macOS, plus formatting, Clippy, and Rust 1.91.0 MSRV tests.
- [x] The RustSec audit passed against the regenerated lockfile on the combined head.
- [x] User-facing documentation, project status, and changelog distinguish proposed work from work merged to `main`.
- [ ] The application is finalized with links to the combined PR, latest CI/audit results, and concrete completed work.
- [ ] A maintainer/program administrator has confirmed the applicable Wave application process and eligibility.

The combined CI and RustSec audit are green. Passing checks do not establish maintainer approval or Drips eligibility; verify the program-specific requirements before submitting.
