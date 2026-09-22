# Drips Issue Quality Audit

- **Audit date:** 2026-09-22
- **Repository commit audited:** 07bef9e
- **Issues classified:** 121 (1 duplicate and 1 merge closed during remediation; 119 open).

Standard applied: Drips *Creating Meaningful Issues* and the Wave maintainer
documentation. Classification is evidence-based; weak issues are flagged, not
hidden. Nothing here claims Drips approval.

## Summary

| Classification | Count |
|---|---:|
| Drips-ready | 89 |
| Needs revision | 3 |
| Must split | 0 |
| Must merge | 1 |
| Remove | 1 |
| Defer | 3 |
| Blocked | 24 |

No issue required splitting: the largest issues are already scoped to a single
capability, and the previously broad areas had already been decomposed.

## Complexity audit

Complexity is assigned from the scope of each individual issue, not from an
aggregate reward target. Drips determines applicable points and budgets
through its own system.

| Complexity | Count |
|---|---:|
| High | 28 |
| Medium | 86 |
| Trivial | 7 |
| **Total** | **121** |

No complexity label was changed during this audit: no issue was found to be
clearly underpriced or overpriced. Complexity was not inflated.

## Issue-by-issue results

### #1 — feat(rpc): implement a live HTTP transport over the official client

- Classification: **Drips-ready**
- Complexity: high
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #2 — feat(rpc): add timeouts and cancellation to RPC calls

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #3 — feat(rpc): add exponential backoff with jitter to retries

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #4 — feat(rpc): add a typed model for getLedgerEntries

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #17
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #5 — feat(rpc): add typed models for getTransaction and getTransactions

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #17
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #6 — feat(rpc): add a typed model for getFeeStats

- Classification: **Drips-ready**
- Complexity: trivial
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #7 — feat(core): add a configuration system with file and environment support

- Classification: **Drips-ready**
- Complexity: medium
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #8 — feat(config): resolve network profiles and custom endpoints

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #7
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #9 — feat(cli): add live RPC diagnostics to doctor

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #1
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #10 — feat(core): add structured logging with levels

- Classification: **Drips-ready**
- Complexity: medium
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #11 — feat(scval): add curated human-readable rendering for common SCVal types

- Classification: **Drips-ready**
- Complexity: high
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #12 — feat(xdr): define a versioned JSON schema for decoded values

- Classification: **Drips-ready**
- Complexity: high
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #13 — feat(security): make XDR decode limits configurable and strict

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #14 — feat(xdr): add strict encode helpers from JSON to XDR

- Classification: **Drips-ready**
- Complexity: high
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #12
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #15 — feat(cli): add batch decoding from files and directories

- Classification: **Drips-ready**
- Complexity: medium
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #16 — test(xdr): add golden fixtures for XDR decoding

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #17 — feat(testing): add a fixture loader and schema

- Classification: **Drips-ready**
- Complexity: high
- Area: testing
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #18 — feat(testing): add mock RPC network scenarios

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #17
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #19 — test(xdr): add property-based round-trip tests

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #14
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #20 — test(security): add a fuzz target for XDR and SCVal decoding

- Classification: **Drips-ready**
- Complexity: high
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #21 — feat(security): harden the HTTP transport against SSRF and unsafe redirects

- Classification: **Blocked by dependency**
- Complexity: high
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #1
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #22 — feat(security): redact secrets in logs and output

- Classification: **Drips-ready**
- Complexity: medium
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #23 — feat(sarif): emit SARIF for security findings

- Classification: **Blocked by dependency**
- Complexity: high
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #78
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #24 — feat(github): add a composite Action to run DevKit checks

- Classification: **Drips-ready**
- Complexity: high
- Area: github
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #25 — feat(contracts): inspect WASM contract specs via soroban-spec

- Classification: **Drips-ready**
- Complexity: high
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #26 — feat(transactions): add an envelope summary command

