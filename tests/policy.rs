//! Checks the organization's baseline policy against its release tree and guides.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value, json};

use crate::support::{Check, Failures, Report, json, strings};

/// The thinking levels, lowest first.
const THINKING: [&str; 6] = ["off", "minimal", "low", "medium", "high", "xhigh"];
/// The settings the baseline leaves to each person.
const PERSONAL: [&str; 12] = [
    "theme",
    "hideThinkingBlock",
    "quietStartup",
    "collapseChangelog",
    "doubleEscapeAction",
    "treeFilterMode",
    "editorPaddingX",
    "autocompleteMaxVisible",
    "showHardwareCursor",
    "daemon.headlessCols",
    "daemon.headlessRows",
    "daemon.scrollbackLimitBytes",
];
/// The top-level keys a policy file may hold.
const POLICY_KEYS: [&str; 8] = [
    "locked",
    "bounds",
    "models",
    "required",
    "roles",
    "credentialHeaders",
    "minimumRelease",
    "updateCheckMinutes",
];

/// The named settings and actions the manifest-mode contract governs.
const LOCKABLE: &str = "\
    enableInstallTelemetry images.blockImages defaultProvider defaultModel systemPrompt \
    appendSystemPrompt manifest manifestAreas manifestAutoUpdate packages extensions skills \
    prompts providers enabledModels transport steeringMode followUpMode enableSkillCommands \
    images.autoResize holdUncheckedText compaction.enabled compaction.reserveTokens \
    compaction.keepRecentTokens branchSummary.reserveTokens branchSummary.skipPrompt \
    retry.enabled retry.maxRetries retry.baseDelayMs retry.provider.timeoutMs \
    retry.provider.maxRetries retry.provider.maxRetryDelayMs themes actions contributions \
    workflows agents instructions mcpServers shellPath sessionDir npmCommand showChangelog \
    credentials.anthropic theme shellCommandPrefix hideThinkingBlock quietStartup \
    collapseChangelog terminal.showImages terminal.imageWidthCells terminal.clearOnShrink \
    terminal.showTerminalProgress doubleEscapeAction treeFilterMode editorPaddingX \
    autocompleteMaxVisible showHardwareCursor markdown.codeBlockIndent \
    warnings.anthropicExtraUsage multiplexer.theme.autoSwitch multiplexer.sidebarWidth \
    multiplexer.sidebarMinWidth multiplexer.sidebarMaxWidth \
    multiplexer.sidebarStartCollapsed multiplexer.sidebarCollapsedMode \
    multiplexer.mobileWidthThreshold multiplexer.mouseCapture multiplexer.copyOnSelect \
    multiplexer.hostCursor multiplexer.rightClickPassthroughModifier \
    multiplexer.redrawOnFocusGained multiplexer.confirmClose multiplexer.promptNewTabName \
    multiplexer.promptNewWorkspaceName multiplexer.paneBorders multiplexer.paneOuterBorders \
    multiplexer.paneScrollbars multiplexer.paneGaps multiplexer.showAgentLabelsOnPaneBorders \
    multiplexer.hideTabBarWhenSingleTab multiplexer.tabBarPosition multiplexer.tabBarRight \
    multiplexer.tabBarRightSeparator multiplexer.windowTitle multiplexer.agentPanelSort \
    multiplexer.statusIndicators multiplexer.sidebar.agents.rowGap \
    multiplexer.sidebar.agents.rows multiplexer.sidebar.agents.rowsByAgent \
    multiplexer.sidebar.spaces.rowGap multiplexer.sidebar.spaces.rows multiplexer.accent \
    multiplexer.toast.delivery multiplexer.toast.delaySeconds \
    multiplexer.toast.workspace.position multiplexer.toast.clipboard.enabled \
    multiplexer.toast.clipboard.position multiplexer.sound.enabled multiplexer.sound.agents \
    multiplexer.remote.manageSshConfig multiplexer.allowNested multiplexer.kittyGraphics \
    multiplexer.revealHiddenCursorForCjkIme multiplexer.cjkImeAgents \
    multiplexer.cjkImeCursorShape multiplexer.switchAsciiInputSourceInPrefix \
    daemon.paneShell daemon.paneShellMode daemon.newPaneCwd daemon.resumeAgentsOnRestore \
    daemon.headlessCols daemon.headlessRows daemon.worktreesDirectory \
    daemon.scrollbackLimitBytes daemon.paneHistory daemon.detectionCatalogCheck \
    update.versionCheck update.channel multiplexer.onboarding multiplexer.theme.name \
    multiplexer.theme.darkName multiplexer.theme.lightName multiplexer.theme.custom \
    multiplexer.mouseScrollLines multiplexer.sound.path multiplexer.sound.donePath \
    multiplexer.sound.requestPath keybindings.app.interrupt keybindings.app.clear \
    keybindings.app.exit keybindings.app.suspend keybindings.app.thinking.cycle \
    keybindings.app.model.cycleForward keybindings.app.model.cycleBackward \
    keybindings.app.model.select keybindings.app.tools.expand \
    keybindings.app.thinking.toggle keybindings.app.session.toggleNamedFilter \
    keybindings.app.editor.external keybindings.app.message.followUp \
    keybindings.app.message.dequeue keybindings.app.clipboard.pasteImage \
    keybindings.app.session.new keybindings.app.session.tree keybindings.app.session.fork \
    keybindings.app.session.resume keybindings.app.tree.foldOrUp \
    keybindings.app.tree.unfoldOrDown keybindings.app.tree.editLabel \
    keybindings.app.tree.toggleLabelTimestamp keybindings.app.session.togglePath \
    keybindings.app.session.toggleSort keybindings.app.session.rename \
    keybindings.app.session.delete keybindings.app.session.deleteNoninvasive \
    keybindings.app.models.save keybindings.app.models.enableAll \
    keybindings.app.models.clearAll keybindings.app.models.toggleProvider \
    keybindings.app.models.reorderUp keybindings.app.models.reorderDown \
    keybindings.app.tree.filter.default keybindings.app.tree.filter.noTools \
    keybindings.app.tree.filter.userOnly keybindings.app.tree.filter.labeledOnly \
    keybindings.app.tree.filter.all keybindings.app.tree.filter.cycleForward \
    keybindings.app.tree.filter.cycleBackward keybindings.tui.editor.cursorUp \
    keybindings.tui.editor.cursorDown keybindings.tui.editor.cursorLeft \
    keybindings.tui.editor.cursorRight keybindings.tui.editor.cursorWordLeft \
    keybindings.tui.editor.cursorWordRight keybindings.tui.editor.cursorLineStart \
    keybindings.tui.editor.cursorLineEnd keybindings.tui.editor.jumpForward \
    keybindings.tui.editor.jumpBackward keybindings.tui.editor.pageUp \
    keybindings.tui.editor.pageDown keybindings.tui.editor.deleteCharBackward \
    keybindings.tui.editor.deleteCharForward keybindings.tui.editor.deleteWordBackward \
    keybindings.tui.editor.deleteWordForward keybindings.tui.editor.deleteToLineStart \
    keybindings.tui.editor.deleteToLineEnd keybindings.tui.editor.yank \
    keybindings.tui.editor.yankPop keybindings.tui.editor.undo keybindings.tui.input.newLine \
    keybindings.tui.input.submit keybindings.tui.input.tab keybindings.tui.input.copy \
    keybindings.tui.select.up keybindings.tui.select.down keybindings.tui.select.pageUp \
    keybindings.tui.select.pageDown keybindings.tui.select.confirm \
    keybindings.tui.select.cancel defaultThinkingLevel thinkingBudgets.minimal \
    thinkingBudgets.low thinkingBudgets.medium thinkingBudgets.high";

