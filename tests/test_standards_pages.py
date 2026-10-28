"""Check that the standards pages explain each principle family and map nothing in tables."""

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOPICS = ["What it is", "Why this repository applies it", "How it is applied here"]
FAMILIES = {
    "engineering": {
        "Think first, then change only what the goal needs": [
            "FND-001", "FND-002", "FND-003", "FND-004",
        ],
        "Keep the design small and plain": [
            "P-001", "P-002", "P-008", "P-009", "P-010", "P-015", "P-016", "P-017",
        ],
        "Say a thing once, but not too early": ["P-003", "P-004", "P-005"],
        "Respect what is already here": ["P-006", "P-007", "P-018"],
        "Refuse bad input and bad states": ["P-011", "P-012", "P-013"],
        "Grant only the authority a task needs": ["P-014", "ENF-011"],
        "Let tests and gates carry the rules": [
            "ENF-005", "ENF-006", "ENF-008", "ENF-009", "ENF-010",
        ],
        "Deliver through signed pull requests": ["ENF-004", "ENF-007"],
        "Pin inputs and keep secrets out": ["ENF-012", "ENF-013", "ENF-014"],
        "Write portable plain text": ["ENF-001", "ENF-002", "ENF-003"],
        "Adopt the rules and prove it": ["C-001", "C-004", "C-005", "C-006"],
    },
    "security": {
        "Keep sensitive data small": ["SEC-001"],
        "Treat everything read as data": ["SEC-002", "SEC-003"],
        "Take authority from enforcement": ["SEC-004", "SEC-005"],
        "Inspect code without running it unsafely": ["SEC-006"],
        "Stop, escalate and report vulnerabilities": ["SEC-007", "SEC-010"],
        "Keep evidence truthful": ["SEC-008", "SEC-009"],
        "Sign every release": ["SEC-011"],
    },
    "northstar": {pillar: [] for pillar in ["Speed", "Quality", "Maintainability", "Security"]},
}
LINKING_PAGES = ["README.md", "docs/development.md", "docs/index.md"]


def read(name):
    """Return the text of a file under the repository root."""
    return (ROOT / name).read_text()


def page(name):
    """Return the text of a standards page."""
    return read(f"docs/standards/{name}.md")


def parts(text, marker):
    """Return the body of each section opened by a heading at the given marker."""
    pieces = re.split(rf"(?m)^{marker} (.+)\n", text)
    return dict(zip(pieces[1::2], pieces[2::2], strict=True))


def words(text):
    """Count the words of a section body."""
    return len(text.split())


def slug(heading):
    """Return the anchor GitHub gives a heading."""
    return re.sub(r"[^\w -]", "", heading.lower()).replace(" ", "-")


class StandardsPages(unittest.TestCase):
    """Keep the engineering, security and Northstar pages explanatory and true."""

    def test_every_family_explains_the_concept_the_reason_and_the_practice(self):
        """Give each principle family a section for what it is, why and how."""
        for name, families in FAMILIES.items():
            sections = parts(page(name), "##")
            for family in families:
                with self.subTest(page=name, family=family):
                    self.assertIn(family, sections)
                    topics = parts(sections[family], "###")
                    for topic in TOPICS:
                        self.assertIn(topic, topics)
                        self.assertGreaterEqual(words(topics[topic]), 20, topic)

    def test_every_golden_rule_is_named_in_its_family(self):
        """Keep each organization rule reachable from the family that explains it."""
        for name, families in FAMILIES.items():
            sections = parts(page(name), "##")
            for family, rules in families.items():
                for rule in rules:
                    with self.subTest(page=name, family=family, rule=rule):
                        self.assertRegex(sections.get(family, ""), rf"\b{rule}\b")

    def test_no_page_has_a_mapping_table_or_rule_by_rule_rows(self):
        """Refuse tables and lines that open with a rule ID."""
        for name in FAMILIES:
            text = page(name)
            with self.subTest(page=name):
                self.assertNotRegex(text, r"(?m)^\s*\|")
                self.assertNotRegex(
                    text, r"(?m)^[\s>*_-]*(?:FND|P|ENF|SEC|C)-\d{3}(?:\*\*|:|\s+[A-Z—–-])"
                )
                self.assertNotIn("not mapped yet", text.lower())

    def test_every_page_opens_as_a_guide_with_contents_for_its_sections(self):
        """Open with a title, a purpose paragraph and numbered links to each section."""
        for name in FAMILIES:
            text = page(name)
            sections = parts(text, "##")
            with self.subTest(page=name):
                self.assertRegex(text, r"\A# [^\n]+\n\n[^#\n]")
                links = re.findall(r"(?m)^\d+\. \[([^\]]+)\]\(#([^)]+)\)$", sections.get("Contents", ""))
                self.assertEqual(
                    links,
                    [(title, slug(title)) for title in sections if title != "Contents"],
                )

    def test_policy_control_explanation_has_one_source(self):
        """Keep shared controls in engineering and link the other standards to them."""
        heading = "What the organization can lock"
        explanation = "These standards govern contributions; they add no runtime policy keys."
        self.assertIn(explanation, parts(page("engineering"), "##")[heading])
        for name in ["northstar", "security"]:
            with self.subTest(page=name):
                self.assertNotIn(explanation, page(name))
                self.assertIn(
                    "[Engineering standards](engineering.md#what-the-organization-can-lock)",
                    parts(page(name), "##")[heading],
                )

    def test_no_page_names_a_retired_mechanism(self):
        """Refuse the generator that no longer writes these pages."""
        for name in FAMILIES:
            with self.subTest(page=name):
                self.assertNotRegex(page(name), r"(?i)rust[-]gate")

    def test_pages_name_only_gates_this_repository_has(self):
        """Resolve each named file, recipe and hook in this checkout."""
        recipes = re.findall(r"^([a-z-]+):", read("justfile"), re.M)
        hooks = re.findall(r"- id: ([\w-]+)", read(".pre-commit-config.yaml"))
        for name in FAMILIES:
            text = page(name)
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
        text = parts(page("security"), "##").get("What this repository protects", "").lower()
        for entry in ["pull requests", "capability packages", "fetched content"]:
            with self.subTest(entry=entry):
                self.assertIn(entry, text)

    def test_northstar_page_states_the_point_and_a_measured_kpi_per_pillar(self):
        """Write the point, then a current value, target and measuring job per pillar."""
        sections = parts(page("northstar"), "##")
        self.assertGreaterEqual(words(sections.get("The point", "")), 20)
        for pillar in FAMILIES["northstar"]:
            kpi = parts(sections.get(pillar, ""), "###").get("KPI", "")
            for label in ["Current", "Target", "Measured by"]:
                with self.subTest(pillar=pillar, label=label):
                    self.assertRegex(kpi, rf"(?m)^- \*\*{label}:\*\* \S")
            self.assertNotIn("none yet", kpi.lower())

    def test_pages_that_link_the_standards_do_not_call_them_generated(self):
        """Describe the standards pages as written by hand where the guides mention them."""
        for name in LINKING_PAGES:
            text = read(name)
            with self.subTest(page=name):
                self.assertNotRegex(text, r"(?i)generated[- ]rule[- ]maps?")
                for paragraph in text.split("\n\n"):
                    if "standards/" in paragraph:
                        self.assertNotRegex(paragraph, r"(?i)generated|rust[-]gate")


if __name__ == "__main__":
    unittest.main()