- Classification: **Drips-ready**
- Complexity: medium
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #27 — feat(events): add a typed getEvents model with pagination

- Classification: **Drips-ready**
- Complexity: high
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #17
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #28 — perf(observability): benchmark decode throughput

- Classification: **Drips-ready**
- Complexity: medium
- Area: observability
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #29 — ci(release): build and attach cross-platform release binaries

- Classification: **Drips-ready**
- Complexity: high
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #30 — feat(cli): add shell completion generation

- Classification: **Drips-ready**
- Complexity: trivial
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #31 — ci(deps): configure automated dependency updates

- Classification: **Drips-ready**
- Complexity: trivial
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #32 — feat(cli): add config init, show, and validate subcommands

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #7
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #33 — test(cli): assert the exit-code contract

- Classification: **Drips-ready**
- Complexity: trivial
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #34 — ci(msrv): pin and test a minimum supported Rust version

- Classification: **Drips-ready**
- Complexity: medium
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #35 — feat(github): add changed-file pull-request analysis workflow

- Classification: **Defer / future work**
- Complexity: medium
- Area: github
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — defer / future work.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #24
- Duplication: PASS.
- Final recommendation: Defer / future work.

### #36 — feat(core): define a categorized error taxonomy

- Classification: **Drips-ready**
- Complexity: medium
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #37 — feat(core): add shared Severity and Confidence models

- Classification: **Drips-ready**
- Complexity: medium
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #38 — feat(core): add a versioned output envelope for machine-readable commands

- Classification: **Drips-ready**
- Complexity: medium
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #39 — refactor(core): add validation primitives for untrusted strings

- Classification: **Drips-ready**
- Complexity: medium
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #40 — test(core): add deterministic snapshot tests for core types

- Classification: **Drips-ready**
- Complexity: trivial
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #36
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #41 — feat(rpc): implement JSON-RPC request construction

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #36
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #42 — feat(rpc): distinguish Horizon and RPC data sources

- Classification: **Defer / future work**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — defer / future work.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Defer / future work.

### #43 — feat(rpc): add cursor-based pagination for list methods

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #41
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #44 — feat(rpc): add a typed JSON-RPC error taxonomy

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #41
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #45 — feat(rpc): handle rate limiting and Retry-After

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #44
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #46 — feat(rpc): add connection reuse and pool configuration

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #1
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #47 — test(rpc): add a local integration test server

- Classification: **Blocked by dependency**
- Complexity: high
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #1
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #48 — feat(scval): fully decode and render maps and vectors

- Classification: **Drips-ready**
- Complexity: medium
- Area: scval
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #11
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #49 — feat(scval): decode and render addresses and contract IDs

- Classification: **Drips-ready**
- Complexity: medium
- Area: scval
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #50 — feat(xdr): inspect TransactionResult and TransactionMeta

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #51 — feat(xdr): inspect diagnostic events

- Classification: **Remove (duplicate)**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — remove (duplicate).
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #50
- Duplication: FAIL — duplicate.
- Final recommendation: Remove (duplicate).

### #52 — feat(xdr): pretty-print ledger entries

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #4
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #53 — feat(scval): add type-safe conversions between SCVal and Rust primitives

- Classification: **Drips-ready**
- Complexity: high
- Area: scval
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #54 — feat(xdr): distinguish envelope v0 and v1 variants

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #55 — feat(xdr): validate bytes, symbol, and string lengths

- Classification: **Needs revision**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Needs revision.

### #56 — feat(transactions): extract source account and sequence number

- Classification: **Drips-ready**
- Complexity: medium
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #26
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #57 — feat(transactions): inspect operations with type-specific detail

- Classification: **Drips-ready**
- Complexity: high
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #58 — feat(transactions): analyze fees (base, resource, and total)

- Classification: **Drips-ready**
- Complexity: medium
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #61
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #59 — feat(transactions): inspect signatures and signer hints

- Classification: **Drips-ready**
- Complexity: medium
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #60 — feat(transactions): handle memo and time/ledger bounds

