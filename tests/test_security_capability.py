"""Check the opt-in security content without running an assessment."""

import json
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
CAPABILITY = ROOT / "manifests/capabilities/security/maestro-security"
ROLE_TOOLS = {
    "security": ["security_doctor", "security_route", "security_assess", "security_workspace"],
    "security-probe": ["security_route", "security_assess"],
    "security-proof": ["security_validate", "security_evidence", "security_disclose"],
    "security-rebuild": ["security_workspace", "security_disclose"],
    "security-cyber": ["security_assess", "security_report_page"],
}


class SecurityCapability(unittest.TestCase):
    """Preserve resource discovery and the package-owned tool ceilings."""

    def test_native_layout_and_role_tool_ceilings(self):
        """Discover five native roles, an offered instruction and one skill."""
        expected = {f"agents/{name}.md" for name in ROLE_TOOLS} | {
            "package.json", "instructions/security-assessment.instructions.md",
            "skills/security-assessment/SKILL.md",
        }
        self.assertEqual({p.relative_to(CAPABILITY).as_posix()
                          for p in CAPABILITY.rglob("*") if p.is_file()}, expected)
        package = json.loads((CAPABILITY / "package.json").read_text())
        self.assertEqual(package["name"], "maestro-security")
        self.assertEqual(package["maestro"], {"skills": ["./skills/security-assessment"]})
        self.assertNotIn("version", package)
        for name, tools in ROLE_TOOLS.items():
            with self.subTest(role=name):
                text = (CAPABILITY / f"agents/{name}.md").read_text()
                frontmatter, body = text.split("---\n")[1:3]
                self.assertIn(f"name: {name}\n", frontmatter)
                self.assertRegex(frontmatter, r"(?m)^description: .+$")
                self.assertIn("tools: " + ", ".join(
                    "maestro-security/" + tool for tool in tools) + "\n", frontmatter)
                self.assertTrue(body.strip())
        instruction = (CAPABILITY / "instructions/security-assessment.instructions.md").read_text()
        self.assertTrue(instruction.startswith('---\napplyTo: "**"\n---\n'))
        skill = (CAPABILITY / "skills/security-assessment/SKILL.md").read_text()
        self.assertRegex(skill, r"\A---\nname: security-assessment\ndescription: [^\n]+\n---\n")
        settings = json.loads((ROOT / "manifests/settings/settings.json").read_text())
        self.assertEqual(settings["manifestAreas"], [])

    def test_assessment_procedure_keeps_lifecycle_and_evidence_gates(self):
        """Keep explicit initiation, all seven phases and evidence-led delivery."""
        skill = (CAPABILITY / "skills/security-assessment/SKILL.md").read_text()
        for requirement in ["/assess", "wasm extension", "MCP stdio", "one target",
                            "recon, target-model, hypotheses, verify, deepen, threat, report",
                            "Confirmed", "NeedsValidation", "Rejected", "SARIF",
                            "immutable", "counterevidence", "blockers", "validation plan",
                            "SEC-002", "SEC-005", "SEC-008", "SEC-009"]:
            self.assertIn(requirement, skill)
        for tool in ["security_doctor", "security_route", "security_assess",
                     "security_validate", "security_evidence", "security_report_page",
                     "security_disclose"]:
            self.assertIn(f"`{tool}`", skill)
        self.assertIn("unhealthy", skill)
        self.assertIn("priorities and guidance only", skill)
        self.assertIn("target, evidence and verdict rules stay fixed", skill)
        instruction = (CAPABILITY / "instructions/security-assessment.instructions.md").read_text()
        for term in ["SEC-002", "SEC-005", "SEC-008", "Check:", "/assess",
                     "Confirmed", "NeedsValidation", "Rejected"]:
            self.assertIn(term, instruction)
        for path in CAPABILITY.rglob("*.md"):
            text = path.read_text()
            self.assertNotIn("/maestro-security", text)
            self.assertIn("Check:", text, path)
            self.assertIn("SEC-008", text, path)
            self.assertNotIn("| SEC-", text, path)

    def test_specialist_roles_allow_wrapped_responsibilities(self):
        """Accept equivalent responsibilities split across Markdown lines."""
        read_text = Path.read_text

        def wrapped_text(path, *args, **kwargs):
            return read_text(path, *args, **kwargs).replace("entry points", "entry\npoints")

        with patch.object(Path, "read_text", wrapped_text):
            self.test_specialist_roles_keep_their_responsibilities()

    def test_specialist_roles_keep_their_responsibilities(self):
        """Keep reconnaissance separate from proof, repair and reporting."""
        contracts = {
            "security": ["one target", "/assess", "doctor", "route", "user"],
            "security-probe": ["technologies", "entry points", "trust boundaries",
                               "Do not test hypotheses", "router"],
            "security-proof": ["counterevidence", "gate", "ledger", "NeedsValidation",
                               "Rejected"],
            "security-rebuild": ["fix", "verification", "disclosure", "Do not weaken"],
            "security-cyber": ["threat", "NeedsValidation", "Rejected", "coverage",
                               "ledger"],
        }
        for name, terms in contracts.items():
            body = (CAPABILITY / f"agents/{name}.md").read_text().split("---\n", 2)[2]
            text = " ".join(body.split())
            for term in terms:
                self.assertIn(term, text, name)


if __name__ == "__main__":
    unittest.main()