/// The locks added after the first baseline, each with the value settings give it.
const NEW_LOCKS: &str = "\
    actions agents branchSummary.reserveTokens branchSummary.skipPrompt compaction.enabled \
    compaction.keepRecentTokens compaction.reserveTokens contributions credentials.anthropic \
    daemon.detectionCatalogCheck daemon.newPaneCwd daemon.paneHistory daemon.paneShell \
    daemon.paneShellMode daemon.resumeAgentsOnRestore daemon.worktreesDirectory \
    enableSkillCommands enabledModels followUpMode images.autoResize instructions mcpServers \
    multiplexer.remote.manageSshConfig multiplexer.allowNested multiplexer.tabBarRight \
    npmCommand providers retry.baseDelayMs retry.enabled retry.maxRetries \
    retry.provider.maxRetries retry.provider.maxRetryDelayMs retry.provider.timeoutMs \
    sessionDir shellCommandPrefix shellPath showChangelog steeringMode themes transport \
    update.channel update.versionCheck warnings.anthropicExtraUsage workflows";

/// The baseline policy and settings files, parsed.
pub struct Baseline {
    /// The parsed `policy.json`.
    pub policy: Value,
    /// The parsed `settings.json`.
    pub settings: Value,
}

impl Baseline {
    /// Reads both files from the manifest root.
    pub fn load() -> Result<Self, Report> {
        Ok(Self {
            policy: json("manifests/settings/policy.json")?,
            settings: json("manifests/settings/settings.json")?,
        })
    }

