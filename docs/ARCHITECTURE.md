# Architecture

## Goals

Provide a coherent developer infrastructure layer for Stellar and Soroban that
composes with official tooling instead of duplicating it. The core is
deterministic, works fully offline, and never requires AI.

## Non-goals

- Reimplementing XDR, strkey, or a JSON-RPC client.
- Replacing `stellar-cli` for build/deploy/invoke.
- Proving contracts secure.
- Network access by default.

## Crate map

```
crates/
  devkit-core   # errors, network profiles, output format (no I/O, no Stellar deps)
  devkit-xdr    # XDR / SCVal / transaction / event inspection (wraps stellar-xdr)
  devkit-cli    # the `stellar-foundry` binary (clap)
```

Dependency direction is strictly one way:

```
devkit-cli ──▶ devkit-xdr ──▶ devkit-core
                     │
                     └──▶ stellar-xdr, stellar-strkey
```

`devkit-core` has no Stellar dependencies so it can be reused cheaply and tested
in isolation. `devkit-xdr` is the only crate that touches XDR types.

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

- All decoding is bounded; `Limits::none()` is used with the understanding that
  inputs are size-limited by callers, and a stricter limit can be threaded
  through later (tracked as an issue).
- Failures are structured, never panics. Malformed input is a first-class test
  case.
- JSON is produced by `stellar-xdr`'s `serde` representation today. A curated,
  versioned schema is future work; until then the JSON shape is documented as
  **unstable**.

## CLI architecture

`stellar-foundry` uses `clap` derive. Global `--format` selects output. Exit
codes are stable: `0` success, `2` usage error, `3` runtime/decode error.
Commands only exist if they do real work; `doctor` reports capabilities and
explicitly states that network access is disabled.

## Configuration, logging, caching

- Configuration and logging are not implemented yet; they are planned and
  tracked as issues. `doctor` reports the current (empty) state honestly.
- No caching or persistence exists yet.

## Security boundaries

- **No network access** in the current implementation. Any future network
  feature must be opt-in, allowlisted, time-limited, size-limited, and fail
  closed.
- Scanned/decoded input is **untrusted**: decoding is bounded and fuzzing is
  planned.
- No secrets are read, logged, or committed. `.gitignore` excludes `.env*`,
  keys, and PEM files.

## Dependency strategy

- Depend on official Stellar crates (`stellar-xdr`, `stellar-strkey`, and later
  `stellar-rpc-client`) rather than reimplementing.
- Keep the dependency surface small; every new dependency must be justified.
- Licenses must be compatible (Apache-2.0 for this project).

## Compatibility and versioning

- Semantic versioning; pre-1.0, so minor releases may break.
- Rule/CLI exit codes and machine output are treated as API and documented.
- The toolkit tracks current Stellar protocol crates (28.x line at the time of
  writing).

## Testing architecture

- Unit tests next to code for pure functions.
- Round-trip tests construct XDR values, encode them, decode them, and assert
  the result — no external fixtures required for correctness.
- Fixture-based and fuzz testing are planned (issues) once the schema stabilizes.
