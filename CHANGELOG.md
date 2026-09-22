# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Workspace with `devkit-core`, `devkit-xdr`, and `devkit-cli`.
- `devkit-core`: `DevkitError`, `Network` profiles with canonical passphrases,
  `OutputFormat`.
- `devkit-xdr`: decode base64 `ScVal`, `TransactionEnvelope`, and
  `ContractEvent` to JSON with bounded reading; classify Stellar strkeys.
- `stellar-foundry` CLI: `version`, `doctor`, `strkey`, `scval`, `envelope`,
  `event`, with terminal and JSON output and stable exit codes.
- Documentation: ecosystem research, architecture, getting started, CLI
  reference, project status, roadmap.
- CI: format, clippy, and tests on Linux, Windows, and macOS.
- Governance: README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT, LICENSE,
  pull-request and issue templates.
