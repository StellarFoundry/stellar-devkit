#!/usr/bin/env python3
"""Append the standard Wave contribution guidelines to Drips-ready issues.

Idempotent: issues that already contain the marker are skipped. Defaults to a
dry run; pass --apply to edit issue bodies.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

from wave_guidelines import MARKER, render_guidelines

REPO = "StellarFoundry/stellar-devkit"


def run(args: list[str]) -> subprocess.CompletedProcess:
    return subprocess.run(args, capture_output=True, text=True, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--label", default="status/ready")
    parser.add_argument("--limit", type=int, default=0)
    args = parser.parse_args()

    result = run([
        "gh", "issue", "list", "--repo", REPO, "--state", "open",
        "--label", args.label, "--limit", "500", "--json", "number,title,body",
    ])
    if result.returncode != 0:
        print("Failed to list issues:", result.stderr, file=sys.stderr)
        return 1

    issues = json.loads(result.stdout or "[]")
    todo = [i for i in issues if MARKER not in (i.get("body") or "")]
    if args.limit:
        todo = todo[: args.limit]

    print(f"Issues with label {args.label}: {len(issues)}. Needing guidelines: {len(todo)}.")
    if not args.apply:
        for i in todo:
            print(f"[dry-run] #{i['number']} {i['title']}")
        return 0

    updated = 0
    for i in todo:
        body = (i.get("body") or "").rstrip() + "\n\n" + render_guidelines(i["title"], i["number"])
        with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False, encoding="utf-8") as fh:
            fh.write(body)
            path = fh.name
        edit = run(["gh", "issue", "edit", str(i["number"]), "--repo", REPO, "--body-file", path])
        Path(path).unlink(missing_ok=True)
        if edit.returncode != 0:
            print(f"FAILED #{i['number']}: {edit.stderr.strip()}", file=sys.stderr)
            continue
        updated += 1
    print(f"Updated {updated} issue(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
