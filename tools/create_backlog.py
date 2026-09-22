#!/usr/bin/env python3
"""Create the contributor backlog on GitHub from structured JSON definitions.

Bodies are rendered into a consistent template containing every required
section. Defaults to a dry run; pass --apply to create issues. Idempotent:
titles that already exist (open or closed) are skipped.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = "StellarFoundry/stellar-devkit"
BACKLOG_DIR = Path(__file__).resolve().parent / "backlog"


def run(args: list[str]) -> subprocess.CompletedProcess:
    return subprocess.run(args, capture_output=True, text=True, encoding="utf-8")


def bullet(items) -> str:
    if not items:
        return "_None._"
    if isinstance(items, str):
        return items
    return "\n".join(f"- {item}" for item in items)


def render_body(issue: dict) -> str:
    return "\n".join(
        [
            "## Problem",
            issue["problem"],
            "",
            "## Why this matters",
            issue["why"],
            "",
            "## Context",
            issue.get("context", "See the repository documentation for background."),
            "",
            "## Scope",
            bullet(issue.get("scope")),
            "",
            "## Out of scope",
            bullet(issue.get("out_of_scope", ["Anything not listed under Scope."])),
            "",
            "## Technical guidance",
            bullet(issue.get("guidance")),
            "",
            "## Relevant modules/files",
            bullet(issue.get("modules", ["To be identified during implementation."])),
            "",
            "## Acceptance criteria",
            bullet(issue.get("acceptance")),
            "",
            "## Tests required",
            bullet(issue.get("tests")),
            "",
            "## Security considerations",
            issue.get("security", "None specific beyond the project-wide guarantees."),
            "",
            "## Performance considerations",
            issue.get("performance", "None specific."),
            "",
            "## Documentation requirements",
            issue.get("docs", "Update the relevant docs and CHANGELOG.md."),
            "",
            "## Dependencies",
            issue.get("deps", "None."),
            "",
            "## Definition of done",
            issue.get("dod", "Merged with tests passing and CI green."),
            "",
            "## Complexity recommendation",
            f"`difficulty/{issue.get('complexity', 'medium')}`",
            "",
        ]
    )


def existing_titles() -> set[str]:
    result = run(
        ["gh", "issue", "list", "--repo", REPO, "--state", "all", "--limit", "2000", "--json", "title"]
    )
    if result.returncode != 0:
        print("Failed to list issues:", result.stderr, file=sys.stderr)
        sys.exit(1)
    return {i["title"] for i in json.loads(result.stdout or "[]")}


def existing_labels() -> set[str]:
    result = run(["gh", "label", "list", "--repo", REPO, "--limit", "500", "--json", "name"])
    if result.returncode != 0:
        return set()
    return {i["name"] for i in json.loads(result.stdout or "[]")}


def create_labels(labels, apply: bool) -> None:
    existing = existing_labels()
    for label in labels:
        if label["name"] in existing:
            continue
        if not apply:
            print(f"[dry-run] label {label['name']}")
            continue
        result = run(
            [
                "gh", "label", "create", label["name"],
                "--repo", REPO,
                "--color", label.get("color", "ededed"),
                "--description", label.get("description", ""),
            ]
        )
        if result.returncode != 0:
            print(f"  (label {label['name']}: {result.stderr.strip()}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--limit", type=int, default=0)
    args = parser.parse_args()

    labels = json.loads((BACKLOG_DIR / "labels.json").read_text(encoding="utf-8"))
    issues: list[dict] = []
    for path in sorted(BACKLOG_DIR.glob("issues_*.json")):
        issues.extend(json.loads(path.read_text(encoding="utf-8")))

    titles = [i["title"] for i in issues]
    if len(set(titles)) != len(titles):
        print("Duplicate titles in backlog definitions.", file=sys.stderr)
        return 1

    print(f"Backlog definitions: {len(issues)}")
    create_labels(labels, args.apply)

    existing = existing_titles()
    to_create = [i for i in issues if i["title"] not in existing]
    if args.limit:
        to_create = to_create[: args.limit]

    print(f"Already present: {len(issues) - len(to_create)}. To create: {len(to_create)}.")
    if not args.apply:
        for issue in to_create:
            print(f"[dry-run] {issue['title']}")
        return 0

    created = 0
    for issue in to_create:
        with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False, encoding="utf-8") as fh:
            fh.write(render_body(issue))
            body_path = fh.name
        cmd = ["gh", "issue", "create", "--repo", REPO, "--title", issue["title"], "--body-file", body_path]
        for label in issue.get("labels", []):
            cmd += ["--label", label]
        result = run(cmd)
        Path(body_path).unlink(missing_ok=True)
        if result.returncode != 0:
            print(f"FAILED: {issue['title']}\n  {result.stderr.strip()}", file=sys.stderr)
            continue
        created += 1
    print(f"Created {created} issue(s).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
