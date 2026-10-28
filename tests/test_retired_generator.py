"""Check that no tracked file names the retired generator."""

import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]


class RetiredGenerator(unittest.TestCase):
    """Keep the repository's own files free of generator claims."""

    def test_no_tracked_file_names_the_generator(self):
        """Find no line that names the retired generator, in any case."""
        # The bracket keeps this file from matching its own pattern.
        result = subprocess.run(
            ["git", "grep", "-n", "-i", "-e", "rust[-]gate"],
            cwd=ROOT,
            text=True,
            capture_output=True,
            timeout=60,
        )
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual(result.stdout, "")

    def test_failed_scan_is_rejected(self):
        """Reject a Git failure even when it produces no stdout."""
        result = subprocess.CompletedProcess([], 128, stdout="", stderr="scan failed")
        with patch("subprocess.run", return_value=result), self.assertRaises(AssertionError):
            self.test_no_tracked_file_names_the_generator()


if __name__ == "__main__":
    unittest.main()
