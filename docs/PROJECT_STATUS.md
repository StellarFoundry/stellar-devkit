# Project Status

**Current phase:** Foundation and developer-tooling implementation in review

**Repository:** https://github.com/StellarFoundry/stellar-devkit

**Default branch:** `main`

**Status note:** The capabilities below are on an open pull-request branch and are not all merged into `main`. PR #133 includes PR #127's security-hardening head as a second parent for combined validation. Both PRs remain open; PR #127 is unmerged.

## Implemented in the combined feature branch

- `devkit-core`: structured errors, network profiles, output formats.
- `devkit-xdr`: bounded decoding of base64 `ScVal`, `TransactionEnvelope`, and `ContractEvent`; strkey classification; malformed-input regression fixtures.
- `stellar-foundry` CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`, `event`, `endpoint`, `inspect-wasm`, `analyze-wasm`, and `rpc-health`; terminal/JSON output, SARIF output for analysis, and exit codes 0/2/3.
- `devkit-rpc`: typed RPC protocol layer, deterministic mock transport, endpoint validation, typed health/network/latest-ledger results, retry policy, and bounded HTTP JSON-RPC transport.
- `devkit-contract`: bounded, non-executing WASM validation, import/export/custom-section metadata, and bounded `contractspecv0` XDR stream decoding into entry kind/name/definition records.
- `devkit-analysis`: deterministic structural findings with stable rule IDs and SARIF 2.1.0 serialization.
- `devkit-config`: strict TOML parsing, timeout validation, environment overrides, and static diagnostics.
- CI: pinned actions, locked dependency resolution, formatting, Clippy, Linux/Windows/macOS tests, and Rust 1.91.0 MSRV tests.
- Security: RustSec dependency audit workflow, bounded malformed-XDR regression corpus, and a manual SARIF upload workflow for GitHub code scanning.
- Release preparation: release-validation workflow, release checklist, implementation-capabilities documentation, updated changelog, and Drips readiness checklist.

## Verified CI status

The previously integrated branch passed formatting, Clippy, Linux, Windows, macOS, and Rust 1.91.0 MSRV tests, plus RustSec. New contract-spec decoding, CLI config wiring, and SARIF workflow changes are awaiting a fresh combined CI and audit run before merge.

## Remaining limitations

- `contractspecv0` entries are decoded into XDR-JSON definitions with variant names and best-effort declared names; this is not yet a fully normalized ABI schema for every entry type.
- Security-analysis rules are structural heuristics, not a comprehensive vulnerability engine.
- SARIF upload is available through a manually triggered workflow; it requires a repository-relative WASM artifact and GitHub code-scanning permissions.
- RPC configuration is wired into `rpc-health`: command-line endpoint overrides environment, which overrides the TOML file. Other commands do not consume RPC configuration because they do not need it.
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
