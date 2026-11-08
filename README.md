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

## Contents

1. [Try the configuration](#try-the-configuration)
2. [Layout](#layout)
3. [Artifact index](#artifact-index)
4. [How composition works](#how-composition-works)
5. [Documentation](#documentation)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Check a change](#check-a-change)
8. [License](#license)

## Try the configuration

This tree follows the [manifest-mode v0.1 contract](https://github.com/Orchestration-Maestro/maestro/blob/main/docs/specs/manifest-mode.md).
The contract also describes runtime features delivered by Maestro's manifest,
subagent, rules and catalog consumers. Content checks here do not prove those
consumers are implemented. See [schemas and checks](docs/schemas-and-checks.md)
for the checks available now and the integration checks required for a release.

Use a Maestro build with the manifest consumer. Select a **published Git tag or
commit** in `.maestro/settings.json`; do not use an unpinned branch or install
this repository as a local manifest. To try a proposed revision, use its full
commit ID as the ref.

The release's common settings choose the default model and thinking level, turn
install telemetry off, block images and leave optional areas unselected:

Excerpt from [`manifests/settings/settings.json`](manifests/settings/settings.json):

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

In your own settings, set `manifest` to
`git:github.com/Orchestration-Maestro/maestro-manifests@` followed by an existing
release tag or full commit. Then start Maestro and run `/manifest status`.
See [settings usage](docs/settings.md#usage) for selection and override rules.
The source installs in the scope of the settings file that names it; common,
standards and core always load. Development starts at version `0.0.0` and
[Release Please](docs/releases-and-updates.md) publishes versioned tags.

The policy locks `manifestAreas`, so the organization opts in: add
`capabilities/practice/evidence-review` to `manifestAreas` in
[`manifests/settings/settings.json`](manifests/settings/settings.json).
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

The opt-in capability adds [review-evidence](manifests/capabilities/practice/evidence-review/workflows/review-evidence.yaml),
a one-step chain using `evidence-reviewer` to check claims against cited sources.

Ask Maestro to list workflows, show one with its source, or run it with a task.
The files contain no approval step. Optional roles and workflows can be replaced
by same-name local files; policy can require a workflow and its release-local
references. Read [workflows](docs/workflows.md) before changing a chain.

## Layout

Everything Maestro reads sits under `manifests/`, the manifest root. Maestro reads
nothing outside it except the catalog's ownership lookup in `.github/CODEOWNERS`.
Documentation, schemas, catalog blueprints, `.github/` and this README stay
outside the root. Paths in settings and policy, such as `manifestAreas` and
`required`, are relative to the manifest root.

The common root, `manifests/` itself, holds `package.json`, `CHANGELOG.md` and
`settings/`. Core, at `manifests/core/`, holds the base prompt, main persona,
four delegated roles, four guard rule files and three workflows. The optional capability at
`manifests/capabilities/practice/evidence-review/` keeps its native package and
all six resources together. Documentation lives under `docs/`; CODEOWNERS
supplies review and catalog attribution. The complete input inventory follows.

Create an area only when it has real content. Supported additional locations are
`manifests/standards/<domain>/`, `manifests/languages/<name>/` and
`manifests/capabilities/<group>/<name>/`. Each capability has its own
`package.json` and resources. Selection uses its path under the manifest root,
not the last directory name. Group folders organize capabilities; they are not
settings layers.

The opt-in security capability lives at
`manifests/capabilities/security/maestro-security/`. It supplies five roles, an
installable instruction and a skill; it contains no executable components.
The [maestro-security package](https://github.com/Orchestration-Maestro/maestro-packages/tree/main/crates/maestro-security)
owns the assessment runner, tools and transports. Install that package separately
before using these resources. The organization selects
`capabilities/security/maestro-security` in its locked `manifestAreas` setting;
selection alone neither installs the instruction nor starts an assessment.

Maestro's development skill pack lives at
`manifests/capabilities/practice/maestro-skills/`. Maestro releases copy it into
their bundle as a default package, so it loads without selection; an organisation
may also select it here.

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
| [`manifests/CHANGELOG.md`](manifests/CHANGELOG.md) | Release Please changelog, which Maestro reads for the entries between releases. |
| [`manifests/capabilities/practice/evidence-review/agents/evidence-reviewer.md`](manifests/capabilities/practice/evidence-review/agents/evidence-reviewer.md) | Opt-in read-only role for checking factual claims against sources. |
| [`manifests/capabilities/practice/evidence-review/instructions/cite-code-claims.instructions.md`](manifests/capabilities/practice/evidence-review/instructions/cite-code-claims.instructions.md) | Installable documentation guidance with applyTo frontmatter. |
| [`manifests/capabilities/practice/evidence-review/package.json`](manifests/capabilities/practice/evidence-review/package.json) | One opt-in capability package with explicit native resource paths. |
| [`manifests/capabilities/practice/evidence-review/prompts/explain-change.md`](manifests/capabilities/practice/evidence-review/prompts/explain-change.md) | A file/audience explanation template with positional arguments. |
| [`manifests/capabilities/practice/evidence-review/skills/review-evidence/SKILL.md`](manifests/capabilities/practice/evidence-review/skills/review-evidence/SKILL.md) | A source-grounded answer-review procedure. |
| [`manifests/capabilities/practice/evidence-review/themes/review-dark.json`](manifests/capabilities/practice/evidence-review/themes/review-dark.json) | A complete 51-token optional terminal theme. |
| [`manifests/capabilities/practice/evidence-review/workflows/review-evidence.yaml`](manifests/capabilities/practice/evidence-review/workflows/review-evidence.yaml) | Opt-in factual review chain using evidence-reviewer. |
| [`manifests/capabilities/practice/maestro-skills/LICENSE`](manifests/capabilities/practice/maestro-skills/LICENSE) | MIT licence of the skill pack. |
| [`manifests/capabilities/practice/maestro-skills/THIRD-PARTY-NOTICES.md`](manifests/capabilities/practice/maestro-skills/THIRD-PARTY-NOTICES.md) | Copyright and permission notices for adapted skill text. |
| [`manifests/capabilities/practice/maestro-skills/package.json`](manifests/capabilities/practice/maestro-skills/package.json) | Content-only skill pack package; skills load by conventional discovery. |
| [`manifests/capabilities/practice/maestro-skills/references/definition-of-done.md`](manifests/capabilities/practice/maestro-skills/references/definition-of-done.md) | Shared reference read by several pack skills. |
| [`manifests/capabilities/practice/maestro-skills/references/issue-tracker.md`](manifests/capabilities/practice/maestro-skills/references/issue-tracker.md) | Shared reference read by several pack skills. |
| [`manifests/capabilities/practice/maestro-skills/references/phase-boundaries.md`](manifests/capabilities/practice/maestro-skills/references/phase-boundaries.md) | Shared reference read by several pack skills. |
| [`manifests/capabilities/practice/maestro-skills/references/report.md`](manifests/capabilities/practice/maestro-skills/references/report.md) | Shared reference read by several pack skills. |
| [`manifests/capabilities/practice/maestro-skills/references/severity.md`](manifests/capabilities/practice/maestro-skills/references/severity.md) | Shared reference read by several pack skills. |
| [`manifests/capabilities/practice/maestro-skills/references/subagents.md`](manifests/capabilities/practice/maestro-skills/references/subagents.md) | Shared reference read by several pack skills. |
| [`manifests/capabilities/practice/maestro-skills/references/untrusted-input.md`](manifests/capabilities/practice/maestro-skills/references/untrusted-input.md) | Shared reference read by several pack skills. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-ask/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-ask/SKILL.md) | Skill `maestro-ask` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-code-review/RE-REVIEW.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-code-review/RE-REVIEW.md) | Supporting document of `maestro-code-review`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-code-review/REVIEWER.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-code-review/REVIEWER.md) | Supporting document of `maestro-code-review`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-code-review/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-code-review/SKILL.md) | Skill `maestro-code-review` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/CONTRACTS.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/CONTRACTS.md) | Supporting document of `maestro-codebase-design`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/DEEPENING.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/DEEPENING.md) | Supporting document of `maestro-codebase-design`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/DESIGN-IT-TWICE.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/DESIGN-IT-TWICE.md) | Supporting document of `maestro-codebase-design`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-codebase-design/SKILL.md) | Skill `maestro-codebase-design` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-diagnose/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-diagnose/SKILL.md) | Skill `maestro-diagnose` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-diagnose/techniques.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-diagnose/techniques.md) | Supporting document of `maestro-diagnose`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-documentation/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-documentation/SKILL.md) | Skill `maestro-documentation` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-domain-modeling/ADR-FORMAT.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-domain-modeling/ADR-FORMAT.md) | Supporting document of `maestro-domain-modeling`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-domain-modeling/GLOSSARY-FORMAT.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-domain-modeling/GLOSSARY-FORMAT.md) | Supporting document of `maestro-domain-modeling`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-domain-modeling/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-domain-modeling/SKILL.md) | Skill `maestro-domain-modeling` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-frontend-ui/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-frontend-ui/SKILL.md) | Skill `maestro-frontend-ui` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-frontend-ui/accessibility.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-frontend-ui/accessibility.md) | Supporting document of `maestro-frontend-ui`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-git-workflow/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-git-workflow/SKILL.md) | Skill `maestro-git-workflow` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-grilling/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-grilling/SKILL.md) | Skill `maestro-grilling` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-handoff/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-handoff/SKILL.md) | Skill `maestro-handoff` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-humanizer/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-humanizer/SKILL.md) | Skill `maestro-humanizer` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-idea-refine/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-idea-refine/SKILL.md) | Skill `maestro-idea-refine` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-idea-refine/frameworks.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-idea-refine/frameworks.md) | Supporting document of `maestro-idea-refine`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-idea-refine/refinement-criteria.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-idea-refine/refinement-criteria.md) | Supporting document of `maestro-idea-refine`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-implement-spec/BRIEF.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-implement-spec/BRIEF.md) | Supporting document of `maestro-implement-spec`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-implement-spec/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-implement-spec/SKILL.md) | Skill `maestro-implement-spec` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-implement/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-implement/SKILL.md) | Skill `maestro-implement` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-improve-codebase-architecture/REPORT.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-improve-codebase-architecture/REPORT.md) | Supporting document of `maestro-improve-codebase-architecture`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-improve-codebase-architecture/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-improve-codebase-architecture/SKILL.md) | Skill `maestro-improve-codebase-architecture` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-observability/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-observability/SKILL.md) | Skill `maestro-observability` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-observability/checklist.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-observability/checklist.md) | Supporting document of `maestro-observability`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-performance/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-performance/SKILL.md) | Skill `maestro-performance` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-performance/optimization-patterns.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-performance/optimization-patterns.md) | Supporting document of `maestro-performance`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-pr/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-pr/SKILL.md) | Skill `maestro-pr` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-prototype/LOGIC.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-prototype/LOGIC.md) | Supporting document of `maestro-prototype`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-prototype/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-prototype/SKILL.md) | Skill `maestro-prototype` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-prototype/UI.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-prototype/UI.md) | Supporting document of `maestro-prototype`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-quality-gates/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-quality-gates/SKILL.md) | Skill `maestro-quality-gates` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-receive-review/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-receive-review/SKILL.md) | Skill `maestro-receive-review` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-research/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-research/SKILL.md) | Skill `maestro-research` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-retro/FORENSICS.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-retro/FORENSICS.md) | Supporting document of `maestro-retro`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-retro/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-retro/SKILL.md) | Skill `maestro-retro` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-security/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-security/SKILL.md) | Skill `maestro-security` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-security/hardening-patterns.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-security/hardening-patterns.md) | Supporting document of `maestro-security`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-ship/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-ship/SKILL.md) | Skill `maestro-ship` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-show-options/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-show-options/SKILL.md) | Skill `maestro-show-options` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-tdd/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-tdd/SKILL.md) | Skill `maestro-tdd` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-tdd/mocking.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-tdd/mocking.md) | Supporting document of `maestro-tdd`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-tdd/tests.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-tdd/tests.md) | Supporting document of `maestro-tdd`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-teach/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-teach/SKILL.md) | Skill `maestro-teach` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-teach/formats.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-teach/formats.md) | Supporting document of `maestro-teach`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-to-questionnaire/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-to-questionnaire/SKILL.md) | Skill `maestro-to-questionnaire` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-to-spec/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-to-spec/SKILL.md) | Skill `maestro-to-spec` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-to-tickets/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-to-tickets/SKILL.md) | Skill `maestro-to-tickets` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-triage/AGENT-BRIEF.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-triage/AGENT-BRIEF.md) | Supporting document of `maestro-triage`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-triage/OUT-OF-SCOPE.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-triage/OUT-OF-SCOPE.md) | Supporting document of `maestro-triage`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-triage/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-triage/SKILL.md) | Skill `maestro-triage` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-verify/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-verify/SKILL.md) | Skill `maestro-verify` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-wait-what/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-wait-what/SKILL.md) | Skill `maestro-wait-what` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-wayfinder/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-wayfinder/SKILL.md) | Skill `maestro-wayfinder` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-wizard/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-wizard/SKILL.md) | Skill `maestro-wizard` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-wizard/template.sh`](manifests/capabilities/practice/maestro-skills/skills/maestro-wizard/template.sh) | Template the `maestro-wizard` skill copies; loading never runs it. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-writing-for-agents/SKILL-MECHANICS.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-writing-for-agents/SKILL-MECHANICS.md) | Supporting document of `maestro-writing-for-agents`. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-writing-for-agents/SKILL.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-writing-for-agents/SKILL.md) | Skill `maestro-writing-for-agents` of Maestro's development skill pack. |
| [`manifests/capabilities/practice/maestro-skills/skills/maestro-writing-for-agents/TESTING.md`](manifests/capabilities/practice/maestro-skills/skills/maestro-writing-for-agents/TESTING.md) | Supporting document of `maestro-writing-for-agents`. |
| [`manifests/capabilities/security/maestro-security/agents/security-cyber.md`](manifests/capabilities/security/maestro-security/agents/security-cyber.md) | Native security assessment content. |
| [`manifests/capabilities/security/maestro-security/agents/security-probe.md`](manifests/capabilities/security/maestro-security/agents/security-probe.md) | Native security assessment content. |
| [`manifests/capabilities/security/maestro-security/agents/security-proof.md`](manifests/capabilities/security/maestro-security/agents/security-proof.md) | Native security assessment content. |
| [`manifests/capabilities/security/maestro-security/agents/security-rebuild.md`](manifests/capabilities/security/maestro-security/agents/security-rebuild.md) | Native security assessment content. |
| [`manifests/capabilities/security/maestro-security/agents/security.md`](manifests/capabilities/security/maestro-security/agents/security.md) | Native security assessment content. |
| [`manifests/capabilities/security/maestro-security/instructions/security-assessment.instructions.md`](manifests/capabilities/security/maestro-security/instructions/security-assessment.instructions.md) | Native security assessment content. |
| [`manifests/capabilities/security/maestro-security/package.json`](manifests/capabilities/security/maestro-security/package.json) | Native security content package; no runner or extension bundled. |
| [`manifests/capabilities/security/maestro-security/skills/security-assessment/SKILL.md`](manifests/capabilities/security/maestro-security/skills/security-assessment/SKILL.md) | Native security assessment content. |
| [`manifests/core/SYSTEM.md`](manifests/core/SYSTEM.md) | Base system prompt and documentation pointers. |
| [`manifests/core/agents/maestro.md`](manifests/core/agents/maestro.md) | Appended main-agent persona. |
| [`manifests/core/agents/planner.md`](manifests/core/agents/planner.md) | Read-only implementation planning with a concrete handoff and its verification. |
| [`manifests/core/agents/reviewer.md`](manifests/core/agents/reviewer.md) | Shell-free code review: findings with location, cause and fix, and a security pass. |
| [`manifests/core/agents/scout.md`](manifests/core/agents/scout.md) | Compressed code context and exact source ranges. |
| [`manifests/core/agents/worker.md`](manifests/core/agents/worker.md) | Test-first implementation in an isolated delegated context, with each command and exit code reported. |
| [`manifests/core/guards/destructive-commands.yaml`](manifests/core/guards/destructive-commands.yaml) | Confirm rules for recursive removal, `sudo` and mode 777. |
| [`manifests/core/guards/pii.yaml`](manifests/core/guards/pii.yaml) | Redact rules for social insurance numbers, payment cards, email addresses, phone numbers and IBANs. |
| [`manifests/core/guards/prompt-injection.yaml`](manifests/core/guards/prompt-injection.yaml) | Block rules for instructions aimed at the agent inside tool output. |
| [`manifests/core/guards/protected-paths.yaml`](manifests/core/guards/protected-paths.yaml) | Block rules for environment files, the Git folder and installed dependencies. |
| [`manifests/core/workflows/implement-and-review.yaml`](manifests/core/workflows/implement-and-review.yaml) | Worker, reviewer, worker chain with previous-output passing. |
| [`manifests/core/workflows/implement.yaml`](manifests/core/workflows/implement.yaml) | Scout, planner, worker chain using original arguments and previous output. |
| [`manifests/core/workflows/scout-and-plan.yaml`](manifests/core/workflows/scout-and-plan.yaml) | Scout, planner chain that returns a plan without implementing. |
| [`manifests/package.json`](manifests/package.json) | Common native package record; no independent npm version. |
| [`manifests/settings/policy.json`](manifests/settings/policy.json) | The baseline: locks, bounds, allowed models, required core resources and the release-check interval. |
| [`manifests/settings/settings.json`](manifests/settings/settings.json) | The values the locks fix: default model and thinking level, telemetry off, images blocked, no optional areas and no automatic optional updates. |
| [`manifests/settings/size-budget.yaml`](manifests/settings/size-budget.yaml) | Lines allowed in an artifact's main file, by kind, for the scorecard Size score. |
| [`version.txt`](version.txt) | Release Please version record for the shared Git release. |

## How composition works

1. Engine defaults provide the starting values.
2. The manifest reads common, all standards by directory name, core, selected
   languages in list order, then selected capabilities in list order.
3. User settings, project settings and run options apply within policy.
4. Runtime changes pass the same policy before any effect.

A later area can narrow a bound or add a lock, but cannot loosen an earlier one.
Stored conflicting preferences resolve with diagnostics without rewriting the
file. Explicit forbidden changes fail before changing state. The policy here
locks broadly, as [policy](docs/policy.md#the-baseline) lists; it checks for newer
releases at session start and every 30 minutes, subject to `update.versionCheck`
and offline mode. Personal theme selection and display stay editable. Credential
sources and behaviour locations can be governed; the baseline locks them.

Instructions are offered for installation, not automatically appended by area
selection. System prompt, persona and installed instructions are different
artifacts; see their separate guides.

## Documentation

Start with the [documentation index](docs/index.md), in reading order. It covers
settings, policy, prompts, instructions, roles, skills, workflows, guards,
themes, models, extensions, capabilities, ownership, releases, integrity,
schemas and the catalog. It also records the models and extensions exceptions.
[CONTEXT.md](CONTEXT.md) defines the vocabulary; [development](docs/development.md)
describes pinned tools, hooks, CI and delivery.

The [engineering](docs/standards/engineering.md),
[security](docs/standards/security.md) and [Northstar](docs/standards/northstar.md)
pages explain how contributors follow the organization's rules here. They are not
runtime `standards/<domain>/` resources.

## What the organization can lock

The [policy guide](docs/policy.md#what-the-organization-can-lock) names the
settings and resources an organization can constrain. This release's
[baseline](docs/policy.md#the-baseline) fixes the prompt, model defaults and area
selection, and requires core roles and workflows. It also locks local theme and
workflow additions, credential sources and behaviour locations. Personal theme
selection and display stay editable. Repository rulesets govern contributions
separately: required checks, signed commits and owner review do not sandbox
runtime code.

<a id="check-a-contribution"></a>

## Check a change

Run `mise trust`, `mise exec -- just setup` and `mise exec -- just check`, which
runs the content checks, the Rust checks and the tests. The real
[justfile](justfile) owns these recipes; see
[development](docs/development.md#local-checks) for the complete file and hooks.

Start with an issue, keep changes within its scope and follow
[AGENTS.md](AGENTS.md), which this repository maintains by hand.
Public content only: no private collections, vendor material, credentials or
private review evidence. Configuration governance is not a sandbox; extensions
retain the user's access.

## License

[MIT](LICENSE). Artwork and font terms are in the
[banner credits](.github/assets/CREDITS.md).
