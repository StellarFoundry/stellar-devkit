# Drips Readiness

This document records the repository's current state for a possible Drips Wave
application or appeal. It does not guarantee approval, funding, payout,
contributor activity, or points.

## Project summary

StellarFoundry DevKit is a Rust developer toolkit for Stellar and Soroban. It
provides bounded XDR decoding, endpoint validation, a typed RPC protocol layer,
bounded HTTP transport, WASM metadata inspection, deterministic structural
findings, SARIF serialization, configuration parsing, and a CLI.

## Problem and ecosystem fit

Stellar developers routinely need to inspect XDR values, transactions, events,
contract artifacts, and RPC behavior. DevKit consolidates these workflows behind
testable Rust APIs and command-line interfaces while composing with ecosystem
crates such as `stellar-xdr` and `stellar-strkey`.

The project complements official Stellar tooling. It does not replace the
official CLI or RPC client, and it is not a source-level vulnerability scanner.
See [WHY_DEVKIT.md](WHY_DEVKIT.md) for the boundary with
`soroban-security-scanner`.

## Implementation status

The implementation is **merged into `main`**. PR #127 (security/CI hardening)
and PR #133 (RPC transport, WASM inspection, analysis, and configuration) merged
on 2026-10-28; the merge commit is `aec759c`.

Merged capabilities:

- Bounded base64 decoding for `ScVal`, `TransactionEnvelope`, and
  `ContractEvent`, with structured errors and malformed-input regression
  fixtures.
- RPC endpoint policy validation and a typed RPC client with deterministic mock
  transport.
- HTTP JSON-RPC transport with timeout, SSRF-hardened same-host redirects, a response-size cap,
  response validation, and structured errors.
- Non-executing WASM validation, structural metadata, and bounded decoding of
  `contractspecv0` XDR entries into entry kind/name/definition records.
- Deterministic structural analysis findings with stable rule IDs and SARIF 2.1.0
  serialization.
- Strict TOML configuration parsing, timeout validation, environment overrides,
  and static diagnostic events, wired into `rpc-health`.
- CLI commands for WASM inspection, WASM analysis, and RPC health checks.
- Pinned CI actions, locked dependency resolution, a dedicated Rust 1.91.0 MSRV
  job, RustSec dependency auditing, fuzz targets, release-validation workflow,
  and release checklist.

See [IMPLEMENTED_CAPABILITIES.md](IMPLEMENTED_CAPABILITIES.md),
[PROJECT_STATUS.md](PROJECT_STATUS.md), and [CHANGELOG.md](../CHANGELOG.md) for
exact scope.

## Verified post-merge evidence

- Merge commit: `aec759c`
- PR #127: https://github.com/StellarFoundry/stellar-devkit/pull/127
- PR #133: https://github.com/StellarFoundry/stellar-devkit/pull/133
- CI on `main`:
  https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141880
- RustSec audit on `main`:
  https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141721

## Known limitations (represented honestly)

- Contract-spec entries are decoded into XDR-JSON definitions with a variant kind
  and best-effort name; a normalized, fully typed ABI schema is future work.
- Analysis rules are structural heuristics, not a complete vulnerability detector
  and not proof of malicious intent.
- SARIF upload is available through a manual workflow dispatch and requires a
  WASM artifact in the repository plus code-scanning permissions.
- RPC config is wired into `rpc-health`; other commands do not use RPC settings.
  Endpoint precedence is command line, then environment, then config file.
- `v0.1.0` is released with Linux, macOS, and Windows binaries and checksums;
  crates.io publication is prepared but still pending (needs a
  `CARGO_REGISTRY_TOKEN` secret).
- The project has limited external adoption: no external contributors have merged
  PRs yet, and the repository is new. Do not present issue counts or labels as
  evidence of traction.

## Contributor workflow

- Issues are labeled by `area/*`, `type/*`, `difficulty/*`, and `phase/*`.
- Contributors request assignment before starting and follow each issue's scope
  and acceptance criteria.
- PRs reference the relevant issue, include tests and documentation, and report
  commands and results truthfully.
- The repository gate is `cargo fmt --all -- --check`,
  `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
  and `cargo test --locked --workspace --all-features`. CI runs on Linux, Windows,
  and macOS and includes Rust 1.91.0 MSRV tests; release validation checks
  packaging.

## Issue quality and backlog

There are 110 open issues: 77 Drips-ready, 24 blocked, 6 needing revision, and 3
deferred. #20 and #33 were closed as completed after the merge; #1, #7, and #25
are partially implemented and marked `status/needs-revision`. The audit and
manifest are regenerated from live GitHub issue numbers:

- [DRIPS_ISSUE_QUALITY_AUDIT.md](DRIPS_ISSUE_QUALITY_AUDIT.md)
- [DRIPS_ISSUE_AUDIT.md](DRIPS_ISSUE_AUDIT.md)
- [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md)
- [WAVE_BOARD.md](WAVE_BOARD.md) — 20 curated, independent, Wave-sized issues

Backlog issues are planned work, not completed features.

## Security and release process

See [SECURITY.md](../SECURITY.md), [TESTING.md](TESTING.md), and
[RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md). Security claims must correspond to
implementation and tests; secrets, endpoint values, and payloads should not be
written to logs.

## Appeal readiness checklist

- [x] Implementation merged into `main` (PR #127 and PR #133, merge `aec759c`).
- [x] Post-merge CI passed on Linux, Windows, and macOS, plus formatting, Clippy,
      and Rust 1.91.0 MSRV tests.
- [x] RustSec audit passed on `main`.
- [x] User-facing documentation, project status, and changelog describe merged
      work as merged and planned work as planned.
- [x] Issue audit, backlog, and Wave board regenerated against `main`; completed
      issues are no longer advertised as open work.
- [x] Scope boundary with `soroban-security-scanner` documented in
      [WHY_DEVKIT.md](WHY_DEVKIT.md).
- [x] Appeal narrative and evidence table prepared in [APPEAL.md](APPEAL.md).
- [ ] Confirm in the Drips dashboard whether the repository is pending or
      rejected, and note the rejection date and any stated reasons.
- [ ] Confirm the appeal window (first appeal no earlier than two weeks after
      rejection) and submit through the dashboard only.
- [ ] Optional but recommended before appealing: tagged `v0.1.0` release with
      binaries/checksums, and 3–5 external contributions or documented usage.

Passing checks do not establish maintainer approval or Drips eligibility. Verify
the program-specific status before submitting.
