# Wave Board (first program)

A curated, single-cycle subset of the backlog for the first Drips Wave, chosen
from issues marked `status/ready`. Every issue here is **independent**,
**completable in one cycle**, and **representative** of the project. It is
deliberately a small subset of the full backlog, with a bounded points budget.

Point values are assigned in the Drips Wave maintainer dashboard (Trivial 100 /
Medium 150 / High 200). The labels here are the honest pre-estimate.

## Budget

| Complexity | Issues | Points each | Subtotal |
| ---------- | -----: | ----------: | -------: |
| Trivial | 6 | 100 | 600 |
| Medium | 10 | 150 | 1,500 |
| High | 4 | 200 | 800 |
| **Total** | **20** | | **2,900** |

## Trivial (100 points)

| # | Title |
| - | ----- |
| [#6](https://github.com/StellarFoundry/stellar-devkit/issues/6) | feat(rpc): add a typed model for getFeeStats |
| [#30](https://github.com/StellarFoundry/stellar-devkit/issues/30) | feat(cli): add shell completion generation |
| [#31](https://github.com/StellarFoundry/stellar-devkit/issues/31) | ci(deps): configure automated dependency updates |
| [#33](https://github.com/StellarFoundry/stellar-devkit/issues/33) | test(cli): assert the exit-code contract |
| [#40](https://github.com/StellarFoundry/stellar-devkit/issues/40) | test(core): add deterministic snapshot tests for core types |
| [#98](https://github.com/StellarFoundry/stellar-devkit/issues/98) | feat(cli): add verbosity control |

## Medium (150 points)

| # | Title |
| - | ----- |
| [#2](https://github.com/StellarFoundry/stellar-devkit/issues/2) | feat(rpc): add timeouts and cancellation to RPC calls |
| [#4](https://github.com/StellarFoundry/stellar-devkit/issues/4) | feat(rpc): add a typed model for getLedgerEntries |
| [#5](https://github.com/StellarFoundry/stellar-devkit/issues/5) | feat(rpc): add typed models for getTransaction and getTransactions |
| [#7](https://github.com/StellarFoundry/stellar-devkit/issues/7) | feat(core): add a configuration system with file and environment support |
| [#10](https://github.com/StellarFoundry/stellar-devkit/issues/10) | feat(core): add structured logging with levels |
| [#13](https://github.com/StellarFoundry/stellar-devkit/issues/13) | feat(security): make XDR decode limits configurable and strict |
| [#26](https://github.com/StellarFoundry/stellar-devkit/issues/26) | feat(transactions): add an envelope summary command |
| [#36](https://github.com/StellarFoundry/stellar-devkit/issues/36) | feat(core): define a categorized error taxonomy |
| [#43](https://github.com/StellarFoundry/stellar-devkit/issues/43) | feat(rpc): add cursor-based pagination for list methods |
| [#48](https://github.com/StellarFoundry/stellar-devkit/issues/48) | feat(scval): fully decode and render maps and vectors |

## High (200 points)

| # | Title |
| - | ----- |
| [#1](https://github.com/StellarFoundry/stellar-devkit/issues/1) | feat(rpc): implement a live HTTP transport over the official client |
| [#12](https://github.com/StellarFoundry/stellar-devkit/issues/12) | feat(xdr): define a versioned JSON schema for decoded values |
| [#20](https://github.com/StellarFoundry/stellar-devkit/issues/20) | test(security): add a fuzz target for XDR and SCVal decoding |
| [#25](https://github.com/StellarFoundry/stellar-devkit/issues/25) | feat(contracts): inspect WASM contract specs via soroban-spec |

## Notes

- No issue in this set depends on another; each can be merged independently.
- All issues are `status/ready`. Issues marked `status/blocked` are intentionally
  excluded until their dependency lands.
- Contributors must request assignment before starting; see [`WAVE.md`](WAVE.md).
- After repository approval, add these issues to the Program. Apply the
  `Stellar Wave` label or select them in the **Maintainers → Issues** dashboard.
- Keep the budget bounded; do not add every open issue to a single Wave.
