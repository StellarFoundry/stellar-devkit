# Drips Wave Appeal Summary

This page prepares an appeal for a rejected repository application. It does not
guarantee reconsideration. Appeals are decided by the Drips Wave program
reviewers.

## Before you submit

1. Open **Maintainers → Orgs and Repos** in the Drips Wave app.
2. Find `StellarFoundry/stellar-devkit`.
3. Confirm the status:
   - **Pending** — wait. There is nothing to appeal yet.
   - **Approved** — no appeal needed; add issues from the dashboard.
   - **Rejected** — note the rejection date and any stated reason.
4. Confirm the timing:
   - First appeal: **no earlier than two weeks after the rejection**.
   - Declined appeal: **one-month cooldown** before the next appeal.
   - Maximum **three appeals** per repository.
5. Submit **only through the app**. Email, Discord, and other channels are
   ignored.

## Appeal rules that shape the response

Reviewers reconsider a rejection only when there has been **substantive change**
to the code, project quality, activity, or the specific concerns raised. A
frivolous appeal consumes one of the three attempts.

Do not submit an appeal that only says the project is good. Every claim needs a
link to a merged PR, commit, CI run, release, or contributor.

## Short appeal text (ready to adapt)

> Since the previous review, StellarFoundry DevKit has changed in four concrete
> ways:
>
> 1. **The implementation is merged.** PR #127 (security/CI hardening) and PR
>    #133 (RPC transport, WASM inspection, analysis, configuration) merged into
>    `main` on 2026-10-28 at commit `aec759c`. CI and the RustSec audit are green
>    on `main`.
> 2. **The documentation now matches the code.** The status, readiness, audit,
>    changelog, roadmap, architecture, and getting-started documents no longer
>    describe merged work as open pull requests.
> 3. **The backlog is accurate.** The issue audit, backlog, and Wave board were
>    regenerated from live GitHub issue numbers. Completed work was removed from
>    the 20-issue Wave board, which now contains only open, independent,
>    Wave-sized issues.
> 4. **The scope is distinct.** DevKit is an XDR/RPC/transaction/contract
>    inspection workflow. It is not a source-level vulnerability scanner; the
>    boundary with `soroban-security-scanner` is documented in `docs/WHY_DEVKIT.md`.
>
> Evidence:
> - Merge: `aec759c`
> - CI: https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141880
> - RustSec: https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141721
> - Status: https://github.com/StellarFoundry/stellar-devkit/blob/main/docs/PROJECT_STATUS.md
> - Scope: https://github.com/StellarFoundry/stellar-devkit/blob/main/docs/WHY_DEVKIT.md
> - Wave board: https://github.com/StellarFoundry/stellar-devkit/blob/main/docs/WAVE_BOARD.md

If the rejection cited a specific concern, answer that concern directly and put
it first. Example:

> The review noted [concern]. Since then, [specific change]. You can verify it at
> [link].

## Evidence table

| Claim | Evidence |
| ----- | -------- |
| Implementation is merged | PR #127, PR #133, merge `aec759c` |
| CI is green on `main` | https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141880 |
| Dependency audit is green | https://github.com/StellarFoundry/stellar-devkit/actions/runs/40690141721 |
| Docs match code | `docs/PROJECT_STATUS.md`, `docs/DRIPS_READINESS.md`, `CHANGELOG.md` |
| Backlog is accurate | `docs/ISSUE_BACKLOG.md`, `docs/DRIPS_ISSUE_AUDIT.md`, `docs/DRIPS_ISSUE_QUALITY_AUDIT.md` |
| Wave board is curated | `docs/WAVE_BOARD.md` (20 issues, 3,050 points) |
| Scope is distinct | `docs/WHY_DEVKIT.md` |
| Repository gate | `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `cargo test --locked --workspace --all-features` |

## Do not claim

- That the project is approved, funded, or guaranteed points.
- That crates.io packages, release binaries, or a tagged release exist.
- That the analysis engine proves a contract is secure.
- That the project has significant adoption or external contributors.
- That another repository's approval applies to this repository.

## Optional improvements before appealing

These are not required, but they strengthen an appeal:

- Tag and publish `v0.1.0` with release binaries and checksums.
- Publish the workspace crates to crates.io.
- Get 3–5 external contributions or documented real-world usage.
- Add a docs site or a recorded end-to-end demo with real testnet data.

## After submitting

- Do not submit the same appeal again or contact reviewers through other
  channels.
- If declined, wait one month and address the new feedback before using another
  attempt.
- If approved, add the curated Wave board issues through the maintainer
  dashboard and set the complexity/points there.
