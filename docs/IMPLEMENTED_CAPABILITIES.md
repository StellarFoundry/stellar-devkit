# Implemented library capabilities

This branch adds reusable library APIs for the remaining developer-tooling workstreams:

- **RPC transport:** `devkit_rpc::HttpTransport` sends JSON-RPC POST requests with a finite timeout, disabled redirects, request ID validation, and a response byte cap enforced before JSON parsing. Endpoint policy continues to allow HTTPS and loopback-only HTTP.
- **WASM inspection:** `devkit_contract::inspect_wasm` validates the WASM container without executing it, and reports imports, exports, custom sections, and decodes the `contractspecv0` section as a bounded stream of official `ScSpecEntry` XDR values. It returns each decoded definition as XDR-JSON with a variant kind and best-effort declared name; the stream is capped at 4,096 entries.
- **Analysis and SARIF:** `devkit_analysis::analyze` emits deterministic findings with stable rule IDs and evidence. `to_sarif` produces SARIF 2.1.0. Current rules are structural heuristics, not proof of a vulnerability.
- **Configuration/diagnostics:** `devkit_config::Config` parses strict TOML, validates timeouts, and supports environment overrides. Verbose diagnostics accept static event names and avoid printing endpoints or payloads.
- **Release readiness:** the release-validation workflow runs tests, formatting/lint checks, and package validation on version tags.

CLI commands are available as `inspect-wasm`, `analyze-wasm`, and `rpc-health`. A manual GitHub Actions workflow can upload SARIF to code scanning when given a repository-relative WASM artifact path. The analysis rules remain structural heuristics, not a comprehensive vulnerability engine. No WASM is executed and no network request is made unless a caller explicitly constructs and uses the HTTP transport.
