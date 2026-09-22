# Ecosystem Research

This document records the research done before implementation. It distinguishes
**verified** facts (checked against crates.io and official documentation on
2026-09-22) from **assumptions** that still need deeper investigation.

Sources consulted:

- Stellar RPC documentation — <https://developers.stellar.org/docs/data/apis/rpc>
- Stellar developer tools — <https://developers.stellar.org/docs/tools/developer-tools>
- Stellar smart contracts — <https://developers.stellar.org/docs/build/smart-contracts>
- Stellar GitHub organization — <https://github.com/stellar>
- crates.io (via `cargo search`) for crate names and versions.

## Existing tooling (verified)

| Crate / tool | Version seen | Purpose | Implication for DevKit |
| ------------ | ------------ | ------- | ---------------------- |
| `stellar-xdr` | 28.0.0 | Official XDR types, encoding/decoding (base64/binary), SCVal types | **Do not reimplement XDR.** Depend on it. |
| `stellar-strkey` | 0.0.18 | Encode/decode Stellar strkeys (accounts, contracts, muxed, preauth, hash-x, signed payloads) | Depend on it for address handling. |
| `stellar-rpc-client` | 28.0.0-rc.1 | Official Rust client for Stellar RPC | **Integrate, do not replace.** DevKit adds workflow/analysis on top. |
| `stellar-cli` | 28.0.0 | Official CLI (contract build/deploy/invoke, XDR utilities, network config) | DevKit must not duplicate; complement it. |
| `soroban-spec` | 28.0.0 | Contract spec (WASM custom section) utilities | Use for contract interface inspection. |
| `soroban-client` | 0.6.0 | Community high-level Soroban client | Adjacent; not a dependency unless proven necessary. |
| `stellar-horizon` | 0.8.0 | Horizon API client | Out of scope for the first phases (RPC-first). |

Verified facts about Stellar RPC (from the official RPC docs):

- Stellar RPC was renamed from Soroban-RPC in November 2024.
- RPC retains only a **bounded, recent window of history (~7 days by default)**.
  It is explicitly **not** an indexer and **not** a drop-in replacement for
  Horizon.
- It exposes methods including `getHealth`, `getNetwork`, `getLatestLedger`,
  `getLedgerEntries`, `getTransaction`, `getTransactions`, `getEvents`,
  `getFeeStats`, `simulateTransaction`, and `sendTransaction`.
- Typical uses are current state, recent transactions, events, fee statistics,
  and contract interaction simulation.

## Overlap analysis

For each DevKit area, we asked: does something exist, and what would be
meaningfully different?

### RPC

- **Exists:** `stellar-rpc-client` (official), plus many language SDKs.
- **DevKit difference:** a *developer workflow* layer — network profiles,
  diagnostics (`doctor`), deterministic request/response **fixtures and mocks**,
  caching, and consistent structured output across commands. DevKit will depend
  on the official client rather than reimplement JSON-RPC.
- **Worth building?** Yes, as a thin layer, not as a client.

### XDR / SCVal

- **Exists:** `stellar-xdr` (types + base64), and `stellar-cli xdr` utilities.
- **DevKit difference:** curated, **stable JSON** suitable for diffing and CI,
  batch decoding, and integration with fixtures/reports. Decoding itself is not
  reimplemented.
- **Worth building?** Yes, but must be conservative: if `stellar-cli` already
  covers a case, DevKit focuses on the workflow/analysis gap.

### Contract inspection / spec

- **Exists:** `soroban-spec`, `stellar-cli`.
- **DevKit difference:** read-only inspection that separates **confirmed facts**
  (from WASM metadata) from **inference**, with JSON output and explicit
  limitations. No claim of correctness.
- **Worth building?** Yes, later phase.

### Testing infrastructure (fixtures, mock RPC)

- **Exists:** `soroban-sdk` test utilities for contract unit tests; nothing
  standard for deterministic **RPC** fixtures at the toolkit level.
- **DevKit difference:** a fixture system and mock RPC transport so RPC-based
  tools can be tested deterministically and offline.
- **Worth building?** Yes — this is a primary differentiator.

### Security analysis

- **Exists:** the separate `AyinkxLab/soroban-security-scanner` project and
  general Rust linters. No widely adopted Stellar-specific toolkit here.
- **DevKit difference:** focused analysis of *toolkit inputs* — malformed XDR,
  unsafe RPC configuration (SSRF, untrusted endpoints), resource exhaustion —
  not claims about contract correctness.
- **Worth building?** Yes, scoped to input/config safety.

## What DevKit will NOT do

- Reimplement XDR encoding/decoding (use `stellar-xdr`).
- Reimplement an RPC client (use `stellar-rpc-client`).
- Replace `stellar-cli` for contract build/deploy/invoke.
- Claim to prove a contract secure.
- Require AI to function.
- Perform network access by default.

## Open questions (tracked as issues, not assumed)

- Exact stable JSON schema for SCVal/transaction/event inspection.
- Which RPC methods to wrap first, and their error semantics.
- Contract spec extraction limits across toolchain versions.
- Whether to expose a TypeScript interface (only if justified).

## Honesty notes

- The crate list above is limited to what `cargo search` returned and what the
  official docs state. It is not an exhaustive audit of every Stellar tool.
- No third-party tool was benchmarked or fully evaluated yet; performance and
  behavior claims about them are deliberately absent.
