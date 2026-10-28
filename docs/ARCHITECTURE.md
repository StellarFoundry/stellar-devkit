# Architecture

## Goals

Provide a coherent developer infrastructure layer for Stellar and Soroban that
composes with official tooling instead of duplicating it. The core is
deterministic, works fully offline by default, and never requires AI.

## Non-goals

- Reimplementing XDR, strkey, or a JSON-RPC client.
- Replacing `stellar-cli` for build/deploy/invoke.
- Proving contracts secure.
- Network access by default.
- Source-level vulnerability detection (that belongs to
  `soroban-security-scanner`).

## Crate map

```
crates/
  devkit-core      # errors, network profiles, output format (no I/O, no Stellar deps)
  devkit-xdr       # XDR / SCVal / transaction / event inspection (wraps stellar-xdr)
  devkit-rpc       # endpoint policy, typed protocol models, mock + HTTP transport
  devkit-contract  # non-executing WASM validation and contractspec metadata
  devkit-analysis  # deterministic structural findings and SARIF serialization
  devkit-config    # strict TOML configuration and secret-safe diagnostics
  devkit-cli       # the `stellar-foundry` binary (clap)
```

Dependency direction is one way:

```
devkit-cli ──▶ devkit-xdr ──▶ devkit-core
     │              └──▶ stellar-xdr, stellar-strkey
     ├──▶ devkit-rpc ──▶ devkit-core
     ├──▶ devkit-contract ──▶ devkit-core, stellar-xdr, wasmparser
     ├──▶ devkit-analysis ──▶ devkit-core, devkit-contract
     └──▶ devkit-config ──▶ (serde, toml)
```

`devkit-core` has no Stellar dependencies so it can be reused cheaply and tested
in isolation. Only `devkit-xdr` and `devkit-contract` touch XDR types.

## Core abstractions

- **`DevkitError`** — the single structured error type. Variants: `Decode`,
  `Encode`, `InvalidInput`, `Config`, `Io`. Every fallible public function
  returns it.
- **`Network`** — public / testnet / futurenet / custom, with the canonical
  network passphrase. Mainnet deliberately has **no** hardcoded RPC endpoint;
  public providers change, so endpoints are configured explicitly.
- **`OutputFormat`** — `Terminal` (human) or `Json` (machine). Commands render
  through one function each so both formats stay in sync.

## XDR / SCVal architecture

`devkit-xdr` exposes pure functions:

- `inspect_scval_base64`, `inspect_envelope_base64`, `inspect_event_base64`
  decode with `stellar-xdr` under explicit **limits**, then serialize to JSON.
- `describe_strkey` classifies a strkey using `stellar-strkey`.

Design rules:

- All decoding is bounded. The decoder does not use `Limits::none()`.
- Failures are structured, never panics. Malformed input is a first-class test
  case; `fixtures/malformed/` and `crates/devkit-xdr/tests/malformed_corpus.rs`
  cover the regression corpus.
- JSON is produced by `stellar-xdr`'s `serde` representation today. A curated,
  versioned schema is future work; until then the JSON shape is documented as
  **unstable**.

## RPC architecture

`devkit-rpc` separates protocol from transport:

- `Endpoint` validates schemes, rejects embedded credentials, and permits
  plaintext HTTP only for loopback hosts.
- `Transport` is the injection point. `MockTransport` is deterministic and used
  by tests; `HttpTransport` performs bounded JSON-RPC over reqwest with a finite
  timeout and redirects disabled.
- Typed models cover `getHealth`, `getNetwork`, and `getLatestLedger`. Other RPC
  methods return raw JSON until they are modeled.
- `RetryPolicy` retries transport failures and rate-limit responses. It does not
  yet implement exponential backoff or jitter for transport failures.

### Rate limits

`HttpTransport` detects HTTP `429 Too Many Requests` and returns a structured
`DevkitError::RateLimited` instead of a generic I/O error. The error carries the
`Retry-After` delay when the endpoint supplies one in seconds.

