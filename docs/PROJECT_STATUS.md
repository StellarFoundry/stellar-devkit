# Project Status

**Current phase:** Developer-tooling implementation merged on `main`; pre-release hardening

**Repository:** https://github.com/StellarFoundry/stellar-devkit

**Default branch:** `main`

**Status note:** PR #127 (security/CI hardening) and PR #133 (RPC transport, WASM
inspection, analysis, and configuration) were merged into `main` on 2026-10-28.
The merge commit is `aec759c`. CI and the RustSec audit are green on `main`.

## Implemented on `main`

- `devkit-core`: structured errors, network profiles, output formats.
- `devkit-xdr`: bounded decoding of base64 `ScVal`, `TransactionEnvelope`, and
  `ContractEvent`; strkey classification; malformed-input regression fixtures.
- `stellar-foundry` CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`,
  `event`, `endpoint`, `inspect-wasm`, `analyze-wasm`, and `rpc-health`;
  terminal/JSON output, SARIF output for analysis, and exit codes 0/2/3.
- `devkit-rpc`: typed RPC protocol layer, deterministic mock transport, endpoint
  validation, typed health/network/latest-ledger results, retry policy, and
  bounded HTTP JSON-RPC transport.
- `devkit-contract`: bounded, non-executing WASM validation, import/export/
  custom-section metadata, and bounded `contractspecv0` XDR stream decoding into
  entry kind/name/definition records.
- `devkit-analysis`: deterministic structural findings with stable rule IDs and
  SARIF 2.1.0 serialization.
- `devkit-config`: strict TOML parsing, timeout validation, environment
  overrides, and static diagnostics.
- CI: pinned actions, locked dependency resolution, formatting, Clippy,
  Linux/Windows/macOS tests, and Rust 1.91.0 MSRV tests.
- Security: RustSec dependency audit workflow, bounded malformed-XDR regression
  corpus, fuzz targets, and a manual SARIF upload workflow for GitHub code
  scanning.
- Release preparation: release-validation workflow, release checklist,
  implementation-capabilities documentation, updated changelog, and Drips
  readiness checklist.

## Verified CI status

The post-merge `main` run passed formatting, Clippy, Linux, Windows, macOS, and
Rust 1.91.0 MSRV tests:

- CI: https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141880
- Dependency Security Audit:
  https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141721

## Release status

The workspace version is `0.1.0`. There is **no tagged release**, no GitHub
Release with binaries, and no crates.io publication yet. The release-validation
workflow and checklist exist, but publishing remains outstanding work.

## Remaining limitations

- `contractspecv0` entries are decoded into XDR-JSON definitions with variant
  names and best-effort declared names; this is not yet a fully normalized ABI
  schema for every entry type.
- Security-analysis rules are structural heuristics, not a comprehensive
  vulnerability engine.
- SARIF upload is available through a manually triggered workflow; it requires a
  repository-relative WASM artifact and GitHub code-scanning permissions.
- RPC configuration is wired into `rpc-health`: command-line endpoint overrides
  environment, which overrides the TOML file. Other commands do not consume RPC
  configuration because they do not make network requests.
- The project is pre-release. Do not describe the workspace version as a
  published release.

## Security status

- XDR and WASM inputs are bounded in the implemented paths.
- The HTTP transport uses a finite timeout, disables redirects, bounds response
  bytes before parsing, and validates JSON-RPC responses.
- Endpoint policy rejects embedded credentials and permits plaintext HTTP only
  for loopback.
- Security claims should be limited to behavior demonstrated by code and tests;
  no system is claimed to be vulnerability-free.

## Contributor backlog

There are 112 open issues: 82 Drips-ready, 24 blocked by dependencies, 3 needing
revision, and 3 deferred. Five of the 82 are implemented or partially
implemented on `main` and are excluded from the Wave board pending closure or
re-scope (#1, #7, #20, #25, #33). The backlog is a roadmap, not a list of
completed features. Audit counts, labels, and issue descriptions must not be presented as
proof that the corresponding work is implemented.

See [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md),
[DRIPS_ISSUE_QUALITY_AUDIT.md](DRIPS_ISSUE_QUALITY_AUDIT.md), and
[WAVE_BOARD.md](WAVE_BOARD.md).

## Drips documents

- [Drips readiness checklist](DRIPS_READINESS.md)
- [Drips application draft](DRIPS_APPLICATION_DRAFT.md)
- [Appeal summary](APPEAL.md)
- [Issue audit](DRIPS_ISSUE_AUDIT.md)
- [Contributing through Drips Wave](WAVE.md)
- [Why DevKit](WHY_DEVKIT.md)
