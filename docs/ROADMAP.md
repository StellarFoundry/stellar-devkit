# Roadmap

Phases are ordered so that each builds on a working foundation. Only completed
items are claimed; everything else is tracked as issues.

## Phase 1 — Foundation (in progress)

- Workspace, core types, network profiles.
- XDR/SCVal/envelope/event decoding to JSON.
- Strkey classification.
- CLI with stable exit codes.
- CI, docs, governance.

## Phase 2 — RPC infrastructure

- Integrate `stellar-rpc-client` behind a typed interface.
- Network profiles, endpoint configuration, timeouts, retries.
- Diagnostics (`doctor` against a live endpoint), rate-limit handling.
- Mock transport for deterministic tests.

## Phase 3 — XDR / SCVal schema

- Curated, versioned JSON schema (stable across releases).
- Batch decoding and diff-friendly output.
- Compatibility fixtures.

## Phase 4 — Transaction and event analysis

- Operations, resources, fees, result codes, events, affected state.
- Honest separation of confirmed facts vs. inference.

## Phase 5 — Contract intelligence

- WASM metadata and spec inspection (via `soroban-spec`).
- Confirmed facts vs. inference, with explicit limitations.

## Phase 6 — Testing infrastructure

- Fixture system and mock network.
- Regression, property, and fuzz testing.

## Phase 7 — Security

- Malformed-input robustness, unsafe RPC configuration, SSRF-safe endpoint
  policy, resource limits.

## Phase 8 — CLI and developer experience

- Configuration files, environment variables, logging.

## Phase 9 — GitHub / CI / SARIF

- GitHub Action, changed-file analysis, SARIF reporting.

## Phase 10 — Observability and performance

- Request tracing, benchmarks, measured performance.

## Phase 11 — Developer experience

- Examples, snippets, guides.

## Phase 12 — Release and production hardening

- Release automation, binaries, crates.io, compatibility policy.
