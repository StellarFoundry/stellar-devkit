# Drips Issue Audit

Generated from `tools/backlog/issues_*.json`. The GitHub issue numbers are
assigned on creation; titles are the stable identifier.

- **Total issues:** 121
- **Trivial:** 7
- **Medium:** 86
- **High:** 28
- **Total pre-multiplier points:** 19200

Complexity is assigned from the actual work required, not from a point
target. There are **not** 125 legitimate high-complexity issues in the
current scope, so a 25,000-point backlog is not supported. See
[BUILD_REPORT.md](BUILD_REPORT.md).

| # | Title | Area | Complexity | Points | Dependencies | Acceptance | Tests | Security |
| - | ----- | ---- | ---------- | ------ | ------------ | ---------- | ----- | -------- |
| 1 | ci(deps): configure automated dependency updates | release | trivial | 100 | None. | yes | yes | yes |
| 2 | feat(cli): add shell completion generation | cli | trivial | 100 | None. | yes | yes | yes |
| 3 | feat(cli): add verbosity control | cli | trivial | 100 | Structured logging issue. | yes | yes | yes |
| 4 | feat(ledger): add ledger sequence and close-time helpers | rpc | trivial | 100 | None. | yes | yes | yes |
| 5 | feat(rpc): add a typed model for getFeeStats | rpc | trivial | 100 | None. | yes | yes | yes |
| 6 | test(cli): assert the exit-code contract | cli | trivial | 100 | None. | yes | yes | yes |
| 7 | test(core): add deterministic snapshot tests for core types | core | trivial | 100 | Error taxonomy issue. | yes | yes | yes |
| 8 | chore(ci): add triage automation for issues and pull requests | github | medium | 150 | None. | yes | yes | yes |
| 9 | ci(msrv): pin and test a minimum supported Rust version | release | medium | 150 | None. | yes | yes | yes |
| 10 | ci(release): add compatibility testing across toolchains | release | medium | 150 | MSRV pin. | yes | yes | yes |
| 11 | ci(release): automate changelog generation | release | medium | 150 | Versioning policy issue. | yes | yes | yes |
| 12 | docs(release): define the versioning and compatibility policy | release | medium | 150 | None. | yes | yes | yes |
| 13 | docs: add a configuration reference | config | medium | 150 | Configuration system issue. | yes | yes | yes |
| 14 | docs: add a contract inspection guide | contract-inspection | medium | 150 | WASM spec extraction issue. | yes | yes | yes |
| 15 | docs: add a security rule authoring guide | security | medium | 150 | Rule engine issue. | yes | yes | yes |
| 16 | docs: add an RPC guide | rpc | medium | 150 | None. | yes | yes | yes |
| 17 | docs: add an XDR and SCVal guide | xdr | medium | 150 | None. | yes | yes | yes |
| 18 | feat(ci): add a release validation workflow | release | medium | 150 | Release binaries issue. | yes | yes | yes |
| 19 | feat(ci): add a reusable workflow for consumers | github | medium | 150 | Composite Action issue. | yes | yes | yes |
| 20 | feat(ci): add dependency and license auditing | security | medium | 150 | None. | yes | yes | yes |
| 21 | feat(cli): add a JSON error output mode | cli | medium | 150 | Error taxonomy issue. | yes | yes | yes |
| 22 | feat(cli): add batch decoding from files and directories | cli | medium | 150 | None. | yes | yes | yes |
| 23 | feat(cli): add config init, show, and validate subcommands | cli | medium | 150 | Configuration system issue. | yes | yes | yes |
| 24 | feat(cli): add configuration profiles | cli | medium | 150 | Configuration system issue. | yes | yes | yes |
| 25 | feat(cli): add live RPC diagnostics to doctor | rpc | medium | 150 | Live transport issue. | yes | yes | yes |
| 26 | feat(cli): add stdin input for decode commands | cli | medium | 150 | None. | yes | yes | yes |
| 27 | feat(cli): improve error presentation with suggestions | cli | medium | 150 | Secret redaction issue. | yes | yes | yes |
| 28 | feat(cli): unify global flags across commands | cli | medium | 150 | None. | yes | yes | yes |
| 29 | feat(config): resolve network profiles and custom endpoints | rpc | medium | 150 | Configuration system issue. | yes | yes | yes |
| 30 | feat(contracts): add the contract inspection CLI command | contract-inspection | medium | 150 | WASM spec extraction issue. | yes | yes | yes |
| 31 | feat(contracts): cache extracted contract specs | contract-inspection | medium | 150 | WASM spec extraction issue. | yes | yes | yes |
| 32 | feat(contracts): discover and validate WASM containers | contract-inspection | medium | 150 | None. | yes | yes | yes |
| 33 | feat(contracts): handle and validate contract IDs | contract-inspection | medium | 150 | None. | yes | yes | yes |
| 34 | feat(contracts): report contract environment and protocol compatibility | contract-inspection | medium | 150 | WASM spec extraction issue. | yes | yes | yes |
| 35 | feat(contracts): represent contract invocations | transactions | medium | 150 | SCVal rendering issue. | yes | yes | yes |
| 36 | feat(core): add a configuration system with file and environment support | core | medium | 150 | None. | yes | yes | yes |
| 37 | feat(core): add a versioned output envelope for machine-readable commands | core | medium | 150 | None. | yes | yes | yes |
| 38 | feat(core): add an optional decode cache | core | medium | 150 | None. | yes | yes | yes |
| 39 | feat(core): add shared Severity and Confidence models | core | medium | 150 | None. | yes | yes | yes |
| 40 | feat(core): add structured logging with levels | core | medium | 150 | None. | yes | yes | yes |
| 41 | feat(core): define a categorized error taxonomy | core | medium | 150 | None. | yes | yes | yes |
| 42 | feat(events): add event filtering by contract and topics | rpc | medium | 150 | Typed getEvents model issue. | yes | yes | yes |
| 43 | feat(events): correlate events with their transaction | events | medium | 150 | Event normalization issue. | yes | yes | yes |
| 44 | feat(events): decode diagnostic events distinctly | events | medium | 150 | Diagnostic events XDR issue. | yes | yes | yes |
| 45 | feat(events): normalize events into a stable structure | events | medium | 150 | Typed getEvents model issue. | yes | yes | yes |
| 46 | feat(github): add a SARIF upload workflow | github | medium | 150 | SARIF emission issue. | yes | yes | yes |
| 47 | feat(github): add changed-file pull-request analysis workflow | github | medium | 150 | Action issue. | yes | yes | yes |
| 48 | feat(rpc): add a typed JSON-RPC error taxonomy | rpc | medium | 150 | Typed JSON-RPC request issue. | yes | yes | yes |
| 49 | feat(rpc): add a typed model for getLedgerEntries | rpc | medium | 150 | Fixture loader issue. | yes | yes | yes |
| 50 | feat(rpc): add connection reuse and pool configuration | rpc | medium | 150 | Live transport issue. | yes | yes | yes |
| 51 | feat(rpc): add cursor-based pagination for list methods | rpc | medium | 150 | Request construction issue. | yes | yes | yes |
| 52 | feat(rpc): add exponential backoff with jitter to retries | rpc | medium | 150 | None. | yes | yes | yes |
| 53 | feat(rpc): add timeouts and cancellation to RPC calls | rpc | medium | 150 | None. | yes | yes | yes |
| 54 | feat(rpc): add typed models for getTransaction and getTransactions | rpc | medium | 150 | Fixture loader issue. | yes | yes | yes |
| 55 | feat(rpc): distinguish Horizon and RPC data sources | rpc | medium | 150 | None. | yes | yes | yes |
| 56 | feat(rpc): handle rate limiting and Retry-After | rpc | medium | 150 | Typed JSON-RPC error taxonomy. | yes | yes | yes |
| 57 | feat(rpc): implement JSON-RPC request construction | rpc | medium | 150 | Error taxonomy issue. | yes | yes | yes |
| 58 | feat(scval): decode and render addresses and contract IDs | scval | medium | 150 | None. | yes | yes | yes |
| 59 | feat(scval): fully decode and render maps and vectors | scval | medium | 150 | Curated SCVal rendering issue. | yes | yes | yes |
| 60 | feat(security): add a SARIF rule catalog with stable identifiers | sarif | medium | 150 | Findings model issue. | yes | yes | yes |
| 61 | feat(security): add a findings model with evidence | security | medium | 150 | Severity/Confidence models. | yes | yes | yes |
| 62 | feat(security): add finding suppression with documented reasons | security | medium | 150 | Findings model issue. | yes | yes | yes |
| 63 | feat(security): make XDR decode limits configurable and strict | xdr | medium | 150 | None. | yes | yes | yes |
| 64 | feat(security): redact secrets in logs and output | security | medium | 150 | None. | yes | yes | yes |
| 65 | feat(security): validate contract metadata for consistency | contract-inspection | medium | 150 | WASM spec extraction issue. | yes | yes | yes |
| 66 | feat(testing): add mock RPC network scenarios | rpc | medium | 150 | Fixture loader issue. | yes | yes | yes |
| 67 | feat(transactions): add an envelope summary command | transactions | medium | 150 | None. | yes | yes | yes |
| 68 | feat(transactions): analyze fees (base, resource, and total) | transactions | medium | 150 | Soroban transaction data issue. | yes | yes | yes |
| 69 | feat(transactions): extract source account and sequence number | transactions | medium | 150 | Envelope summary issue. | yes | yes | yes |
| 70 | feat(transactions): handle memo and time/ledger bounds | transactions | medium | 150 | None. | yes | yes | yes |
| 71 | feat(transactions): inspect signatures and signer hints | transactions | medium | 150 | None. | yes | yes | yes |
| 72 | feat(xdr): distinguish envelope v0 and v1 variants | xdr | medium | 150 | None. | yes | yes | yes |
| 73 | feat(xdr): inspect TransactionResult and TransactionMeta | xdr | medium | 150 | None. | yes | yes | yes |
| 74 | feat(xdr): inspect diagnostic events | xdr | medium | 150 | Transaction meta issue. | yes | yes | yes |
| 75 | feat(xdr): pretty-print ledger entries | xdr | medium | 150 | getLedgerEntries model issue. | yes | yes | yes |
| 76 | feat(xdr): validate bytes, symbol, and string lengths | xdr | medium | 150 | None. | yes | yes | yes |
| 77 | perf(observability): benchmark decode throughput | observability | medium | 150 | None. | yes | yes | yes |
| 78 | perf(rpc): measure and document RPC call overhead | rpc | medium | 150 | Local integration test server issue. | yes | yes | yes |
| 79 | perf(xdr): optimize SCVal decoding for large maps | scval | medium | 150 | SCVal collections issue. | yes | yes | yes |
| 80 | perf: add performance regression tests to CI | performance | medium | 150 | Decode and RPC benchmarks. | yes | yes | yes |
| 81 | perf: measure memory usage on large inputs | performance | medium | 150 | None. | yes | yes | yes |
| 82 | refactor(core): add validation primitives for untrusted strings | core | medium | 150 | None. | yes | yes | yes |
| 83 | test(ci): add a scheduled fuzz smoke job | security | medium | 150 | Fuzz targets. | yes | yes | yes |
| 84 | test(events): add event fixture generation tooling | events | medium | 150 | Encode helpers issue. | yes | yes | yes |
| 85 | test(rpc): add snapshot tests for RPC response models | rpc | medium | 150 | None. | yes | yes | yes |
| 86 | test(security): add false-negative regression tests | testing | medium | 150 | Rule engine issue. | yes | yes | yes |
| 87 | test(security): add false-positive regression tests | testing | medium | 150 | Rule engine issue. | yes | yes | yes |
| 88 | test(security): fuzz RPC response parsing | rpc | medium | 150 | Untrusted RPC response hardening issue. | yes | yes | yes |
| 89 | test(xdr): add a malformed XDR corpus | xdr | medium | 150 | None. | yes | yes | yes |
| 90 | test(xdr): add golden fixtures for XDR decoding | xdr | medium | 150 | None. | yes | yes | yes |
| 91 | test(xdr): add property-based round-trip tests | xdr | medium | 150 | Encode helpers issue. | yes | yes | yes |
| 92 | test: add a regression corpus runner | testing | medium | 150 | Fixture loader issue. | yes | yes | yes |
| 93 | test: add cross-platform and endianness tests | testing | medium | 150 | None. | yes | yes | yes |
| 94 | ci(release): build and attach cross-platform release binaries | release | high | 200 | None. | yes | yes | yes |
| 95 | ci(release): generate an SBOM and artifact attestations | security | high | 200 | Release binaries issue. | yes | yes | yes |
| 96 | feat(contracts): inspect WASM contract specs via soroban-spec | contract-inspection | high | 200 | None. | yes | yes | yes |
| 97 | feat(contracts): inspect contract instance storage | contract-inspection | high | 200 | Ledger entry pretty-printing issue. | yes | yes | yes |
| 98 | feat(contracts): inspect persistent and temporary storage entries | contract-inspection | high | 200 | Ledger entry pretty-printing issue. | yes | yes | yes |
| 99 | feat(contracts): map contract events to spec types | events | high | 200 | WASM spec extraction issue and event normalization issue. | yes | yes | yes |
| 100 | feat(events): add a typed getEvents model with pagination | rpc | high | 200 | Fixture loader issue. | yes | yes | yes |
| 101 | feat(github): add a composite Action to run DevKit checks | github | high | 200 | None. | yes | yes | yes |
| 102 | feat(github): add baseline scanning support | security | high | 200 | Findings model issue. | yes | yes | yes |
| 103 | feat(release): prepare crates.io packaging | release | high | 200 | MSRV pin. | yes | yes | yes |
| 104 | feat(rpc): implement a live HTTP transport over the official client | rpc | high | 200 | None. | yes | yes | yes |
| 105 | feat(sarif): emit SARIF for security findings | security | high | 200 | Security analysis issue. | yes | yes | yes |
| 106 | feat(scval): add curated human-readable rendering for common SCVal types | xdr | high | 200 | None. | yes | yes | yes |
| 107 | feat(scval): add type-safe conversions between SCVal and Rust primitives | scval | high | 200 | None. | yes | yes | yes |
| 108 | feat(security): add WASM safety checks | contract-inspection | high | 200 | WASM container validation issue. | yes | yes | yes |
| 109 | feat(security): add a security fixture corpus | testing | high | 200 | Rule engine issue. | yes | yes | yes |
| 110 | feat(security): design and implement the rule engine architecture | security | high | 200 | Severity/Confidence models. | yes | yes | yes |
| 111 | feat(security): harden against untrusted RPC responses | rpc | high | 200 | Live transport issue. | yes | yes | yes |
| 112 | feat(security): harden the HTTP transport against SSRF and unsafe redirects | rpc | high | 200 | Live transport issue. | yes | yes | yes |
| 113 | feat(testing): add a fixture loader and schema | testing | high | 200 | None. | yes | yes | yes |
| 114 | feat(transactions): inspect Soroban transaction data and footprint | scval | high | 200 | Ledger entry pretty-printing issue. | yes | yes | yes |
| 115 | feat(transactions): inspect operations with type-specific detail | transactions | high | 200 | None. | yes | yes | yes |
| 116 | feat(transactions): interpret transaction results and failure codes | transactions | high | 200 | Transaction result decoding issue. | yes | yes | yes |
| 117 | feat(xdr): add strict encode helpers from JSON to XDR | xdr | high | 200 | Versioned schema issue. | yes | yes | yes |
| 118 | feat(xdr): define a versioned JSON schema for decoded values | xdr | high | 200 | None. | yes | yes | yes |
| 119 | test(contracts): add fixture WASM contracts | contract-inspection | high | 200 | WASM spec extraction issue. | yes | yes | yes |
| 120 | test(rpc): add a local integration test server | rpc | high | 200 | Live transport issue. | yes | yes | yes |
| 121 | test(security): add a fuzz target for XDR and SCVal decoding | security | high | 200 | None. | yes | yes | yes |

## Notes

- Duplicate check: titles are unique across the backlog definitions.
- Every issue contains problem, why, context, scope, out-of-scope, guidance,
  acceptance criteria, required tests, security considerations, documentation
  requirements, dependencies, and a definition of done.
- Complexity is `difficulty/trivial|medium|high`; wave points are assigned
  by the maintainer in the Drips dashboard, not by these labels.
