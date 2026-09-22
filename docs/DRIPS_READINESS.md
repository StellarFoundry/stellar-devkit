# Drips Readiness

This document prepares the repository for a possible Drips Wave application. It
contains **no guarantee** of approval, payout, contributor activity, or points.

## Project summary

StellarFoundry DevKit is a developer infrastructure toolkit for Stellar and
Soroban. It composes with official crates (`stellar-xdr`, `stellar-strkey`, and
later `stellar-rpc-client`) instead of reimplementing them, and provides a
workflow layer: decoding and inspection with structured output, a typed RPC
protocol layer with deterministic mocks, fixtures, analysis, and a CLI.

## Problem solved

Stellar developers repeatedly need to decode XDR/SCVal, inspect transactions and
events, talk to RPC, and test those flows deterministically. That glue code is
rebuilt per project. DevKit centralizes it with stable, testable interfaces and
no AI dependency.

## Why it belongs in the Stellar ecosystem

- It targets Stellar and Soroban specifically and depends on official crates.
- It does not duplicate `stellar-cli` or the official RPC client; it complements
  them with workflow, testing, and analysis.
- It is useful to the same developers the Stellar ecosystem funds.

## Current implementation status

- **Working:** workspace with `devkit-core`, `devkit-xdr`, `devkit-rpc`,
  `devkit-cli`; bounded base64 decoding of `ScVal`/`TransactionEnvelope`/
  `ContractEvent` to JSON; strkey classification; endpoint security validation;
  typed RPC protocol layer with a deterministic mock transport; CLI with stable
  exit codes.
- **Not implemented:** live RPC transport, contract inspection, security
  analysis, SARIF, GitHub Action, configuration, logging, benchmarks.
- See [PROJECT_STATUS.md](PROJECT_STATUS.md).

## Contributor workflow

- Issues are labeled by `area/*`, `type/*`, `difficulty/*`, and `phase/*`.
- `good first issue` marks small, well-scoped tasks.
- Each issue states problem, why, context, scope, out-of-scope, guidance,
  acceptance criteria, required tests, security considerations, documentation
  requirements, dependencies, and a definition of done.
- Contributors request assignment before starting; PRs use `Closes #<id>` and
  run the local gate (fmt, clippy, tests).

## Issue categories

Core, RPC, XDR, SCVal, transactions, events, contract inspection, testing,
security, CLI, configuration, GitHub, SARIF, observability, documentation,
release. See [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md).

## Issue quality standards

- No fabricated or duplicate issues; titles are unique and each is traceable to
  real functionality or maintenance work.
- Complexity is honest (`trivial`/`medium`/`high`); points are assigned in the
  Drips dashboard.

## Testing standards

- Unit tests for pure functions; malformed-input tests for every decoder;
  fixture-based tests; property tests and fuzzing planned.
- The local gate is `cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test --all`.

## Documentation standards

Every public feature is documented; `CHANGELOG.md` and `PROJECT_STATUS.md` are
kept factual.

## Maintainer responsiveness plan

- Initial response to new issues and PRs within 48 hours during an active Wave.
- Review decision within 72 hours of a reviewable PR.
- Assign contributors promptly; unresolved issues roll over.

## Security process

See [SECURITY.md](../SECURITY.md). No network access by default; untrusted input
is bounded and decoded with structured errors; every security claim is backed by
implementation and tests.

## Release process

See [ROADMAP.md](ROADMAP.md) and the release issue. Semantic versioning,
changelog, and (planned) cross-platform binaries.

## Links

- Repository: <https://github.com/StellarFoundry/stellar-devkit>
- Backlog: [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- Ecosystem research: [ECOSYSTEM_RESEARCH.md](ECOSYSTEM_RESEARCH.md)

## Quality audit outcome

Every open issue was audited against the Drips maintainer criteria. Results
(see [DRIPS_ISSUE_QUALITY_AUDIT.md](DRIPS_ISSUE_QUALITY_AUDIT.md)):

- **89 Drips-ready** issues (`status/ready`).
- **24 blocked by dependency** (`status/blocked`) — not to be added to a Wave
  until their dependency lands.
- **3 deferred / future work** (`status/deferred`).
- **3 needs revision** (`status/needs-revision`, clarified on the issue).
- **1 duplicate removed**, **1 merged** (issue count reduced from 121 to 119).

Complexity labels were not altered. Drips determines applicable points and
budgets through its own system.

## Known limitations

- The backlog contains 119 open issues. Most are **not implemented**; the
  project is early (foundation and RPC protocol layer only). See
  [BUILD_REPORT.md](BUILD_REPORT.md).
- No live RPC, no release binaries, no published crates.
- Traction is minimal and is reported honestly.

## Future roadmap

See [ROADMAP.md](ROADMAP.md).
