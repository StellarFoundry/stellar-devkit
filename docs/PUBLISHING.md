# Publishing to crates.io

The workspace is packaged but **not yet published** to crates.io. Publishing is
irreversible: a published version cannot be overwritten, only yanked. Read this
page before running the workflow.

## Prerequisites

1. A crates.io account with a verified email.
2. A crates.io API token with the `publish-new` and `publish-update` scopes.
3. The token stored as the `CARGO_REGISTRY_TOKEN` repository secret:
   **Settings → Secrets and variables → Actions → New repository secret**.
4. The version in `[workspace.package]` (`Cargo.toml`) and `CHANGELOG.md` must
   agree, and the working tree must be clean.

## Dry run

Run the **Publish** workflow with `dry_run = true` (the default). It executes:

```bash
cargo package --workspace --no-verify
```

This produces `.crate` archives under `target/package/` without contacting the
registry. To inspect a package:

```bash
cargo package -p devkit-core --no-verify
tar -tf target/package/devkit-core-0.1.0.crate
```

## Publish order

Crates must be published after their dependencies are visible on crates.io:

1. `devkit-core`
2. `devkit-config`
3. `devkit-xdr`
4. `devkit-rpc`
5. `devkit-contract`
6. `devkit-analysis`
7. `devkit-cli`

The workflow waits 30 seconds between publishes so the crates.io index can
expose each version before the next crate resolves it.

## Manual fallback

If the workflow cannot run, use the same order locally:

```bash
cargo login
cargo publish -p devkit-core --locked
sleep 30
cargo publish -p devkit-config --locked
sleep 30
cargo publish -p devkit-xdr --locked
sleep 30
cargo publish -p devkit-rpc --locked
sleep 30
cargo publish -p devkit-contract --locked
sleep 30
cargo publish -p devkit-analysis --locked
sleep 30
cargo publish -p devkit-cli --locked
```

## After publishing

1. Confirm every crate appears on crates.io.
2. Install from crates.io:
   ```bash
   cargo install devkit-cli --version 0.1.0
   stellar-foundry version
   ```
3. Update `README.md`, `docs/PROJECT_STATUS.md`, and `CHANGELOG.md` to say the
   crates are published.
4. Do not delete the `v0.1.0` tag or the GitHub Release.

## Notes

- The crate names `devkit-*` are currently unclaimed on crates.io.
- Do not publish from a dirty working tree; the workflow uses the checked-out
  commit.
- If a publish fails partway through, fix the cause and continue from the failed
  crate; already-published crates stay published.
