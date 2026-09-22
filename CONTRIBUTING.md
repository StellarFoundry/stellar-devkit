# Contributing to StellarFoundry DevKit

Thanks for helping build developer infrastructure for Stellar and Soroban.

## Ground rules

- Be respectful; see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
- Never commit secrets, keys, or credentials.
- Do not claim a feature works unless it is implemented and tested.
- Keep the core usable without AI and without network access by default.

## Contributing through a Wave

If you are contributing through an open-source Wave:

- **Request assignment on an issue before starting.** Do not open a pull request
  for an issue you have not been assigned.
- Each issue has a `difficulty/*` label describing its scope. Drips determines
  applicable points and budgets through its own system.
- Your pull request must include `Closes #<issue-number>`.
- Run the local gate before opening the PR: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`.

See [`docs/WAVE.md`](docs/WAVE.md) for the full process.

## Setup

```bash
git clone https://github.com/StellarFoundry/stellar-devkit
cd stellar-devkit
cargo build
cargo test
```

A recent stable Rust toolchain is required.

## Before opening a pull request

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## Architecture

Read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) first. The crate dependency
direction is one-way: `devkit-cli → devkit-xdr → devkit-core`. Do not add a
Stellar dependency to `devkit-core`.

## Adding functionality

- Prefer composing with official crates (`stellar-xdr`, `stellar-strkey`, and
  later `stellar-rpc-client`) over reimplementing.
- Every fallible function returns `devkit_core::DevkitError`.
- Add unit tests; decoding changes must include malformed-input tests.
- Update documentation and `CHANGELOG.md` for user-visible changes.

## Commits

Use Conventional Commits: `feat`, `fix`, `refactor`, `test`, `docs`, `ci`,
`security`, `perf`, `build`.

## Finding work

Issues are labeled by `area/*`, `type/*`, `difficulty/*`, and `phase/*`. Start
with `difficulty/trivial` or `good first issue`. Search existing issues before
filing a new one.

## Definition of done

- Code compiles with clippy `-D warnings`.
- `cargo fmt` is clean.
- Tests added and passing.
- Documentation updated.
- No secrets, placeholders, or fake implementations.

## License

Contributions are licensed under Apache-2.0.
