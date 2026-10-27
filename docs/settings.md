# Settings

Settings choose the manifest, optional areas and defaults. They use ordinary JSON
keys, not a separate descriptor format. Policy is a different document: values
belong in `settings/settings.json`, constraints in `settings/policy.json`.

## Locations and order

The configuration root is `~/.maestro/agent` by default; the existing
`MAESTRO_CODING_AGENT_DIR` override relocates it. Local settings live at
`~/.maestro/agent/settings.json` and `.maestro/settings.json` in a project. A
manifest area contributes `settings/settings.json`.

Resolution order is engine defaults, manifest, user, project, run options and
runtime changes. The manifest reads its own areas in this order:

| Area | Path | Selection |
| --- | --- | --- |
| Common | Release root | Always |
| Standards | `standards/<domain>/` | All domains, directory-name order |
| Core | `core/` | Always |
| Languages | `languages/<name>/` | `manifestAreas`, list order |
| Capabilities | `capabilities/<group>/<name>/` | `manifestAreas`, list order, after languages |

Settings combine with the ordinary one-level merge. Do not assume arbitrary
recursive merging. Policy combines differently: locks join, model sets intersect
and bounds narrow. See [policy](policy.md).

## Complete example

To use just the common and core content shipped here, write this entire project
`.maestro/settings.json` after the first release is published:

```json
{
  "manifest": "git:github.com/Orchestration-Maestro/maestro-manifests@v0.1.0",
  "manifestAreas": [],
  "manifestAutoUpdate": false,
  "defaultThinkingLevel": "medium"
}
```

For a proposed revision use its full commit instead of the release tag. The ref
must exist. A branch without an explicit pin, local path, npm source, wrong JSON
kind or missing release is not a valid manifest selection. No manifest flag
exists. An absent setting, empty string or object with empty `source` selects
internal mode.

The selection may be an object with `source` and package filters. For example,
this complete selection keeps every skill except one optional path:

```json
{
  "manifest": {
    "source": "git:github.com/Orchestration-Maestro/maestro-manifests@v0.1.0",
    "skills": ["!core/skills/experimental/**"]
  }
}
```

Filter paths are relative to the release root. A filter cannot disable a required
resource. Only select paths for optional areas that actually exist, for example
`languages/rust` or `capabilities/security/maestro-security` once those areas are
included. Unknown paths or prefixes invalidate the manifest. Duplicates load once.

## What an area can set

The common settings shipped here are a complete minimal document:

```json
{
  "manifestAreas": [],
  "manifestAutoUpdate": false
}
```

Language and capability settings cannot set or constrain `manifest`,
`manifestAreas`, `minimumRelease` or `updateCheckMinutes`. Group folders supply
no settings. The manifest must not set `extensions`, `skills`, `prompts` or
`themes` path arrays; area resource folders supply those inputs. Its `packages`
entries are required packages. See the governed-key table in [policy](policy.md)
for the permitted defaults and constraints; machine paths and credentials are
user-only.

## Override and refusal

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

## Check a change

Run the [content checks](schemas-and-checks.md#available-checks) and load a pinned
commit in a manifest-capable Maestro build. `/manifest status` must name the
expected settings file and ref. Inspect `/settings` for manifest-set and locked
values. Try the intended override and confirm it succeeds or reports the specific
policy constraint. `/reload` keeps the last accepted composition if the new
configuration fails; a clean start must also succeed before release.
