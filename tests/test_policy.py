"""Check the organization's baseline policy against its release tree and guides."""

import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "manifests"
POLICY_TEXT = (MANIFEST / "settings/policy.json").read_text()
SETTINGS_TEXT = (MANIFEST / "settings/settings.json").read_text()
POLICY = json.loads(POLICY_TEXT)
SETTINGS = json.loads(SETTINGS_TEXT)
THINKING = ["off", "minimal", "low", "medium", "high", "xhigh"]

# The named settings and actions governed by the manifest-mode contract.
LOCKABLE = {
    "enableInstallTelemetry", "images.blockImages", "defaultProvider", "defaultModel",
    "systemPrompt", "appendSystemPrompt", "manifest", "manifestAreas", "manifestAutoUpdate",
    "packages", "extensions", "skills", "prompts", "providers", "enabledModels", "transport",
    "steeringMode", "followUpMode", "enableSkillCommands", "images.autoResize",
    "holdUncheckedText", "compaction.enabled", "compaction.reserveTokens",
    "compaction.keepRecentTokens", "branchSummary.reserveTokens", "branchSummary.skipPrompt",
    "retry.enabled", "retry.maxRetries", "retry.baseDelayMs", "retry.provider.timeoutMs",
    "retry.provider.maxRetries", "retry.provider.maxRetryDelayMs", "themes", "actions",
    "contributions", "workflows", "agents", "instructions", "mcpServers", "shellPath",
    "sessionDir", "npmCommand", "showChangelog", "credentials.anthropic", "theme",
    "shellCommandPrefix", "hideThinkingBlock", "quietStartup", "collapseChangelog",
    "terminal.showImages", "terminal.imageWidthCells", "terminal.clearOnShrink",
    "terminal.showTerminalProgress", "doubleEscapeAction", "treeFilterMode", "editorPaddingX",
    "autocompleteMaxVisible", "showHardwareCursor", "markdown.codeBlockIndent",
    "warnings.anthropicExtraUsage", "multiplexer.theme.autoSwitch", "multiplexer.sidebarWidth",
    "multiplexer.sidebarMinWidth", "multiplexer.sidebarMaxWidth",
    "multiplexer.sidebarStartCollapsed", "multiplexer.sidebarCollapsedMode",
    "multiplexer.mobileWidthThreshold", "multiplexer.mouseCapture", "multiplexer.copyOnSelect",
    "multiplexer.hostCursor", "multiplexer.rightClickPassthroughModifier",
    "multiplexer.redrawOnFocusGained", "multiplexer.confirmClose", "multiplexer.promptNewTabName",
    "multiplexer.promptNewWorkspaceName", "multiplexer.paneBorders",
    "multiplexer.paneOuterBorders", "multiplexer.paneScrollbars", "multiplexer.paneGaps",
    "multiplexer.showAgentLabelsOnPaneBorders", "multiplexer.hideTabBarWhenSingleTab",
    "multiplexer.tabBarPosition", "multiplexer.tabBarRight", "multiplexer.tabBarRightSeparator",
    "multiplexer.windowTitle", "multiplexer.agentPanelSort", "multiplexer.statusIndicators",
    "multiplexer.sidebar.agents.rowGap", "multiplexer.sidebar.agents.rows",
    "multiplexer.sidebar.agents.rowsByAgent", "multiplexer.sidebar.spaces.rowGap",
    "multiplexer.sidebar.spaces.rows", "multiplexer.accent", "multiplexer.toast.delivery",
    "multiplexer.toast.delaySeconds", "multiplexer.toast.workspace.position",
    "multiplexer.toast.clipboard.enabled", "multiplexer.toast.clipboard.position",
    "multiplexer.sound.enabled", "multiplexer.sound.agents", "multiplexer.remote.manageSshConfig",
    "multiplexer.allowNested", "multiplexer.kittyGraphics",
    "multiplexer.revealHiddenCursorForCjkIme", "multiplexer.cjkImeAgents",
    "multiplexer.cjkImeCursorShape", "multiplexer.switchAsciiInputSourceInPrefix",
    "daemon.paneShell", "daemon.paneShellMode", "daemon.newPaneCwd",
    "daemon.resumeAgentsOnRestore", "daemon.headlessCols", "daemon.headlessRows",
    "daemon.worktreesDirectory", "daemon.scrollbackLimitBytes", "daemon.paneHistory",
    "daemon.detectionCatalogCheck", "update.versionCheck", "update.channel",
    "multiplexer.onboarding", "multiplexer.theme.name", "multiplexer.theme.darkName",
    "multiplexer.theme.lightName", "multiplexer.theme.custom", "multiplexer.mouseScrollLines",
    "multiplexer.sound.path", "multiplexer.sound.donePath", "multiplexer.sound.requestPath",
    "keybindings.app.interrupt", "keybindings.app.clear", "keybindings.app.exit",
    "keybindings.app.suspend", "keybindings.app.thinking.cycle",
    "keybindings.app.model.cycleForward", "keybindings.app.model.cycleBackward",
    "keybindings.app.model.select", "keybindings.app.tools.expand",
    "keybindings.app.thinking.toggle", "keybindings.app.session.toggleNamedFilter",
    "keybindings.app.editor.external", "keybindings.app.message.followUp",
    "keybindings.app.message.dequeue", "keybindings.app.clipboard.pasteImage",
    "keybindings.app.session.new", "keybindings.app.session.tree", "keybindings.app.session.fork",
    "keybindings.app.session.resume", "keybindings.app.tree.foldOrUp",
    "keybindings.app.tree.unfoldOrDown", "keybindings.app.tree.editLabel",
    "keybindings.app.tree.toggleLabelTimestamp", "keybindings.app.session.togglePath",
    "keybindings.app.session.toggleSort", "keybindings.app.session.rename",
    "keybindings.app.session.delete", "keybindings.app.session.deleteNoninvasive",
    "keybindings.app.models.save", "keybindings.app.models.enableAll",
    "keybindings.app.models.clearAll", "keybindings.app.models.toggleProvider",
    "keybindings.app.models.reorderUp", "keybindings.app.models.reorderDown",
    "keybindings.app.tree.filter.default", "keybindings.app.tree.filter.noTools",
    "keybindings.app.tree.filter.userOnly", "keybindings.app.tree.filter.labeledOnly",
    "keybindings.app.tree.filter.all", "keybindings.app.tree.filter.cycleForward",
    "keybindings.app.tree.filter.cycleBackward", "keybindings.tui.editor.cursorUp",
    "keybindings.tui.editor.cursorDown", "keybindings.tui.editor.cursorLeft",
    "keybindings.tui.editor.cursorRight", "keybindings.tui.editor.cursorWordLeft",
    "keybindings.tui.editor.cursorWordRight", "keybindings.tui.editor.cursorLineStart",
    "keybindings.tui.editor.cursorLineEnd", "keybindings.tui.editor.jumpForward",
    "keybindings.tui.editor.jumpBackward", "keybindings.tui.editor.pageUp",
    "keybindings.tui.editor.pageDown", "keybindings.tui.editor.deleteCharBackward",
    "keybindings.tui.editor.deleteCharForward", "keybindings.tui.editor.deleteWordBackward",
    "keybindings.tui.editor.deleteWordForward", "keybindings.tui.editor.deleteToLineStart",
    "keybindings.tui.editor.deleteToLineEnd", "keybindings.tui.editor.yank",
    "keybindings.tui.editor.yankPop", "keybindings.tui.editor.undo",
    "keybindings.tui.input.newLine", "keybindings.tui.input.submit", "keybindings.tui.input.tab",
    "keybindings.tui.input.copy", "keybindings.tui.select.up", "keybindings.tui.select.down",
    "keybindings.tui.select.pageUp", "keybindings.tui.select.pageDown",
    "keybindings.tui.select.confirm", "keybindings.tui.select.cancel", "defaultThinkingLevel",
    "thinkingBudgets.minimal", "thinkingBudgets.low", "thinkingBudgets.medium",
    "thinkingBudgets.high"
}
NEW_LOCKS = {
    "actions", "agents", "branchSummary.reserveTokens", "branchSummary.skipPrompt",
    "compaction.enabled", "compaction.keepRecentTokens", "compaction.reserveTokens",
    "contributions", "credentials.anthropic", "daemon.detectionCatalogCheck",
    "daemon.newPaneCwd", "daemon.paneHistory", "daemon.paneShell",
    "daemon.paneShellMode", "daemon.resumeAgentsOnRestore", "daemon.worktreesDirectory",
    "enableSkillCommands", "enabledModels", "followUpMode", "images.autoResize",
    "instructions", "mcpServers", "multiplexer.remote.manageSshConfig",
    "multiplexer.allowNested", "multiplexer.tabBarRight", "npmCommand", "providers",
    "retry.baseDelayMs",
    "retry.enabled", "retry.maxRetries", "retry.provider.maxRetries",
    "retry.provider.maxRetryDelayMs", "retry.provider.timeoutMs", "sessionDir",
    "shellCommandPrefix", "shellPath", "showChangelog", "steeringMode", "themes",
    "transport", "update.channel", "update.versionCheck",
    "warnings.anthropicExtraUsage", "workflows",
}
CORE_ROLES = sorted(path.stem for path in (MANIFEST / "core/agents").glob("*.md"))


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

    def test_remaining_governed_surfaces_are_locked(self):
        """Lock behavior and local additions while leaving personal UI settings free."""
        self.assertLessEqual(NEW_LOCKS, set(POLICY.get("locked", [])))
        explicit = {
            "credentials.anthropic": ["environment", "signIn"],
            "showChangelog": True,
        }
        for key in NEW_LOCKS:
            with self.subTest(key=key):
                self.assertEqual(setting(key), explicit.get(key))
        self.assertNotIn("lastChangelogVersion", POLICY.get("locked", []))
        self.assertEqual(POLICY.get("required"), ["core/agents", "core/guards", "core/workflows"])

    def test_personal_ui_is_not_locked(self):
        """Leave presentation, workspace UI and all 72 named bindings editable."""
        personal = {
            "theme", "hideThinkingBlock", "quietStartup", "collapseChangelog",
            "doubleEscapeAction", "treeFilterMode", "editorPaddingX",
            "autocompleteMaxVisible", "showHardwareCursor", "daemon.headlessCols",
            "daemon.headlessRows", "daemon.scrollbackLimitBytes",
        } | {key for key in LOCKABLE if key.startswith(
            ("terminal.", "markdown.", "multiplexer.", "keybindings."))} - NEW_LOCKS
        self.assertEqual(sum(key.startswith("keybindings.") for key in personal), 72)
        self.assertFalse(personal & set(POLICY.get("locked", [])))
        for key in personal:
            with self.subTest(key=key):
                self.assertIsNone(setting(key))
        self.assertIn("actions", POLICY.get("locked", []))

    def test_output_holding_waits_for_guards(self):
        """Do not author or lock output holding until guards ship."""
        self.assertNotIn("holdUncheckedText", POLICY.get("locked", []))
        self.assertNotIn("holdUncheckedText", SETTINGS)

    def test_npm_command_keeps_the_engine_default(self):
        """Lock the absent command so Git installs retain --omit=dev."""
        self.assertIn("npmCommand", POLICY.get("locked", []))
        self.assertNotIn("npmCommand", SETTINGS)

    def test_core_roles_guards_and_workflows_are_required(self):
        """Require every core role, guard file and workflow, and only paths the release holds."""
        required = POLICY.get("required", [])
        self.assertTrue(required)
        for path in required:
            with self.subTest(path=path):
                self.assertTrue((MANIFEST / path).exists(), "required path is missing")
        for resource in [*(MANIFEST / "core/agents").glob("*.md"),
                         *(MANIFEST / "core/guards").glob("*.yaml"),
                         *(MANIFEST / "core/workflows").glob("*.yaml")]:
            relative = resource.relative_to(MANIFEST).as_posix()
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

    def test_every_model_a_core_role_declares_is_allowed(self):
        """Allow each model a core role declares, since its model lock fixes that choice."""
        for name in CORE_ROLES:
            frontmatter = (MANIFEST / f"core/agents/{name}.md").read_text().split("---\n")[1]
            declared = [line.split(": ", 1)[1] for line in frontmatter.splitlines()
                        if line.startswith("model: ")]
            for model in declared:
                with self.subTest(role=name):
                    self.assertIn(model, POLICY.get("models", []))

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
