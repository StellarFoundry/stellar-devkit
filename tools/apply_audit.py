#!/usr/bin/env python3
"""Apply the Drips quality-audit outcome to GitHub issues.

- Creates `status/*` labels and applies them per classification.
- Closes duplicate/merged issues with an explanatory comment.
- Appends a clarification to issues flagged as needing revision.

Run with --apply. Requires `gh` authenticated for StellarFoundry.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from audit_backlog import BLOCKED, DEFERRED, DUPLICATES, MERGES, NEEDS_REVISION  # noqa: E402

REPO = "StellarFoundry/stellar-devkit"

CLARIFICATIONS = {
    55: "Use the length limits defined by the Stellar protocol for `ScSymbol`, "
        "`ScString`, and `ScBytes`. Cite the exact numeric limits in the rule, the "
        "documentation, and the boundary tests. If the limits cannot be cited from an "
        "authoritative source, omit that type rather than guessing.",
    76: "Define the ledger-to-close-time relationship precisely using the documented "
        "target close time, and label the result as an approximation. Provide "
        "deterministic range formatting. Do not present an estimate as an exact time.",
    110: "Do not implement the cache until a measured workload exists. Reference the "
        "decode benchmark (#28) and include before/after numbers demonstrating repeated "
        "decoding of the same value. If no such workload exists, this work is premature.",
}


def run(args: list[str]) -> subprocess.CompletedProcess:
    return subprocess.run(args, capture_output=True, text=True, encoding="utf-8")


def ensure_labels() -> None:
    labels = [
        ("status/ready", "0e8a16", "Audited and Wave-ready"),
        ("status/blocked", "b60205", "Blocked by an unimplemented dependency"),
        ("status/deferred", "fbca04", "Valid idea, not ready for a Wave"),
        ("status/needs-revision", "d93f0b", "Needs revision before it is Wave-ready"),
    ]
    existing = run(["gh", "label", "list", "--repo", REPO, "--limit", "500", "--json", "name"])
    have = {i["name"] for i in json.loads(existing.stdout or "[]")} if existing.returncode == 0 else set()
    for name, color, desc in labels:
        if name in have:
            continue
        run(["gh", "label", "create", name, "--repo", REPO, "--color", color, "--description", desc])


def add_label(number: int, label: str) -> None:
    result = run(["gh", "issue", "edit", str(number), "--repo", REPO, "--add-label", label])
    if result.returncode != 0:
        print(f"  label {label} on #{number}: {result.stderr.strip()}")


def close_with_comment(number: int, comment: str) -> None:
    run(["gh", "issue", "comment", str(number), "--repo", REPO, "--body", comment])
    run(["gh", "issue", "close", str(number), "--repo", REPO, "--reason", "not planned"])


def revise(number: int, title: str, text: str) -> None:
    view = run(["gh", "issue", "view", str(number), "--repo", REPO, "--json", "body"])
    if view.returncode != 0:
        print(f"  view #{number}: {view.stderr.strip()}")
        return
    body = json.loads(view.stdout)["body"].replace("\r\n", "\n").rstrip()
    if "## Clarification (quality audit)" in body:
        return
    body += f"\n\n## Clarification (quality audit)\n\n{text}\n"
    with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False, encoding="utf-8") as fh:
        fh.write(body)
        path = fh.name
    result = run(["gh", "issue", "edit", str(number), "--repo", REPO, "--body-file", path])
    Path(path).unlink(missing_ok=True)
    if result.returncode != 0:
        print(f"  revise #{number}: {result.stderr.strip()}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("issues", help="path to gh issues JSON")
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    issues = json.loads(Path(args.issues).read_text(encoding="utf-8-sig"))
    numbers = {i["number"] for i in issues}

    non_ready = set(BLOCKED) | set(DEFERRED) | set(NEEDS_REVISION) | set(DUPLICATES) | set(MERGES)
    ready = sorted(numbers - non_ready)

    print(f"ready={len(ready)} blocked={len(set(BLOCKED) & numbers)} "
          f"deferred={len(set(DEFERRED) & numbers)} needs_revision={len(set(NEEDS_REVISION) & numbers)}")
    if not args.apply:
        return 0

    ensure_labels()
    for n in ready:
        add_label(n, "status/ready")
    for n in sorted(set(BLOCKED) & numbers):
        add_label(n, "status/blocked")
    for n in sorted(set(DEFERRED) & numbers):
        add_label(n, "status/deferred")
    for n in sorted(set(NEEDS_REVISION) & numbers):
        add_label(n, "status/needs-revision")

    for dup, canonical in DUPLICATES.items():
        if dup in numbers:
            close_with_comment(dup, f"Duplicate of #{canonical}. Closing to keep one canonical "
                                    f"event-decoding issue. No replacement issue is created.")
    for src, into in MERGES.items():
        if src in numbers:
            close_with_comment(src, f"Merged into #{into}: endpoint/network profile resolution "
                                    f"already covers this. Closing to avoid duplicate configuration issues.")

    for n, text in CLARIFICATIONS.items():
        if n in numbers:
            add_label(n, "status/needs-revision")
            revise(n, "", text)

    print("Applied audit changes.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
