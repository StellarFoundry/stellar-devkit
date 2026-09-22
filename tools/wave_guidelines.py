"""Shared Drips Wave contribution guidelines for issue bodies.

Used by ``apply_wave_guidelines.py`` to append a consistent conventions block to
issues that are added to a Wave.
"""

from __future__ import annotations

REPO_URL = "https://github.com/StellarFoundry/stellar-devkit"
MARKER = "## Contribution guidelines"


def render_guidelines(title: str, number: int | None = None) -> str:
    """Render the standard contribution-guidelines block for an issue."""
    closes = f"`Closes #{number}`" if number else "`Closes #` followed by this issue's number"
    return "\n".join(
        [
            MARKER,
            "",
            "- **Assignment required before starting.** Request assignment on this "
            "issue; do not open a pull request until a maintainer has assigned you.",
            f"- Your pull request must include {closes}.",
            "- Run the local gate before opening the PR:",
            "",
            "  ```",
            "  cargo fmt --all -- --check",
            "  cargo clippy --all-targets --all-features -- -D warnings",
            "  cargo test --all",
            "  ```",
            "",
            f"- Example commit message: `{title}`",
            "- Suggested timeframe: complete within the active Wave cycle.",
            "- Complexity is tagged with a `difficulty/*` label. Drips determines the "
            "applicable points and budget through its own system.",
            f"- See [docs/WAVE.md]({REPO_URL}/blob/main/docs/WAVE.md) and "
            f"[CONTRIBUTING.md]({REPO_URL}/blob/main/CONTRIBUTING.md).",
            "",
        ]
    )
