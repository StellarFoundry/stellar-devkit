# Release checklist

Before publishing a version:

1. Confirm the version and changelog entries agree.
2. Run `cargo fmt --all -- --check`.
3. Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
4. Run `cargo test --workspace --all-features` on Linux, Windows, and macOS.
5. Run `cargo package --workspace --no-verify` and review packaging warnings.
6. Confirm the lockfile is regenerated and committed after dependency changes.
7. Review the generated SARIF against the SARIF 2.1.0 schema.
8. Verify artifacts and checksums before publishing; do not include credentials or local configuration.
9. State known limitations in the release notes. Structural analysis findings are heuristics and do not prove a vulnerability.