    /// Returns the policy's `locked` keys.
    pub fn locked(&self) -> Result<BTreeSet<&str>, Report> {
        Ok(strings(&self.policy, "locked")?.into_iter().collect())
    }

    /// Returns the policy's `roles` object.
    pub fn roles(&self) -> Map<String, Value> {
        self.policy
            .get("roles")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns the policy's `bounds` object.
    pub fn bounds(&self) -> Map<String, Value> {
        self.policy
            .get("bounds")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns the settings value at the dotted `path`, or `None` when unset.
    pub fn setting(&self, path: &str) -> Option<&Value> {
        let mut value = &self.settings;
        for part in path.split('.') {
            value = value.as_object()?.get(part)?;
        }
        Some(value).filter(|found| !found.is_null())
    }

    /// Records each of `keys` that the policy does not lock.
    fn require_locked(&self, failures: &mut Failures, keys: &[&str]) -> Check {
        let locked = self.locked()?;
        for key in keys {
            failures.require(locked.contains(key), key, "is not locked");
        }
        Ok(())
    }
}

/// Splits a table of names into a set.
fn names(table: &'static str) -> BTreeSet<&'static str> {
    table.split_whitespace().collect()
}

/// Returns the settings that stay personal, which the baseline neither locks nor sets.
fn personal_settings() -> BTreeSet<&'static str> {
    let new_locks = names(NEW_LOCKS);
    let prefixes = ["terminal.", "markdown.", "multiplexer.", "keybindings."];
    let surface = names(LOCKABLE).into_iter().filter(|key| {
        prefixes.iter().any(|prefix| key.starts_with(prefix)) && !new_locks.contains(key)
    });
    PERSONAL.into_iter().chain(surface).collect()
}

/// Locks telemetry off and image blocking on, with the values in settings.
#[test]
fn privacy_and_supply_chain_are_locked() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;

    baseline.require_locked(
        &mut failures,
        &["enableInstallTelemetry", "images.blockImages"],
    )?;
    failures.require_eq(
        "enableInstallTelemetry",
        &baseline.setting("enableInstallTelemetry"),
        &Some(&json!(false)),
    );
    failures.require_eq(
        "images.blockImages",
        &baseline.setting("images.blockImages"),
        &Some(&json!(true)),
    );

    failures.finish()
}

/// Allows only the models the core roles use and locks the default among them.
#[test]
fn models_are_the_three_core_role_models() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let mut models = strings(&baseline.policy, "models")?;
    models.sort_unstable();
    let default_model = baseline
        .setting("defaultModel")
        .and_then(Value::as_str)
        .unwrap_or_default();

    failures.require_eq(
        "models",
        &models,
        &vec!["claude-haiku-5-5", "claude-opus-5-5", "claude-sonnet-5-5"],
    );
    baseline.require_locked(&mut failures, &["defaultProvider", "defaultModel"])?;
    failures.require_eq(
        "defaultProvider",
        &baseline.setting("defaultProvider"),
        &Some(&json!("anthropic")),
    );
    failures.require(
        models.contains(&default_model),
        "defaultModel",
        "is not an allowed model",
    );

    failures.finish()
}

/// Caps the thinking level and each budget at the engine default.
#[test]
fn thinking_is_bounded_and_the_default_sits_inside() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let bounds = baseline.bounds();
    let budgets: Map<String, Value> = bounds
        .iter()
        .filter(|(key, _)| key.starts_with("thinkingBudgets."))
        .map(|(key, bound)| (key.clone(), bound.clone()))
        .collect();
    let expected = json!({
        "thinkingBudgets.minimal": {"max": 1024},
        "thinkingBudgets.low": {"max": 2048},
        "thinkingBudgets.medium": {"max": 8192},
        "thinkingBudgets.high": {"max": 16384},
    });
    let ceiling = bounds
        .get("defaultThinkingLevel")
        .and_then(|bound| bound.get("max"))
        .and_then(Value::as_str);
    let level = baseline
        .setting("defaultThinkingLevel")
        .and_then(Value::as_str);
    let index = |name: Option<&str>| THINKING.iter().position(|known| Some(*known) == name);

    failures.require_eq("thinkingBudgets", &Value::Object(budgets), &expected);
    match (index(level), index(ceiling)) {
        (Some(level), Some(ceiling)) => failures.require(
            level <= ceiling,
            "defaultThinkingLevel",
            "is above its ceiling",
        ),
        _ => failures.require(
            false,
            "defaultThinkingLevel",
            "or its ceiling is not a thinking level",
        ),
    }

    failures.finish()
}

