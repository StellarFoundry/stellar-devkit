# Roadmap

This roadmap distinguishes **Completed** (works today), **In progress**, and
**Planned** (backed by real issues). Nothing planned is claimed as implemented.

## Completed (works today)

- Workspace: `devkit-core`, `devkit-xdr`, `devkit-rpc`, `devkit-contract`,
  `devkit-analysis`, `devkit-config`, and `devkit-cli`.
- Bounded base64 decoding of `ScVal`, `TransactionEnvelope`, and
  `ContractEvent` to JSON via `stellar-xdr`.
- Strkey classification via `stellar-strkey`.
- Endpoint security validation (HTTPS; loopback HTTP; no credentials).
- Typed RPC protocol layer, deterministic mock transport, retry policy, and
  bounded HTTP JSON-RPC transport with a finite timeout.
- Non-executing WASM validation, import/export/custom-section metadata, and
  bounded `contractspecv0` decoding.
- Deterministic structural findings and SARIF 2.1.0 serialization.
- Strict TOML configuration with environment overrides, wired into `rpc-health`.
- CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`, `event`, `endpoint`,
  `inspect-wasm`, `analyze-wasm`, `rpc-health`.
- CI (fmt, Clippy, tests) on Linux, Windows, and macOS; Rust 1.91.0 MSRV;
  RustSec audit; malformed-XDR corpus; fuzz targets.

## In progress

- Release packaging: tag `v0.1.0`, publish release binaries with checksums, and
  publish the workspace crates to crates.io.
- Typed RPC response models for `getLedgerEntries`, `getTransaction(s)`, and
  `getFeeStats`, plus pagination for list methods.

## Planned (issue-backed phases)

There are 110 open issues. The counts below come from the current labels.

| Phase | Focus | Issues |
| ----- | ----- | ------ |
| 01 Foundation | Error taxonomy, shared models, output envelope, validation primitives | 5 |
| 02 RPC | Typed models, request construction, error taxonomy, pagination, rate limits, integration server | 15 |
| 03 XDR / SCVal | Collections, addresses, result/meta, diagnostic events, ledger entries, typed conversions, schema | 13 |
| 04 Analysis | Envelope/operation/fee/signature/memo/bounds/footprint/result analysis, events, ledger helpers | 15 |
| 05 Contracts | Contract IDs, storage, events-to-spec, environment, CLI | 10 |
| 06 Testing | Fixture loader, snapshots, cross-platform, corpus runner, WASM fixtures | 7 |
| 07 Security | Rule engine, findings, suppression, corpus, FP/FN tests, metadata/WASM/RPC hardening, SARIF catalog | 11 |
| 08 CLI / config | Configuration, profiles, global flags, error UX, verbosity, JSON errors, stdin | 7 |
| 09 GitHub / CI | Action, reusable workflow, SARIF upload, baseline, triage, auditing | 10 |
| 10 Performance / observability | RPC and decode benchmarks, cache, regression tests, logging | 5 |
| 12 Release | Versioning policy, crates.io readiness, SBOM, compatibility matrix | 7 |
| 13 Documentation | RPC, XDR/SCVal, contract, security authoring, configuration guides | 5 |

The exact issue numbers, titles, and dependencies are in
[ISSUE_BACKLOG.md](ISSUE_BACKLOG.md). See also [PROJECT_STATUS.md](PROJECT_STATUS.md)
and [WAVE_BOARD.md](WAVE_BOARD.md).

## Future (not yet specified)

- Editor integration (diagnostics provider).
- Correlation of findings across transactions.
- Optional, clearly labeled assistance with network safeguards (never required).

## Dependency shape

```
foundation ─▶ xdr/scval ─▶ transactions ─▶ contracts ─▶ inspection
     └─────▶ rpc protocol ─▶ live transport ─▶ resilience ─▶ events/ledger
```

Structural security analysis and SARIF are implemented for the checks the
project actually ships. Source-level vulnerability detection remains the
responsibility of `soroban-security-scanner`.

Issues state their dependencies explicitly. Many are independent and can be
started in parallel.
