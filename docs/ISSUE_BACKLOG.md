# Issue Backlog

Generated from `tools/backlog/issues_*.json`. Closed and superseded
definitions are excluded.

**Total open issues:** 119

Complexity: 7 trivial, 84 medium, 28 high — 18900 pre-multiplier points.

## phase/01-foundation

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #36 | feat(core): define a categorized error taxonomy | core | medium | None. |
| #37 | feat(core): add shared Severity and Confidence models | core | medium | None. |
| #38 | feat(core): add a versioned output envelope for machine-readable commands | core | medium | None. |
| #39 | refactor(core): add validation primitives for untrusted strings | core | medium | None. |
| #40 | test(core): add deterministic snapshot tests for core types | core | trivial | Error taxonomy issue. |

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
| #41 | feat(rpc): implement JSON-RPC request construction | rpc | medium | Error taxonomy issue. |
| #42 | feat(rpc): distinguish Horizon and RPC data sources | rpc | medium | None. |
| #43 | feat(rpc): add cursor-based pagination for list methods | rpc | medium | Request construction issue. |
| #44 | feat(rpc): add a typed JSON-RPC error taxonomy | rpc | medium | Typed JSON-RPC request issue. |
| #45 | feat(rpc): handle rate limiting and Retry-After | rpc | medium | Typed JSON-RPC error taxonomy. |
| #46 | feat(rpc): add connection reuse and pool configuration | rpc | medium | Live transport issue. |
| #47 | test(rpc): add a local integration test server | rpc | high | Live transport issue. |

## phase/03-xdr-scval

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #11 | feat(scval): add curated human-readable rendering for common SCVal types | xdr | high | None. |
| #12 | feat(xdr): define a versioned JSON schema for decoded values | xdr | high | None. |
| #13 | feat(security): make XDR decode limits configurable and strict | xdr | medium | None. |
| #14 | feat(xdr): add strict encode helpers from JSON to XDR | xdr | high | Versioned schema issue. |
| #15 | feat(cli): add batch decoding from files and directories | cli | medium | None. |
| #16 | test(xdr): add golden fixtures for XDR decoding | xdr | medium | None. |
| #48 | feat(scval): fully decode and render maps and vectors | scval | medium | Curated SCVal rendering issue. |
| #49 | feat(scval): decode and render addresses and contract IDs | scval | medium | None. |
| #50 | feat(xdr): inspect TransactionResult and TransactionMeta | xdr | medium | None. |
| #52 | feat(xdr): pretty-print ledger entries | xdr | medium | getLedgerEntries model issue. |
| #53 | feat(scval): add type-safe conversions between SCVal and Rust primitives | scval | high | None. |
| #54 | feat(xdr): distinguish envelope v0 and v1 variants | xdr | medium | None. |
| #55 | feat(xdr): validate bytes, symbol, and string lengths | xdr | medium | None. |

## phase/04-analysis

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #26 | feat(transactions): add an envelope summary command | transactions | medium | None. |
| #27 | feat(events): add a typed getEvents model with pagination | rpc | high | Fixture loader issue. |
| #56 | feat(transactions): extract source account and sequence number | transactions | medium | Envelope summary issue. |
| #57 | feat(transactions): inspect operations with type-specific detail | transactions | high | None. |
| #58 | feat(transactions): analyze fees (base, resource, and total) | transactions | medium | Soroban transaction data issue. |
| #59 | feat(transactions): inspect signatures and signer hints | transactions | medium | None. |
| #60 | feat(transactions): handle memo and time/ledger bounds | transactions | medium | None. |
| #61 | feat(transactions): inspect Soroban transaction data and footprint | scval | high | Ledger entry pretty-printing issue. |
| #62 | feat(transactions): interpret transaction results and failure codes | transactions | high | Transaction result decoding issue. |
| #72 | feat(events): add event filtering by contract and topics | rpc | medium | Typed getEvents model issue. |
| #73 | feat(events): decode diagnostic events distinctly | events | medium | Diagnostic events XDR issue. |
| #74 | feat(events): normalize events into a stable structure | events | medium | Typed getEvents model issue. |
| #75 | feat(events): correlate events with their transaction | events | medium | Event normalization issue. |
| #76 | feat(ledger): add ledger sequence and close-time helpers | rpc | trivial | None. |
| #77 | test(events): add event fixture generation tooling | events | medium | Encode helpers issue. |

