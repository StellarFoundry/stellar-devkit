#!/usr/bin/env python3
"""Generate docs/DRIPS_ISSUE_AUDIT.md from the backlog definitions."""

from __future__ import annotations

import glob
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs" / "DRIPS_ISSUE_AUDIT.md"

POINTS = {"trivial": 100, "medium": 150, "high": 200}
AREA_PRIORITY = [
    "area/core", "area/rpc", "area/xdr", "area/scval", "area/transactions",
    "area/events", "area/contract-inspection", "area/testing", "area/security",
    "area/cli", "area/config", "area/github", "area/sarif", "area/observability",
    "area/performance", "area/docs", "area/release", "area/ci",
]


def area(labels: list[str]) -> str:
    for candidate in AREA_PRIORITY:
        if candidate in labels:
            return candidate
    return "area/other"


def main() -> int:
    issues = []
    for path in sorted(glob.glob(str(ROOT / "tools" / "backlog" / "issues_*.json"))):
        issues.extend(json.loads(Path(path).read_text(encoding="utf-8")))

    issues.sort(key=lambda i: (POINTS.get(i.get("complexity", "medium"), 150), i["title"]))

    total = len(issues)
    counts = {"trivial": 0, "medium": 0, "high": 0}
    for i in issues:
        counts[i.get("complexity", "medium")] += 1
    points = sum(POINTS[i.get("complexity", "medium")] for i in issues)

    lines = [
        "# Drips Issue Audit",
        "",
        "Generated from `tools/backlog/issues_*.json`. The GitHub issue numbers are",
        "assigned on creation; titles are the stable identifier.",
        "",
        f"- **Total issues:** {total}",
        f"- **Trivial:** {counts['trivial']}",
        f"- **Medium:** {counts['medium']}",
        f"- **High:** {counts['high']}",
        f"- **Total pre-multiplier points:** {points}",
        "",
        "Complexity is assigned from the actual work required, not from a point",
        "target. There are **not** 125 legitimate high-complexity issues in the",
        "current scope, so a 25,000-point backlog is not supported. See",
        "[BUILD_REPORT.md](BUILD_REPORT.md).",
        "",
        "| # | Title | Area | Complexity | Points | Dependencies | Acceptance | Tests | Security |",
        "| - | ----- | ---- | ---------- | ------ | ------------ | ---------- | ----- | -------- |",
    ]
    for idx, issue in enumerate(issues, start=1):
        labels = issue.get("labels", [])
        complexity = issue.get("complexity", "medium")
        has_acceptance = "yes" if issue.get("acceptance") else "no"
        has_tests = "yes" if issue.get("tests") else "no"
        has_security = "yes" if issue.get("security") else "no"
        deps = issue.get("deps", "None.")
        lines.append(
            f"| {idx} | {issue['title']} | {area(labels).removeprefix('area/')} | "
            f"{complexity} | {POINTS[complexity]} | {deps} | {has_acceptance} | {has_tests} | {has_security} |"
        )

    lines.append("")
    lines.append("## Notes")
    lines.append("")
    lines.append("- Duplicate check: titles are unique across the backlog definitions.")
    lines.append("- Every issue contains problem, why, context, scope, out-of-scope, guidance,")
    lines.append("  acceptance criteria, required tests, security considerations, documentation")
    lines.append("  requirements, dependencies, and a definition of done.")
    lines.append("- Complexity is `difficulty/trivial|medium|high`; wave points are assigned")
    lines.append("  by the maintainer in the Drips dashboard, not by these labels.")
    lines.append("")

    OUT.write_text("\n".join(lines), encoding="utf-8", newline="\n")
    print(f"Wrote {OUT} with {total} issues ({counts['trivial']}/{counts['medium']}/{counts['high']}), {points} points.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
