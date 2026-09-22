#!/usr/bin/env python3
"""Generate docs/ISSUE_BACKLOG.md from backlog definitions and GitHub numbers."""

from __future__ import annotations

import glob
import json
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BACKLOG = Path(__file__).resolve().parent / "backlog"
REPO = "StellarFoundry/stellar-devkit"
OUT = ROOT / "docs" / "ISSUE_BACKLOG.md"
POINTS = {"trivial": 100, "medium": 150, "high": 200}
AREA_PRIORITY = [
    "area/core", "area/rpc", "area/xdr", "area/scval", "area/transactions",
    "area/events", "area/contract-inspection", "area/testing", "area/security",
    "area/cli", "area/config", "area/github", "area/sarif", "area/observability",
    "area/docs", "area/release", "area/ci",
]
PHASE_ORDER = [
    "phase/01-foundation", "phase/02-rpc", "phase/03-xdr-scval", "phase/04-analysis",
    "phase/05-contracts", "phase/06-testing", "phase/07-security", "phase/08-cli",
    "phase/09-github", "phase/10-observability", "phase/11-devex", "phase/12-release",
]


def gh_numbers() -> dict[str, int]:
    result = subprocess.run(
        ["gh", "issue", "list", "--repo", REPO, "--state", "open", "--limit", "2000",
         "--json", "number,title"],
        capture_output=True, text=True, encoding="utf-8",
    )
    if result.returncode != 0:
        print(result.stderr, file=sys.stderr)
        sys.exit(1)
    return {i["title"]: i["number"] for i in json.loads(result.stdout or "[]")}


def area(labels: list[str]) -> str:
    for c in AREA_PRIORITY:
        if c in labels:
            return c.split("/", 1)[1]
    return "other"


def main() -> int:
    numbers = gh_numbers()
    issues = []
    for path in sorted(glob.glob(str(BACKLOG / "issues_*.json"))):
        for issue in json.loads(Path(path).read_text(encoding="utf-8")):
            number = numbers.get(issue["title"])
            if number is None:
                continue
            labels = issue.get("labels", [])
            phase = next((l for l in labels if l.startswith("phase/")), "phase/other")
            issues.append({
                "title": issue["title"], "number": number, "phase": phase,
                "area": area(labels), "complexity": issue.get("complexity", "medium"),
                "deps": issue.get("deps", "None."),
            })

    by_phase = defaultdict(list)
    for issue in issues:
        by_phase[issue["phase"]].append(issue)
    for phase in by_phase:
        by_phase[phase].sort(key=lambda i: i["number"])

    counts = {"trivial": 0, "medium": 0, "high": 0}
    for issue in issues:
        counts[issue["complexity"]] += 1
    points = sum(POINTS[i["complexity"]] for i in issues)

    lines = [
        "# Issue Backlog", "",
        "Generated from `tools/backlog/issues_*.json`. Closed and superseded",
        "definitions are excluded.", "",
        f"**Total open issues:** {len(issues)}", "",
        f"Complexity: {counts['trivial']} trivial, {counts['medium']} medium, "
        f"{counts['high']} high — {points} pre-multiplier points.", "",
    ]
    for phase in PHASE_ORDER + [p for p in by_phase if p not in PHASE_ORDER]:
        if phase not in by_phase:
            continue
        lines.append(f"## {phase}")
        lines.append("")
        lines.append("| # | Title | Area | Complexity | Dependencies |")
        lines.append("| - | ----- | ---- | ---------- | ------------ |")
        for issue in by_phase[phase]:
            lines.append(
                f"| #{issue['number']} | {issue['title']} | {issue['area']} | "
                f"{issue['complexity']} | {issue['deps']} |"
            )
        lines.append("")

    lines.append("## Summary")
    lines.append("")
    lines.append("| Phase | Count |")
    lines.append("| ----- | ----- |")
    for phase in PHASE_ORDER:
        if phase in by_phase:
            lines.append(f"| {phase} | {len(by_phase[phase])} |")
    lines.append(f"| **Total** | **{len(issues)}** |")
    lines.append("")

    OUT.write_text("\n".join(lines), encoding="utf-8", newline="\n")
    print(f"Wrote {OUT} with {len(issues)} open issues.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
