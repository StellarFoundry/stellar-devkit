# Wave Board (first program)

A curated, single-cycle subset of the backlog for a Drips Wave, chosen from
issues marked `status/ready`. Every issue here is **open**, **unassigned**,
**independent**, **completable in one cycle**, and **not already implemented on
`main`**. It is deliberately a small subset of the full backlog, with a bounded
points budget.

Verified against `main` at commit `aec759c` on 2026-10-28.

Point values are assigned in the Drips Wave maintainer dashboard (Trivial 100 /
Medium 150 / High 200). The labels here are the honest pre-estimate.

## Budget

| Complexity | Issues | Points each | Subtotal |
| ---------- | -----: | ----------: | -------: |
| Trivial | 3 | 100 | 300 |
| Medium | 13 | 150 | 1,950 |
| High | 4 | 200 | 800 |
| **Total** | **20** | | **3,050** |

The repository budget is 25,000 points per Wave, so this board uses about 12%
of it and leaves room to add more later.

## Trivial (100 points)

| # | Title |
| - | ----- |
| [#6](https://github.com/StellarFoundry/stellar-devkit/issues/6) | feat(rpc): add a typed model for getFeeStats |
| [#30](https://github.com/StellarFoundry/stellar-devkit/issues/30) | feat(cli): add shell completion generation |
| [#31](https://github.com/StellarFoundry/stellar-devkit/issues/31) | ci(deps): configure automated dependency updates |

## Medium (150 points)

| # | Title |
| - | ----- |
| [#3](https://github.com/StellarFoundry/stellar-devkit/issues/3) | feat(rpc): add exponential backoff with jitter to retries |
| [#10](https://github.com/StellarFoundry/stellar-devkit/issues/10) | feat(core): add structured logging with levels |
| [#15](https://github.com/StellarFoundry/stellar-devkit/issues/15) | feat(cli): add batch decoding from files and directories |
| [#26](https://github.com/StellarFoundry/stellar-devkit/issues/26) | feat(transactions): add an envelope summary command |
| [#37](https://github.com/StellarFoundry/stellar-devkit/issues/37) | feat(core): add shared Severity and Confidence models |
| [#38](https://github.com/StellarFoundry/stellar-devkit/issues/38) | feat(core): add a versioned output envelope for machine-readable commands |
| [#39](https://github.com/StellarFoundry/stellar-devkit/issues/39) | refactor(core): add validation primitives for untrusted strings |
| [#59](https://github.com/StellarFoundry/stellar-devkit/issues/59) | feat(transactions): inspect signatures and signer hints |
| [#60](https://github.com/StellarFoundry/stellar-devkit/issues/60) | feat(transactions): handle memo and time/ledger bounds |
| [#105](https://github.com/StellarFoundry/stellar-devkit/issues/105) | chore(ci): add triage automation for issues and pull requests |
| [#112](https://github.com/StellarFoundry/stellar-devkit/issues/112) | docs(release): define the versioning and compatibility policy |
| [#117](https://github.com/StellarFoundry/stellar-devkit/issues/117) | docs: add an RPC guide |
| [#118](https://github.com/StellarFoundry/stellar-devkit/issues/118) | docs: add an XDR and SCVal guide |

## High (200 points)

| # | Title |
| - | ----- |
| [#12](https://github.com/StellarFoundry/stellar-devkit/issues/12) | feat(xdr): define a versioned JSON schema for decoded values |
| [#24](https://github.com/StellarFoundry/stellar-devkit/issues/24) | feat(github): add a composite Action to run DevKit checks |
| [#53](https://github.com/StellarFoundry/stellar-devkit/issues/53) | feat(scval): add type-safe conversions between SCVal and Rust primitives |
| [#57](https://github.com/StellarFoundry/stellar-devkit/issues/57) | feat(transactions): inspect operations with type-specific detail |

## Notes

- No issue in this set depends on another issue in this set; each can be merged
  independently.
- All issues are `status/ready`, open, and unassigned.
- Contributors must request assignment before starting; see [`WAVE.md`](WAVE.md).
- After repository approval, add these issues to the Program. Apply the
  `Stellar Wave` label or select them in the **Maintainers → Issues** dashboard.
- Keep the budget bounded; do not add every open issue to a single Wave.

## Excluded after the post-merge audit

These issues were on older board drafts but are implemented or partially
implemented on `main`. They are excluded until a maintainer closes or re-scopes
them:

| # | Status |
| - | ------ |
| #1 | Bounded HTTP transport exists; official `stellar-rpc-client` integration remains |
| #7 | Configuration exists and is wired into `rpc-health`; `docs/CONFIGURATION.md` now documents it; precedence tests remain |
| #20 | Fuzz crate, three targets, and a scheduled workflow exist |
| #25 | `contractspecv0` decoding exists; a fixture-WASM spec test and normalized ABI remain |
| #33 | CLI exit-code assertions exist in `crates/devkit-cli/tests/json_errors.rs` |

`#40` and `#98` are also excluded from this board because they depend on issues
not in this set (error taxonomy and structured logging respectively).
