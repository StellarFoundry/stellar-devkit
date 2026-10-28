# Drips Application Draft

This is a factual draft for a Drips Wave application or appeal. It does not
guarantee acceptance, points, funding, or payout. Every claim below is true of
`main` at commit `aec759c` (2026-10-28) unless marked as a limitation.

## Project description

StellarFoundry DevKit is a Rust developer toolkit for Stellar and Soroban. It
provides bounded decoding of `ScVal`, `TransactionEnvelope`, and
`ContractEvent` XDR; Stellar strkey classification; RPC endpoint validation; a
typed RPC protocol layer with deterministic mocks and a bounded HTTP transport;
non-executing WASM inspection; deterministic structural findings with SARIF 2.1.0
output; strict TOML configuration; and a CLI named `stellar-foundry`.

The implementation is merged into `main`. There is no tagged release or crates.io
publication yet.

## Problem and ecosystem relevance

Stellar developers often need to inspect XDR values, contract artifacts, events,
and RPC behavior during development and testing. DevKit centralizes several of
these workflows behind reusable Rust APIs and a CLI, with an emphasis on bounded
handling of untrusted inputs, deterministic tests, and structured output.

The project composes with ecosystem crates such as `stellar-xdr` and
`stellar-strkey`. It complements official Stellar tooling and does not replace
the official CLI or RPC client. It is a developer workflow and inspection layer,
not a source-level vulnerability scanner; see
[WHY_DEVKIT.md](WHY_DEVKIT.md) for the boundary with
`soroban-security-scanner`.

## Current implementation and evidence

- Repository: [StellarFoundry/stellar-devkit](https://github.com/StellarFoundry/stellar-devkit)
- Merge commit: `aec759c` (2026-10-28)
- Security/CI hardening PR: [PR #127](https://github.com/StellarFoundry/stellar-devkit/pull/127) — merged
- Implementation PR: [PR #133](https://github.com/StellarFoundry/stellar-devkit/pull/133) — merged
- Post-merge CI on `main`: [CI run 40690141880](https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141880)
- Post-merge RustSec audit: [audit run 40690141721](https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141721)

The post-merge CI run passed formatting, Clippy, Linux, Windows, macOS, and Rust
1.91.0 MSRV tests. The RustSec audit passed against the committed lockfile.

Merged capabilities:

- Bounded XDR decoding with structured errors and a malformed-input corpus.
- RPC endpoint validation, typed protocol models, deterministic mock transport,
  retry policy, timeout, and bounded HTTP transport.
- Non-executing WASM inspection and bounded `contractspecv0` decoding.
- Deterministic structural findings and SARIF 2.1.0 serialization.
- Strict TOML configuration and secret-safe diagnostics wired into `rpc-health`.
- CLI commands `inspect-wasm`, `analyze-wasm`, and `rpc-health`.
- Pinned CI, locked dependencies, MSRV job, RustSec audit, fuzz targets, and
  release-validation workflow.

## What changed since the previous review

Use this section for an appeal. Replace the bracketed text with the rejection
reason and confirm dates in the Drips dashboard.

| Review concern | Change since review | Evidence |
| -------------- | ------------------- | -------- |
| [Fill in the stated reason] | PR #127 and PR #133 are merged; the implementation is on `main`, not an open branch | Merge `aec759c`; CI 40690141880; audit 40690141721 |
| Documentation did not match the code | Status, readiness, audit, changelog, roadmap, architecture, and getting-started docs now describe merged work accurately | `docs/PROJECT_STATUS.md`, `docs/DRIPS_READINESS.md`, `CHANGELOG.md`, `docs/ARCHITECTURE.md` |
| Backlog advertised completed work | Issue audit, backlog, and Wave board regenerated from live GitHub issues; completed items removed from the board | `docs/WAVE_BOARD.md`, `docs/ISSUE_BACKLOG.md` |
| Possible overlap with another org repo | Scope boundary documented: DevKit is XDR/RPC/transaction inspection; `soroban-security-scanner` is source-level detection | `docs/WHY_DEVKIT.md` |
| [Any other stated reason] | [Describe the specific change] | [Link] |

## Contributor opportunities

The backlog contains 112 open issues: 82 Drips-ready, 24 blocked by dependencies,
3 needing revision, and 3 deferred. Five of the 82 are implemented or partially
implemented on `main` and are excluded from the board pending closure or re-scope
(#1, #7, #20, #25, #33). [WAVE_BOARD.md](WAVE_BOARD.md) curates 20 independent,
Wave-sized issues (3 trivial, 13 medium, 4 high; 3,050 points).
Backlog issues are proposed work, not completed functionality. Check current
issue states and assignment requirements before describing specific issues as
available.

## Current limitations

- `contractspecv0` entries are decoded with the official Stellar XDR types into
  XDR-JSON definitions, variant names, and best-effort names; a normalized ABI
  schema remains future work.
- Security-analysis rules are structural heuristics, not a comprehensive
  vulnerability detector.
- SARIF can be uploaded using the manual workflow when a repository-relative WASM
  artifact exists and code-scanning permissions are available.
- RPC configuration is wired into `rpc-health`; other commands do not consume
  RPC settings.
- No tagged release, release binaries, or crates.io publication exists yet.
- External adoption is limited; no external contributor has merged a PR yet.

## Supporting documentation

- [Why DevKit](WHY_DEVKIT.md)
- [Architecture](ARCHITECTURE.md)
- [Ecosystem research](ECOSYSTEM_RESEARCH.md)
- [Project status](PROJECT_STATUS.md)
- [Implemented capabilities](IMPLEMENTED_CAPABILITIES.md)
- [Testing guide](TESTING.md)
- [Release checklist](RELEASE_CHECKLIST.md)
- [Contributing through Drips Wave](WAVE.md)
- [Appeal summary](APPEAL.md)

## Final verification before submission

- [x] Post-merge CI is green on `main`.
- [x] RustSec audit passes against the committed lockfile.
- [ ] Confirm the repository's current application status in the Drips dashboard.
- [ ] Confirm the appeal window and submit through the dashboard only.
- [ ] Replace bracketed text with the actual rejection reason and dates.
- [ ] Do not claim approval, funding, user traction, or contributor activity
      without evidence.

No application or appeal has been submitted by this draft.
