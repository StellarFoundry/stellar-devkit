# Roadmap

This roadmap distinguishes **Completed** (works today), **In progress**, and
**Planned** (backed by real issues). Nothing planned is claimed as implemented.

## Completed (works today)

- Workspace: `devkit-core`, `devkit-xdr`, `devkit-rpc`, `devkit-cli`.
- Bounded base64 decoding of `ScVal`, `TransactionEnvelope`, and
  `ContractEvent` to JSON via `stellar-xdr`.
- Strkey classification via `stellar-strkey`.
- Endpoint security validation (https only; loopback http; no credentials).
- Typed RPC protocol layer with a deterministic mock transport and retry policy.
- CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`, `event`, `endpoint`.
- CI (fmt, clippy, tests) on Linux, Windows, and macOS.

## In progress

- Nothing is partially implemented; the next work item is the live RPC transport
  (issue-backed).

## Planned (issue-backed phases)

| Phase | Focus | Issues |
| ----- | ----- | ------ |
| 01 Foundation | Error taxonomy, shared models, output envelope, validation primitives | 5 |
| 02 RPC | Live transport, request construction, error taxonomy, pagination, rate limits, integration server | 13 |
| 03 XDR / SCVal | Collections, addresses, result/meta, diagnostic events, ledger entries, typed conversions, schema | 14 |
| 04 Analysis | Envelope/operation/fee/signature/memo/bounds/footprint/result analysis, events, ledger helpers | 17 |
| 05 Contracts | Contract IDs, WASM validation, spec inspection, storage, events-to-spec, environment, CLI | 12 |
| 06 Testing | Fixture loader, malformed corpus, snapshots, cross-platform, corpus runner, WASM fixtures, fuzz CI | 12 |
| 07 Security | Rule engine, findings, suppression, corpus, FP/FN tests, metadata/WASM/RPC hardening, SARIF catalog | 11 |
| 08 CLI / config | Configuration, profiles, global flags, error UX, verbosity, JSON errors, stdin | 12 |
| 09 GitHub / CI | Action, reusable workflow, SARIF upload, baseline, release validation, triage, auditing | 10 |
| 10 Performance / observability | RPC and decode benchmarks, memory, cache, regression tests, logging | 9 |
| 11 Developer experience | Docs and integration guides | 5 |
| 12 Release | Versioning policy, changelog automation, crates.io readiness, SBOM, compatibility matrix | 8 |
| 13 Documentation | RPC, XDR/SCVal, contract, security authoring, configuration guides | 5 |

The exact issue counts per phase come from
[ISSUE_BACKLOG.md](ISSUE_BACKLOG.md). See also [PROJECT_STATUS.md](PROJECT_STATUS.md).

## Future (not yet specified)

- Editor integration (diagnostics provider).
- Correlation of findings across transactions.
- Optional, clearly labeled assistance with network safeguards (never required).

## Dependency shape

```
foundation ─▶ xdr/scval ─▶ transactions ─▶ contracts ─▶ security engine ─▶ SARIF/GitHub
     └─────▶ rpc protocol ─▶ live transport ─▶ resilience ─▶ events/ledger
```

Issues state their dependencies explicitly. Many are independent and can be
started in parallel.