- Classification: **Drips-ready**
- Complexity: medium
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #61 — feat(transactions): inspect Soroban transaction data and footprint

- Classification: **Drips-ready**
- Complexity: high
- Area: scval
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #52
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #62 — feat(transactions): interpret transaction results and failure codes

- Classification: **Drips-ready**
- Complexity: high
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #50
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #63 — feat(contracts): handle and validate contract IDs

- Classification: **Drips-ready**
- Complexity: medium
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #64 — feat(contracts): discover and validate WASM containers

- Classification: **Drips-ready**
- Complexity: medium
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #65 — feat(contracts): cache extracted contract specs

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #25
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #66 — feat(contracts): represent contract invocations

- Classification: **Drips-ready**
- Complexity: medium
- Area: transactions
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #11
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #67 — feat(contracts): inspect contract instance storage

- Classification: **Blocked by dependency**
- Complexity: high
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #52
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #68 — feat(contracts): inspect persistent and temporary storage entries

- Classification: **Blocked by dependency**
- Complexity: high
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #52
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #69 — feat(contracts): map contract events to spec types

- Classification: **Blocked by dependency**
- Complexity: high
- Area: events
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #25
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #70 — feat(contracts): report contract environment and protocol compatibility

- Classification: **Defer / future work**
- Complexity: medium
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — defer / future work.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #25
- Duplication: PASS.
- Final recommendation: Defer / future work.

### #71 — feat(contracts): add the contract inspection CLI command

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #25
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #72 — feat(events): add event filtering by contract and topics

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #27
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #73 — feat(events): decode diagnostic events distinctly

- Classification: **Drips-ready**
- Complexity: medium
- Area: events
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #74 — feat(events): normalize events into a stable structure

- Classification: **Drips-ready**
- Complexity: medium
- Area: events
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #27
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #75 — feat(events): correlate events with their transaction

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: events
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #74
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #76 — feat(ledger): add ledger sequence and close-time helpers

- Classification: **Needs revision**
- Complexity: trivial
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Needs revision.

### #77 — test(events): add event fixture generation tooling

- Classification: **Drips-ready**
- Complexity: medium
- Area: events
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #14
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #78 — feat(security): design and implement the rule engine architecture

- Classification: **Drips-ready**
- Complexity: high
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #37
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #79 — feat(security): add a findings model with evidence

- Classification: **Drips-ready**
- Complexity: medium
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #37
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #80 — feat(security): add finding suppression with documented reasons

- Classification: **Drips-ready**
- Complexity: medium
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #79
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #81 — feat(security): add a security fixture corpus

- Classification: **Drips-ready**
- Complexity: high
- Area: testing
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #78
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #82 — test(security): add false-positive regression tests

- Classification: **Drips-ready**
- Complexity: medium
- Area: testing
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #78
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #83 — test(security): add false-negative regression tests

- Classification: **Drips-ready**
- Complexity: medium
- Area: testing
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #78
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #84 — feat(security): validate contract metadata for consistency

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #25
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #85 — feat(security): add WASM safety checks

- Classification: **Blocked by dependency**
- Complexity: high
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #64
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #86 — feat(security): harden against untrusted RPC responses

- Classification: **Drips-ready**
- Complexity: high
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #1
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #87 — feat(security): add a SARIF rule catalog with stable identifiers

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: sarif
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #79
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #88 — test(security): fuzz RPC response parsing

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #86
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #89 — test(xdr): add a malformed XDR corpus

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #90 — test(rpc): add snapshot tests for RPC response models

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #91 — test: add cross-platform and endianness tests

- Classification: **Drips-ready**
- Complexity: medium
- Area: testing
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #92 — test: add a regression corpus runner

- Classification: **Drips-ready**
- Complexity: medium
- Area: testing
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #17
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #93 — test(contracts): add fixture WASM contracts

