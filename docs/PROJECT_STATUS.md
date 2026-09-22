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

- **119 open issues** after a quality audit (down from 121).
- Audit result: **89 Drips-ready**, 24 blocked by dependency, 3 deferred, 3 needing
  revision. See [DRIPS_ISSUE_QUALITY_AUDIT.md](DRIPS_ISSUE_QUALITY_AUDIT.md).
- Status labels `status/ready`, `status/blocked`, `status/deferred`, and
  `status/needs-revision` are applied.
- Manifest: [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md); prior audit:
  [DRIPS_ISSUE_AUDIT.md](DRIPS_ISSUE_AUDIT.md).
- The backlog is a roadmap: most issues are **not implemented**.

## Drips documents

- [DRIPS_READINESS.md](DRIPS_READINESS.md)
- [DRIPS_APPLICATION_DRAFT.md](DRIPS_APPLICATION_DRAFT.md)
- [DRIPS_ISSUE_AUDIT.md](DRIPS_ISSUE_AUDIT.md)
