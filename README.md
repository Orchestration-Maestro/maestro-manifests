<p align="center">
  <img src=".github/assets/maestro-manifests.jpg" alt="Maestro Manifests: Declare once. Govern everywhere." width="100%" />
</p>

# Maestro manifests

This repository holds public organization configuration for Maestro: settings,
policy, a system prompt, agent roles, workflows and an opt-in evidence-review
capability. A manifest is an optional source of the same authored artifacts
developers use locally. Without a manifest,
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

The release's common settings leave optional areas unselected:

From [`settings/settings.json`](settings/settings.json):

```json
{
  "manifestAreas": [],
  "manifestAutoUpdate": false
}
```

In your own settings, set `manifest` to
`git:github.com/Orchestration-Maestro/maestro-manifests@` followed by an existing
release tag or full commit. Then start Maestro and run `/manifest status`.
See [settings usage](docs/settings.md#usage) for selection and override rules.
The source installs in the scope of the settings file that names it; common,
standards and core always load. Development starts at version `0.0.0` and
[Release Please](docs/releases-and-updates.md) publishes versioned tags.

To opt in, add `capabilities/practice/evidence-review` to your own `manifestAreas`.
It offers a skill, role, instruction, prompt, theme and workflow, with no added
executable extension or model provider. Selecting it does not install its
instruction or choose its theme automatically. Its review rule lives in the role
and skill, so it applies only when evidence is reviewed; the instruction
selects only files under `docs/`.

Core supplies three chains:

| Workflow | Steps | Final output |
| --- | --- | --- |
| `implement-and-review` | worker, reviewer, worker | Implementation with review feedback applied |
| `implement` | scout, planner, worker | Implemented plan |
| `scout-and-plan` | scout, planner | Plan only; no implementation |

The opt-in capability adds [review-evidence](capabilities/practice/evidence-review/workflows/review-evidence.yaml),
a one-step chain using `evidence-reviewer` to check claims against cited sources.

Ask Maestro to list workflows, show one with its source, or run it with a task.
The files contain no approval step. Optional roles and workflows can be replaced
by same-name local files; policy can require a workflow and its release-local
references. Read [workflows](docs/workflows.md) before changing a chain.

## Layout

The common root holds `package.json` and `settings/`. Core holds the base prompt,
main persona, four delegated roles and three workflows. The optional capability
at `capabilities/practice/evidence-review/` keeps its native package and all six
resources together. Documentation lives under `docs/`; CODEOWNERS supplies
review and catalog attribution. The complete input inventory follows.

Create an area only when it has real content. Supported additional locations are
`standards/<domain>/`, `languages/<name>/` and
`capabilities/<group>/<name>/`. Each capability has its own `package.json` and
resources. Selection uses its full path, not the last directory name. Group
folders organize capabilities; they are not settings layers.

The security capability is maintained separately and is not included in this
revision. Its integration must use
`capabilities/security/maestro-security/`, native role files and one package.
Do not copy another lane's unpublished content into this tree.

## Artifact index

Every authored manifest input is listed below in file-name order, together with
the ownership and shared release-version records. Contributor tooling, the
standards pages and documentation are not runtime artifacts. Models and executable
extensions are intentionally absent; their explicit
[exceptions](docs/index.md#example-coverage-and-exceptions) let checks distinguish
them from missing files. Keep every row linked to its real file and add a row
whenever an artifact is added.

| File | Provides |
| --- | --- |
| [`.github/CODEOWNERS`](.github/CODEOWNERS) | Last-match review and catalog ownership for every path. |
| [`capabilities/practice/evidence-review/agents/evidence-reviewer.md`](capabilities/practice/evidence-review/agents/evidence-reviewer.md) | Opt-in read-only role for checking factual claims against sources. |
| [`capabilities/practice/evidence-review/instructions/cite-code-claims.instructions.md`](capabilities/practice/evidence-review/instructions/cite-code-claims.instructions.md) | Installable documentation guidance with applyTo frontmatter. |
| [`capabilities/practice/evidence-review/package.json`](capabilities/practice/evidence-review/package.json) | One opt-in capability package with explicit native resource paths. |
| [`capabilities/practice/evidence-review/prompts/explain-change.md`](capabilities/practice/evidence-review/prompts/explain-change.md) | A file/audience explanation template with positional arguments. |
| [`capabilities/practice/evidence-review/skills/review-evidence/SKILL.md`](capabilities/practice/evidence-review/skills/review-evidence/SKILL.md) | A source-grounded answer-review procedure. |
| [`capabilities/practice/evidence-review/themes/review-dark.json`](capabilities/practice/evidence-review/themes/review-dark.json) | A complete 51-token optional terminal theme. |
| [`capabilities/practice/evidence-review/workflows/review-evidence.yaml`](capabilities/practice/evidence-review/workflows/review-evidence.yaml) | Opt-in factual review chain using evidence-reviewer. |
| [`core/SYSTEM.md`](core/SYSTEM.md) | Base system prompt and documentation pointers. |
| [`core/agents/maestro.md`](core/agents/maestro.md) | Appended main-agent persona. |
| [`core/agents/planner.md`](core/agents/planner.md) | Read-only implementation planning with a concrete handoff and its verification. |
| [`core/agents/reviewer.md`](core/agents/reviewer.md) | Shell-free code review: findings with location, cause and fix, and a security pass. |
| [`core/agents/scout.md`](core/agents/scout.md) | Compressed code context and exact source ranges. |
| [`core/agents/worker.md`](core/agents/worker.md) | Test-first implementation in an isolated delegated context, with each command and exit code reported. |
| [`core/workflows/implement-and-review.yaml`](core/workflows/implement-and-review.yaml) | Worker, reviewer, worker chain with previous-output passing. |
| [`core/workflows/implement.yaml`](core/workflows/implement.yaml) | Scout, planner, worker chain using original arguments and previous output. |
| [`core/workflows/scout-and-plan.yaml`](core/workflows/scout-and-plan.yaml) | Scout, planner chain that returns a plan without implementing. |
| [`package.json`](package.json) | Common native package record; no independent npm version. |
| [`settings/policy.json`](settings/policy.json) | Common release-check interval, with no locks or minimum release. |
| [`settings/settings.json`](settings/settings.json) | Common defaults: no optional areas and no automatic optional updates. |
| [`version.txt`](version.txt) | Release Please version record for the shared Git release. |

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

Start with the [documentation index](docs/index.md), in reading order. It covers
settings, policy, prompts, instructions, roles, skills, workflows, themes, models,
extensions, capabilities, ownership, releases, integrity, schemas and the catalog.
It also records the models and extensions exceptions.
[CONTEXT.md](CONTEXT.md) defines the vocabulary; [development](docs/development.md)
describes pinned tools, hooks, CI and delivery.

The [engineering](docs/standards/engineering.md),
[security](docs/standards/security.md) and [Northstar](docs/standards/northstar.md)
pages explain how contributors follow the organization's rules here. They are not
runtime `standards/<domain>/` resources.

## Check a contribution

Run `mise trust`, `mise exec -- just setup` and `mise exec -- just check`, which
runs the content checks and the tests. The real [justfile](justfile) owns these
recipes; see [development](docs/development.md#local-checks) for the complete file
and hooks.

Start with an issue, keep changes within its scope and follow
[AGENTS.md](AGENTS.md). Managed files remain generated by `rust-gate`.
Public content only: no private collections, vendor material, credentials or
private review evidence. Configuration governance is not a sandbox; extensions
retain the user's access.

## License

[MIT](LICENSE). Artwork and font terms are in the
[banner credits](.github/assets/CREDITS.md).
