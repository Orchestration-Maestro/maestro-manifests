"""Check authored role, workflow and theme examples without running agents."""

import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "manifests"
CAPABILITY = MANIFEST / "capabilities/practice/evidence-review"


def skill_rules(skill):
    """Return the skill's rule table as rows of stripped cells, header first."""
    return [[cell.strip() for cell in row.strip("|").split("|")]
            for row in re.findall(r"^\|.+\|$", skill, re.M)]


class ManifestExamples(unittest.TestCase):
    """Keep example content and its documented contracts in sync."""

    def test_core_roles_keep_contracts_with_maestro_wording(self):
        """Preserve permissions, models and output sections during prose edits."""
        for role, tools, model, headings, former_intro in [
            ("scout", "read, grep, find, ls, bash", "claude-haiku-5-5",
             ["Files retrieved", "Key code", "Architecture", "Start here"],
             "Quickly investigate a codebase"),
            ("planner", "read, grep, find, ls", "claude-opus-5-5",
             ["Goal", "Plan", "Files to modify", "New files (if any)", "Verification",
              "Risks"],
             "You are a planning specialist"),
            ("reviewer", "read, grep, find, ls", "claude-opus-5-5",
             ["Files reviewed", "Critical (blocks)", "Important (blocks)",
              "Minor (fix in this change)", "Security", "Summary"],
             "You are a senior code reviewer"),
            ("worker", None, "claude-sonnet-5-5",
             ["Completed", "Files changed", "Patch", "Commands run", "Notes (if any)"],
             "without polluting the main conversation"),
        ]:
            with self.subTest(role=role):
                text = (MANIFEST / f"core/agents/{role}.md").read_text()
                frontmatter, body = text.split("---\n")[1:3]
                self.assertIn(f"name: {role}\n", frontmatter)
                self.assertIn(f"model: {model}\n", frontmatter)
                if tools:
                    self.assertIn(f"tools: {tools}\n", frontmatter)
                else:
                    self.assertNotIn("tools:", frontmatter)
                self.assertEqual(re.findall(r"^## (.+)$", body, re.M), headings)
                self.assertNotIn(former_intro, body)
                self.assertNotIn("path/to/file.ts", body)
                self.assertNotIn("```typescript", body)

    def test_core_workflows_keep_steps_and_handoffs_with_maestro_tasks(self):
        """Retain argument and previous-output passing in all three chains."""
        for name, roles, original_arguments in [
            ("implement-and-review", ["worker", "reviewer", "worker"], [0]),
            ("implement", ["scout", "planner", "worker"], [0, 1]),
            ("scout-and-plan", ["scout", "planner"], [0, 1]),
        ]:
            with self.subTest(workflow=name):
                text = (MANIFEST / f"core/workflows/{name}.yaml").read_text()
                self.assertIn("apiVersion: maestro.workflow/v1\n", text)
                self.assertIn(f"kind: Workflow\nname: {name}\n", text)
                self.assertEqual(re.findall(r"^  - agent: (.+)$", text, re.M), roles)
                tasks = re.findall(r"^    task: (.+)$", text, re.M)
                self.assertEqual(len(tasks), len(roles))
                for index, task in enumerate(tasks):
                    self.assertEqual(task.count("$@"), int(index in original_arguments))
                    self.assertEqual(task.count("{previous}"), int(index > 0))
                self.assertNotIn('task: "Implement: $@"', text)
                self.assertNotIn("Find all code relevant to:", text)
                if name == "scout-and-plan":
                    self.assertIn("Do not implement", tasks[-1])
                if name == "implement-and-review":
                    for guide in ["workflows", "schemas-and-checks", "catalog"]:
                        self.assertIn(f"```yaml\n{text}```",
                                      (ROOT / f"docs/{guide}.md").read_text())

    def test_review_dark_uses_forge_roles_for_all_tokens(self):
        """Use Forge dark colors with readable text on every tool panel."""
        theme = json.loads((CAPABILITY / "themes/review-dark.json").read_text())
        groups = {
            "#D9A066": "accent borderAccent customMessageLabel mdHeading mdLink mdCode "
                       "mdListBullet syntaxKeyword syntaxFunction thinkingLow "
                       "thinkingMedium warning syntaxNumber muted dim thinkingText "
                       "toolOutput mdLinkUrl mdQuote toolDiffContext syntaxComment "
                       "thinkingMinimal",
            "#5E7383": "border borderMuted mdCodeBlockBorder mdQuoteBorder mdHr thinkingOff",
            "#FF6A1A": "success toolDiffAdded syntaxString bashMode error toolDiffRemoved "
                       "syntaxType thinkingHigh thinkingXhigh",
            "#F2E8DC": "mdCodeBlock syntaxVariable syntaxOperator syntaxPunctuation",
            "#2A1F1A": "selectedBg userMessageBg customMessageBg toolPendingBg "
                       "toolSuccessBg toolErrorBg",
            "": "text userMessageText customMessageText toolTitle",
        }
        expected = {key: color for color, keys in groups.items() for key in keys.split()}
        self.assertEqual(len(expected), 51)
        colors = {key: theme.get("vars", {}).get(value, value)
                  for key, value in theme["colors"].items()}
        self.assertEqual(colors, expected)
        self.assertEqual(theme["name"], "review-dark")
        text = (CAPABILITY / "themes/review-dark.json").read_text()
        self.assertIn(f"```json\n{text}```", (ROOT / "docs/themes.md").read_text())

    def test_evidence_review_workflow_is_opt_in_and_indexed(self):
        """Offer a read-only capability workflow without selecting the area."""
        path = CAPABILITY / "workflows/review-evidence.yaml"
        self.assertTrue(path.is_file(), "Missing opt-in evidence-review workflow")
        text = path.read_text()
        self.assertIn("apiVersion: maestro.workflow/v1\n", text)
        self.assertIn("kind: Workflow\nname: review-evidence\n", text)
        self.assertEqual(re.findall(r"^  - agent: (.+)$", text, re.M), ["evidence-reviewer"])
        tasks = re.findall(r"^    task: (.+)$", text, re.M)
        self.assertEqual(len(tasks), 1)
        self.assertIn("$@", tasks[0])
        self.assertNotIn("{previous}", tasks[0])
        relative = path.relative_to(ROOT).as_posix()
        self.assertIn(f"[`{relative}`]({relative})", (ROOT / "README.md").read_text())
        self.assertIn(f"```yaml\n{text}```", (ROOT / "docs/workflows.md").read_text())
        settings = json.loads((MANIFEST / "settings/settings.json").read_text())
        self.assertEqual(settings["manifestAreas"], [])
        self.assertNotIn("theme", settings)
        self.assertEqual((CAPABILITY / "agents/evidence-reviewer.md").read_text()
                         .split("tools: ")[1].splitlines()[0], "read, grep, find, ls")


    def test_guards_page_quotes_match_the_guard_files(self):
        """Keep the guards page's protected-paths quote and pii excerpt equal to their sources."""
        page = (ROOT / "docs/guards.md").read_text()
        paths = (MANIFEST / "core/guards/protected-paths.yaml").read_text()
        self.assertIn(f"```yaml\n{paths}```", page)
        pii = (MANIFEST / "core/guards/pii.yaml").read_text()
        head, first, _ = re.split(r"(?m)^(?=  - id: )", pii, maxsplit=2)
        self.assertIn(f"```yaml\n{head}{first}```", page)

    def test_evidence_review_rule_applies_only_to_review_work(self):
        """Keep the review rule out of sessions that do not review evidence."""
        for path in CAPABILITY.rglob("*.instructions.md"):
            self.assertNotRegex(path.read_text(), r'(?m)^applyTo: "?\*\*"?$', path)

    def test_shipped_instruction_is_a_real_example_scoped_to_docs(self):
        """Keep one installable instruction, indexed and quoted, with no exception."""
        path = CAPABILITY / "instructions/cite-code-claims.instructions.md"
        security = MANIFEST / "capabilities/security/maestro-security/instructions/security-assessment.instructions.md"
        self.assertEqual(set(ROOT.rglob("*.instructions.md")), {path, security})
        text = path.read_text()
        self.assertTrue(text.startswith('---\napplyTo: "docs/**/*.md"\n---\n'), text)
        relative = path.relative_to(ROOT).as_posix()
        self.assertIn(f"[`{relative}`]({relative})", (ROOT / "README.md").read_text())
        for guide in ["instructions", "integrity"]:
            self.assertIn(f"```markdown\n{text}```", (ROOT / f"docs/{guide}.md").read_text())
        index = (ROOT / "docs/index.md").read_text()
        self.assertNotIn("*.instructions.md", index)
        self.assertIn("skill, instruction, prompt, theme and workflow", index)

    def test_evidence_review_rules_are_checkable_and_map_to_truthful_evidence(self):
        """Hold the role and skill to one report whose parts a reviewer can check."""
        files = [("agents/evidence-reviewer.md", "roles"),
                 ("skills/review-evidence/SKILL.md", "skills")]
        for name, guide in files:
            with self.subTest(file=name):
                text = (CAPABILITY / name).read_text()
                self.assertIn("SEC-008", text)
                for term in ["Corrected answer", "Findings", "Checks run", "Not run",
                             "path:line", "no source", "Verified failure", "Hypothesis"]:
                    self.assertIn(f"`{term}`", text)
                self.assertIn(f"```markdown\n{text}```", (ROOT / f"docs/{guide}.md").read_text())
        skill = (CAPABILITY / files[1][0]).read_text()
        rows = skill_rules(skill)
        self.assertEqual(rows[:1], [["Rule", "Required", "Checked by"]])
        self.assertEqual([row[0] for row in rows[2:]],
                         ["Cite", "Classify", "Report runs", "Report gaps",
                          "Source honesty", "Private evidence"])
        for row in rows[2:]:
            self.assertTrue(all(row), row)
        task = re.findall(r"^    task: (.+)$",
                          (CAPABILITY / "workflows/review-evidence.yaml").read_text(), re.M)[0]
        self.assertIn("report", task)

    def test_evidence_review_skill_requires_cited_corrected_answers(self):
        """Keep the standalone skill's corrected answer cited, as the role requires."""
        skill = (CAPABILITY / "skills/review-evidence/SKILL.md").read_text()
        self.assertRegex(skill, r"(?m)^\d+\. Return .*corrected answer .*cites its sources")
        cite = next(row for row in skill_rules(skill) if row[0] == "Cite")
        self.assertIn("`Corrected answer`", cite[1])
        self.assertIn("`Corrected answer`", cite[2])
        self.assertIn(f"```markdown\n{skill}```", (ROOT / "docs/skills.md").read_text())

    def test_evidence_review_rules_compare_gaps_and_results_with_observations(self):
        """Fail a report that skips a check without a reason or trusts a claimed result."""
        skill = (CAPABILITY / "skills/review-evidence/SKILL.md").read_text()
        role = (CAPABILITY / "agents/evidence-reviewer.md").read_text()
        checked = {row[0]: row[2] for row in skill_rules(skill)[2:]}
        for rule, cell in checked.items():
            self.assertNotRegex(cell, r"^The section exists\.?$", rule)
        for term in ["reason", "`None` appears only when", "claims"]:
            self.assertIn(term, checked["Report gaps"])
        required = {row[0]: row[1] for row in skill_rules(skill)[2:]}["Report gaps"]
        self.assertIn("exactly `None`", required)
        self.assertIn("exactly `None`, or a nonempty list", checked["Report gaps"])
        self.assertIn("A heading with no entries fails", checked["Report gaps"])
        for term in ["`Findings`", "`Corrected answer`", "output you observed"]:
            self.assertIn(term, checked["Source honesty"])
        compare = "Compare each result the answer reports with the output you observed"
        self.assertRegex(skill, rf"(?m)^\d+\. {compare}, and mark each mismatch\.$")
        self.assertIn(f"{compare}, and report a mismatch as a `Verified failure`.", role)

    def test_evidence_review_text_uses_maestro_voice(self):
        """Give each instruction its own line and avoid the words Maestro avoids."""
        avoided = (r"\b(blazing|revolutionary|magic|seamless|supercharge|just|simply|leverage|"
                   r"utilize|oops|awesome|robust|please note)\b")
        for name in ["agents/evidence-reviewer.md", "skills/review-evidence/SKILL.md",
                     "prompts/explain-change.md", "workflows/review-evidence.yaml"]:
            with self.subTest(file=name):
                text = (CAPABILITY / name).read_text()
                self.assertNotRegex(text.lower(), avoided)
                lines = (re.findall(r"^    task: (.+)$", text, re.M) if name.endswith(".yaml")
                         else text.split("---\n")[2].splitlines())
                for line in lines:
                    if not line.startswith("|"):
                        sentences = re.findall(r"[.!?](?:\s|$)", re.sub(r"^\d+\. ", "", line))
                        self.assertLessEqual(len(sentences), 1, line)


if __name__ == "__main__":
    unittest.main()
