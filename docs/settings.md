# Settings

Settings choose the manifest, optional areas and defaults. They use ordinary JSON
keys, not a separate descriptor format. Policy is a different document: values
belong in `manifests/settings/settings.json`, constraints in
`manifests/settings/policy.json`.

## Contents

1. [Locations](#locations)
2. [Example](#example)
3. [Usage](#usage)
4. [Format](#format)
5. [What the organization can lock](#what-the-organization-can-lock)
6. [Validation](#validation)
7. [Check a change](#check-a-change)

## Locations

The configuration root is `~/.maestro/agent` by default; the existing
`MAESTRO_CODING_AGENT_DIR` override relocates it. Local settings live at
`~/.maestro/agent/settings.json` and `.maestro/settings.json` in a project. A
manifest area contributes `settings/settings.json`, relative to the area. The
manifest root is `manifests/`, and every area path in this guide is relative to it.

Resolution order is engine defaults, manifest, user, project, run options and
runtime changes. The manifest reads its own areas in this order:

| Area | Path under the manifest root | Selection |
| --- | --- | --- |
| Common | The manifest root itself | Always |
| Standards | `standards/<domain>/` | All domains, directory-name order |
| Core | `core/` | Always |
| Languages | `languages/<name>/` | `manifestAreas`, list order |
| Capabilities | `capabilities/<group>/<name>/` | `manifestAreas`, list order, after languages |

Settings combine with the ordinary one-level merge. Do not assume arbitrary
recursive merging. Policy combines differently: locks join, model sets intersect
and bounds narrow. See [policy](policy.md).

## Example

The common area supplies the release defaults and the values its locks fix. Its
settings select no optional capability and do not set a manifest source:

From [`manifests/settings/settings.json`](../manifests/settings/settings.json):

```json
{
  "defaultProvider": "anthropic",
  "defaultModel": "claude-sonnet-5-5",
  "defaultThinkingLevel": "medium",
  "enableInstallTelemetry": false,
  "images": {
    "blockImages": true
  },
  "manifestAreas": [],
  "manifestAutoUpdate": false
}
```

`manifestAreas: []` leaves only common, standards and core selected. The policy
locks `manifestAreas`, so a user cannot add the evidence-review capability; the
organization selects it by adding its full path,
`capabilities/practice/evidence-review`, to `manifestAreas` in this document.
Do not edit the installed common document to make a personal selection.

## Usage

In user or project settings, set `manifest` to
`git:github.com/Orchestration-Maestro/maestro-manifests@` followed by an existing
release tag or full commit. For a contribution, use the exact fetchable commit.
An absent setting, empty string or object with empty `source` means internal
mode. Local paths, npm sources, unpinned sources and wrong JSON kinds are invalid
manifest selections. There is no manifest-selection flag.

The selection can also be an object with `source` and the ordinary `extensions`,
`skills`, `prompts` and `themes` package filters. Filters use release-relative
paths; they cannot disable a required resource. `manifestAreas` accepts existing
`languages/<name>` and `capabilities/<group>/<name>` paths. Unknown paths or
prefixes invalidate selection; duplicates load once.

## Format

Language and capability settings cannot set or constrain `manifest`,
`manifestAreas`, `minimumRelease` or `updateCheckMinutes`. Group folders supply
no settings. The manifest must not set `extensions`, `skills`, `prompts` or
`themes` path arrays; area resource folders supply those inputs. Its `packages`
entries are required packages. See the governed-key table in [policy](policy.md)
for the permitted defaults and constraints; machine paths and credentials are
user-only.

## What the organization can lock

User settings normally lose to project settings. If both select the same Git
identity, the project ref wins. For different identities, the user's manifest is
loaded first; its `manifest` lock can ignore the project selection with a
diagnostic. If that user manifest cannot load, the selection is broken.

The manifest installs in the scope of the file selecting it, and selection never
writes an entry into `packages`. Reads do not copy effective values into raw user
or project settings. A stored conflict resolves with a diagnostic; an explicit
forbidden change is refused before any effect. A lock applies in every process,
including the workspace daemon; that daemon reads the persisted global view and
never project settings.

## Validation

JSON parsing checks syntax. The manifest loader checks area paths, forbidden
resource-path settings and combined policy. A stored conflicting preference is
resolved without rewriting its file; an explicit conflicting request is refused.

## Check a change

Run the [content checks](schemas-and-checks.md#available-checks) and load a pinned
commit in a manifest-capable Maestro build. `/manifest status` must name the
expected settings file and ref. Inspect `/settings` for manifest-set and locked
values. Try the intended override and confirm it succeeds or reports the specific
policy constraint. `/reload` keeps the last accepted composition if the new
configuration fails; a clean start must also succeed before release.
