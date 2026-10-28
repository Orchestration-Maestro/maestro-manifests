"""Check the contracts the core roles state, without running agents."""

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ROLES = ["maestro", "scout", "planner", "reviewer", "worker"]
CURRENT_MODELS = {"claude-haiku-5-5", "claude-sonnet-5-5", "claude-opus-5-5"}
SHELL_AND_WRITE_TOOLS = {"bash", "edit", "write"}
AVOIDED_WORDS = (
    r"just|simply|leverage|utilize|oops|awesome|robust|seamless|magic|blazing|"
    r"revolutionary|supercharge|please note"
)


def read_role(name):
    """Return a core role's frontmatter fields and prompt body."""
    _, frontmatter, body = (ROOT / f"core/agents/{name}.md").read_text().split("---\n", 2)
    fields = dict(line.split(": ", 1) for line in frontmatter.splitlines())
    return fields, body


class CoreRoles(unittest.TestCase):
    """Hold each role to the rule it serves."""

    def test_roles_name_only_current_models(self):
        """Keep the stale model names out of every role and guide."""
        for name in ROLES:
            fields, _ = read_role(name)
            if "model" in fields:
                self.assertIn(fields["model"], CURRENT_MODELS, name)
        sources = [*ROOT.glob("core/**/*.md"), *ROOT.glob("docs/**/*.md"),
                   *ROOT.glob("capabilities/**/*.md"), ROOT / "README.md"]
        for path in sources:
            self.assertNotRegex(path.read_text(), r"claude-[a-z]+-4-5", path.name)

    def test_reviewer_authority_comes_from_its_tool_list(self):
        """Hold the reviewer to read tools, with no shell to ask it to limit."""
        fields, body = read_role("reviewer")
        self.assertEqual(fields["tools"], "read, grep, find, ls")
        self.assertNotRegex(body, r"(?i)\b(bash|shell|git)\b")

    def test_no_role_prohibits_a_tool_it_holds(self):
        """Reject a prompt sentence that limits a shell or write tool the role has."""
        for name in ROLES:
            fields, body = read_role(name)
            held = SHELL_AND_WRITE_TOOLS
            if "tools" in fields:
                held = held & set(fields["tools"].split(", "))
            for sentence in re.split(r"(?<=[.:!?])\s+", body):
                for tool in held:
                    if re.search(rf"\b{tool}\b", sentence):
                        self.assertNotRegex(sentence, r"(?i)\b(only|not|never|avoid)\b", name)

    def test_reviewer_findings_give_location_cause_fix_and_security_pass(self):
        """Require each finding's location, cause and fix, and a pass per security rule."""
        _, body = read_role("reviewer")
        for required in ["path:line", "cause", "smallest fix"]:
            self.assertIn(required, body)
        security_map = (ROOT / "docs/standards/security.md").read_text()
        rules = set(re.findall(r"SEC-\d{3}", body))
        self.assertGreaterEqual(len(rules), 5)
        for rule in rules:
            self.assertIn(f"| {rule} ", security_map)

    def test_planner_states_how_done_is_checked(self):
        """Require the check that means done and the tests to add first."""
        _, body = read_role("planner")
        self.assertIn("Done when", body)
        self.assertIn("Tests first", body)

    def test_worker_works_test_first_and_reports_exit_codes(self):
        """Require the failing test commit first and an exit code for each command."""
        _, body = read_role("worker")
        self.assertRegex(body, r"(?i)commit the failing test")
        self.assertIn("exit code", body)
        self.assertRegex(body, r"(?i)never claim")

    def test_worker_report_lets_a_reviewer_without_a_shell_find_the_change(self):
        """Pass line ranges from the worker so the reviewer can read the change."""
        self.assertIn("line range", read_role("worker")[1])
        self.assertIn("Files changed", read_role("reviewer")[1])

    def test_roles_speak_in_maestro_voice(self):
        """Lead the persona with the result and the next action, and avoid hype words."""
        for name in ROLES:
            with self.subTest(role=name):
                fields, body = read_role(name)
                self.assertNotRegex(fields["description"] + body, rf"(?i)\b({AVOIDED_WORDS})\b")
        persona = read_role("maestro")[1]
        self.assertRegex(persona, r"(?i)result")
        self.assertRegex(persona, r"(?i)next action")


if __name__ == "__main__":
    unittest.main()