## phase/05-contracts

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #25 | feat(contracts): inspect WASM contract specs via soroban-spec | contract-inspection | high | None. |
| #63 | feat(contracts): handle and validate contract IDs | contract-inspection | medium | None. |
| #64 | feat(contracts): discover and validate WASM containers | contract-inspection | medium | None. |
| #65 | feat(contracts): cache extracted contract specs | contract-inspection | medium | WASM spec extraction issue. |
| #66 | feat(contracts): represent contract invocations | transactions | medium | SCVal rendering issue. |
| #67 | feat(contracts): inspect contract instance storage | contract-inspection | high | Ledger entry pretty-printing issue. |
| #68 | feat(contracts): inspect persistent and temporary storage entries | contract-inspection | high | Ledger entry pretty-printing issue. |
| #69 | feat(contracts): map contract events to spec types | events | high | WASM spec extraction issue and event normalization issue. |
| #70 | feat(contracts): report contract environment and protocol compatibility | contract-inspection | medium | WASM spec extraction issue. |
| #71 | feat(contracts): add the contract inspection CLI command | contract-inspection | medium | WASM spec extraction issue. |

## phase/06-testing

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #17 | feat(testing): add a fixture loader and schema | testing | high | None. |
| #18 | feat(testing): add mock RPC network scenarios | rpc | medium | Fixture loader issue. |
| #19 | test(xdr): add property-based round-trip tests | xdr | medium | Encode helpers issue. |
| #89 | test(xdr): add a malformed XDR corpus | xdr | medium | None. |
| #90 | test(rpc): add snapshot tests for RPC response models | rpc | medium | None. |
| #91 | test: add cross-platform and endianness tests | testing | medium | None. |
| #92 | test: add a regression corpus runner | testing | medium | Fixture loader issue. |
| #93 | test(contracts): add fixture WASM contracts | contract-inspection | high | WASM spec extraction issue. |
| #94 | test(ci): add a scheduled fuzz smoke job | security | medium | Fuzz targets. |

## phase/07-security

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #20 | test(security): add a fuzz target for XDR and SCVal decoding | security | high | None. |
| #21 | feat(security): harden the HTTP transport against SSRF and unsafe redirects | rpc | high | Live transport issue. |
| #22 | feat(security): redact secrets in logs and output | security | medium | None. |
| #78 | feat(security): design and implement the rule engine architecture | security | high | Severity/Confidence models. |
| #79 | feat(security): add a findings model with evidence | security | medium | Severity/Confidence models. |
| #80 | feat(security): add finding suppression with documented reasons | security | medium | Findings model issue. |
| #81 | feat(security): add a security fixture corpus | testing | high | Rule engine issue. |
| #82 | test(security): add false-positive regression tests | testing | medium | Rule engine issue. |
| #83 | test(security): add false-negative regression tests | testing | medium | Rule engine issue. |
| #84 | feat(security): validate contract metadata for consistency | contract-inspection | medium | WASM spec extraction issue. |
| #85 | feat(security): add WASM safety checks | contract-inspection | high | WASM container validation issue. |
| #86 | feat(security): harden against untrusted RPC responses | rpc | high | Live transport issue. |
| #88 | test(security): fuzz RPC response parsing | rpc | medium | Untrusted RPC response hardening issue. |

