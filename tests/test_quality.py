"""Exercise the contributor commands in disposable Git indexes."""

import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class QualityCommands(unittest.TestCase):
    """Check content through the recipe that `check` runs for contributors and CI."""

    def setUp(self):
        """Copy tracked inputs without changing the contributor's index."""
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        paths = subprocess.check_output(
            ["git", "ls-files", "-z"], cwd=ROOT
        ).decode().split("\0")
        for name in filter(None, paths):
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        self.run_command("git", "init", "-q")
        self.run_command("git", "add", ".")

    def run_command(self, *command):
        """Return captured output and refuse commands that hang."""
        return subprocess.run(
            command, cwd=self.root, text=True, capture_output=True, timeout=120,
            env={**os.environ, "PREK_ALLOW_NO_CONFIG": "0"},
        )

    def test_check_runs_the_content_checks_the_rust_gate_and_the_tests_without_recursion(self):
        """Run Rust formatting, lints and tests in `check`, which CI runs, but not in `content`."""
        check = self.run_command("just", "--dry-run", "check")
        self.assertEqual(check.returncode, 0, check.stderr)
        for command in [
            "prek run --all-files --stage manual",
            "cargo fmt --check",
            "cargo clippy --all-targets --locked -- -D warnings",
            "cargo test --locked",
            "unittest discover",
        ]:
            with self.subTest(command=command):
                self.assertIn(command, check.stderr)
        content = self.run_command("just", "--dry-run", "content")
        self.assertEqual(content.returncode, 0, content.stderr)
        self.assertNotIn("cargo", content.stderr)
        self.assertNotIn("unittest", content.stderr)

    def test_check(self):
        """Accept the checkout and reject malformed structured content."""
        result = self.run_command("just", "content")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        for suffix, content, hook in [
            ("yaml", "value: [\n", "check-yaml"),
            ("json", '{"value":\n', "check-json"),
            ("toml", "value = [\n", "check-toml"),
        ]:
            with self.subTest(suffix=suffix):
                path = self.root / f"broken.{suffix}"
                path.write_text(content)
                self.run_command("git", "add", path.name)
                result = self.run_command("just", "content")
                self.assertNotEqual(result.returncode, 0, result.stdout)
                self.assertIn(f"- hook id: {hook}", result.stdout + result.stderr)
                self.run_command("git", "rm", "--cached", path.name)
                path.unlink()

    def test_pre_commit(self):
        """Check staged files without rejecting unrelated untracked content."""
        path = self.root / "broken.yaml"
        path.write_text("value: [\n")
        result = self.run_command("just", "pre-commit")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.run_command("git", "add", path.name)
        result = self.run_command("prek", "run", "--stage", "pre-commit")
        self.assertNotEqual(result.returncode, 0, result.stdout)
        self.assertIn("- hook id: check-yaml", result.stdout + result.stderr)
        path.write_text("value: []\n")
        self.run_command("git", "add", path.name)
        result = self.run_command("prek", "run", "--stage", "pre-commit")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main()
