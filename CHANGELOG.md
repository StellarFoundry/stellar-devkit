# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `devkit-rpc`: detect HTTP 429 rate-limit responses, honor numeric
  `Retry-After` delays within a configurable cumulative cap (60 seconds by
  default), and return a clear `DevkitError::RateLimited` when the cap would be
  exceeded.

## [Unreleased]

### Added

- Cross-platform tests for XDR big-endian encoding, path separator handling,
  `..` normalization, and CRLF input across the CI operating systems. See
  `docs/TESTING.md`.

## [0.1.0] - 2026-10-28

First tagged release. Prebuilt binaries for Linux x86_64, macOS arm64, and
Windows x86_64 are attached to the GitHub Release with SHA256 checksums.
crates.io publication is still pending.

### Added

- `devkit-core`: structured errors, network profiles, and output formats.
- `devkit-xdr`: bounded base64 decoding for `ScVal`, `TransactionEnvelope`, and
  `ContractEvent`, plus strkey classification and malformed-input regression
  fixtures.
- `devkit-rpc`: endpoint validation, typed protocol models, deterministic mock
  transport, retry policy, and bounded HTTP JSON-RPC transport.
- `devkit-contract`: bounded, non-executing WASM validation and structural
  metadata for imports, exports, and custom sections, including bounded
  `contractspecv0` decoding.
- `devkit-analysis`: deterministic structural findings with stable rule IDs and
  SARIF 2.1.0 serialization.
- `devkit-config`: strict TOML configuration parsing, timeout validation,
  environment overrides, and static diagnostic events.
- CLI commands `endpoint`, `inspect-wasm`, `analyze-wasm`, and `rpc-health`, in
  addition to `version`, `doctor`, `strkey`, `scval`, `envelope`, and `event`.
- `rpc-health` consumes bounded TOML configuration, timeout settings, and
  environment overrides; endpoint precedence is documented in the CLI reference.
- Release-validation workflow, release checklist, Drips readiness checklist, and
  manual SARIF upload workflow for GitHub code scanning.
- RustSec dependency auditing, declared Rust 1.91 workspace MSRV, and a
  dedicated MSRV CI job.
- Malformed-XDR regression corpus, bounded-input hardening, and fuzz targets for
  `ScVal`, `TransactionEnvelope`, and `ContractEvent`.
- CI: formatting, Clippy, and Linux, Windows, and macOS tests.

### Changed

- Documentation now describes merged work as merged: project status, readiness,
  architecture, getting started, roadmap, issue audit, backlog, and Wave board
  were regenerated against `main`.
- `docs/WAVE_BOARD.md` now contains only open, independent, Wave-sized issues.
- Added `docs/WHY_DEVKIT.md` to document the boundary with official Stellar
  tooling and `soroban-security-scanner`.

### Fixed

- `devkit-xdr`: decoding now applies bounded reader limits instead of
  `Limits::none()`. A crafted XDR length prefix can no longer request an
  unbounded allocation (memory-exhaustion denial of service); such inputs are
  rejected as a structured decode error. Found by the `decode_scval` fuzz target.

### Limitations

- crates.io publication is still pending; install from the GitHub Release or
  build from source.
- `contractspecv0` XDR entries are decoded into official XDR-JSON definitions
  with a variant kind and best-effort name; a normalized ABI schema remains
  future work.
- Analysis findings are structural heuristics, not a comprehensive vulnerability
  detector.
- SARIF upload requires manual workflow dispatch, a repository-relative WASM
  artifact, and GitHub code-scanning permissions.
- The configuration API is currently wired into `rpc-health`; other commands do
  not consume RPC configuration because they do not make network requests.
