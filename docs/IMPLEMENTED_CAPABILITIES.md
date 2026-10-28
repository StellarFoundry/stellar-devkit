# Implemented library capabilities

The merged implementation on `main` provides reusable library APIs for the
developer-tooling workstreams below. This page describes scope, not a roadmap.

- **RPC transport:** `devkit_rpc::HttpTransport` sends JSON-RPC POST requests
  with a finite timeout, disabled redirects, request ID validation, and a
  response byte cap enforced before JSON parsing. Endpoint policy continues to
  allow HTTPS and loopback-only HTTP.
- **WASM inspection:** `devkit_contract::inspect_wasm` validates the WASM
  container without executing it, and reports imports, exports, custom sections,
  and decodes the `contractspecv0` section as a bounded stream of official
  `ScSpecEntry` XDR values. It returns each decoded definition as XDR-JSON with
  a variant kind and best-effort declared name; the stream is capped at 4,096
  entries.
- **Analysis and SARIF:** `devkit_analysis::analyze` emits deterministic findings
  with stable rule IDs and evidence. `to_sarif` produces SARIF 2.1.0. Current
  rules are structural heuristics, not proof of a vulnerability.
- **Configuration/diagnostics:** `devkit_config::Config` parses strict TOML,
  validates timeouts, and supports environment overrides. Verbose diagnostics
  accept static event names and avoid printing endpoints or payloads.
- **Release readiness:** the release-validation workflow runs tests,
  formatting/lint checks, and package validation on version tags.

## CLI

```text
stellar-foundry version
stellar-foundry doctor
stellar-foundry strkey <VALUE>
stellar-foundry scval <BASE64|->
stellar-foundry envelope <BASE64|->
stellar-foundry event <BASE64|->
stellar-foundry endpoint <URL>
stellar-foundry inspect-wasm <PATH>
stellar-foundry analyze-wasm [--format sarif] <PATH>
stellar-foundry rpc-health [ENDPOINT]
```

`--format json` produces machine-readable output and structured JSON errors.
`--format sarif` is accepted by `analyze-wasm`. Exit codes are `0` success, `2`
usage error, and `3` runtime/decode error.

A manual GitHub Actions workflow can upload SARIF to code scanning when given a
repository-relative WASM artifact path. The analysis rules remain structural
heuristics, not a comprehensive vulnerability engine. No WASM is executed, and
no network request is made unless a caller explicitly runs `rpc-health` or
constructs `HttpTransport`.

## Not implemented yet

- Normalized ABI schema for every `contractspecv0` entry type.
- Typed models for `getLedgerEntries`, `getTransaction(s)`, `getFeeStats`, and
  paginated `getEvents`.
- Exponential backoff with jitter, cancellation, and `Retry-After` handling.
- Composite GitHub Action, baseline scanning, and SBOM/attestations.
- Tagged release, release binaries, and crates.io publication.
- Caching, persistence, and structured logging beyond static diagnostics.

These are tracked as open issues; see [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md).
