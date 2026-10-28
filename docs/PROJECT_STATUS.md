# Project Status

**Current phase:** Foundation and developer-tooling implementation in review

**Repository:** https://github.com/StellarFoundry/stellar-devkit

**Default branch:** `main`

**Status note:** The capabilities below include work on open pull-request branches. They are not all merged into `main`. See PRs [#127](https://github.com/StellarFoundry/stellar-devkit/pull/127) and [#133](https://github.com/StellarFoundry/stellar-devkit/pull/133).

## Implemented in the current code / feature branch

- `devkit-core`: structured errors, network profiles, output formats.
- `devkit-xdr`: bounded decoding of base64 `ScVal`, `TransactionEnvelope`, and `ContractEvent`; strkey classification; malformed-input regression fixtures.
- `stellar-foundry` CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`, `event`, `endpoint`, `inspect-wasm`, `analyze-wasm`, and `rpc-health`; terminal/JSON output, SARIF output for analysis, and exit codes 0/2/3.
- `devkit-rpc`: typed RPC protocol layer, deterministic mock transport, endpoint validation, typed health/network/latest-ledger results, retry policy, and bounded HTTP JSON-RPC transport.
- `devkit-contract`: bounded, non-executing WASM validation and import/export/custom-section metadata.
- `devkit-analysis`: deterministic structural findings with stable rule IDs and SARIF 2.1.0 serialization.
- `devkit-config`: strict TOML parsing, timeout validation, environment overrides, and static diagnostics.
- CI: formatting, Clippy, and tests on Linux, Windows, and macOS; Rust 1.91 MSRV job, scheduled fuzz smoke tests, and RustSec dependency audit.
- Release preparation: release-validation workflow, release checklist, implementation-capabilities documentation, and updated Drips readiness checklist.

## Verified CI status

- The pre-integration PR #133 head passed formatting, Clippy, and Linux, Windows, and macOS tests.
- The pre-integration PR #127 head passed its recorded CI jobs and RustSec audit.
- The combined merge commit must pass a fresh CI run before the integration is considered validated.

## Remaining limitations and integration work

- `contractspecv0` is detected and sized but individual XDR specification entries are not yet decoded into typed function/struct/enum metadata.
- Security-analysis rules are structural heuristics, not a comprehensive vulnerability engine.
- SARIF serialization is implemented; automatic GitHub code-scanning upload is not configured.
- The configuration library exists, but it is not yet fully wired into every CLI command. `rpc-health` takes an explicit endpoint argument.
- Do not describe unmerged work as already released or available from `main`.

## Security status

- XDR and WASM inputs are bounded in the implemented paths.
- The HTTP transport uses a finite timeout, disables redirects, bounds response bytes before parsing, and validates JSON-RPC responses.
- Endpoint policy rejects embedded credentials and permits plaintext HTTP only for loopback.
- Security claims should be limited to behavior demonstrated by code and tests; no system is claimed to be vulnerability-free.

## Contributor backlog

The issue backlog is a roadmap, not a list of completed features. Audit counts, labels, and issue descriptions must not be presented as proof that the corresponding work is implemented. See [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md) and [DRIPS_ISSUE_QUALITY_AUDIT.md](DRIPS_ISSUE_QUALITY_AUDIT.md).

## Drips documents

- [Drips readiness checklist](DRIPS_READINESS.md)
- [Drips application draft](DRIPS_APPLICATION_DRAFT.md)
- [Issue audit](DRIPS_ISSUE_AUDIT.md)
- [Contributing through Drips Wave](WAVE.md)
