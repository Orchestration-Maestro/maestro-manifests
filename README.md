<p align="center">
  <img src=".github/assets/maestro-manifests.jpg" alt="Maestro Manifests: Declare once. Govern everywhere." width="100%" />
</p>

# Maestro manifests

This repository holds public organization configuration for Maestro: settings,
policy, a system prompt, agent roles and workflows. A manifest is an optional
source of the same authored artifacts developers use locally. Without a manifest,
Maestro runs in internal mode; selecting one adds an organization layer below
user and project configuration.

This tree follows the [manifest-mode v0.1 contract](https://github.com/Orchestration-Maestro/maestro/blob/main/docs/specs/manifest-mode.md).
The contract also describes runtime features delivered by Maestro's manifest,
subagent, rules and catalog consumers. Content checks here do not prove those
consumers are implemented. See [schemas and checks](docs/schemas-and-checks.md)
for the checks available now and the integration checks required for a release.

## Try the configuration

Use a Maestro build with the manifest consumer. Select a **published Git tag or
commit** in `.maestro/settings.json`; do not use an unpinned branch or install
this repository as a local manifest. To try a proposed revision, use its full
commit ID as the ref.

A complete project selection for the first release is:

```json
{
  "manifest": "git:github.com/Orchestration-Maestro/maestro-manifests@v0.1.0",
  "manifestAreas": [],
  "manifestAutoUpdate": false
}
```

The tag must exist before this example can load. Development starts at version
`0.0.0`; [Release Please](docs/releases-and-updates.md) publishes the first tag.
Start Maestro in that project and run `/manifest status`. The manifest installs
in project scope and the startup listing identifies its source, pin and settings
file. No capability is selected by the empty `manifestAreas` list. Common,
standards and core areas always load.

This repository supplies three chains:

| Workflow | Steps | Final output |
| --- | --- | --- |
| `implement-and-review` | worker, reviewer, worker | Implementation with review feedback applied |
| `implement` | scout, planner, worker | Implemented plan |
| `scout-and-plan` | scout, planner | Plan only; no implementation |

Ask Maestro to list workflows, show one with its source, or run it with a task.
The files contain no approval step. Optional roles and workflows can be replaced
by same-name local files; policy can require a workflow and its release-local
references. Read [workflows](docs/workflows.md) before changing a chain.

## Layout

Actual authored content in this revision:

```text
package.json                    # common package; native maestro resource key
settings/
  settings.json                 # no optional areas or automatic updates selected
  policy.json                   # release-check interval; no mandatory locks
core/
  SYSTEM.md                     # base system prompt
  agents/
    maestro.md                  # appended main-agent persona
    scout.md
    planner.md
    reviewer.md
    worker.md
  workflows/
    implement-and-review.yaml
    implement.yaml
    scout-and-plan.yaml
docs/                           # formats, composition and contributor checks
.github/CODEOWNERS              # review and catalog ownership
```

Create an area only when it has real content. Supported additional locations are
`standards/<domain>/`, `languages/<name>/` and
`capabilities/<group>/<name>/`. Each capability has its own `package.json` and
resources. Selection uses its full path, not the last directory name. Group
folders organize capabilities; they are not settings layers.

The security capability is maintained separately and is not included in this
revision. Its integration must use
`capabilities/security/maestro-security/`, native role files and one package.
Do not copy another lane's unpublished content into this tree.

## How composition works

1. Engine defaults provide the starting values.
2. The manifest reads common, all standards by directory name, core, selected
   languages in list order, then selected capabilities in list order.
3. User settings, project settings and run options apply within policy.
4. Runtime changes pass the same policy before any effect.

A later area can narrow a bound or add a lock, but cannot loosen an earlier one.
Stored conflicting preferences resolve with diagnostics without rewriting the
file. Explicit forbidden changes fail before changing state. The initial policy
here sets no locks, bounds or required resources; it checks for newer releases at
session start and every 30 minutes, subject to `update.versionCheck` and offline
mode. Choosing a theme, credentials and machine-local state remains local.

Instructions are offered for installation, not automatically appended by area
selection. System prompt, persona and installed instructions are different
artifacts; see their separate guides.

## Documentation

| Write or change | Guide |
| --- | --- |
| Settings layers and area selection | [Settings](docs/settings.md) |
| Locks, bounds and required resources | [Policy](docs/policy.md) |
| Base prompt, persona and context | [System prompt and persona](docs/system-prompt-and-persona.md) |
| Installable guidance | [Instructions](docs/instructions.md) |
| Delegated-agent definitions | [Roles](docs/roles.md) |
| On-demand procedures | [Skills](docs/skills.md) |
| Slash-command text | [Prompt templates](docs/prompt-templates.md) |
| Multi-step runs | [Workflows](docs/workflows.md) |
| Built code integrations | [Extensions](docs/extensions.md) |
| Terminal colors | [Themes](docs/themes.md) |
| Providers, models and credentials | [Models](docs/models.md) |
| Team bundles and path ownership | [Capabilities and ownership](docs/capabilities-and-ownership.md) |
| Version pins, minimums and rollback | [Releases and updates](docs/releases-and-updates.md) |
| Checkout and installed-copy repair | [Integrity](docs/integrity.md) |
| Content checks and runtime validation | [Schemas and checks](docs/schemas-and-checks.md) |
| Cards, ownership and dependency relations | [Catalog](docs/catalog.md) |

[CONTEXT.md](CONTEXT.md) defines the vocabulary.
[Development](docs/development.md) covers pinned tools, hooks, CI and delivery.
The repository's [engineering](docs/standards/engineering.md),
[security](docs/standards/security.md) and
[quality](docs/standards/northstar.md) maps describe its contributor standards.
They are documentation, not `standards/<domain>/` manifest resources.

## Check a contribution

```sh
mise trust
mise exec -- just setup
mise exec -- just check
mise exec -- just test
```

Start with an issue, keep changes within its scope and follow
[AGENTS.md](AGENTS.md). Managed files remain generated by `rust-gate`.
Public content only: no private collections, vendor material, credentials or
private review evidence. Configuration governance is not a sandbox; extensions
retain the user's access.

## License

[MIT](LICENSE). Artwork and font terms are in the
[banner credits](.github/assets/CREDITS.md).
