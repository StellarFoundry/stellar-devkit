# Drips Issue Audit

Generated from `tools/backlog/issues_*.json`. The GitHub issue numbers are
assigned on creation; titles are the stable identifier.

- **Total issues:** 119
- **Trivial:** 7
- **Medium:** 84
- **High:** 28

Complexity is assigned from the scope of each individual issue, not from an
aggregate reward target. Drips determines applicable points and budgets
through its own system.

| # | Title | Area | Complexity | Dependencies | Acceptance | Tests | Security |
| - | ----- | ---- | ---------- | ------------ | ---------- | ----- | -------- |
| 1 | ci(deps): configure automated dependency updates | release | trivial | None. | yes | yes | yes |
| 2 | feat(cli): add shell completion generation | cli | trivial | None. | yes | yes | yes |
| 3 | feat(cli): add verbosity control | cli | trivial | Structured logging issue. | yes | yes | yes |
| 4 | feat(ledger): add ledger sequence and close-time helpers | rpc | trivial | None. | yes | yes | yes |
| 5 | feat(rpc): add a typed model for getFeeStats | rpc | trivial | None. | yes | yes | yes |
| 6 | test(cli): assert the exit-code contract | cli | trivial | None. | yes | yes | yes |
| 7 | test(core): add deterministic snapshot tests for core types | core | trivial | Error taxonomy issue. | yes | yes | yes |
| 8 | chore(ci): add triage automation for issues and pull requests | github | medium | None. | yes | yes | yes |
| 9 | ci(msrv): pin and test a minimum supported Rust version | release | medium | None. | yes | yes | yes |
| 10 | ci(release): add compatibility testing across toolchains | release | medium | MSRV pin. | yes | yes | yes |
| 11 | ci(release): automate changelog generation | release | medium | Versioning policy issue. | yes | yes | yes |
| 12 | docs(release): define the versioning and compatibility policy | release | medium | None. | yes | yes | yes |
| 13 | docs: add a configuration reference | config | medium | Configuration system issue. | yes | yes | yes |
| 14 | docs: add a contract inspection guide | contract-inspection | medium | WASM spec extraction issue. | yes | yes | yes |
| 15 | docs: add a security rule authoring guide | security | medium | Rule engine issue. | yes | yes | yes |
| 16 | docs: add an RPC guide | rpc | medium | None. | yes | yes | yes |
| 17 | docs: add an XDR and SCVal guide | xdr | medium | None. | yes | yes | yes |
| 18 | feat(ci): add a release validation workflow | release | medium | Release binaries issue. | yes | yes | yes |
| 19 | feat(ci): add a reusable workflow for consumers | github | medium | Composite Action issue. | yes | yes | yes |
| 20 | feat(ci): add dependency and license auditing | security | medium | None. | yes | yes | yes |
| 21 | feat(cli): add a JSON error output mode | cli | medium | Error taxonomy issue. | yes | yes | yes |
| 22 | feat(cli): add batch decoding from files and directories | cli | medium | None. | yes | yes | yes |
| 23 | feat(cli): add config init, show, and validate subcommands | cli | medium | Configuration system issue. | yes | yes | yes |
| 24 | feat(cli): add live RPC diagnostics to doctor | rpc | medium | Live transport issue. | yes | yes | yes |
| 25 | feat(cli): add stdin input for decode commands | cli | medium | None. | yes | yes | yes |
| 26 | feat(cli): improve error presentation with suggestions | cli | medium | Secret redaction issue. | yes | yes | yes |
| 27 | feat(cli): unify global flags across commands | cli | medium | None. | yes | yes | yes |
| 28 | feat(config): resolve network profiles and custom endpoints | rpc | medium | Configuration system issue. | yes | yes | yes |
| 29 | feat(contracts): add the contract inspection CLI command | contract-inspection | medium | WASM spec extraction issue. | yes | yes | yes |
| 30 | feat(contracts): cache extracted contract specs | contract-inspection | medium | WASM spec extraction issue. | yes | yes | yes |
| 31 | feat(contracts): discover and validate WASM containers | contract-inspection | medium | None. | yes | yes | yes |
| 32 | feat(contracts): handle and validate contract IDs | contract-inspection | medium | None. | yes | yes | yes |
| 33 | feat(contracts): report contract environment and protocol compatibility | contract-inspection | medium | WASM spec extraction issue. | yes | yes | yes |
| 34 | feat(contracts): represent contract invocations | transactions | medium | SCVal rendering issue. | yes | yes | yes |
| 35 | feat(core): add a configuration system with file and environment support | core | medium | None. | yes | yes | yes |
| 36 | feat(core): add a versioned output envelope for machine-readable commands | core | medium | None. | yes | yes | yes |
| 37 | feat(core): add an optional decode cache | core | medium | None. | yes | yes | yes |
| 38 | feat(core): add shared Severity and Confidence models | core | medium | None. | yes | yes | yes |
| 39 | feat(core): add structured logging with levels | core | medium | None. | yes | yes | yes |
| 40 | feat(core): define a categorized error taxonomy | core | medium | None. | yes | yes | yes |
| 41 | feat(events): add event filtering by contract and topics | rpc | medium | Typed getEvents model issue. | yes | yes | yes |
| 42 | feat(events): correlate events with their transaction | events | medium | Event normalization issue. | yes | yes | yes |
| 43 | feat(events): decode diagnostic events distinctly | events | medium | Diagnostic events XDR issue. | yes | yes | yes |
| 44 | feat(events): normalize events into a stable structure | events | medium | Typed getEvents model issue. | yes | yes | yes |
| 45 | feat(github): add a SARIF upload workflow | github | medium | SARIF emission issue. | yes | yes | yes |
| 46 | feat(github): add changed-file pull-request analysis workflow | github | medium | Action issue. | yes | yes | yes |
| 47 | feat(rpc): add a typed JSON-RPC error taxonomy | rpc | medium | Typed JSON-RPC request issue. | yes | yes | yes |
| 48 | feat(rpc): add a typed model for getLedgerEntries | rpc | medium | Fixture loader issue. | yes | yes | yes |
| 49 | feat(rpc): add connection reuse and pool configuration | rpc | medium | Live transport issue. | yes | yes | yes |
| 50 | feat(rpc): add cursor-based pagination for list methods | rpc | medium | Request construction issue. | yes | yes | yes |
| 51 | feat(rpc): add exponential backoff with jitter to retries | rpc | medium | None. | yes | yes | yes |
| 52 | feat(rpc): add timeouts and cancellation to RPC calls | rpc | medium | None. | yes | yes | yes |
| 53 | feat(rpc): add typed models for getTransaction and getTransactions | rpc | medium | Fixture loader issue. | yes | yes | yes |
| 54 | feat(rpc): distinguish Horizon and RPC data sources | rpc | medium | None. | yes | yes | yes |
| 55 | feat(rpc): handle rate limiting and Retry-After | rpc | medium | Typed JSON-RPC error taxonomy. | yes | yes | yes |
| 56 | feat(rpc): implement JSON-RPC request construction | rpc | medium | Error taxonomy issue. | yes | yes | yes |
| 57 | feat(scval): decode and render addresses and contract IDs | scval | medium | None. | yes | yes | yes |
| 58 | feat(scval): fully decode and render maps and vectors | scval | medium | Curated SCVal rendering issue. | yes | yes | yes |
| 59 | feat(security): add a SARIF rule catalog with stable identifiers | sarif | medium | Findings model issue. | yes | yes | yes |
| 60 | feat(security): add a findings model with evidence | security | medium | Severity/Confidence models. | yes | yes | yes |
| 61 | feat(security): add finding suppression with documented reasons | security | medium | Findings model issue. | yes | yes | yes |
| 62 | feat(security): make XDR decode limits configurable and strict | xdr | medium | None. | yes | yes | yes |
| 63 | feat(security): redact secrets in logs and output | security | medium | None. | yes | yes | yes |
| 64 | feat(security): validate contract metadata for consistency | contract-inspection | medium | WASM spec extraction issue. | yes | yes | yes |
| 65 | feat(testing): add mock RPC network scenarios | rpc | medium | Fixture loader issue. | yes | yes | yes |
| 66 | feat(transactions): add an envelope summary command | transactions | medium | None. | yes | yes | yes |
| 67 | feat(transactions): analyze fees (base, resource, and total) | transactions | medium | Soroban transaction data issue. | yes | yes | yes |
| 68 | feat(transactions): extract source account and sequence number | transactions | medium | Envelope summary issue. | yes | yes | yes |
| 69 | feat(transactions): handle memo and time/ledger bounds | transactions | medium | None. | yes | yes | yes |
| 70 | feat(transactions): inspect signatures and signer hints | transactions | medium | None. | yes | yes | yes |
| 71 | feat(xdr): distinguish envelope v0 and v1 variants | xdr | medium | None. | yes | yes | yes |
| 72 | feat(xdr): inspect TransactionResult and TransactionMeta | xdr | medium | None. | yes | yes | yes |
| 73 | feat(xdr): pretty-print ledger entries | xdr | medium | getLedgerEntries model issue. | yes | yes | yes |
| 74 | feat(xdr): validate bytes, symbol, and string lengths | xdr | medium | None. | yes | yes | yes |
| 75 | perf(observability): benchmark decode throughput | observability | medium | None. | yes | yes | yes |
| 76 | perf(rpc): measure and document RPC call overhead | rpc | medium | Local integration test server issue. | yes | yes | yes |
| 77 | perf(xdr): optimize SCVal decoding for large maps | scval | medium | SCVal collections issue. | yes | yes | yes |
| 78 | perf: add performance regression tests to CI | performance | medium | Decode and RPC benchmarks. | yes | yes | yes |
| 79 | perf: measure memory usage on large inputs | performance | medium | None. | yes | yes | yes |
| 80 | refactor(core): add validation primitives for untrusted strings | core | medium | None. | yes | yes | yes |
| 81 | test(ci): add a scheduled fuzz smoke job | security | medium | Fuzz targets. | yes | yes | yes |
| 82 | test(events): add event fixture generation tooling | events | medium | Encode helpers issue. | yes | yes | yes |
| 83 | test(rpc): add snapshot tests for RPC response models | rpc | medium | None. | yes | yes | yes |
| 84 | test(security): add false-negative regression tests | testing | medium | Rule engine issue. | yes | yes | yes |
| 85 | test(security): add false-positive regression tests | testing | medium | Rule engine issue. | yes | yes | yes |
| 86 | test(security): fuzz RPC response parsing | rpc | medium | Untrusted RPC response hardening issue. | yes | yes | yes |
| 87 | test(xdr): add a malformed XDR corpus | xdr | medium | None. | yes | yes | yes |
| 88 | test(xdr): add golden fixtures for XDR decoding | xdr | medium | None. | yes | yes | yes |
| 89 | test(xdr): add property-based round-trip tests | xdr | medium | Encode helpers issue. | yes | yes | yes |
| 90 | test: add a regression corpus runner | testing | medium | Fixture loader issue. | yes | yes | yes |
| 91 | test: add cross-platform and endianness tests | testing | medium | None. | yes | yes | yes |
| 92 | ci(release): build and attach cross-platform release binaries | release | high | None. | yes | yes | yes |
| 93 | ci(release): generate an SBOM and artifact attestations | security | high | Release binaries issue. | yes | yes | yes |
| 94 | feat(contracts): inspect WASM contract specs via soroban-spec | contract-inspection | high | None. | yes | yes | yes |
| 95 | feat(contracts): inspect contract instance storage | contract-inspection | high | Ledger entry pretty-printing issue. | yes | yes | yes |
| 96 | feat(contracts): inspect persistent and temporary storage entries | contract-inspection | high | Ledger entry pretty-printing issue. | yes | yes | yes |
| 97 | feat(contracts): map contract events to spec types | events | high | WASM spec extraction issue and event normalization issue. | yes | yes | yes |
| 98 | feat(events): add a typed getEvents model with pagination | rpc | high | Fixture loader issue. | yes | yes | yes |
| 99 | feat(github): add a composite Action to run DevKit checks | github | high | None. | yes | yes | yes |
| 100 | feat(github): add baseline scanning support | security | high | Findings model issue. | yes | yes | yes |
| 101 | feat(release): prepare crates.io packaging | release | high | MSRV pin. | yes | yes | yes |
| 102 | feat(rpc): implement a live HTTP transport over the official client | rpc | high | None. | yes | yes | yes |
| 103 | feat(sarif): emit SARIF for security findings | security | high | Security analysis issue. | yes | yes | yes |
| 104 | feat(scval): add curated human-readable rendering for common SCVal types | xdr | high | None. | yes | yes | yes |
| 105 | feat(scval): add type-safe conversions between SCVal and Rust primitives | scval | high | None. | yes | yes | yes |
| 106 | feat(security): add WASM safety checks | contract-inspection | high | WASM container validation issue. | yes | yes | yes |
| 107 | feat(security): add a security fixture corpus | testing | high | Rule engine issue. | yes | yes | yes |
| 108 | feat(security): design and implement the rule engine architecture | security | high | Severity/Confidence models. | yes | yes | yes |
| 109 | feat(security): harden against untrusted RPC responses | rpc | high | Live transport issue. | yes | yes | yes |
| 110 | feat(security): harden the HTTP transport against SSRF and unsafe redirects | rpc | high | Live transport issue. | yes | yes | yes |
| 111 | feat(testing): add a fixture loader and schema | testing | high | None. | yes | yes | yes |
| 112 | feat(transactions): inspect Soroban transaction data and footprint | scval | high | Ledger entry pretty-printing issue. | yes | yes | yes |
| 113 | feat(transactions): inspect operations with type-specific detail | transactions | high | None. | yes | yes | yes |
| 114 | feat(transactions): interpret transaction results and failure codes | transactions | high | Transaction result decoding issue. | yes | yes | yes |
| 115 | feat(xdr): add strict encode helpers from JSON to XDR | xdr | high | Versioned schema issue. | yes | yes | yes |
| 116 | feat(xdr): define a versioned JSON schema for decoded values | xdr | high | None. | yes | yes | yes |
| 117 | test(contracts): add fixture WASM contracts | contract-inspection | high | WASM spec extraction issue. | yes | yes | yes |
| 118 | test(rpc): add a local integration test server | rpc | high | Live transport issue. | yes | yes | yes |
| 119 | test(security): add a fuzz target for XDR and SCVal decoding | security | high | None. | yes | yes | yes |

## Notes

- Duplicate check: titles are unique across the backlog definitions.
- Every issue contains problem, why, context, scope, out-of-scope, guidance,
  acceptance criteria, required tests, security considerations, documentation
  requirements, dependencies, and a definition of done.
- Complexity is `difficulty/trivial|medium|high`; wave points are assigned
  by the maintainer in the Drips dashboard, not by these labels.
