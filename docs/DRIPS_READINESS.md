# Drips Readiness

This document records the repository's current state for a possible Drips Wave application. It does not guarantee approval, funding, payout, contributor activity, or points.

## Project summary

StellarFoundry DevKit is a Rust developer toolkit for Stellar and Soroban. It provides bounded XDR decoding, endpoint validation, a typed RPC protocol layer, WASM metadata inspection, deterministic structural findings, SARIF serialization, configuration parsing, and a CLI.

## Problem and ecosystem fit

Stellar developers routinely need to inspect XDR values, transactions, events, contract artifacts, and RPC behavior. DevKit aims to consolidate these workflows behind testable Rust APIs and command-line interfaces while composing with ecosystem crates such as `stellar-xdr` and `stellar-strkey`.

The project complements official Stellar tooling; it should not claim to replace the official CLI or RPC client.

## Implementation status on the current feature branch

Implemented in the PR under review (not yet merged to `main`):

- Bounded base64 decoding for `ScVal`, `TransactionEnvelope`, and `ContractEvent`, with structured errors and malformed-input regression fixtures.
- RPC endpoint policy validation and a typed RPC client with deterministic mock transport.
- HTTP JSON-RPC transport with timeout, redirects disabled, a response-size cap, response validation, and structured errors.
- Non-executing WASM validation and structural metadata for imports, exports, and custom sections.
- Deterministic structural analysis findings with stable rule IDs and SARIF 2.1.0 serialization.
- Strict TOML configuration parsing, timeout validation, environment overrides, and static diagnostic events.
- CLI commands for WASM inspection, WASM analysis, and explicit RPC health checks.
- CI for formatting, Clippy, and Linux/Windows/macOS tests; a RustSec dependency-audit workflow; release-validation workflow and release checklist.

See [IMPLEMENTED_CAPABILITIES.md](IMPLEMENTED_CAPABILITIES.md), [PROJECT_STATUS.md](PROJECT_STATUS.md), and the open pull requests for exact scope.

## Known limitations (must be represented honestly)

- The `contractspecv0` section is detected and measured; individual XDR entries are not yet decoded into typed function, struct, and enum metadata.
- Analysis rules are structural heuristics, not a complete vulnerability detector and not proof of malicious intent.
- SARIF serialization exists, but automatic upload to GitHub code scanning is not configured.
- The configuration library parses TOML and environment overrides, but it is not yet fully wired into all CLI commands; `rpc-health` currently accepts an explicit endpoint argument.
- The feature and hardening PR branches have passed their recorded CI runs individually, but a combined integration run is required after reconciling their divergent bases.
- No claim is made here that the project has published crates or release binaries, has a large user base, or is approved for a Wave.

## Contributor workflow

- Issues are labeled by `area/*`, `type/*`, `difficulty/*`, and `phase/*`.
- Contributors request assignment before starting and follow each issue's scope and acceptance criteria.
- PRs reference the relevant issue, include tests and documentation, and report commands and results truthfully.
- The repository gate is `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace --all-features`. CI additionally exercises Linux, Windows, and macOS; release validation checks packaging.

## Issue quality and backlog

The backlog audit and issue labels are documented in [DRIPS_ISSUE_QUALITY_AUDIT.md](DRIPS_ISSUE_QUALITY_AUDIT.md) and [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md). Backlog issues are planned work, not completed features. Do not present issue count or labels as evidence that those features already exist.

## Security and release process

See [SECURITY.md](../SECURITY.md), [TESTING.md](TESTING.md), and [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md). Security claims must correspond to implementation and tests; secrets, endpoint values, and payloads should not be written to logs.

## Appeal readiness checklist

Before applying, confirm all of the following:

- [ ] Both pull requests have been reviewed against the current `main` base and their integration strategy is explicit.
- [ ] A combined change set has passed required CI after reconciliation, including Linux, Windows, macOS, formatting, and Clippy.
- [ ] The lockfile and dependency audit are consistent with the final manifest.
- [ ] User-facing documentation, project status, and changelog accurately reflect what is merged versus still proposed.
- [ ] The application describes concrete completed work, links to the actual PRs and tests, and distinguishes limitations from delivered functionality.
- [ ] A maintainer/program administrator has confirmed the applicable Wave application process and eligibility.

Passing CI is necessary but does not by itself establish acceptance criteria, maintainer approval, or Drips eligibility. Do not submit an appeal until the integration and program-specific requirements are verified.
