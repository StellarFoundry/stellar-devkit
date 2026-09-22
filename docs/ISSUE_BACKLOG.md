# Issue Backlog

Generated from `tools/backlog/issues_*.json`. Closed and superseded
definitions are excluded.

**Total open issues:** 35

Complexity: 4 trivial, 19 medium, 12 high — 5650 pre-multiplier points.

## phase/02-rpc

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #1 | feat(rpc): implement a live HTTP transport over the official client | rpc | high | None. |
| #2 | feat(rpc): add timeouts and cancellation to RPC calls | rpc | medium | None. |
| #3 | feat(rpc): add exponential backoff with jitter to retries | rpc | medium | None. |
| #4 | feat(rpc): add a typed model for getLedgerEntries | rpc | medium | Fixture loader issue. |
| #5 | feat(rpc): add typed models for getTransaction and getTransactions | rpc | medium | Fixture loader issue. |
| #6 | feat(rpc): add a typed model for getFeeStats | rpc | trivial | None. |
| #8 | feat(config): resolve network profiles and custom endpoints | rpc | medium | Configuration system issue. |
| #9 | feat(cli): add live RPC diagnostics to doctor | rpc | medium | Live transport issue. |

## phase/03-xdr-scval

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #11 | feat(scval): add curated human-readable rendering for common SCVal types | xdr | high | None. |
| #12 | feat(xdr): define a versioned JSON schema for decoded values | xdr | high | None. |
| #13 | feat(security): make XDR decode limits configurable and strict | xdr | medium | None. |
| #14 | feat(xdr): add strict encode helpers from JSON to XDR | xdr | high | Versioned schema issue. |
| #15 | feat(cli): add batch decoding from files and directories | cli | medium | None. |
| #16 | test(xdr): add golden fixtures for XDR decoding | xdr | medium | None. |

## phase/04-analysis

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #26 | feat(transactions): add an envelope summary command | transactions | medium | None. |
| #27 | feat(events): add a typed getEvents model with pagination | rpc | high | Fixture loader issue. |

## phase/05-contracts

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #25 | feat(contracts): inspect WASM contract specs via soroban-spec | contract-inspection | high | None. |

## phase/06-testing

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #17 | feat(testing): add a fixture loader and schema | testing | high | None. |
| #18 | feat(testing): add mock RPC network scenarios | rpc | medium | Fixture loader issue. |
| #19 | test(xdr): add property-based round-trip tests | xdr | medium | Encode helpers issue. |

## phase/07-security

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #20 | test(security): add a fuzz target for XDR and SCVal decoding | security | high | None. |
| #21 | feat(security): harden the HTTP transport against SSRF and unsafe redirects | rpc | high | Live transport issue. |
| #22 | feat(security): redact secrets in logs and output | security | medium | None. |

## phase/08-cli

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #7 | feat(core): add a configuration system with file and environment support | core | medium | None. |
| #10 | feat(core): add structured logging with levels | core | medium | None. |
| #30 | feat(cli): add shell completion generation | cli | trivial | None. |
| #32 | feat(cli): add config init, show, and validate subcommands | cli | medium | Configuration system issue. |
| #33 | test(cli): assert the exit-code contract | cli | trivial | None. |

## phase/09-github

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #23 | feat(sarif): emit SARIF for security findings | security | high | Security analysis issue. |
| #24 | feat(github): add a composite Action to run DevKit checks | github | high | None. |
| #35 | feat(github): add changed-file pull-request analysis workflow | github | medium | Action issue. |

## phase/10-observability

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #28 | perf(observability): benchmark decode throughput | observability | medium | None. |

## phase/12-release

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #29 | ci(release): build and attach cross-platform release binaries | release | high | None. |
| #31 | ci(deps): configure automated dependency updates | release | trivial | None. |
| #34 | ci(msrv): pin and test a minimum supported Rust version | release | medium | None. |

## Summary

| Phase | Count |
| ----- | ----- |
| phase/02-rpc | 8 |
| phase/03-xdr-scval | 6 |
| phase/04-analysis | 2 |
| phase/05-contracts | 1 |
| phase/06-testing | 3 |
| phase/07-security | 3 |
| phase/08-cli | 5 |
| phase/09-github | 3 |
| phase/10-observability | 1 |
| phase/12-release | 3 |
| **Total** | **35** |
