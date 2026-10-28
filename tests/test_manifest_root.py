"""Check that the manifest root holds every manifest artifact and nothing else."""

import json
import re
import unittest
from pathlib import Path

from test_instructions import tracked_files

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "manifests"
AREAS = ["settings", "core", "standards", "languages", "capabilities"]
COMMON_FILES = ["CHANGELOG.md", "package.json"]
GUIDES = ["README.md", "CONTEXT.md", "AGENTS.md", ".github/copilot-instructions.md",
          *(path.relative_to(ROOT).as_posix() for path in sorted((ROOT / "docs").rglob("*.md")))]


def under_manifest_root():
    """Return the tracked paths inside the manifest root, relative to the repository."""
    return [name for name in tracked_files() if name.startswith("manifests/")]


class ManifestRoot(unittest.TestCase):
    """Keep manifest content under `manifests/` and repository tooling outside it."""

    def test_no_artifact_remains_outside_the_manifest_root(self):
        """Find no area folder, changelog or package record beside the tooling."""
        outside = [name for name in tracked_files()
                   if name.split("/")[0] in [*AREAS, *COMMON_FILES]]
        self.assertEqual(outside, [])
        for name in [*COMMON_FILES, "settings/settings.json", "settings/policy.json",
                     "core/SYSTEM.md"]:
            with self.subTest(name=name):
                self.assertTrue((MANIFEST / name).is_file())

    def test_nothing_in_the_manifest_root_is_repository_tooling(self):
        """Hold the root to the common files and the area folders."""
        names = {name.split("/")[1] for name in under_manifest_root()}
        self.assertTrue(names)
        self.assertLessEqual(names, {*AREAS, *COMMON_FILES})

    def test_codeowners_names_each_group_and_capability_under_the_manifest_root(self):
        """Give every group folder and capability folder its owner, by repository path."""
        lines = (ROOT / ".github/CODEOWNERS").read_text().splitlines()
        patterns = [line.split()[0] for line in lines if line.strip() and line[0] != "#"]
        self.assertEqual([p for p in patterns if p != "*" and not p.startswith("/manifests/")], [])
        folders = [*MANIFEST.glob("capabilities/*"), *MANIFEST.glob("capabilities/*/*")]
        self.assertTrue(folders)
        for folder in folders:
            with self.subTest(folder=folder.name):
                self.assertIn(f"/{folder.relative_to(ROOT).as_posix()}/", patterns)

    def test_release_please_writes_the_changelog_in_the_manifest_root(self):
        """Point the release at the changelog Maestro reads, and quote that in the guide."""
        text = (ROOT / ".github/release-please/config.json").read_text()
        changelog = json.loads(text)["packages"]["."]["changelog-path"]
        self.assertEqual(changelog, "manifests/CHANGELOG.md")
        self.assertTrue((ROOT / changelog).is_file())
        guide = (ROOT / "docs/releases-and-updates.md").read_text()
        self.assertIn(f"```json\n{text}```", guide)

    def test_content_check_skips_the_generated_changelog_at_its_new_path(self):
        """Exclude only the manifest root's changelog from the Markdown check."""
        config = (ROOT / ".pre-commit-config.yaml").read_text()
        (exclude,) = re.findall(r"id: rumdl\n.*?exclude: '([^']+)'", config, re.S)
        self.assertRegex("manifests/CHANGELOG.md", exclude)
        self.assertNotRegex("README.md", exclude)
        self.assertNotRegex("docs/releases-and-updates.md", exclude)

    def test_readme_index_links_every_manifest_file(self):
        """Index each file in the root by the path that holds it."""
        readme = (ROOT / "README.md").read_text()
        files = under_manifest_root()
        self.assertTrue(files)
        for name in files:
            with self.subTest(name=name):
                self.assertIn(f"[`{name}`]({name})", readme)

    def test_documentation_links_resolve_into_the_manifest_root(self):
        """Resolve every link and caption to a file, and none to a moved location."""
        for guide in GUIDES:
            path = ROOT / guide
            text = re.sub(r"```.*?```", "", path.read_text(), flags=re.S)
            for target in re.findall(r"\]\(([^)\s#]+)(?:#[^)\s]*)?\)", text):
                if re.match(r"[a-z][a-z0-9+.-]*:", target):
                    continue
                with self.subTest(guide=guide, target=target):
                    resolved = (path.parent / target).resolve()
                    self.assertTrue(resolved.exists())
                    relative = resolved.relative_to(ROOT).as_posix()
                    self.assertNotIn(relative.split("/")[0], [*AREAS, *COMMON_FILES])
            for caption, target in re.findall(r"(?m)^From \[`([^`]+)`\]\(([^)]+)\):$", text):
                with self.subTest(guide=guide, caption=caption):
                    self.assertEqual((path.parent / target).resolve(), ROOT / caption)


if __name__ == "__main__":
    unittest.main()
