# Contributing through Drips Wave

This project is prepared for participation in an open-source Wave. This page
explains how issues are scoped, how to claim one, and what to expect.

## Complexity

Every issue carries one complexity label:

| Label | Typical work |
| ----- | ------------ |
| `difficulty/trivial` | Small, clearly bounded change with obvious acceptance criteria. |
| `difficulty/medium` | A standard feature or logic touching several parts of the codebase. |
| `difficulty/high` | Complex engineering: integrations or architectural changes. |

Complexity is assigned from the scope of each individual issue, not from an
aggregate reward target. Drips determines the applicable points and budget
through its own system.

## Claiming an issue

1. Pick an issue. `good first issue` and `difficulty/trivial` are the easiest
   entry points.
2. **Request assignment before starting.** Comment on the issue to ask for it.
   Do not open a pull request for an issue you have not been assigned.
3. Wait for a maintainer to assign you, then begin.

## Working on an issue

1. Fork the repository and create a branch named for the change, for example
   `feat/rpc-live-transport`.
2. Follow the issue's **Scope**, **Out of scope**, **Technical guidance**, and
   **Acceptance criteria**; those define "done".
3. Write the tests the issue asks for.
4. Run the local gate:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   ```

5. Open a pull request whose description includes `Closes #<issue-number>` and
   how you verified the change.

## What maintainers provide

- An initial response to new issues and pull requests promptly during an active
  Wave.
- A review decision (approve, request changes, or close with an explanation)
  once a pull request is reviewable.
- Clear, actionable feedback and a merge when the acceptance criteria are met.

## Repository onboarding

Participation requires the repository to be applied to a Wave Program and
approved by the program organizers. See the Drips Wave maintainer documentation.
Approval is at the organizers' discretion.

## Security

Do not discuss a security vulnerability in a public issue or pull request.
Follow [`SECURITY.md`](../SECURITY.md).

## Definition of done

- The change matches the acceptance criteria.
- The requested tests are added and the local gate passes.
- Documentation and `CHANGELOG.md` are updated for user-visible changes.
- The pull request is reviewed and merged.
