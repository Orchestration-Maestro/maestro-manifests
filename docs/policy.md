# Policy

An area's `settings/policy.json` constrains its settings and resources. All eight
members are optional. Unknown members, unknown governed keys, invalid bounds and
conflicting area constraints invalidate the manifest; a misspelled lock never
silently becomes a preference.

## Contents

1. [Example](#example)
2. [Locations](#locations)
3. [Format](#format)
4. [What the organization can lock](#what-the-organization-can-lock)
5. [The baseline](#the-baseline)
6. [Combining areas and local values](#combining-areas-and-local-values)
7. [Validation](#validation)
8. [Check a change](#check-a-change)

## Example

The shipped common policy sets the [baseline](#the-baseline). Locks take their
values from the common [settings](settings.md#example).

From [`manifests/settings/policy.json`](../manifests/settings/policy.json):

```json
{
  "locked": [
    "enableInstallTelemetry",
    "images.blockImages",
    "defaultProvider",
    "defaultModel",
    "systemPrompt",
    "appendSystemPrompt",
    "manifest",
    "manifestAreas",
    "manifestAutoUpdate",
    "packages",
    "extensions",
    "skills",
    "prompts",
    "providers",
    "enabledModels",
    "transport",
    "steeringMode",
    "followUpMode",
    "enableSkillCommands",
    "images.autoResize",
    "holdUncheckedText",
    "compaction.enabled",
    "compaction.reserveTokens",
    "compaction.keepRecentTokens",
    "branchSummary.reserveTokens",
    "branchSummary.skipPrompt",
    "retry.enabled",
    "retry.maxRetries",
    "retry.baseDelayMs",
    "retry.provider.timeoutMs",
    "retry.provider.maxRetries",
    "retry.provider.maxRetryDelayMs",
    "themes",
    "actions",
    "contributions",
    "workflows",
    "agents",
    "instructions",
    "mcpServers",
    "shellPath",
    "sessionDir",
    "npmCommand",
    "showChangelog",
    "credentials.anthropic",
    "theme",
    "shellCommandPrefix",
    "hideThinkingBlock",
    "quietStartup",
    "collapseChangelog",
    "terminal.showImages",
    "terminal.imageWidthCells",
    "terminal.clearOnShrink",
    "terminal.showTerminalProgress",
    "doubleEscapeAction",
    "treeFilterMode",
    "editorPaddingX",
    "autocompleteMaxVisible",
    "showHardwareCursor",
    "markdown.codeBlockIndent",
    "warnings.anthropicExtraUsage",
    "multiplexer.theme.autoSwitch",
    "multiplexer.sidebarWidth",
    "multiplexer.sidebarMinWidth",
    "multiplexer.sidebarMaxWidth",
    "multiplexer.sidebarStartCollapsed",
    "multiplexer.sidebarCollapsedMode",
    "multiplexer.mobileWidthThreshold",
    "multiplexer.mouseCapture",
    "multiplexer.copyOnSelect",
    "multiplexer.hostCursor",
    "multiplexer.rightClickPassthroughModifier",
    "multiplexer.redrawOnFocusGained",
    "multiplexer.confirmClose",
    "multiplexer.promptNewTabName",
    "multiplexer.promptNewWorkspaceName",
    "multiplexer.paneBorders",
    "multiplexer.paneOuterBorders",
    "multiplexer.paneScrollbars",
    "multiplexer.paneGaps",
    "multiplexer.showAgentLabelsOnPaneBorders",
    "multiplexer.hideTabBarWhenSingleTab",
    "multiplexer.tabBarPosition",
    "multiplexer.tabBarRight",
    "multiplexer.tabBarRightSeparator",
    "multiplexer.windowTitle",
    "multiplexer.agentPanelSort",
    "multiplexer.statusIndicators",
    "multiplexer.sidebar.agents.rowGap",
    "multiplexer.sidebar.agents.rows",
    "multiplexer.sidebar.agents.rowsByAgent",
    "multiplexer.sidebar.spaces.rowGap",
    "multiplexer.sidebar.spaces.rows",
    "multiplexer.accent",
    "multiplexer.toast.delivery",
    "multiplexer.toast.delaySeconds",
    "multiplexer.toast.workspace.position",
    "multiplexer.toast.clipboard.enabled",
    "multiplexer.toast.clipboard.position",
    "multiplexer.sound.enabled",
    "multiplexer.sound.agents",
    "multiplexer.remote.manageSshConfig",
    "multiplexer.allowNested",
    "multiplexer.kittyGraphics",
    "multiplexer.revealHiddenCursorForCjkIme",
    "multiplexer.cjkImeAgents",
    "multiplexer.cjkImeCursorShape",
    "multiplexer.switchAsciiInputSourceInPrefix",
    "daemon.paneShell",
    "daemon.paneShellMode",
    "daemon.newPaneCwd",
    "daemon.resumeAgentsOnRestore",
    "daemon.headlessCols",
    "daemon.headlessRows",
    "daemon.worktreesDirectory",
    "daemon.scrollbackLimitBytes",
    "daemon.paneHistory",
    "daemon.detectionCatalogCheck",
    "update.versionCheck",
    "update.channel",
    "multiplexer.onboarding",
    "multiplexer.theme.name",
    "multiplexer.theme.darkName",
    "multiplexer.theme.lightName",
    "multiplexer.theme.custom",
    "multiplexer.mouseScrollLines",
    "multiplexer.sound.path",
    "multiplexer.sound.donePath",
    "multiplexer.sound.requestPath",
    "keybindings.app.interrupt",
    "keybindings.app.clear",
    "keybindings.app.exit",
    "keybindings.app.suspend",
    "keybindings.app.thinking.cycle",
    "keybindings.app.model.cycleForward",
    "keybindings.app.model.cycleBackward",
    "keybindings.app.model.select",
    "keybindings.app.tools.expand",
    "keybindings.app.thinking.toggle",
    "keybindings.app.session.toggleNamedFilter",
    "keybindings.app.editor.external",
    "keybindings.app.message.followUp",
    "keybindings.app.message.dequeue",
    "keybindings.app.clipboard.pasteImage",
    "keybindings.app.session.new",
    "keybindings.app.session.tree",
    "keybindings.app.session.fork",
    "keybindings.app.session.resume",
    "keybindings.app.tree.foldOrUp",
    "keybindings.app.tree.unfoldOrDown",
    "keybindings.app.tree.editLabel",
    "keybindings.app.tree.toggleLabelTimestamp",
    "keybindings.app.session.togglePath",
    "keybindings.app.session.toggleSort",
    "keybindings.app.session.rename",
    "keybindings.app.session.delete",
    "keybindings.app.session.deleteNoninvasive",
    "keybindings.app.models.save",
    "keybindings.app.models.enableAll",
    "keybindings.app.models.clearAll",
    "keybindings.app.models.toggleProvider",
    "keybindings.app.models.reorderUp",
    "keybindings.app.models.reorderDown",
    "keybindings.app.tree.filter.default",
    "keybindings.app.tree.filter.noTools",
    "keybindings.app.tree.filter.userOnly",
    "keybindings.app.tree.filter.labeledOnly",
    "keybindings.app.tree.filter.all",
    "keybindings.app.tree.filter.cycleForward",
    "keybindings.app.tree.filter.cycleBackward",
    "keybindings.tui.editor.cursorUp",
    "keybindings.tui.editor.cursorDown",
    "keybindings.tui.editor.cursorLeft",
    "keybindings.tui.editor.cursorRight",
    "keybindings.tui.editor.cursorWordLeft",
    "keybindings.tui.editor.cursorWordRight",
    "keybindings.tui.editor.cursorLineStart",
    "keybindings.tui.editor.cursorLineEnd",
    "keybindings.tui.editor.jumpForward",
    "keybindings.tui.editor.jumpBackward",
    "keybindings.tui.editor.pageUp",
    "keybindings.tui.editor.pageDown",
    "keybindings.tui.editor.deleteCharBackward",
    "keybindings.tui.editor.deleteCharForward",
    "keybindings.tui.editor.deleteWordBackward",
    "keybindings.tui.editor.deleteWordForward",
    "keybindings.tui.editor.deleteToLineStart",
    "keybindings.tui.editor.deleteToLineEnd",
    "keybindings.tui.editor.yank",
    "keybindings.tui.editor.yankPop",
    "keybindings.tui.editor.undo",
    "keybindings.tui.input.newLine",
    "keybindings.tui.input.submit",
    "keybindings.tui.input.tab",
    "keybindings.tui.input.copy",
    "keybindings.tui.select.up",
    "keybindings.tui.select.down",
    "keybindings.tui.select.pageUp",
    "keybindings.tui.select.pageDown",
    "keybindings.tui.select.confirm",
    "keybindings.tui.select.cancel"
  ],
  "bounds": {
    "defaultThinkingLevel": { "max": "high" },
    "thinkingBudgets.minimal": { "max": 1024 },
    "thinkingBudgets.low": { "max": 2048 },
    "thinkingBudgets.medium": { "max": 8192 },
    "thinkingBudgets.high": { "max": 16384 }
  },
  "models": ["claude-haiku-5-5", "claude-opus-5-5", "claude-sonnet-5-5"],
  "required": ["core/agents", "core/workflows"],
  "roles": {
    "maestro": { "locked": ["model", "thinking", "tools"] },
    "planner": { "locked": ["model", "thinking", "tools"] },
    "reviewer": { "locked": ["model", "thinking", "tools"] },
    "scout": { "locked": ["model", "thinking", "tools"] },
    "worker": { "locked": ["model", "thinking", "tools"] }
  },
  "updateCheckMinutes": 30
}
```

An `updateCheckMinutes` value of 30 means every 30 minutes during a running
session, plus every session start, subject to `update.versionCheck` and offline mode. The interval is policy,
not a user-overridable preference. `0` means session start only.

## Locations

Each selected area may contribute `settings/policy.json`. Only common and core
may set `minimumRelease` or `updateCheckMinutes`. Ownership comes from the
release's CODEOWNERS, not fields in this file.

## Format

| Member | Meaning |
| --- | --- |
| `locked` | Dotted settings paths and resource lock names; `manifest` fixes source identity |
| `bounds` | `min` and/or `max` for eligible numeric values and thinking levels |
| `models` | Allowed model patterns, in `enabledModels` syntax; all selection routes must match |
| `required` | Release-relative paths; a resource at or below a listed path is required |
| `roles` | Per-role `locked` (`model`, `thinking`, `tools`), `bounds.thinking` and `models` |
| `credentialHeaders` | Extra case-insensitive credential header names; defaults include Authorization and Proxy-Authorization |
| `minimumRelease` | Semantic version floor, optional leading `v`; common or core only |
| `updateCheckMinutes` | Non-negative integer, default 30; `0` means session start only; common or core only |

The `models` patterns restrict every model selector, not just the default.
A newer release whose `minimumRelease` exceeds the current pin requires an
update. Do not set a floor higher than the release you publish.

Thinking order is `off < minimal < low < medium < high < xhigh`. Policy checks
the requested level before the model's capability clamp; a non-reasoning model
still runs with thinking off.

## What the organization can lock

| Keys | Permitted policy and values |
| --- | --- |
| `defaultProvider`, `defaultModel` | Default, lock, constrain through `models` |
| `defaultThinkingLevel` | Default, lock, bound |
| `enabledModels` | Default, lock; effective set always intersects `models` |
| `thinkingBudgets.minimal`, `.low`, `.medium`, `.high` | Default, lock, maximum bound |
| `transport`, `steeringMode`, `followUpMode`, `enableSkillCommands`, `images.autoResize` | Default, lock |
| `compaction.enabled`, `.reserveTokens`, `.keepRecentTokens` | Default, lock; numeric members may be bounded |
| `branchSummary.reserveTokens`, `.skipPrompt` | Default, lock; numeric members may be bounded |
| `retry.enabled`, `.maxRetries`, `.baseDelayMs`, `retry.provider.timeoutMs`, `.maxRetries`, `.maxRetryDelayMs` | Default, lock; numeric members may be bounded |
| `holdUncheckedText` | Default, lock; output guards require `true` |
| `images.blockImages`, `enableInstallTelemetry` | Default, lock; normally true and false respectively |
| `packages`, `extensions`, `skills`, `prompts`, `themes`, `actions`, `contributions`, `workflows`, `agents`, `instructions`, `mcpServers` | Lock local additions, require entries; manifest resource folders supply paths |
| `manifest` | Lock identity |
| `manifestAreas`, `manifestAutoUpdate` | Default, lock |
| `multiplexer.tabBarRight`, `.allowNested`, `.remote.manageSshConfig` | Default, lock |
| `daemon.resumeAgentsOnRestore`, `.detectionCatalogCheck` | Default, lock |
| `update.versionCheck`, `.channel` | Default, lock |
| `shellPath`, `sessionDir`, `daemon.paneShell`, `daemon.worktreesDirectory` | Lock only; an authored location needs its matching lock |
| `npmCommand` | Lock only; the whole argument vector, never a partial command |
| `keybindings.<action>` | Lock one named action to its authored binding or engine default |
| `showChangelog` | Lock startup display; `/changelog` stays available |
| `credentials.<provider>` | Lock allowed sources: `environment`, `command`, `signIn`, `storedFile`; never a secret |
| `theme`, `shellCommandPrefix` and other presentation settings | Default, lock |
| `lastChangelogVersion` | State, never a lock |

`agents` is the lock name for local roles; it is not a settings array. The same
is true of `workflows`, `instructions` and `mcpServers`. An instruction lock does
not suppress context files such as `AGENTS.md` or `CLAUDE.md`. Actions and
contributions become governed when their kinds are collected. Required
instructions stay present through the rules system. Guard rules need no lock:
they come only from the manifest.

## The baseline

The organization locks broadly. Every row is in `manifests/settings/policy.json`;
an explicit value is in `manifests/settings/settings.json`. When a value is unset,
the lock fixes the engine default, not a user or project preference. This keeps
platform-specific defaults portable without allowing local overrides. The new
rows below are candidates pending owner sign-off before merge; the earlier
approved values remain unchanged.

| Keys | Policy | Reason |
| --- | --- | --- |
| `enableInstallTelemetry` | Locked off | Install telemetry stays off for every user |
| `images.blockImages` | Locked on | Images are withheld from model providers |
| `defaultProvider`, `defaultModel` | Locked to `anthropic` and `claude-sonnet-5-5` | One default model for the main agent |
| `models` | `claude-haiku-5-5`, `claude-opus-5-5`, `claude-sonnet-5-5` | Approved model set; every selection route must match it |
| `defaultThinkingLevel` | Bounded to `high` at most | Spend ceiling; the default is `medium` |
| `thinkingBudgets.minimal` | Bounded to 1024 tokens at most | Engine default is the ceiling |
| `thinkingBudgets.low` | Bounded to 2048 tokens at most | Engine default is the ceiling |
| `thinkingBudgets.medium` | Bounded to 8192 tokens at most | Engine default is the ceiling |
| `thinkingBudgets.high` | Bounded to 16384 tokens at most | Engine default is the ceiling |
| `systemPrompt`, `appendSystemPrompt` | Locked | Local files and flags cannot replace the core prompt or persona |
| `manifest`, `manifestAreas`, `manifestAutoUpdate` | Locked; no optional area, no automatic optional update | Users cannot change the source, areas or update mode |
| `packages`, `extensions`, `skills`, `prompts` | Locked | No local additions |
| `core/agents`, `core/workflows` | Required | Core roles and workflows cannot be replaced or removed |
| `maestro`, `planner`, `reviewer`, `scout`, `worker` | `model`, `thinking` and `tools` locked per role | A role runs on the model, thinking level and tools its file declares |
| `providers` | Locked; unset (engine default) | No local provider definitions |
| `enabledModels`, `transport`, `steeringMode`, `followUpMode`, `enableSkillCommands` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `images.autoResize`, `compaction.enabled`, `compaction.reserveTokens`, `compaction.keepRecentTokens`, `branchSummary.reserveTokens` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `branchSummary.skipPrompt`, `retry.enabled`, `retry.maxRetries`, `retry.baseDelayMs`, `retry.provider.timeoutMs` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `retry.provider.maxRetries`, `retry.provider.maxRetryDelayMs`, `theme`, `shellCommandPrefix`, `hideThinkingBlock` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `quietStartup`, `collapseChangelog`, `terminal.showImages`, `terminal.imageWidthCells`, `terminal.clearOnShrink` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `terminal.showTerminalProgress`, `doubleEscapeAction`, `treeFilterMode`, `editorPaddingX`, `autocompleteMaxVisible` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `showHardwareCursor`, `markdown.codeBlockIndent`, `warnings.anthropicExtraUsage`, `update.versionCheck`, `update.channel` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `themes`, `actions`, `contributions`, `workflows`, `agents` | Locked; unset (engine default) | No local additions; area folders supply resources |
| `instructions`, `mcpServers` | Locked; unset (engine default) | No local additions; area folders supply resources |
| `shellPath`, `sessionDir`, `daemon.paneShell`, `daemon.worktreesDirectory` | Locked; unset (engine default) | Use the engine location resolver; do not hard-code one host's paths |
| `multiplexer.theme.autoSwitch`, `multiplexer.sidebarWidth`, `multiplexer.sidebarMinWidth`, `multiplexer.sidebarMaxWidth`, `multiplexer.sidebarStartCollapsed` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.sidebarCollapsedMode`, `multiplexer.mobileWidthThreshold`, `multiplexer.mouseCapture`, `multiplexer.copyOnSelect`, `multiplexer.hostCursor` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.rightClickPassthroughModifier`, `multiplexer.redrawOnFocusGained`, `multiplexer.confirmClose`, `multiplexer.promptNewTabName`, `multiplexer.promptNewWorkspaceName` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.paneBorders`, `multiplexer.paneOuterBorders`, `multiplexer.paneScrollbars`, `multiplexer.paneGaps`, `multiplexer.showAgentLabelsOnPaneBorders` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.hideTabBarWhenSingleTab`, `multiplexer.tabBarPosition`, `multiplexer.tabBarRight`, `multiplexer.tabBarRightSeparator`, `multiplexer.windowTitle` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.agentPanelSort`, `multiplexer.statusIndicators`, `multiplexer.sidebar.agents.rowGap`, `multiplexer.sidebar.agents.rows`, `multiplexer.sidebar.agents.rowsByAgent` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.sidebar.spaces.rowGap`, `multiplexer.sidebar.spaces.rows`, `multiplexer.accent`, `multiplexer.toast.delivery`, `multiplexer.toast.delaySeconds` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.toast.workspace.position`, `multiplexer.toast.clipboard.enabled`, `multiplexer.toast.clipboard.position`, `multiplexer.sound.enabled`, `multiplexer.sound.agents` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.remote.manageSshConfig`, `multiplexer.allowNested`, `multiplexer.kittyGraphics`, `multiplexer.revealHiddenCursorForCjkIme`, `multiplexer.cjkImeAgents` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.cjkImeCursorShape`, `multiplexer.switchAsciiInputSourceInPrefix`, `multiplexer.onboarding`, `multiplexer.theme.name`, `multiplexer.theme.darkName` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.theme.lightName`, `multiplexer.theme.custom`, `multiplexer.mouseScrollLines`, `multiplexer.sound.path`, `multiplexer.sound.donePath` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `multiplexer.sound.requestPath` | Locked; unset (engine default) | Keep the workspace presentation and remote settings at engine defaults |
| `daemon.paneShellMode`, `daemon.newPaneCwd`, `daemon.resumeAgentsOnRestore`, `daemon.headlessCols`, `daemon.headlessRows` | Locked; unset (engine default) | Keep daemon behavior at engine defaults |
| `daemon.scrollbackLimitBytes`, `daemon.paneHistory`, `daemon.detectionCatalogCheck` | Locked; unset (engine default) | Keep daemon behavior at engine defaults |
| `keybindings.app.interrupt`, `keybindings.app.clear`, `keybindings.app.exit`, `keybindings.app.suspend`, `keybindings.app.thinking.cycle` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.model.cycleForward`, `keybindings.app.model.cycleBackward`, `keybindings.app.model.select`, `keybindings.app.tools.expand`, `keybindings.app.thinking.toggle` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.session.toggleNamedFilter`, `keybindings.app.editor.external`, `keybindings.app.message.followUp`, `keybindings.app.message.dequeue`, `keybindings.app.clipboard.pasteImage` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.session.new`, `keybindings.app.session.tree`, `keybindings.app.session.fork`, `keybindings.app.session.resume`, `keybindings.app.tree.foldOrUp` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.tree.unfoldOrDown`, `keybindings.app.tree.editLabel`, `keybindings.app.tree.toggleLabelTimestamp`, `keybindings.app.session.togglePath`, `keybindings.app.session.toggleSort` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.session.rename`, `keybindings.app.session.delete`, `keybindings.app.session.deleteNoninvasive`, `keybindings.app.models.save`, `keybindings.app.models.enableAll` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.models.clearAll`, `keybindings.app.models.toggleProvider`, `keybindings.app.models.reorderUp`, `keybindings.app.models.reorderDown`, `keybindings.app.tree.filter.default` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.tree.filter.noTools`, `keybindings.app.tree.filter.userOnly`, `keybindings.app.tree.filter.labeledOnly`, `keybindings.app.tree.filter.all`, `keybindings.app.tree.filter.cycleForward` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.app.tree.filter.cycleBackward`, `keybindings.tui.editor.cursorUp`, `keybindings.tui.editor.cursorDown`, `keybindings.tui.editor.cursorLeft`, `keybindings.tui.editor.cursorRight` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.tui.editor.cursorWordLeft`, `keybindings.tui.editor.cursorWordRight`, `keybindings.tui.editor.cursorLineStart`, `keybindings.tui.editor.cursorLineEnd`, `keybindings.tui.editor.jumpForward` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.tui.editor.jumpBackward`, `keybindings.tui.editor.pageUp`, `keybindings.tui.editor.pageDown`, `keybindings.tui.editor.deleteCharBackward`, `keybindings.tui.editor.deleteCharForward` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.tui.editor.deleteWordBackward`, `keybindings.tui.editor.deleteWordForward`, `keybindings.tui.editor.deleteToLineStart`, `keybindings.tui.editor.deleteToLineEnd`, `keybindings.tui.editor.yank` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.tui.editor.yankPop`, `keybindings.tui.editor.undo`, `keybindings.tui.input.newLine`, `keybindings.tui.input.submit`, `keybindings.tui.input.tab` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.tui.input.copy`, `keybindings.tui.select.up`, `keybindings.tui.select.down`, `keybindings.tui.select.pageUp`, `keybindings.tui.select.pageDown` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `keybindings.tui.select.confirm`, `keybindings.tui.select.cancel` | Locked; unset (engine default) | Keep the shipped action binding, including platform-specific defaults |
| `npmCommand` | Locked to `["npm"]` | Fix the whole program-and-arguments vector; no local wrappers or extra arguments |
| `holdUncheckedText` | Locked to `true` | Hold output until checks finish; guard files wait for #27 |
| `showChangelog` | Locked to `true` | Keep release changes visible at startup |
| `credentials.anthropic` | Locked to `["environment", "signIn"]` | Allow environment credentials and sign-in; refuse command-produced and literal stored keys |

The `maestro` role takes no model or thinking level; the main agent's come from
settings, so the `model` and `thinking` locks on that role do not select them.
The `manifestAreas` lock means a user cannot select the evidence-review
capability; the organization selects it in `manifests/settings/settings.json`.

Guard files in `required` wait for #27. Core skills and instructions also wait
until the core ships them; then add their release-relative paths to `required`.
The local-addition locks do not substitute for those required entries. Existing
core roles and workflows remain required. Locations left unset use engine
resolution, including host and environment fallbacks; they are not a portable
fixed executable or a filesystem sandbox. Credential restrictions apply to
`anthropic`; no secret is shipped. Thinking remains bounded rather than locked,
as previously approved.

## Combining areas and local values

Locks and required paths join. Minimum bounds take the largest minimum, maximum
bounds the smallest maximum, and allowed model sets intersect. A later area
cannot change a locked value, unlock a key or widen an earlier constraint. An
impossible bound or a manifest default outside combined constraints invalidates
the release. `credentialHeaders` additions combine across selected areas.

Stored user/project preferences are resolved without rewriting their file:
locked values take the manifest value, bounded values clamp, and a forbidden
model falls back to the manifest default if allowed. Otherwise Maestro uses the
first available catalog model matching the earliest usable policy pattern. Each
conflict reports a diagnostic.

Explicit changes are different. Flags, SDK or RPC requests, settings edits,
extension calls and resource toggles are refused **before any effect** when they
violate policy. A refusal changes no file, history, pending edit or active state.
Local additions forbidden by a resource lock never execute. Extensions themselves
still have the user's filesystem and process access.

## Validation

Unknown members, invalid constraint classes and impossible combined bounds make
the release invalid. A lock takes the manifest value or engine default when
unset. If you lock a privacy setting, set its intended value in the same area's
settings rather than assuming what the engine default is. Required workflow paths
also require their referenced roles from this release; they do not add approvals.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), then load a
pinned revision with Maestro. Test one allowed change and one forbidden explicit
request. Confirm a stored conflicting preference resolves with a diagnostic and
its raw file stays unchanged. Test every selected language/capability combination
that narrows these constraints. Schema validation cannot prove required-reference
closure or that a chosen model is available; the loading check must do that.
