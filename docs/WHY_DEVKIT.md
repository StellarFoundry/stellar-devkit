# Why DevKit

This page explains what StellarFoundry DevKit is, what it is not, and how it
relates to existing Stellar tooling. It exists so maintainers, reviewers, and
contributors can judge scope without guessing.

## One sentence

DevKit is a **developer workflow and inspection layer** for Stellar and Soroban:
it turns XDR, transactions, events, contract artifacts, and RPC responses into
stable, structured output that developers can use in scripts, tests, and CI.

## What DevKit is

- A Rust workspace with reusable library APIs and a `stellar-foundry` CLI.
- A bounded, offline-by-default decoder for `ScVal`, `TransactionEnvelope`, and
  `ContractEvent`.
- A safe RPC layer: endpoint policy, typed protocol models, deterministic mocks,
  retries, timeouts, and a bounded HTTP transport.
- A read-only contract inspector: WASM container validation, imports/exports,
  custom sections, and `contractspecv0` metadata. No contract code is executed.
- A deterministic structural analysis and SARIF emitter for the checks it
  actually implements.

## What DevKit is not

- Not a replacement for `stellar-cli` (build, deploy, invoke).
- Not a reimplementation of `stellar-xdr`, `stellar-strkey`, or a JSON-RPC
  client.
- Not an indexer or a Horizon replacement.
- Not a source-level vulnerability scanner.
- Not a guarantee that a contract is safe.
- Not an AI product; AI is never required.

## Boundary with `soroban-security-scanner`

Both projects live in the StellarFoundry organization and both care about
security. They operate at different layers:

| Concern | DevKit | `soroban-security-scanner` |
| ------- | ------ | -------------------------- |
| Primary input | XDR, transactions, events, RPC responses, compiled WASM | Soroban/Rust **source code** and project layout |
| Primary job | Decode, inspect, validate, and explain developer data | Detect source-level vulnerability patterns |
| Analysis depth | Structural metadata and a small set of honest heuristics | Rule engine with a detector corpus, baselines, and suppressions |
| Output | Terminal/JSON, SARIF for implemented checks | Findings, SARIF, GitHub Action, reports |
| Typical user | Developer debugging a transaction, event, or contract artifact | Security reviewer or CI pipeline scanning a contract repo |
| Execution | Read-only; never runs contract code | Static analysis; does not execute the contract |

The overlap is deliberate but narrow: both can emit SARIF and both understand
contract artifacts. DevKit does **not** claim to replace the scanner, and the
scanner does **not** replace DevKit's decode/RPC workflow. Changes that belong
to source-level detection should go to `soroban-security-scanner`; changes that
belong to XDR/RPC/transaction inspection belong here.

## Boundary with official Stellar tooling

| Tool | Relationship |
| ---- | ------------ |
| `stellar-xdr` | DevKit depends on it for XDR types and base64 decoding. |
| `stellar-strkey` | DevKit depends on it for address classification. |
| `stellar-cli` | DevKit complements it. Use `stellar-cli` to build/deploy/invoke; use DevKit to inspect and script the resulting data. |
| Stellar RPC | DevKit validates endpoints, models responses, and provides bounded transport plus mocks. It does not replace the official client. |

## Workflows DevKit is designed for

1. **Debug a failing transaction** — decode the envelope, inspect operations and
   signatures, and compare the result with the RPC response.
2. **Triage a contract event** — decode `ContractEvent` values and correlate
   them with the ledger/transaction that produced them.
3. **Inspect a contract artifact before deployment** — list imports/exports,
   custom sections, and declared spec entries without executing the WASM.
4. **Test RPC-dependent tooling offline** — use the deterministic mock transport
   and fixtures instead of a live network.
5. **Emit machine-readable evidence in CI** — stable JSON and SARIF for the
   checks that are actually implemented.

## Honest limitations

- `contractspecv0` decoding is XDR-JSON plus best-effort names, not a normalized
  ABI.
- Structural analysis is heuristic, not a vulnerability proof.
- RPC configuration currently applies to `rpc-health`.
- `v0.1.0` is released with Linux, macOS, and Windows binaries; crates.io
  publication is still pending.

If a workflow needs something not listed here, treat it as planned work only
when an open issue says so.
