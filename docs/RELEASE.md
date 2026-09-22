# Release guide

This guide describes how DevKit release notes are produced. Publishing releases
is handled separately by the maintainer.

## Versioning

The workspace follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
The version lives in the `[workspace.package]` table of the root `Cargo.toml`.

## Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Release notes are generated from [Conventional Commits](https://www.conventionalcommits.org/)
so the file cannot drift from merged work, while remaining hand-editable.

### How generation works

`tools/changelog.py` reads the commits in a range and groups them by type:

| Commit type | Section |
| ----------- | ------- |
| `feat` | Added |
| `fix` | Fixed |
| `perf` | Performance |
| `refactor` | Changed |
| `security` | Security |
| `docs` | Documentation |
| `test` | Tests |
| `ci` | CI |
| `build` | Build |

A `!` after the type/scope, or a `BREAKING CHANGE:` footer, moves the entry to
**Breaking Changes**. Non-releasable types such as `chore` and `style`, and
non-conventional subjects, are ignored. Entries are sorted by section, scope,
and description, so the same range always produces the same section.

If the range contains no releasable changes, the tool fails with a clear error
instead of emitting an empty section.

### Dry run (reviewable)

Preview the next section without touching the file:

```bash
python tools/changelog.py --version 0.2.0 --to HEAD
```

By default the range starts at the latest tag. Use `--from`/`--to` to pin a
sample range explicitly, and `--output FILE` to save the section for review:

```bash
python tools/changelog.py --from v0.1.0 --to HEAD --version 0.2.0 --output build/section.md
```

### Writing the changelog

Apply the generated block to `CHANGELOG.md`:

```bash
python tools/changelog.py --version 0.2.0 --write
```

Only the region between the `<!-- changelog:generated:start -->` and
`<!-- changelog:generated:end -->` markers is replaced. Text outside the markers
is preserved, so you can keep hand-written notes alongside generated entries.

## Automated generation

`.github/workflows/changelog.yml` runs the generator on demand
(`workflow_dispatch`) and whenever a release is published. It performs a dry run
and uploads the generated section as the `changelog-section` artifact for
review. The workflow never commits on its own; a maintainer applies the section
with `--write` during the release.

## Release checklist

1. Confirm the version in the root `Cargo.toml`.
2. Generate and review the section (dry run above).
3. Apply it with `--write` and edit prose as needed.
4. Run the local gate:
   `cargo fmt --all -- --check`,
   `cargo clippy --all-targets --all-features -- -D warnings`,
   `cargo test --all`.
5. Tag the release, publish it, and confirm the workflow uploaded the artifact.
