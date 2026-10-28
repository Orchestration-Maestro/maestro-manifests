"""Check the public author guides without claiming runtime validation."""

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PAGES = [ROOT / "README.md", *sorted((ROOT / "docs").rglob("*.md"))]


class AuthorDocs(unittest.TestCase):
    """Keep author navigation and verification cues usable."""

    def test_guides_have_contents_constraints_and_change_checks(self):
        """Show every section and distinguish requirements from optional guidance."""
        for path in PAGES:
            text = re.sub(r"```.*?```", "", path.read_text(), flags=re.S)
            sections = re.findall(r"(?m)^## (.+)$", text)
            with self.subTest(page=path.relative_to(ROOT)):
                self.assertRegex(text, r"(?s)# [^\n]+\n\n[^#\n].*?\n\n## Contents\n")
                contents = text.split("## Contents\n", 1)[1].split("\n## ", 1)[0]
                self.assertEqual(
                    re.findall(r"(?m)^\d+\. \[([^]]+)\]", contents),
                    [section for section in sections if section != "Contents"],
                )
                self.assertIn("What the organization can lock", sections)
                self.assertIn("Check a change", sections)
                self.assertNotRegex(text, r"\bshould\b|when useful")


if __name__ == "__main__":
    unittest.main()