`Client` retries rate-limit responses only while the delay fits inside the
policy's cumulative cap (`RetryPolicy::max_rate_limit_wait`, 60 seconds by
default). A missing or non-numeric `Retry-After` falls back to a 1-second
backoff. If the requested wait — or the sum of waits for one call — would exceed
the cap, the client returns a clear rate-limit error immediately and never
sleeps past the cap. `RetryPolicy::with_max_rate_limit_wait` lowers or raises the
cap for a caller; a zero cap turns waiting off.

The sleep is synchronous and bounded. Callers that cannot block should use a
custom transport and handle `DevkitError::RateLimited` themselves.

## Contract inspection

`devkit-contract` never executes contract code. It validates the WASM container,
reports imports, exports, and custom sections, and decodes `contractspecv0` as a
bounded stream of official `ScSpecEntry` XDR values. Entry definitions are
returned as XDR-JSON with a variant kind and best-effort name; a normalized ABI
schema is future work.

## Analysis and SARIF

`devkit-analysis` emits deterministic findings with stable rule IDs and evidence,
and serializes SARIF 2.1.0. The rules are structural heuristics, not a
comprehensive vulnerability engine, and findings are not proof of malicious
intent. SARIF upload is a manual workflow that requires a repository-relative
WASM artifact and code-scanning permissions.

## Configuration and diagnostics

- `devkit-config` parses strict TOML, validates timeouts, applies environment
  overrides, and emits static diagnostic event names. It never logs endpoint
  values or payloads.
- Configuration is wired into `rpc-health`. Other commands do not make network
  requests, so they do not consume RPC settings.
- No caching or persistence exists yet.

## CLI architecture

`stellar-foundry` uses `clap` derive. Global `--format` selects terminal, JSON,
or SARIF output. Exit codes are stable: `0` success, `2` usage error, `3`
runtime/decode error. Commands only exist if they do real work; `doctor` reports
capabilities and states that network access is explicit and limited to
`rpc-health`.

Commands: `version`, `doctor`, `strkey`, `scval`, `envelope`, `event`,
`endpoint`, `inspect-wasm`, `analyze-wasm`, `rpc-health`.

## Security boundaries

- **No network access by default.** The only networked command is `rpc-health`,
  which requires an explicit endpoint (argument, environment, or config).
- The HTTP transport uses a finite timeout, disables redirects, caps response
  bytes before parsing, and validates JSON-RPC responses.
- Scanned/decoded input is **untrusted**: decoding is bounded and fuzz targets
  exist for `ScVal`, `TransactionEnvelope`, and `ContractEvent`.
- No secrets are read, logged, or committed. `.gitignore` excludes `.env*`,
  keys, and PEM files.
- WASM inspection is read-only and never executes contract code.

## Dependency strategy

- Depend on official Stellar crates (`stellar-xdr`, `stellar-strkey`) rather
  than reimplementing.
- Keep the dependency surface small; every new dependency must be justified.
- Licenses must be compatible (Apache-2.0 for this project).

## Compatibility and versioning

- Semantic versioning; pre-1.0, so minor releases may break.
- Rule/CLI exit codes and machine output are treated as API and documented.
- The toolkit tracks current Stellar protocol crates (28.x line at the time of
  writing).
- The workspace version is `0.1.0`; `v0.1.0` is tagged and released. crates.io
  publication is still pending.

## Testing architecture

- Unit tests next to code for pure functions.
- Round-trip tests construct XDR values, encode them, decode them, and assert
  the result.
- Fixture-based regression tests cover malformed XDR.
- Fuzz targets live in the standalone `fuzz/` crate and run in a scheduled
  workflow.

## RPC response safety limits

The RPC layer rejects response values whose compact JSON representation would
exceed 4 MiB or whose nesting exceeds 64 levels. `HttpTransport` enforces the
byte cap on the raw response before parsing. Typed response models require their
non-optional fields; schema failures return `DevkitError::Decode` and are not
retried as transport failures. Unknown fields remain tolerated for forward
compatibility with Stellar RPC additions.