/// Fixes the prompts, the manifest choice and every lockable local addition.
#[test]
fn prompts_manifest_selection_and_local_additions_are_locked() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;

    baseline.require_locked(
        &mut failures,
        &[
            "systemPrompt",
            "appendSystemPrompt",
            "manifest",
            "manifestAreas",
            "manifestAutoUpdate",
            "packages",
            "extensions",
            "skills",
            "prompts",
        ],
    )?;
    failures.require_eq(
        "manifestAreas",
        &baseline.setting("manifestAreas"),
        &Some(&json!([])),
    );
    failures.require_eq(
        "manifestAutoUpdate",
        &baseline.setting("manifestAutoUpdate"),
        &Some(&json!(false)),
    );
    for key in ["packages", "extensions", "skills", "prompts"] {
        failures.require(
            baseline.settings.get(key).is_none(),
            key,
            "is set, but area folders supply these resources",
        );
    }

    failures.finish()
}

/// Locks behavior and local additions while leaving personal UI settings free.
#[test]
fn remaining_governed_surfaces_are_locked() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let explicit = BTreeMap::from([
        ("credentials.anthropic", json!(["environment", "signIn"])),
        ("showChangelog", json!(true)),
    ]);

    baseline.require_locked(
        &mut failures,
        &names(NEW_LOCKS).into_iter().collect::<Vec<_>>(),
    )?;
    for key in names(NEW_LOCKS) {
        failures.require_eq(key, &baseline.setting(key), &explicit.get(key));
    }
    failures.require(
        !baseline.locked()?.contains("lastChangelogVersion"),
        "lastChangelogVersion",
        "is locked",
    );
    failures.require_eq(
        "required",
        &strings(&baseline.policy, "required")?,
        &vec!["core/agents", "core/workflows"],
    );

    failures.finish()
}

/// Leaves presentation, workspace UI and all 72 named bindings editable.
#[test]
fn personal_ui_is_not_locked() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let personal = personal_settings();
    let locked = baseline.locked()?;

    failures.require_eq(
        "keybindings",
        &personal
            .iter()
            .filter(|key| key.starts_with("keybindings."))
            .count(),
        &72,
    );
    for key in &personal {
        failures.require(!locked.contains(key), key, "is locked");
        failures.require(baseline.setting(key).is_none(), key, "is set");
    }
    failures.require(locked.contains("actions"), "actions", "is not locked");

    failures.finish()
}

/// Does not author or lock output holding until guards ship.
#[test]
fn output_holding_waits_for_guards() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;

    failures.require(
        !baseline.locked()?.contains("holdUncheckedText"),
        "holdUncheckedText",
        "is locked",
    );
    failures.require(
        baseline.settings.get("holdUncheckedText").is_none(),
        "holdUncheckedText",
        "is set",
    );

    failures.finish()
}

/// Locks the absent command so Git installs retain `--omit=dev`.
#[test]
fn npm_command_keeps_the_engine_default() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;

    baseline.require_locked(&mut failures, &["npmCommand"])?;
    failures.require(
        baseline.settings.get("npmCommand").is_none(),
        "npmCommand",
        "is set",
    );

    failures.finish()
}

/// Refuses a lock or bound the specification does not allow.
#[test]
fn policy_uses_only_the_lockable_surface() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let lockable = names(LOCKABLE);
    let top_level = baseline.policy.as_object().cloned().unwrap_or_default();

    for key in baseline.locked()?.difference(&lockable) {
        failures.require(false, key, "is locked but not lockable");
    }
    for key in baseline
        .bounds()
        .keys()
        .filter(|key| !lockable.contains(key.as_str()))
    {
        failures.require(false, key, "is bounded but not lockable");
    }
    for key in top_level
        .keys()
        .filter(|key| !POLICY_KEYS.contains(&key.as_str()))
    {
        failures.require(false, key, "is not a policy key");
    }
    failures.require(
        !baseline.locked()?.is_empty(),
        "locked",
        "the baseline locks nothing",
    );

    failures.finish()
}