- Classification: **Blocked by dependency**
- Complexity: high
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #25
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #94 — test(ci): add a scheduled fuzz smoke job

- Classification: **Drips-ready**
- Complexity: medium
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #20
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #95 — feat(cli): add configuration profiles

- Classification: **Merge**
- Complexity: medium
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — merge.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #7
- Duplication: PASS.
- Final recommendation: Merge.

### #96 — feat(cli): unify global flags across commands

- Classification: **Drips-ready**
- Complexity: medium
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #97 — feat(cli): improve error presentation with suggestions

- Classification: **Drips-ready**
- Complexity: medium
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #22
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #98 — feat(cli): add verbosity control

- Classification: **Drips-ready**
- Complexity: trivial
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #10
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #99 — feat(cli): add a JSON error output mode

- Classification: **Drips-ready**
- Complexity: medium
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #36
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #100 — feat(cli): add stdin input for decode commands

- Classification: **Drips-ready**
- Complexity: medium
- Area: cli
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #101 — feat(ci): add a reusable workflow for consumers

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: github
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #24
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #102 — feat(github): add a SARIF upload workflow

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: github
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #23
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #103 — feat(github): add baseline scanning support

- Classification: **Blocked by dependency**
- Complexity: high
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #79
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #104 — feat(ci): add a release validation workflow

- Classification: **Drips-ready**
- Complexity: medium
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #29
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #105 — chore(ci): add triage automation for issues and pull requests

- Classification: **Drips-ready**
- Complexity: medium
- Area: github
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #106 — feat(ci): add dependency and license auditing

- Classification: **Drips-ready**
- Complexity: medium
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #107 — perf(rpc): measure and document RPC call overhead

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #47
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #108 — perf(xdr): optimize SCVal decoding for large maps

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: scval
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #48
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #109 — perf: measure memory usage on large inputs

- Classification: **Drips-ready**
- Complexity: medium
- Area: performance
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #110 — feat(core): add an optional decode cache

- Classification: **Needs revision**
- Complexity: medium
- Area: core
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Needs revision.

### #111 — perf: add performance regression tests to CI

- Classification: **Drips-ready**
- Complexity: medium
- Area: ci
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #112 — docs(release): define the versioning and compatibility policy

- Classification: **Drips-ready**
- Complexity: medium
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #113 — ci(release): automate changelog generation

- Classification: **Drips-ready**
- Complexity: medium
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #112
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #114 — feat(release): prepare crates.io packaging

- Classification: **Drips-ready**
- Complexity: high
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #34
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #115 — ci(release): generate an SBOM and artifact attestations

- Classification: **Drips-ready**
- Complexity: high
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #29
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #116 — ci(release): add compatibility testing across toolchains

- Classification: **Drips-ready**
- Complexity: medium
- Area: release
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #34
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #117 — docs: add an RPC guide

- Classification: **Drips-ready**
- Complexity: medium
- Area: rpc
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #118 — docs: add an XDR and SCVal guide

- Classification: **Drips-ready**
- Complexity: medium
- Area: xdr
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: PASS — a single capability, Wave-sized.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: None.
- Duplication: PASS.
- Final recommendation: Drips-ready.

### #119 — docs: add a contract inspection guide

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: contract-inspection
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #25
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #120 — docs: add a security rule authoring guide

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: security
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #78
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

### #121 — docs: add a configuration reference

- Classification: **Blocked by dependency**
- Complexity: medium
- Area: config
- Impact: PASS — maps to a real capability of the toolkit.
- Context: PASS — problem, why, current state, and scope sections present.
- Scope: FAIL — blocked by dependency.
- Acceptance criteria: PASS — testable criteria present.
- Testing: PASS — tests requested.
- Security: N/A or addressed where relevant.
- Dependencies: #7
- Duplication: PASS.
- Final recommendation: Blocked by dependency.

## Drips-ready issue list

