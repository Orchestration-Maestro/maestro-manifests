# Policy

An area's `settings/policy.json` constrains its settings and resources. All eight
members are optional. Unknown members, unknown governed keys, invalid bounds and
conflicting area constraints invalidate the manifest; a misspelled lock never
silently becomes a preference.

## Complete example

This file uses the roles and workflow in this repository and needs no extra
resource to satisfy its `required` path. It illustrates a stricter alternative to
the shipped policy; it is not the default policy here.

```json
{
  "locked": ["systemPrompt", "appendSystemPrompt", "enableInstallTelemetry"],
  "bounds": {
    "defaultThinkingLevel": { "min": "medium", "max": "xhigh" },
    "retry.maxRetries": { "max": 5 }
  },
  "required": ["core/workflows/implement-and-review.yaml"],
  "roles": {
    "reviewer": {
      "locked": ["tools"],
      "bounds": { "thinking": { "min": "medium", "max": "xhigh" } }
    }
  },
  "credentialHeaders": ["X-Org-Token"],
  "updateCheckMinutes": 30
}
```

A lock takes its value from the manifest settings, or from the engine default if
not set. To force telemetry off, pair this policy with the following area
`settings/settings.json`:

```json
{
  "enableInstallTelemetry": false
}
```

Requiring the workflow also requires its worker and reviewer references from this
release. It does not add a human approval step or sandbox the worker's tools.

## Members

| Member | Meaning |
| --- | --- |
| `locked` | Dotted setting paths, plus `systemPrompt`, `appendSystemPrompt`, `providers`; `manifest` fixes source identity |
| `bounds` | `min` and/or `max` for eligible numeric values and thinking levels |
| `models` | Allowed model patterns, in `enabledModels` syntax; all selection routes must match |
| `required` | Release-relative paths; a resource at or below a listed path is required |
| `roles` | Per-role `locked` (`model`, `thinking`, `tools`), `bounds.thinking` and `models` |
| `credentialHeaders` | Extra case-insensitive credential header names; defaults include Authorization and Proxy-Authorization |
| `minimumRelease` | Semantic version floor, optional leading `v`; common or core only |
| `updateCheckMinutes` | Non-negative integer, default 30; `0` means session start only; common or core only |

For example, `"models": ["anthropic/claude-*", "openai/gpt-6.1-sol"]`
restricts every model selector, not only the default. `"minimumRelease": "1.5.1"`
requires clients below that version to update when they discover the newer
release. Do not set a floor higher than the release you publish.

Thinking order is `off < minimal < low < medium < high < xhigh`. Policy checks
the requested level before the model's capability clamp; a non-reasoning model
still runs with thinking off.

## Governed keys

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
| `images.blockImages`, `enableInstallTelemetry` | Default, lock; normally true and false respectively |
| `packages`, `extensions`, `skills`, `prompts`, `actions`, `contributions` | Lock local additions, require entries; manifest resource folders supply paths |
| `manifest` | Lock identity |
| `manifestAreas`, `manifestAutoUpdate` | Default, lock |
| `multiplexer.tabBarRight`, `.allowNested`, `.remote.manageSshConfig` | Default, lock |
| `daemon.resumeAgentsOnRestore`, `.detectionCatalogCheck` | Default, lock |
| `update.versionCheck`, `.channel` | Default, lock |
| `theme`, `themes`, `shellCommandPrefix` | Defaults only; never locks or bounds |
| Every other setting | User-only, including presentation, keybindings, machine paths and credentials |

There is no policy lock that forbids new local workflows or roles. Each workflow
step still follows model, tool and role policy. Actions and contributions become
governed when their kinds are collected. Required-instruction persistence belongs
to the rules-system contract, not this manifest's installation mechanism.

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

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), then load a
pinned revision with Maestro. Test one allowed change and one forbidden explicit
request. Confirm a stored conflicting preference resolves with a diagnostic and
its raw file stays unchanged. Test every selected language/capability combination
that narrows these constraints. Schema validation cannot prove required-reference
closure or that a chosen model is available; the loading check must do that.
