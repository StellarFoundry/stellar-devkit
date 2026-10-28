# Drips Application Draft

This is a factual draft for review before any application is submitted. It does not guarantee acceptance, points, funding, or payout. Claims below distinguish the current open PR branch from features already merged to `main`.

## Project description

StellarFoundry DevKit is a Rust developer toolkit for Stellar and Soroban. Its current implementation provides bounded decoding of `ScVal`, `TransactionEnvelope`, and `ContractEvent` XDR; Stellar strkey classification; RPC endpoint validation; a typed RPC protocol layer with deterministic mock transport; and a CLI named `stellar-foundry`.

The implementation branch also adds bounded HTTP JSON-RPC transport, non-executing WASM inspection, bounded decoding of contractspecv0 XDR entries, deterministic structural findings, SARIF 2.1.0 serialization and a manual code-scanning upload workflow, strict TOML configuration wired into `rpc-health`, and release/readiness workflows. These changes are still under review until PR #133 merges.

## Problem and ecosystem relevance

Stellar developers often need to inspect XDR values, contract artifacts, events, and RPC behavior during development and testing. DevKit centralizes several of these workflows behind reusable Rust APIs and a CLI, with an emphasis on bounded handling of untrusted inputs, deterministic tests, and structured output.

The project composes with ecosystem crates such as `stellar-xdr` and `stellar-strkey`. It is intended to complement official Stellar tooling, not replace the official CLI or RPC client.

## Current implementation and evidence

- Repository: [StellarFoundry/stellar-devkit](https://github.com/StellarFoundry/stellar-devkit)
- Combined implementation and hardening PR: [PR #133](https://github.com/StellarFoundry/stellar-devkit/pull/133)
- Security/CI hardening PR: [PR #127](https://github.com/StellarFoundry/stellar-devkit/pull/127)
- Latest combined CI run: [CI run #40679489331](https://github.com/StellarFoundry/stellar-devkit/actions/runs/40679489331)

The combined CI run passed formatting, Clippy, Linux, Windows, macOS, and Rust 1.91.0 MSRV tests: [CI run #40680053061](https://github.com/StellarFoundry/stellar-devkit/actions/runs/40680053061). The regenerated lockfile includes the new transport and WASM-parser dependencies, and the RustSec audit passed: [audit run #40680053077](https://github.com/StellarFoundry/stellar-devkit/actions/runs/40680053077).

## Current limitations

- `contractspecv0` entries are decoded with the official Stellar XDR types into XDR-JSON definitions, variant names, and best-effort names; a normalized ABI schema remains future work.
- Security-analysis rules are structural heuristics, not a comprehensive vulnerability detector.
- SARIF can be uploaded using the manual workflow when a repository-relative WASM artifact exists and code-scanning permissions are available.
- RPC configuration is wired into `rpc-health`; other commands do not consume RPC settings.
- The implementation PRs remain open. Do not state that all proposed features are already available on `main`.

## Contributor opportunities

The issue backlog documents potential work across RPC, XDR/SCVal, contract inspection, testing, security, CLI, configuration, SARIF, performance, observability, release engineering, and documentation. Backlog issues are proposed work, not completed functionality. Check current issue states and assignment requirements before describing specific issues as available.

See [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md), [DRIPS_ISSUE_AUDIT.md](DRIPS_ISSUE_AUDIT.md), and [DRIPS_READINESS.md](DRIPS_READINESS.md).

## Supporting documentation

- [Architecture](ARCHITECTURE.md)
- [Ecosystem research](ECOSYSTEM_RESEARCH.md)
- [Project status](PROJECT_STATUS.md)
- [Testing guide](TESTING.md)
- [Release checklist](RELEASE_CHECKLIST.md)
- [Contributing through Drips Wave](WAVE.md)

## Final verification before submission

- Confirm the latest combined CI remains green after final documentation updates.
- Confirm the RustSec audit passes against the committed lockfile.
- Verify the current Drips Wave program's application process, eligibility, required evidence, and deadlines with the program maintainers.
- Replace this draft only with verified facts from the final repository state; do not claim approval, funding, user traction, or contributor activity without evidence.

No application has been submitted by this draft.
