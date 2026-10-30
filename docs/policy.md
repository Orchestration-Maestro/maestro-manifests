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
    "shellCommandPrefix",
    "warnings.anthropicExtraUsage",
    "multiplexer.remote.manageSshConfig",
    "multiplexer.allowNested",
    "multiplexer.tabBarRight",
    "daemon.paneShell",
    "daemon.paneShellMode",
    "daemon.newPaneCwd",
    "daemon.resumeAgentsOnRestore",
    "daemon.worktreesDirectory",
    "daemon.paneHistory",
    "daemon.detectionCatalogCheck",
    "update.versionCheck",
    "update.channel"
  ],
  "bounds": {
    "defaultThinkingLevel": { "max": "high" },
    "thinkingBudgets.minimal": { "max": 1024 },
    "thinkingBudgets.low": { "max": 2048 },
    "thinkingBudgets.medium": { "max": 8192 },
    "thinkingBudgets.high": { "max": 16384 }
  },
  "models": ["claude-haiku-5-5", "claude-opus-5-5", "claude-sonnet-5-5"],
  "required": ["core/agents", "core/guards", "core/workflows"],
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

The organization locks behavior and local additions, leaving personal UI editable.
Every locked row is in `manifests/settings/policy.json`;
an explicit value is in `manifests/settings/settings.json`. When a value is unset,
the lock fixes the engine default, not a user or project preference. This keeps
platform-specific defaults portable without allowing local overrides. The owner
approved this baseline on 2026-10-28: lock behaviour, leave personal UI.

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
| `core/agents`, `core/guards`, `core/workflows` | Required | Core roles, guard files and workflows cannot be replaced or removed |
| `maestro`, `planner`, `reviewer`, `scout`, `worker` | `model`, `thinking` and `tools` locked per role | A role runs on the model, thinking level and tools its file declares |
| `providers` | Locked; unset (engine default) | No local provider definitions |
| `enabledModels`, `transport`, `steeringMode`, `followUpMode`, `enableSkillCommands` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `images.autoResize`, `compaction.enabled`, `compaction.reserveTokens`, `compaction.keepRecentTokens`, `branchSummary.reserveTokens` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `branchSummary.skipPrompt`, `retry.enabled`, `retry.maxRetries`, `retry.baseDelayMs`, `retry.provider.timeoutMs` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `retry.provider.maxRetries`, `retry.provider.maxRetryDelayMs`, `shellCommandPrefix` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `warnings.anthropicExtraUsage`, `update.versionCheck`, `update.channel` | Locked; unset (engine default) | Keep the existing engine behavior; refuse local overrides |
| `themes`, `actions`, `contributions`, `workflows`, `agents` | Locked; unset (engine default) | No local additions; area folders supply resources |
| `instructions`, `mcpServers` | Locked; unset (engine default) | No local additions; area folders supply resources |
| `shellPath`, `sessionDir`, `daemon.paneShell`, `daemon.worktreesDirectory` | Locked; unset (engine default) | Use the engine location resolver; do not hard-code one host's paths |
| `daemon.paneShellMode`, `daemon.newPaneCwd`, `daemon.resumeAgentsOnRestore` | Locked; unset (engine default) | Keep daemon behavior at engine defaults |
| `daemon.paneHistory`, `daemon.detectionCatalogCheck` | Locked; unset (engine default) | Keep daemon behavior at engine defaults |
| `multiplexer.remote.manageSshConfig`, `multiplexer.allowNested`, `multiplexer.tabBarRight` | Locked; unset (engine default) | Keep SSH configuration, nested sessions and status commands at engine defaults |
| `npmCommand` | Locked; unset (engine default) | Git installs keep `install --omit=dev`; explicit `["npm"]` changes this to `install` |
| `showChangelog` | Locked to `true` | Keep release changes visible at startup |
| `credentials.anthropic` | Locked to `["environment", "signIn"]` | Allow environment credentials and sign-in; refuse command-produced and literal stored keys |

The `maestro` role takes no model or thinking level; the main agent's come from
settings, so the `model` and `thinking` locks on that role do not select them.
The `manifestAreas` lock means a user cannot select the evidence-review
capability; the organization selects it in `manifests/settings/settings.json`.

Personal UI is not locked: `theme`, `hideThinkingBlock`, `quietStartup`,
`collapseChangelog`, `terminal.*`, `doubleEscapeAction`, `treeFilterMode`,
`editorPaddingX`, `autocompleteMaxVisible`, `showHardwareCursor`, `markdown.*`,
workspace presentation settings, `daemon.headlessCols`, `daemon.headlessRows`,
`daemon.scrollbackLimitBytes` and all 72 named `keybindings.*` actions remain
editable. The `actions` lock forbids local action additions; it does not lock
those keybindings. Workspace behavior remains locked: SSH configuration, nested
sessions and `multiplexer.tabBarRight`, which can run periodic commands.
`holdUncheckedText` is absent from the baseline; guards lock
it on when they ship.

Core skills and instructions wait until the core ships them; then add their
release-relative paths to `required`. The local-addition locks do not substitute
for those required entries. Core roles, guard files and workflows are required. Locations left unset use engine
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
