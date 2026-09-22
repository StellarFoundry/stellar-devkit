# Drips Issue Audit

Generated from `tools/backlog/issues_*.json`. The GitHub issue numbers are
assigned on creation; titles are the stable identifier.

- **Total issues:** 35
- **Trivial:** 4
- **Medium:** 19
- **High:** 12
- **Total pre-multiplier points:** 5650

Complexity is assigned from the actual work required, not from a point
target. There are **not** 125 legitimate high-complexity issues in the
current scope, so a 25,000-point backlog is not supported. See
[BUILD_REPORT.md](BUILD_REPORT.md).

| # | Title | Area | Complexity | Points | Dependencies | Acceptance | Tests | Security |
| - | ----- | ---- | ---------- | ------ | ------------ | ---------- | ----- | -------- |
| 1 | ci(deps): configure automated dependency updates | release | trivial | 100 | None. | yes | yes | yes |
| 2 | feat(cli): add shell completion generation | cli | trivial | 100 | None. | yes | yes | yes |
| 3 | feat(rpc): add a typed model for getFeeStats | rpc | trivial | 100 | None. | yes | yes | yes |
| 4 | test(cli): assert the exit-code contract | cli | trivial | 100 | None. | yes | yes | yes |
| 5 | ci(msrv): pin and test a minimum supported Rust version | release | medium | 150 | None. | yes | yes | yes |
| 6 | feat(cli): add batch decoding from files and directories | cli | medium | 150 | None. | yes | yes | yes |
| 7 | feat(cli): add config init, show, and validate subcommands | cli | medium | 150 | Configuration system issue. | yes | yes | yes |
| 8 | feat(cli): add live RPC diagnostics to doctor | rpc | medium | 150 | Live transport issue. | yes | yes | yes |
| 9 | feat(config): resolve network profiles and custom endpoints | rpc | medium | 150 | Configuration system issue. | yes | yes | yes |
| 10 | feat(core): add a configuration system with file and environment support | core | medium | 150 | None. | yes | yes | yes |
| 11 | feat(core): add structured logging with levels | core | medium | 150 | None. | yes | yes | yes |
| 12 | feat(github): add changed-file pull-request analysis workflow | github | medium | 150 | Action issue. | yes | yes | yes |
| 13 | feat(rpc): add a typed model for getLedgerEntries | rpc | medium | 150 | Fixture loader issue. | yes | yes | yes |
| 14 | feat(rpc): add exponential backoff with jitter to retries | rpc | medium | 150 | None. | yes | yes | yes |
| 15 | feat(rpc): add timeouts and cancellation to RPC calls | rpc | medium | 150 | None. | yes | yes | yes |
| 16 | feat(rpc): add typed models for getTransaction and getTransactions | rpc | medium | 150 | Fixture loader issue. | yes | yes | yes |
| 17 | feat(security): make XDR decode limits configurable and strict | xdr | medium | 150 | None. | yes | yes | yes |
| 18 | feat(security): redact secrets in logs and output | security | medium | 150 | None. | yes | yes | yes |
| 19 | feat(testing): add mock RPC network scenarios | rpc | medium | 150 | Fixture loader issue. | yes | yes | yes |
| 20 | feat(transactions): add an envelope summary command | transactions | medium | 150 | None. | yes | yes | yes |
| 21 | perf(observability): benchmark decode throughput | observability | medium | 150 | None. | yes | yes | yes |
| 22 | test(xdr): add golden fixtures for XDR decoding | xdr | medium | 150 | None. | yes | yes | yes |
| 23 | test(xdr): add property-based round-trip tests | xdr | medium | 150 | Encode helpers issue. | yes | yes | yes |
| 24 | ci(release): build and attach cross-platform release binaries | release | high | 200 | None. | yes | yes | yes |
| 25 | feat(contracts): inspect WASM contract specs via soroban-spec | contract-inspection | high | 200 | None. | yes | yes | yes |
| 26 | feat(events): add a typed getEvents model with pagination | rpc | high | 200 | Fixture loader issue. | yes | yes | yes |
| 27 | feat(github): add a composite Action to run DevKit checks | github | high | 200 | None. | yes | yes | yes |
| 28 | feat(rpc): implement a live HTTP transport over the official client | rpc | high | 200 | None. | yes | yes | yes |
| 29 | feat(sarif): emit SARIF for security findings | security | high | 200 | Security analysis issue. | yes | yes | yes |
| 30 | feat(scval): add curated human-readable rendering for common SCVal types | xdr | high | 200 | None. | yes | yes | yes |
| 31 | feat(security): harden the HTTP transport against SSRF and unsafe redirects | rpc | high | 200 | Live transport issue. | yes | yes | yes |
| 32 | feat(testing): add a fixture loader and schema | testing | high | 200 | None. | yes | yes | yes |
| 33 | feat(xdr): add strict encode helpers from JSON to XDR | xdr | high | 200 | Versioned schema issue. | yes | yes | yes |
| 34 | feat(xdr): define a versioned JSON schema for decoded values | xdr | high | 200 | None. | yes | yes | yes |
| 35 | test(security): add a fuzz target for XDR and SCVal decoding | security | high | 200 | None. | yes | yes | yes |

## Notes

- Duplicate check: titles are unique across the backlog definitions.
- Every issue contains problem, why, context, scope, out-of-scope, guidance,
  acceptance criteria, required tests, security considerations, documentation
  requirements, dependencies, and a definition of done.
- Complexity is `difficulty/trivial|medium|high`; wave points are assigned
  by the maintainer in the Drips dashboard, not by these labels.
