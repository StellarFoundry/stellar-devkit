#!/usr/bin/env python3
"""Tests for the deterministic changelog generator."""

from __future__ import annotations

import io
import sys
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import changelog  # noqa: E402

REPO_ROOT = Path(__file__).resolve().parent.parent.parent


class ParseTests(unittest.TestCase):
    def test_feat_with_scope(self) -> None:
        entry = changelog.parse_commit("feat(rpc): add live transport")
        self.assertEqual(entry.section, "Added")
        self.assertEqual(entry.scope, "rpc")
        self.assertEqual(entry.description, "add live transport")

    def test_breaking_bang_flag(self) -> None:
        entry = changelog.parse_commit("feat(cli)!: drop legacy flag")
        self.assertEqual(entry.section, "Breaking Changes")

    def test_breaking_footer(self) -> None:
        entry = changelog.parse_commit(
            "refactor(core): reshape errors", "BREAKING CHANGE: error layout moved"
        )
        self.assertEqual(entry.section, "Breaking Changes")

    def test_non_releasable_type_ignored(self) -> None:
        self.assertIsNone(changelog.parse_commit("chore: bump cache"))
        self.assertIsNone(changelog.parse_commit("style: format imports"))

    def test_non_conventional_ignored(self) -> None:
        self.assertIsNone(changelog.parse_commit("Merge pull request #1"))


class RenderTests(unittest.TestCase):
    def test_sections_are_ordered_and_sorted(self) -> None:
        commits = [
            ("fix(z): late fix", ""),
            ("feat(b): second", ""),
            ("feat(a): first", ""),
            ("perf: faster", ""),
        ]
        entries = changelog.collect_entries(commits)
        rendered = changelog.render(entries, "1.0.0", "2026-09-22")
        self.assertLess(rendered.index("### Added"), rendered.index("### Fixed"))
        self.assertLess(rendered.index("### Fixed"), rendered.index("### Performance"))
        self.assertLess(rendered.index("first"), rendered.index("second"))

    def test_render_is_deterministic(self) -> None:
        commits = [("feat(a): one", ""), ("fix(b): two", "")]
        first = changelog.render(changelog.collect_entries(commits), "Unreleased", None)
        second = changelog.render(changelog.collect_entries(reversed(commits)), "Unreleased", None)
        self.assertEqual(first, second)


class CliTests(unittest.TestCase):
    def test_no_releasable_changes_fails(self) -> None:
        stderr = io.StringIO()
        stdout = io.StringIO()
        with redirect_stderr(stderr), redirect_stdout(stdout):
            code = changelog.main(
                ["--from", "HEAD", "--to", "HEAD", "--repo-root", str(REPO_ROOT)]
            )
        self.assertEqual(code, 1)
        self.assertIn("no releasable changes", stderr.getvalue())

    def test_dry_run_sample_range(self) -> None:
        import subprocess

        root = subprocess.run(
            ["git", "rev-list", "--max-parents=0", "HEAD"],
            cwd=REPO_ROOT,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        stdout = io.StringIO()
        with redirect_stdout(stdout):
            code = changelog.main(
                ["--from", root, "--to", "HEAD", "--repo-root", str(REPO_ROOT)]
            )
        self.assertEqual(code, 0)
        self.assertIn("### Added", stdout.getvalue())


if __name__ == "__main__":
    unittest.main()
