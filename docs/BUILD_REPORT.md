# Build Report

## Repository

<https://github.com/StellarFoundry/stellar-devkit> — default branch `main`.

## Architecture

Four crates with a one-way dependency direction:

```
devkit-cli ──▶ devkit-rpc ──▶ devkit-core
     └──────▶ devkit-xdr ──▶ devkit-core
```

See [ARCHITECTURE.md](ARCHITECTURE.md).

## Implemented components

- `devkit-core`: `DevkitError`, `Network` profiles with canonical passphrases,
  `OutputFormat`.
- `devkit-xdr`: bounded base64 decoding of `ScVal`, `TransactionEnvelope`, and
  `ContractEvent` to JSON; strkey classification.
- `devkit-rpc`: `Transport` trait, deterministic `MockTransport`, validated
  `Endpoint` (https only; loopback http; no embedded credentials), typed
  `Health`/`NetworkInfo`/`LatestLedger`, `getEvents`/`getFeeStats` passthrough,
  retry policy.
- `devkit-cli`: `stellar-foundry version|doctor|strkey|scval|envelope|event|endpoint`.

## Tests

22 unit tests (4 core, 5 xdr, 13 rpc). Run with `cargo test --all`. Results come
from actual local runs; CI runs the same suite on Linux, Windows, and macOS.

## CI

`CI` workflow: format, clippy (`-D warnings`), and tests on three operating
systems. Passing on all commits at the time of writing.

## Documentation

`README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`,
`CHANGELOG.md`, `docs/ECOSYSTEM_RESEARCH.md`, `docs/ARCHITECTURE.md`,
`docs/GETTING_STARTED.md`, `docs/CLI.md`, `docs/ROADMAP.md`,
`docs/PROJECT_STATUS.md`, `docs/ISSUE_BACKLOG.md`, `docs/DRIPS_*`, this report.

## Security

No network access by default; bounded decoding; structured errors; endpoint
policy rejects plaintext remote HTTP, embedded credentials, and unknown schemes;
no secrets read or logged. Fuzzing and SSRF hardening are tracked as issues.

## Release status

No releases yet. No binaries, no crates.io publication.

## Issue backlog

- **Total open issues:** 35
- **Trivial:** 4
- **Medium:** 19
- **High:** 12
- **Total pre-multiplier points:** 5,650

The requested 125-issue / 25,000-point target is **not supported** by the current
legitimate scope. There are 12 genuinely high-complexity issues; inflating the
rest to reach a number was explicitly avoided.

## Drips readiness

See [DRIPS_READINESS.md](DRIPS_READINESS.md) and
[DRIPS_APPLICATION_DRAFT.md](DRIPS_APPLICATION_DRAFT.md). Approval, points, and
payout are not guaranteed.

## Known limitations

- No live RPC transport; the RPC layer is mock-only.
- No contract inspection, security analysis, SARIF, or GitHub Action.
- JSON output shape is unstable until a versioned schema lands.
- No MSRV pin; no release automation.

## Recommended next steps

1. Implement the live HTTP transport over `stellar-rpc-client` and the SSRF
   hardening that must accompany it.
2. Add the configuration system and logging.
3. Define the versioned JSON schema.
4. Build the fixtures and testing infrastructure.
5. Grow the backlog honestly as real scope is implemented.
