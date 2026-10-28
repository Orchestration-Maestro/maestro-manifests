"""Check the opt-in security content without running an assessment."""

import json
import unittest
from pathlib import Path

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


if __name__ == "__main__":
    unittest.main()
