"""Check the organization's baseline policy against its release tree and guides."""

import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
POLICY_TEXT = (ROOT / "settings/policy.json").read_text()
SETTINGS_TEXT = (ROOT / "settings/settings.json").read_text()
POLICY = json.loads(POLICY_TEXT)
SETTINGS = json.loads(SETTINGS_TEXT)
THINKING = ["off", "minimal", "low", "medium", "high", "xhigh"]

# The keys the manifest-mode specification lets a manifest lock, plus its lock names.
LOCKABLE = {
    "systemPrompt", "appendSystemPrompt", "providers",
    "defaultProvider", "defaultModel", "defaultThinkingLevel", "enabledModels",
    "thinkingBudgets.minimal", "thinkingBudgets.low", "thinkingBudgets.medium",
    "thinkingBudgets.high", "transport", "steeringMode", "followUpMode",
    "enableSkillCommands", "images.autoResize", "images.blockImages",
    "enableInstallTelemetry", "holdUncheckedText",
    "compaction.enabled", "compaction.reserveTokens", "compaction.keepRecentTokens",
    "branchSummary.reserveTokens", "branchSummary.skipPrompt",
    "retry.enabled", "retry.maxRetries", "retry.baseDelayMs",
    "retry.provider.timeoutMs", "retry.provider.maxRetries",
    "retry.provider.maxRetryDelayMs",
    "packages", "extensions", "skills", "prompts", "actions", "contributions",
    "manifest", "manifestAreas", "manifestAutoUpdate",
    "multiplexer.tabBarRight", "multiplexer.allowNested",
    "multiplexer.remote.manageSshConfig",
    "daemon.resumeAgentsOnRestore", "daemon.detectionCatalogCheck",
    "update.versionCheck", "update.channel",
}
CORE_ROLES = sorted(path.stem for path in (ROOT / "core/agents").glob("*.md"))


def setting(path):
    """Return the manifest value at a dotted settings path, or None when unset."""
    value = SETTINGS
    for part in path.split("."):
        if not isinstance(value, dict) or part not in value:
            return None
        value = value[part]
    return value


class BaselinePolicy(unittest.TestCase):
    """Keep the baseline locks, bounds and required paths as the owner approved them."""

    def test_privacy_and_supply_chain_are_locked(self):
        """Lock telemetry off and image blocking on, with the values in settings."""
        self.assertLessEqual({"enableInstallTelemetry", "images.blockImages"},
                             set(POLICY.get("locked", [])))
        self.assertIs(setting("enableInstallTelemetry"), False)
        self.assertIs(setting("images.blockImages"), True)

    def test_models_are_the_three_core_role_models(self):
        """Allow only the models the core roles use and lock the default among them."""
        self.assertEqual(sorted(POLICY.get("models", [])),
                         ["claude-haiku-5-5", "claude-opus-5-5", "claude-sonnet-5-5"])
        self.assertLessEqual({"defaultProvider", "defaultModel"},
                             set(POLICY.get("locked", [])))
        self.assertEqual(setting("defaultProvider"), "anthropic")
        self.assertIn(setting("defaultModel"), POLICY.get("models", []))

    def test_thinking_is_bounded_and_the_default_sits_inside(self):
        """Cap the thinking level and each budget at the engine default."""
        bounds = POLICY.get("bounds", {})
        self.assertEqual(
            {key: bound for key, bound in bounds.items() if key.startswith("thinkingBudgets.")},
            {"thinkingBudgets.minimal": {"max": 1024}, "thinkingBudgets.low": {"max": 2048},
             "thinkingBudgets.medium": {"max": 8192}, "thinkingBudgets.high": {"max": 16384}})
        ceiling = bounds.get("defaultThinkingLevel", {}).get("max")
        level = setting("defaultThinkingLevel")
        self.assertIn(ceiling, THINKING)
        self.assertIn(level, THINKING)
        self.assertLessEqual(THINKING.index(str(level)), THINKING.index(str(ceiling)))

    def test_prompts_manifest_selection_and_local_additions_are_locked(self):
        """Fix the prompts, the manifest choice and every lockable local addition."""
        self.assertLessEqual(
            {"systemPrompt", "appendSystemPrompt", "manifest", "manifestAreas",
             "manifestAutoUpdate", "packages", "extensions", "skills", "prompts"},
            set(POLICY.get("locked", [])))
        self.assertEqual(setting("manifestAreas"), [])
        self.assertIs(setting("manifestAutoUpdate"), False)
        for key in ["packages", "extensions", "skills", "prompts"]:
            with self.subTest(key=key):
                self.assertNotIn(key, SETTINGS, "area folders supply these resources")

    def test_core_roles_and_workflows_are_required(self):
        """Require every core role and workflow, and only paths the release holds."""
        required = POLICY.get("required", [])
        self.assertTrue(required)
        for path in required:
            with self.subTest(path=path):
                self.assertTrue((ROOT / path).exists(), "required path is missing")
        for resource in [*(ROOT / "core/agents").glob("*.md"),
                         *(ROOT / "core/workflows").glob("*.yaml")]:
            relative = resource.relative_to(ROOT).as_posix()
            with self.subTest(resource=relative):
                self.assertTrue(any(relative == path or relative.startswith(f"{path}/")
                                    for path in required))

    def test_every_core_role_locks_model_thinking_and_tools(self):
        """Fix what each core role runs on and may use."""
        roles = POLICY.get("roles", {})
        self.assertEqual(sorted(roles), CORE_ROLES)
        for name, role in roles.items():
            with self.subTest(role=name):
                self.assertEqual(sorted(role["locked"]), ["model", "thinking", "tools"])

    def test_policy_uses_only_the_lockable_surface(self):
        """Refuse a lock or bound the specification does not allow."""
        self.assertFalse(set(POLICY.get("locked", [])) - LOCKABLE)
        self.assertFalse(set(POLICY.get("bounds", {})) - LOCKABLE)
        self.assertLessEqual(set(POLICY), {
            "locked", "bounds", "models", "required", "roles", "credentialHeaders",
            "minimumRelease", "updateCheckMinutes"})
        self.assertTrue(POLICY.get("locked"), "the baseline locks nothing")

    def test_policy_guide_lists_the_baseline(self):
        """Quote both files and name every lock, model and required path in the guide."""
        guide = (ROOT / "docs/policy.md").read_text()
        self.assertIn(f"```json\n{POLICY_TEXT}```", guide)
        self.assertIn(f"```json\n{SETTINGS_TEXT}```", (ROOT / "docs/settings.md").read_text())
        self.assertIn("## The baseline", guide)
        baseline = guide.split("## The baseline")[1].split("\n## ")[0]
        names = [*POLICY.get("locked", []), *POLICY.get("models", []),
                 *POLICY.get("required", []), *POLICY.get("bounds", {}),
                 *POLICY.get("roles", {})]
        self.assertTrue(names)
        for name in names:
            with self.subTest(name=name):
                self.assertIn(f"`{name}`", baseline)


if __name__ == "__main__":
    unittest.main()
