# Project Status

**Current phase:** Phase 1 — Foundation (in progress)

**Repository:** https://github.com/StellarFoundry/stellar-devkit

**Default branch:** `main`

## Completed

- Workspace: `devkit-core`, `devkit-xdr`, `devkit-cli`.
- `devkit-core`: structured errors, network profiles (public/testnet/futurenet/
  custom) with canonical passphrases, output format.
- `devkit-xdr`: bounded decoding of base64 `ScVal`, `TransactionEnvelope`, and
  `ContractEvent` to JSON; strkey classification.
- `stellar-foundry` CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`,
  `event`, `endpoint`; terminal/JSON output; exit codes 0/2/3.
- `devkit-rpc`: typed RPC protocol layer with an injectable transport, a
  deterministic mock, endpoint validation, typed health/network/latest-ledger
  results, and a retry policy. **No live network transport yet.**
- CI: fmt, clippy, tests on Linux/Windows/macOS.
- Docs: ecosystem research, architecture, getting started, CLI, roadmap, this
  file.
- Governance: README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT, LICENSE,
  CHANGELOG, PR/issue templates.

## Not started (tracked as issues)

- RPC layer (integration with `stellar-rpc-client`, mock transport, diagnostics).
- Configuration system and logging.
- Fixtures and a mock network for deterministic tests.
- Contract inspection (WASM metadata / spec).
- Transaction and event analysis (resources, fees, auth, affected state).
- Security analysis (malformed XDR, unsafe RPC configuration, SSRF policy).
- SARIF output and GitHub Action.
- Performance benchmarks and observability.
- Release engineering (binaries, crates.io, release automation).

## Verified facts

- `cargo build`, `cargo test`, `cargo clippy -D warnings`, `cargo fmt --check`
  pass locally (run on 2026-09-22). Test count is reported in the build report.
- CLI smoke test: real contract strkey classified as `contract`.

## Known limitations

- JSON shapes come from `stellar-xdr`'s serde representation and are **unstable**
  until a curated, versioned schema is defined.
- Decoding uses a permissive limit today; a stricter, configurable limit is
  planned.
- No network functionality exists; `doctor` states this explicitly.
- No MSRV is pinned yet.
- `devkit-core` has no configuration loader yet.

## Security status

- No network access, no secrets, bounded decoding, structured errors.
- Fuzzing is planned, not yet implemented.

## Contributor backlog

- **35 open issues** across core, RPC, XDR/SCVal, transactions, events, contract
  inspection, testing, security, CLI, configuration, GitHub/SARIF,
  observability, and release.
- Complexity: 4 trivial, 19 medium, 12 high (5,650 pre-multiplier points).
- Manifest: [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md); audit:
  [DRIPS_ISSUE_AUDIT.md](DRIPS_ISSUE_AUDIT.md).
- The 125-issue / 25,000-point target is **not** supported by the current
  legitimate scope; complexity is not inflated. See
  [BUILD_REPORT.md](BUILD_REPORT.md).

## Drips documents

- [DRIPS_READINESS.md](DRIPS_READINESS.md)
- [DRIPS_APPLICATION_DRAFT.md](DRIPS_APPLICATION_DRAFT.md)
- [DRIPS_ISSUE_AUDIT.md](DRIPS_ISSUE_AUDIT.md)
