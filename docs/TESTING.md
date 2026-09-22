# Testing guide

## The local gate

Run the same checks as CI before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

Unit and integration tests live next to the code (`#[cfg(test)]` modules) and in
`tests/`. Decoding changes must include malformed-input tests.

## Fuzzing

Fuzz targets live in the standalone `fuzz/` crate (it is not part of the main
workspace). Targets exercise the untrusted-input decoders:

| Target | Entry point |
| ------ | ----------- |
| `decode_scval` | `devkit_xdr::inspect_scval_base64` |
| `decode_envelope` | `devkit_xdr::inspect_envelope_base64` |
| `decode_event` | `devkit_xdr::inspect_event_base64` |

Fuzzing requires a nightly toolchain and `cargo-fuzz`:

```bash
cargo install cargo-fuzz
cd fuzz
cargo +nightly fuzz run decode_scval -- -max_total_time=60
```

Targets must not touch the filesystem and must not panic on arbitrary input:
every failure should be a structured `DevkitError`.

### Adding a target

1. Add `fuzz/fuzz_targets/<name>.rs` using the `libfuzzer_sys::fuzz_target!`
   macro.
2. Register a `[[bin]]` entry for it in `fuzz/Cargo.toml`.
3. The scheduled job discovers targets automatically; no workflow change needed.

## Scheduled fuzz smoke job

`.github/workflows/fuzz.yml` runs weekly (Mondays at 03:00 UTC) and on demand via
`workflow_dispatch`. It:

1. Discovers every target under `fuzz/fuzz_targets/`.
2. Runs each target for a bounded time (`60` seconds by default, overridable
   from the dispatch input).
3. Fails if any target crashes or leaves a crash artifact in `fuzz/artifacts`.
4. Uploads `fuzz/artifacts` as the `fuzz-crashes` artifact when a run fails.

Runs are intentionally short so they never slow normal pull-request CI.

### Pinning the nightly

The workflow uses `dtolnay/rust-toolchain@nightly`. To pin a specific nightly,
change that reference to a dated toolchain, for example:

```yaml
- uses: dtolnay/rust-toolchain@master
  with:
    toolchain: nightly-2026-01-01
```

## Reproducing a scheduled failure

Download the `fuzz-crashes` artifact, then re-run the offending input:

```bash
cd fuzz
cargo +nightly fuzz run decode_scval artifacts/decode_scval/<crash-file>
```
