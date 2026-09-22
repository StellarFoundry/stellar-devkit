#!/usr/bin/env python3
"""Generate a changelog section from Conventional Commits.

The generator is deterministic and reviewable: the same commit range always
produces the same section, and by default the section is only printed to stdout
(a dry run). Pass ``--write`` to splice the generated block into ``CHANGELOG.md``
between the ``changelog:generated`` markers, leaving human-written text alone.
"""

from __future__ import annotations

import argparse
import datetime as dt
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Sequence

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_CHANGELOG = ROOT / "CHANGELOG.md"

START_MARKER = "<!-- changelog:generated:start -->"
END_MARKER = "<!-- changelog:generated:end -->"

CONVENTIONAL_RE = re.compile(
    r"^(?P<type>[a-zA-Z]+)(?:\((?P<scope>[^)]*)\))?(?P<breaking>!)?: (?P<desc>.+)$"
)
BREAKING_RE = re.compile(r"^BREAKING[ -]CHANGE:", re.MULTILINE)

TYPE_TO_SECTION = {
    "feat": "Added",
    "fix": "Fixed",
    "perf": "Performance",
    "refactor": "Changed",
    "security": "Security",
    "docs": "Documentation",
    "test": "Tests",
    "ci": "CI",
    "build": "Build",
}

SECTION_ORDER = [
    "Breaking Changes",
    "Added",
    "Fixed",
    "Performance",
    "Changed",
    "Security",
    "Documentation",
    "Tests",
    "CI",
    "Build",
]

RELEASABLE = frozenset(TYPE_TO_SECTION)


@dataclass(frozen=True)
class Entry:
    """A single changelog entry derived from one commit."""

    section: str
    scope: str
    description: str


def parse_commit(subject: str, body: str = "") -> Entry | None:
    """Classify one commit into a changelog entry, or return ``None``.

    Non-conventional commits and non-releasable types (for example ``chore`` or
    ``style``) are ignored. Breaking changes are always releasable and land in
    the ``Breaking Changes`` section.
    """
    match = CONVENTIONAL_RE.match(subject.strip())
    if not match:
        return None
    commit_type = match.group("type").lower()
    breaking = bool(match.group("breaking")) or bool(BREAKING_RE.search(body))
    if breaking:
        section = "Breaking Changes"
    elif commit_type in RELEASABLE:
        section = TYPE_TO_SECTION[commit_type]
    else:
        return None
    return Entry(section, (match.group("scope") or "").strip(), match.group("desc").strip())


def collect_entries(commits: Iterable[tuple[str, str]]) -> list[Entry]:
    """Classify ``(subject, body)`` pairs and return them in a stable order."""
    entries = [entry for entry in (parse_commit(s, b) for s, b in commits) if entry]
    return sorted(entries, key=lambda e: (SECTION_ORDER.index(e.section), e.scope, e.description))


def render(entries: Sequence[Entry], version: str, date: str | None) -> str:
    """Render a Markdown changelog section for ``entries``."""
    heading = f"## [{version}]" + (f" - {date}" if date else "")
    lines = [heading]
    for section in SECTION_ORDER:
        subset = [e for e in entries if e.section == section]
        if not subset:
            continue
        lines += ["", f"### {section}", ""]
        for entry in subset:
            prefix = f"**{entry.scope}:** " if entry.scope else ""
            lines.append(f"- {prefix}{entry.description}")
    return "\n".join(lines) + "\n"


def _git(*args: str, cwd: Path) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=cwd,
        check=True,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    return result.stdout


def default_from_ref(cwd: Path) -> str | None:
    """Return the most recent tag, or ``None`` to use the whole history."""
    try:
        return _git("describe", "--tags", "--abbrev=0", cwd=cwd).strip()
    except subprocess.CalledProcessError:
        return None


def read_commits(from_ref: str | None, to_ref: str, cwd: Path) -> list[tuple[str, str]]:
    """Read ``(subject, body)`` pairs for the range ``from_ref..to_ref``."""
    revision = f"{from_ref}..{to_ref}" if from_ref else to_ref
    raw = _git("log", "--no-merges", "--format=%s%x1f%b%x1e", revision, cwd=cwd)
    commits = []
    for record in raw.split("\x1e"):
        record = record.strip("\n")
        if not record:
            continue
        subject, _, body = record.partition("\x1f")
        commits.append((subject, body))
    return commits


def splice(changelog: str, block: str) -> str:
    """Replace the generated markers in ``changelog`` with ``block``."""
    generated = f"{START_MARKER}\n{block}{END_MARKER}"
    if START_MARKER in changelog and END_MARKER in changelog:
        pattern = re.compile(
            re.escape(START_MARKER) + r".*?" + re.escape(END_MARKER), re.DOTALL
        )
        return pattern.sub(lambda _: generated, changelog, count=1)
    anchor = changelog.find("## [")
    if anchor == -1:
        return changelog.rstrip("\n") + "\n\n" + generated + "\n"
    return changelog[:anchor] + generated + "\n\n" + changelog[anchor:]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--from", dest="from_ref", help="start revision (exclusive)")
    parser.add_argument("--to", dest="to_ref", default="HEAD", help="end revision (inclusive)")
    parser.add_argument("--version", default="Unreleased", help="version heading")
    parser.add_argument("--date", default=None, help="release date (YYYY-MM-DD)")
    parser.add_argument(
        "--changelog", type=Path, default=DEFAULT_CHANGELOG, help="path to CHANGELOG.md"
    )
    parser.add_argument("--output", type=Path, help="write the section here instead of stdout")
    parser.add_argument(
        "--write", action="store_true", help="splice the section into CHANGELOG.md"
    )
    parser.add_argument("--repo-root", type=Path, default=ROOT, help="repository root")
    return parser


def _clean_ref(value: str | None) -> str | None:
    return value.strip() if value and value.strip() else None


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    to_ref = _clean_ref(args.to_ref) or "HEAD"
    from_ref = _clean_ref(args.from_ref)
    if from_ref is None:
        from_ref = default_from_ref(args.repo_root)

    try:
        commits = read_commits(from_ref, to_ref, args.repo_root)
    except subprocess.CalledProcessError as error:
        print(f"error: git failed: {error}", file=sys.stderr)
        return 2

    entries = collect_entries(commits)
    if not entries:
        revision = f"{from_ref}..{to_ref}" if from_ref else to_ref
        print(
            f"error: no releasable changes found in {revision}; "
            "use a conventional-commit prefix such as feat, fix, or perf",
            file=sys.stderr,
        )
        return 1

    date = args.date
    if date is None and args.version != "Unreleased":
        date = dt.date.today().isoformat()
    block = render(entries, args.version, date)

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(block, encoding="utf-8", newline="\n")
        print(f"wrote {args.output}")
    elif args.write:
        changelog = args.changelog.read_text(encoding="utf-8")
        args.changelog.write_text(splice(changelog, block), encoding="utf-8", newline="\n")
        print(f"updated {args.changelog}")
    else:
        sys.stdout.write(block)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
