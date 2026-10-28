"""Check that instruction and configuration files state only what holds."""

import re
import subprocess
import unittest
from fnmatch import fnmatch
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def tracked_files():
    """Return the repository-relative paths Git tracks."""
    output = subprocess.check_output(["git", "ls-files", "-z"], cwd=ROOT, timeout=60)
    return [name for name in output.decode().split("\0") if name]


def expand_braces(pattern):
    """Expand one `{a,b}` group the way EditorConfig reads it."""
    match = re.search(r"\{([^{}]*)\}", pattern)
    if not match:
        return [pattern]
    return [
        expanded
        for option in match.group(1).split(",")
        for expanded in expand_braces(pattern[: match.start()] + option + pattern[match.end() :])
    ]


class InstructionClaims(unittest.TestCase):
    """Keep each stated check and setting true of this repository."""

    def test_setup_is_not_detected_by_the_required_workflow(self):
        """Find no claim that CI notices a skipped `just setup`."""
        text = (ROOT / "AGENTS.md").read_text()
        setup = text.split("## Setup", 1)[1].split("\n## ", 1)[0]
        self.assertNotRegex(setup, r"(?i)required\s+workflow")

    def test_secret_scan_is_not_attributed_to_ci(self):
        """Find gitleaks in the commit hook row, not in CI."""
        text = (ROOT / "docs/standards/engineering.md").read_text()
        (row,) = [line for line in text.splitlines() if line.startswith("| ENF-013")]
        self.assertNotRegex(row, r"\bCI\b")
        self.assertIn("gitleaks", row)

    def test_every_editorconfig_section_matches_a_tracked_file(self):
        """Find no section for a kind of file the repository does not hold."""
        sections = re.findall(r"^\[(.+)\]$", (ROOT / ".editorconfig").read_text(), re.M)
        names = [Path(name).name for name in tracked_files()]
        for section in sections:
            with self.subTest(section=section):
                patterns = expand_braces(section.lstrip("/"))
                self.assertTrue(
                    any(fnmatch(name, pattern) for name in names for pattern in patterns)
                )


if __name__ == "__main__":
    unittest.main()
