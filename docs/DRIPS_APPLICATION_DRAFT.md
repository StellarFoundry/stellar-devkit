# Drips Application Draft

Factual draft answers. Nothing here guarantees approval, points, or payout, and
no prior program participation, funding, community size, or contributor activity
is claimed.

## Project description

StellarFoundry DevKit is a developer infrastructure toolkit for Stellar and
Soroban. It provides bounded XDR/SCVal/transaction/event decoding with
structured output, a typed RPC protocol layer with a deterministic mock
transport, endpoint security validation, and a CLI (`stellar-foundry`). It
composes with official crates rather than reimplementing them, works fully
offline, and never requires AI.

## Stellar ecosystem relevance

The toolkit depends on official Stellar crates (`stellar-xdr`,
`stellar-strkey`) and targets Stellar/Soroban workflows directly. It complements
`stellar-cli` and the official RPC client instead of duplicating them, and it
serves developers building on Stellar.

## Planned issues

35 issues across core, RPC, XDR/SCVal, transactions, events, contract
inspection, testing, security, CLI, configuration, GitHub/SARIF, observability,
and release. Complexity: 4 trivial, 19 medium, 12 high (5,650 pre-multiplier
points). See [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md) and
[DRIPS_ISSUE_AUDIT.md](DRIPS_ISSUE_AUDIT.md).

## Contributor opportunities

Small, well-scoped `good first issue` tasks (completions, exit-code tests,
Dependabot) through medium features (typed RPC models, config system) to high
work (live transport, fuzzing, SSRF hardening, SARIF, release automation).

## Supporting links

- Repository: <https://github.com/StellarFoundry/stellar-devkit>
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- Ecosystem research: [ECOSYSTEM_RESEARCH.md](ECOSYSTEM_RESEARCH.md)

## Current project status

Early. Working foundation plus RPC protocol layer with CI on Linux/Windows/macOS.
See [PROJECT_STATUS.md](PROJECT_STATUS.md). Live RPC and higher-level analysis are
not implemented.

## Maintainer experience

Maintained under the StellarFoundry organization by the account `Ayinkx01`.
StellarFoundry is not claimed to be independent of its maintainer; it is
maintained by `Ayinkx01`.

## Technical roadmap

Twelve phases from foundation through release; see [ROADMAP.md](ROADMAP.md).

## Honesty statement

- No approval, payout, or contributor activity is guaranteed.
- The issue set is smaller than 125; we do not inflate complexity to reach a
  target.
- Metrics such as stars, contributors, and funding are not claimed unless
  verified.