## phase/08-cli

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #7 | feat(core): add a configuration system with file and environment support | core | medium | None. |
| #10 | feat(core): add structured logging with levels | core | medium | None. |
| #30 | feat(cli): add shell completion generation | cli | trivial | None. |
| #32 | feat(cli): add config init, show, and validate subcommands | cli | medium | Configuration system issue. |
| #33 | test(cli): assert the exit-code contract | cli | trivial | None. |
| #96 | feat(cli): unify global flags across commands | cli | medium | None. |
| #97 | feat(cli): improve error presentation with suggestions | cli | medium | Secret redaction issue. |
| #98 | feat(cli): add verbosity control | cli | trivial | Structured logging issue. |
| #99 | feat(cli): add a JSON error output mode | cli | medium | Error taxonomy issue. |
| #100 | feat(cli): add stdin input for decode commands | cli | medium | None. |

## phase/09-github

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #23 | feat(sarif): emit SARIF for security findings | security | high | Security analysis issue. |
| #24 | feat(github): add a composite Action to run DevKit checks | github | high | None. |
| #35 | feat(github): add changed-file pull-request analysis workflow | github | medium | Action issue. |
| #87 | feat(security): add a SARIF rule catalog with stable identifiers | sarif | medium | Findings model issue. |
| #101 | feat(ci): add a reusable workflow for consumers | github | medium | Composite Action issue. |
| #102 | feat(github): add a SARIF upload workflow | github | medium | SARIF emission issue. |
| #103 | feat(github): add baseline scanning support | security | high | Findings model issue. |
| #104 | feat(ci): add a release validation workflow | release | medium | Release binaries issue. |
| #105 | chore(ci): add triage automation for issues and pull requests | github | medium | None. |
| #106 | feat(ci): add dependency and license auditing | security | medium | None. |

## phase/10-observability

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #28 | perf(observability): benchmark decode throughput | observability | medium | None. |
| #107 | perf(rpc): measure and document RPC call overhead | rpc | medium | Local integration test server issue. |
| #108 | perf(xdr): optimize SCVal decoding for large maps | scval | medium | SCVal collections issue. |
| #109 | perf: measure memory usage on large inputs | performance | medium | None. |
| #110 | feat(core): add an optional decode cache | core | medium | None. |
| #111 | perf: add performance regression tests to CI | performance | medium | Decode and RPC benchmarks. |

## phase/12-release

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #29 | ci(release): build and attach cross-platform release binaries | release | high | None. |
| #31 | ci(deps): configure automated dependency updates | release | trivial | None. |
| #34 | ci(msrv): pin and test a minimum supported Rust version | release | medium | None. |
| #112 | docs(release): define the versioning and compatibility policy | release | medium | None. |
| #113 | ci(release): automate changelog generation | release | medium | Versioning policy issue. |
| #114 | feat(release): prepare crates.io packaging | release | high | MSRV pin. |
| #115 | ci(release): generate an SBOM and artifact attestations | security | high | Release binaries issue. |
| #116 | ci(release): add compatibility testing across toolchains | release | medium | MSRV pin. |

## phase/13-docs

| # | Title | Area | Complexity | Dependencies |
| - | ----- | ---- | ---------- | ------------ |
| #117 | docs: add an RPC guide | rpc | medium | None. |
| #118 | docs: add an XDR and SCVal guide | xdr | medium | None. |
| #119 | docs: add a contract inspection guide | contract-inspection | medium | WASM spec extraction issue. |
| #120 | docs: add a security rule authoring guide | security | medium | Rule engine issue. |
| #121 | docs: add a configuration reference | config | medium | Configuration system issue. |

## Summary

| Phase | Count |
| ----- | ----- |
| phase/01-foundation | 5 |
| phase/02-rpc | 15 |
| phase/03-xdr-scval | 13 |
| phase/04-analysis | 15 |
| phase/05-contracts | 10 |
| phase/06-testing | 9 |
| phase/07-security | 13 |
| phase/08-cli | 10 |
| phase/09-github | 10 |
| phase/10-observability | 6 |
| phase/12-release | 8 |
| **Total** | **119** |
