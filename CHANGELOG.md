# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `devkit-contract`: bounded, non-executing WASM validation and structural
  metadata for imports, exports, and custom sections.
- `devkit-analysis`: deterministic structural findings with stable rule IDs
  and SARIF 2.1.0 serialization.
- `devkit-config`: strict TOML configuration parsing, timeout validation,
  environment overrides, and static diagnostic events.
- `devkit-rpc`: bounded HTTP JSON-RPC transport with timeout, redirects
  disabled, response-size limits, response validation, and structured errors.
- CLI commands `inspect-wasm`, `analyze-wasm`, and `rpc-health`.
- Release-validation workflow, release checklist, and Drips readiness checklist.
- RustSec dependency auditing and declared Rust 1.91 workspace MSRV in PR #127.
- Malformed-XDR regression corpus and bounded-input hardening.
- CI: formatting, Clippy, and Linux, Windows, and macOS tests.

### Fixed

- `devkit-xdr`: decoding now applies bounded reader limits instead of
  `Limits::none()`. A crafted XDR length prefix can no longer request an
  unbounded allocation (memory-exhaustion denial of service); such inputs are
  rejected as a structured decode error. Found by the `decode_scval` fuzz target.

### Limitations

- These changes are on open pull-request branches and are not all merged to
  `main`.
- WASM contract specification entries are detected and sized, but individual
  `contractspecv0` XDR entries are not yet decoded into typed metadata.
- Analysis findings are structural heuristics, not a comprehensive
  vulnerability detector.
- SARIF serialization is available; automated GitHub code-scanning upload is
  not configured.
- The configuration API is not yet wired into every CLI command.