| Issue | Title | Complexity | Area | Why it is Wave-ready |
|---|---|---:|---|---|
| #1 | feat(rpc): implement a live HTTP transport over the official client | high | rpc | Depends only on implemented or independent work; testable. |
| #2 | feat(rpc): add timeouts and cancellation to RPC calls | medium | rpc | Depends only on implemented or independent work; testable. |
| #3 | feat(rpc): add exponential backoff with jitter to retries | medium | rpc | Depends only on implemented or independent work; testable. |
| #4 | feat(rpc): add a typed model for getLedgerEntries | medium | rpc | Depends only on implemented or independent work; testable. |
| #5 | feat(rpc): add typed models for getTransaction and getTransactions | medium | rpc | Depends only on implemented or independent work; testable. |
| #6 | feat(rpc): add a typed model for getFeeStats | trivial | rpc | Depends only on implemented or independent work; testable. |
| #7 | feat(core): add a configuration system with file and environment support | medium | core | Depends only on implemented or independent work; testable. |
| #8 | feat(config): resolve network profiles and custom endpoints | medium | rpc | Depends only on implemented or independent work; testable. |
| #10 | feat(core): add structured logging with levels | medium | core | Depends only on implemented or independent work; testable. |
| #11 | feat(scval): add curated human-readable rendering for common SCVal types | high | xdr | Depends only on implemented or independent work; testable. |
| #12 | feat(xdr): define a versioned JSON schema for decoded values | high | xdr | Depends only on implemented or independent work; testable. |
| #13 | feat(security): make XDR decode limits configurable and strict | medium | xdr | Depends only on implemented or independent work; testable. |
| #14 | feat(xdr): add strict encode helpers from JSON to XDR | high | xdr | Depends only on implemented or independent work; testable. |
| #15 | feat(cli): add batch decoding from files and directories | medium | cli | Depends only on implemented or independent work; testable. |
| #16 | test(xdr): add golden fixtures for XDR decoding | medium | xdr | Depends only on implemented or independent work; testable. |
| #17 | feat(testing): add a fixture loader and schema | high | testing | Depends only on implemented or independent work; testable. |
| #18 | feat(testing): add mock RPC network scenarios | medium | rpc | Depends only on implemented or independent work; testable. |
| #19 | test(xdr): add property-based round-trip tests | medium | xdr | Depends only on implemented or independent work; testable. |
| #20 | test(security): add a fuzz target for XDR and SCVal decoding | high | security | Depends only on implemented or independent work; testable. |
| #22 | feat(security): redact secrets in logs and output | medium | security | Depends only on implemented or independent work; testable. |
| #24 | feat(github): add a composite Action to run DevKit checks | high | github | Depends only on implemented or independent work; testable. |
| #25 | feat(contracts): inspect WASM contract specs via soroban-spec | high | contract-inspection | Depends only on implemented or independent work; testable. |
| #26 | feat(transactions): add an envelope summary command | medium | transactions | Depends only on implemented or independent work; testable. |
| #27 | feat(events): add a typed getEvents model with pagination | high | rpc | Depends only on implemented or independent work; testable. |
| #28 | perf(observability): benchmark decode throughput | medium | observability | Depends only on implemented or independent work; testable. |
| #29 | ci(release): build and attach cross-platform release binaries | high | release | Depends only on implemented or independent work; testable. |
| #30 | feat(cli): add shell completion generation | trivial | cli | Depends only on implemented or independent work; testable. |
| #31 | ci(deps): configure automated dependency updates | trivial | release | Depends only on implemented or independent work; testable. |
| #33 | test(cli): assert the exit-code contract | trivial | cli | Depends only on implemented or independent work; testable. |
| #34 | ci(msrv): pin and test a minimum supported Rust version | medium | release | Depends only on implemented or independent work; testable. |
| #36 | feat(core): define a categorized error taxonomy | medium | core | Depends only on implemented or independent work; testable. |
| #37 | feat(core): add shared Severity and Confidence models | medium | core | Depends only on implemented or independent work; testable. |
| #38 | feat(core): add a versioned output envelope for machine-readable commands | medium | core | Depends only on implemented or independent work; testable. |
| #39 | refactor(core): add validation primitives for untrusted strings | medium | core | Depends only on implemented or independent work; testable. |
| #40 | test(core): add deterministic snapshot tests for core types | trivial | core | Depends only on implemented or independent work; testable. |
| #41 | feat(rpc): implement JSON-RPC request construction | medium | rpc | Depends only on implemented or independent work; testable. |
| #43 | feat(rpc): add cursor-based pagination for list methods | medium | rpc | Depends only on implemented or independent work; testable. |
| #44 | feat(rpc): add a typed JSON-RPC error taxonomy | medium | rpc | Depends only on implemented or independent work; testable. |
| #45 | feat(rpc): handle rate limiting and Retry-After | medium | rpc | Depends only on implemented or independent work; testable. |
| #48 | feat(scval): fully decode and render maps and vectors | medium | scval | Depends only on implemented or independent work; testable. |
| #49 | feat(scval): decode and render addresses and contract IDs | medium | scval | Depends only on implemented or independent work; testable. |
| #50 | feat(xdr): inspect TransactionResult and TransactionMeta | medium | xdr | Depends only on implemented or independent work; testable. |
| #52 | feat(xdr): pretty-print ledger entries | medium | xdr | Depends only on implemented or independent work; testable. |
| #53 | feat(scval): add type-safe conversions between SCVal and Rust primitives | high | scval | Depends only on implemented or independent work; testable. |
| #54 | feat(xdr): distinguish envelope v0 and v1 variants | medium | xdr | Depends only on implemented or independent work; testable. |
| #56 | feat(transactions): extract source account and sequence number | medium | transactions | Depends only on implemented or independent work; testable. |
| #57 | feat(transactions): inspect operations with type-specific detail | high | transactions | Depends only on implemented or independent work; testable. |
| #58 | feat(transactions): analyze fees (base, resource, and total) | medium | transactions | Depends only on implemented or independent work; testable. |
| #59 | feat(transactions): inspect signatures and signer hints | medium | transactions | Depends only on implemented or independent work; testable. |
| #60 | feat(transactions): handle memo and time/ledger bounds | medium | transactions | Depends only on implemented or independent work; testable. |
| #61 | feat(transactions): inspect Soroban transaction data and footprint | high | scval | Depends only on implemented or independent work; testable. |
| #62 | feat(transactions): interpret transaction results and failure codes | high | transactions | Depends only on implemented or independent work; testable. |
| #63 | feat(contracts): handle and validate contract IDs | medium | contract-inspection | Depends only on implemented or independent work; testable. |
| #64 | feat(contracts): discover and validate WASM containers | medium | contract-inspection | Depends only on implemented or independent work; testable. |
| #66 | feat(contracts): represent contract invocations | medium | transactions | Depends only on implemented or independent work; testable. |
| #72 | feat(events): add event filtering by contract and topics | medium | rpc | Depends only on implemented or independent work; testable. |
| #73 | feat(events): decode diagnostic events distinctly | medium | events | Depends only on implemented or independent work; testable. |
| #74 | feat(events): normalize events into a stable structure | medium | events | Depends only on implemented or independent work; testable. |
| #77 | test(events): add event fixture generation tooling | medium | events | Depends only on implemented or independent work; testable. |
| #78 | feat(security): design and implement the rule engine architecture | high | security | Depends only on implemented or independent work; testable. |
| #79 | feat(security): add a findings model with evidence | medium | security | Depends only on implemented or independent work; testable. |
| #80 | feat(security): add finding suppression with documented reasons | medium | security | Depends only on implemented or independent work; testable. |
| #81 | feat(security): add a security fixture corpus | high | testing | Depends only on implemented or independent work; testable. |
| #82 | test(security): add false-positive regression tests | medium | testing | Depends only on implemented or independent work; testable. |
| #83 | test(security): add false-negative regression tests | medium | testing | Depends only on implemented or independent work; testable. |
| #86 | feat(security): harden against untrusted RPC responses | high | rpc | Depends only on implemented or independent work; testable. |
| #88 | test(security): fuzz RPC response parsing | medium | rpc | Depends only on implemented or independent work; testable. |
| #89 | test(xdr): add a malformed XDR corpus | medium | xdr | Depends only on implemented or independent work; testable. |
| #90 | test(rpc): add snapshot tests for RPC response models | medium | rpc | Depends only on implemented or independent work; testable. |
| #91 | test: add cross-platform and endianness tests | medium | testing | Depends only on implemented or independent work; testable. |
| #92 | test: add a regression corpus runner | medium | testing | Depends only on implemented or independent work; testable. |
| #94 | test(ci): add a scheduled fuzz smoke job | medium | security | Depends only on implemented or independent work; testable. |
| #96 | feat(cli): unify global flags across commands | medium | cli | Depends only on implemented or independent work; testable. |
| #97 | feat(cli): improve error presentation with suggestions | medium | cli | Depends only on implemented or independent work; testable. |
| #98 | feat(cli): add verbosity control | trivial | cli | Depends only on implemented or independent work; testable. |
| #99 | feat(cli): add a JSON error output mode | medium | cli | Depends only on implemented or independent work; testable. |
| #100 | feat(cli): add stdin input for decode commands | medium | cli | Depends only on implemented or independent work; testable. |
| #104 | feat(ci): add a release validation workflow | medium | release | Depends only on implemented or independent work; testable. |
| #105 | chore(ci): add triage automation for issues and pull requests | medium | github | Depends only on implemented or independent work; testable. |
| #106 | feat(ci): add dependency and license auditing | medium | security | Depends only on implemented or independent work; testable. |
| #109 | perf: measure memory usage on large inputs | medium | performance | Depends only on implemented or independent work; testable. |
| #111 | perf: add performance regression tests to CI | medium | ci | Depends only on implemented or independent work; testable. |
| #112 | docs(release): define the versioning and compatibility policy | medium | release | Depends only on implemented or independent work; testable. |
| #113 | ci(release): automate changelog generation | medium | release | Depends only on implemented or independent work; testable. |
| #114 | feat(release): prepare crates.io packaging | high | release | Depends only on implemented or independent work; testable. |
| #115 | ci(release): generate an SBOM and artifact attestations | high | security | Depends only on implemented or independent work; testable. |
| #116 | ci(release): add compatibility testing across toolchains | medium | release | Depends only on implemented or independent work; testable. |
| #117 | docs: add an RPC guide | medium | rpc | Depends only on implemented or independent work; testable. |
| #118 | docs: add an XDR and SCVal guide | medium | xdr | Depends only on implemented or independent work; testable. |

## Issues requiring work

| Issue | Problem | Required Action |
|---|---|---|
| #9 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #21 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #23 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #32 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #35 | Defer / future work | Label status/deferred; revisit when the subsystem is planned. |
| #42 | Defer / future work | Label status/deferred; revisit when the subsystem is planned. |
| #46 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #47 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #51 | Remove (duplicate) | Close as duplicate of #73; keep the canonical event issue. |
| #55 | Needs revision | Revise the issue body before it can be Wave-ready. |
| #65 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #67 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #68 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #69 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #70 | Defer / future work | Label status/deferred; revisit when the subsystem is planned. |
| #71 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #75 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #76 | Needs revision | Revise the issue body before it can be Wave-ready. |
| #84 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #85 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #87 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #93 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #95 | Merge | Close and fold into #8 (network profiles). |
| #101 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #102 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #103 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #107 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #108 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #110 | Needs revision | Revise the issue body before it can be Wave-ready. |
| #119 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #120 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
| #121 | Blocked by dependency | Label status/blocked; add to a Wave only after the dependency lands. |
