"""Check that the rule maps give every rule its own holder and evidence."""

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RULES = {
    "engineering": [f"FND-{n:03}" for n in range(1, 5)]
    + [f"P-{n:03}" for n in range(1, 19)]
    + [f"ENF-{n:03}" for n in range(1, 15)]
    + ["C-001", "C-004", "C-005", "C-006"],
    "security": [f"SEC-{n:03}" for n in range(1, 12)],
}
PILLARS = ["Speed", "Quality", "Maintainability", "Security"]


def read(name):
    """Return the text of a rule map."""
    return (ROOT / f"docs/standards/{name}.md").read_text()


def rows(text):
    """Return the cells of each table row that opens with a rule ID."""
    cells = (
        [cell.strip() for cell in line.strip("| \n").split(" | ")]
        for line in text.splitlines()
        if line.startswith("| ")
    )
    return [row for row in cells if re.match(r"(?:FND|P|ENF|SEC|C)-\d{3} ", row[0])]


def section(text, heading):
    """Return the body of a level-two section, or an empty string."""
    found = re.search(rf"^## {heading}\n(.*?)(?=^## |\Z)", text, re.M | re.S)
    return found.group(1).strip() if found else ""


class StandardsPages(unittest.TestCase):
    """Keep the engineering, security and Northstar pages mapped and checkable."""

    def test_no_page_has_an_unmapped_row(self):
        """Refuse the placeholder on every page."""
        for name in ["engineering", "security", "northstar"]:
            with self.subTest(page=name):
                self.assertNotIn("not mapped yet", read(name).lower())

    def test_every_rule_has_a_holder_and_its_evidence(self):
        """Give each rule a held-by cell and an evidence cell, and no extra rule."""
        for name, expected in RULES.items():
            with self.subTest(page=name):
                found = rows(read(name))
                self.assertEqual([row[0].split(" ")[0] for row in found], expected)
                for row in found:
                    self.assertEqual(len(row), 3, row[0])
                    self.assertTrue(all(row), row[0])
                    self.assertRegex(row[1], r"^(Gate|Review|Organization|GitHub)\b", row[0])

    def test_no_two_rules_share_a_holder(self):
        """Refuse one holder text standing in for several rules."""
        holders = [row[1] for name in RULES for row in rows(read(name))]
        self.assertEqual(len(holders), len(set(holders)))
        self.assertNotIn("names the principle a change breaks", read("engineering"))

    def test_no_page_names_a_retired_mechanism(self):
        """Refuse the generator that no longer writes these pages."""
        for name in ["engineering", "security", "northstar"]:
            with self.subTest(page=name):
                self.assertNotRegex(read(name), r"(?i)rust[-]gate")

    def test_holders_name_only_gates_this_repository_has(self):
        """Resolve each named file, recipe and hook in this checkout."""
        recipes = re.findall(r"^([a-z-]+):", (ROOT / "justfile").read_text(), re.M)
        hooks = re.findall(r"- id: ([\w-]+)", (ROOT / ".pre-commit-config.yaml").read_text())
        for name in ["engineering", "security", "northstar"]:
            text = read(name)
            for token in re.findall(r"`([^`]+)`", text):
                with self.subTest(page=name, token=token):
                    if re.fullmatch(r"[\w./-]+\.(?:md|json|yaml|yml|toml|lock|txt|py)", token) \
                            or token.endswith("/"):
                        self.assertTrue((ROOT / token).exists())
                    if token.startswith("just "):
                        self.assertIn(token.split(" ")[1], recipes)
            for group in re.findall(r"\bhooks? ((?:`[a-z-]+`(?:, | and )?)+)", text):
                for hook in re.findall(r"`([a-z-]+)`", group):
                    with self.subTest(page=name, hook=hook):
                        self.assertIn(hook, hooks)

    def test_security_page_says_what_it_protects(self):
        """Name the asset and the three places untrusted input enters."""
        text = section(read("security"), "What this repository protects").lower()
        for entry in ["pull requests", "capability packages", "fetched content"]:
            with self.subTest(entry=entry):
                self.assertIn(entry, text)

    def test_northstar_page_states_the_point_and_one_measured_kpi_per_pillar(self):
        """Write the point, then a KPI, target and measuring job for each pillar."""
        text = read("northstar")
        self.assertTrue(section(text, "The point"))
        kpis = {
            cells[0]: cells
            for cells in (
                [cell.strip() for cell in line.strip("| \n").split(" | ")]
                for line in section(text, "KPIs").splitlines()
                if line.startswith("| ")
            )
        }
        self.assertEqual([pillar for pillar in kpis if pillar in PILLARS], PILLARS)
        for pillar in PILLARS:
            with self.subTest(pillar=pillar):
                self.assertEqual(len(kpis[pillar]), 5)
                self.assertTrue(all(kpis[pillar]))
                self.assertNotEqual(kpis[pillar][4].lower(), "none yet")


if __name__ == "__main__":
    unittest.main()
