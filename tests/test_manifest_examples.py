"""Check authored role, workflow and theme examples without running agents."""

import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CAPABILITY = ROOT / "capabilities/practice/evidence-review"


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
             ["Completed", "Files changed", "Commands run", "Notes (if any)"],
             "without polluting the main conversation"),
        ]:
            with self.subTest(role=role):
                text = (ROOT / f"core/agents/{role}.md").read_text()
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
                text = (ROOT / f"core/workflows/{name}.yaml").read_text()
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
        settings = json.loads((ROOT / "settings/settings.json").read_text())
        self.assertEqual(settings["manifestAreas"], [])
        self.assertNotIn("theme", settings)
        self.assertEqual((CAPABILITY / "agents/evidence-reviewer.md").read_text()
                         .split("tools: ")[1].splitlines()[0], "read, grep, find, ls")


if __name__ == "__main__":
    unittest.main()
