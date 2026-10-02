<p align="center">
  <img src=".github/assets/maestro-manifests.jpg" alt="Maestro Manifests: Declare once. Govern everywhere. A copper seal press above a glowing mold in a pattern vault." width="100%" />
</p>

<h1 align="center">🧭 Maestro Manifests</h1>

<p align="center">
  <strong>Declare once. Govern everywhere.</strong><br />
  Maestro's organization configuration and capability marketplace: one owner-first catalog of workspace resources.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Schema-maestro--source%2F2-C98C62?style=for-the-badge" alt="Schema: maestro-source/2" />
  <img src="https://img.shields.io/badge/Catalog-owner--first-080A0D?style=for-the-badge" alt="Owner-first catalog" />
  <img src="https://img.shields.io/badge/Format-TOML%20%2B%20Markdown-C98C62?style=for-the-badge" alt="TOML and Markdown" />
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/Licence-MIT-F3E9DC" alt="MIT licence" /></a>
</p>

## ⚡ Quick start

Declare an evidence-review capability in four files under
`capabilities/practice/review/`. The package selects its resources by qualified
ID; a directory name alone selects nothing.

### 1. Declare the package

`capabilities/practice/review/package.toml`:

```toml
kind = "package"
name = "review"
version = "1.2.3"
owners = ["@synthetic/knowledge"]
maintainers = ["@synthetic/maintainer", "@synthetic/other"]
description = "Review answers against their evidence."
status = "active"

[metadata]
schema = "maestro-source/2"
maturity = "reviewed"
rows = ["chat.M036 objects"]
workflows = ["evidence-review"]
requires = ["skill:review/cite-evidence", "instructions:review/evidence"]
```

### 2. Add the skill

`capabilities/practice/review/skills/cite-evidence/SKILL.md`:

```markdown
---
name: cite-evidence
description: Cite the evidence supporting an answer.
metadata:
  maestro.schema: maestro-source/2
  maestro.maturity: reviewed
  maestro.rows: chat.M019 descriptor
  maestro.workflows: evidence-review
---

Cite every answer with the evidence passage it rests on.
```

A skill keeps its Maestro metadata inside `SKILL.md`. List-valued metadata uses
strings separated by `;`, not YAML arrays.

### 3. Add the instructions

`capabilities/practice/review/instructions/evidence.instructions.md`:

```markdown
---
applyTo: "**"
---

Answer from the knowledge tools and cite their evidence.
```

### 4. Pair the instruction sidecar

`capabilities/practice/review/instructions/evidence.maestro.toml`:

```toml
schema = "maestro-source/2"
maturity = "reviewed"
rows = ["chat.M036 objects"]
workflows = ["evidence-review"]
requires = []
```

These files adapt the checker's synthetic fixtures. Replace the synthetic area
principals with approved GitHub identities and obtain protected review before
using `reviewed` maturity. A maturity label is a checked declaration, not review
approval or runtime permission.

- `schema`: metadata format version; the checker reads `maestro-source/2`.
- `maturity`: declared review level; selected resources must be `reviewed`.
- `rows`: architecture requirement IDs naming the behaviors or objects this resource covers.
- `workflows`: usage labels naming the workflows this resource serves.
- `requires`: qualified resource IDs declaring dependencies; usage labels select nothing.

## 🎯 Objectives

- **Declare once:** keep native resource content and its checked metadata together.
- **Owner-first:** group common, framework, language, standard and team resources by area.
- **Reviewed before use:** admit only reviewed members to a selected dependency closure.
- **Replaceable:** select explicit qualified IDs; removing a package exposes its dependants.
- **Data, not code:** descriptors define formats; reviewed checker hooks supply validation.

## 🗂 Layout

Catalog paths follow the owning area. This example uses the same `review`
package as the quick start:

```text
maestro-manifests/
├── package.toml                            # package:common
├── skills/<name>/SKILL.md
├── core/
│   ├── package.toml                        # package:core
│   ├── agents/{<name>.agent.md,<name>.maestro.toml}
│   ├── skills/<name>/SKILL.md
│   ├── instructions/{<name>.instructions.md,<name>.maestro.toml}
│   └── llm/models/<role>/<name>.toml
├── languages/<language>/
│   ├── package.toml                        # language:<language>
│   ├── skills/<name>/SKILL.md
│   └── instructions/{<name>.instructions.md,<name>.maestro.toml}
├── standards/<domain>/
│   ├── package.toml                        # standard:<domain>
│   ├── skills/<name>/SKILL.md
│   └── instructions/{<name>.instructions.md,<name>.maestro.toml}
├── capabilities/<group>/review/
│   ├── package.toml                        # package:review
│   ├── agents/{<name>.agent.md,<name>.maestro.toml}
│   ├── skills/cite-evidence/SKILL.md
│   ├── instructions/{evidence.instructions.md,evidence.maestro.toml}
│   └── llm/models/<role>/<name>.toml
└── presets/<name>.toml
```

Braces mean separate files. Omit unused directories. Group folders provide
navigation, not inherited ownership. Area namespaces are globally unique;
moving a package between groups preserves its IDs.

### What each root holds

| Root | Responsibility |
| --- | --- |
| Root `package.toml` and `skills/` | The `common` area: shared procedures and its ownership record |
| `core/` | Framework agents, skills, instructions and model cards under `package:core` |
| `languages/<language>/` | A language's ownership record, skills and instructions; selected as `language:<language>` |
| `standards/<domain>/` | A standard domain's ownership record, skills and instructions; selected as `standard:<domain>` |
| `capabilities/<group>/<name>/` | One self-contained team package, with its ownership record and resources together |
| `presets/` | Named selections with qualified `requires`, optional inventory selectors and checked settings |

Public catalog content stays separate from private inputs. No private or vendor
content belongs in this repository.

## 🧩 Resource kinds

Paths below are relative to the owning area, except presets. `common` and `core`
are reserved area names; each language, standard and team package has its own
namespace.

| Kind | File | Maestro metadata | ID |
| --- | --- | --- | --- |
| Package | `package.toml` at the root, in `core/` or in a team package | `[metadata]` in the file | `package:common`, `package:core`, `package:review` |
| Language | `languages/<name>/package.toml`, `kind = "language"` | `[metadata]` in the file | `language:rust` |
| Standard | `standards/<name>/package.toml`, `kind = "standard"` | `[metadata]` in the file | `standard:security` |
| Agent | `agents/<name>.agent.md` in core or a team package | `<name>.maestro.toml` beside it | `agent:core/maestro` |
| Skill | `skills/<name>/SKILL.md` in any area | `metadata.maestro.*` strings inside `SKILL.md`; no sidecar | `skill:review/cite-evidence` |
| Instructions | `instructions/<name>.instructions.md` in core, team, language or standard areas | `<name>.maestro.toml` beside it | `instructions:review/evidence` |
| Model card | `llm/models/<role>/<name>.toml` in core or a team package | `[metadata]` in the file | `model-card:core/search-encoder` |
| Preset | `presets/<name>.toml` | `[metadata]` in the file | `preset:knowledge-client` |

Source agent and skill `name` stays local and matches its file stem or directory.
Resource IDs use `kind:namespace/local-name`; area and preset roots use
`kind:name`. Each segment is lowercase and hyphenated, at most 64 characters.
Model-card roles are `embedder`, `reranker` and `answerer`; the path role must
match the kernel-validated identity.

Ownership comes only from the area's `package.toml`: a nonempty `owners` list
and optional `maintainers`. Every area record needs owners; a resource cannot
name its own `owner`, `owners` or `maintainers`. Package versions are exact
SemVer; a supplied metadata version must agree with the top-level version.

## 🔒 Rules

The checker refuses the inputs below. Proofs link to Maestro Core at
[`ad5cf89`](https://github.com/Orchestration-Maestro/maestro-core/tree/ad5cf89/crates/maestro-catalog).
The `file:line` references are relative to `crates/maestro-catalog/src/source/tests/`.

| Rule | What the checker refuses | Proof |
| --- | --- | --- |
| Layout | Nested or unknown areas, unregistered nonempty trees, links and stray resource files | [`nested_or_unknown_area_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/layout.rs#L123), `layout.rs:123`; [`unsupported_kinds_stray_entries_and_links_are_refused`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/layout.rs#L46), `layout.rs:46` |
| Native file pairing | An agent name that differs from its stem, or agent/instruction Markdown without its sidecar | [`agent_name_must_equal_its_stem_and_pair_one_sidecar`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/layout.rs#L13), `layout.rs:13` |
| Source versions | Old or mixed layouts and envelopes other than `maestro-source/2` | [`old_or_mixed_layout_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/qualified.rs#L107), `qualified.rs:107`; [`schema_stage_owner_rows_and_workflows_are_checked`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/schema.rs#L96), `schema.rs:96` |
| Package versions | A non-exact SemVer or disagreement between package and metadata versions | [`package_fields_and_path_refuse_invalid_neighbours`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/area_packages.rs#L128), `area_packages.rs:128`; [`package_version_overlap_refuses_disagreement`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/area_regressions.rs#L14), `area_regressions.rs:14` |
| Qualified names | Unqualified or malformed IDs, duplicate full IDs and duplicate area namespaces | [`old_and_malformed_ids_refuse_without_rebinding`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/qualified.rs#L137), `qualified.rs:137`; [`duplicate_kind_namespace_name_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/qualified.rs#L71), `qualified.rs:71`; [`duplicate_area_namespace_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/qualified.rs#L85), `qualified.rs:85` |
| Dependency layers | Common or standards requiring core; core or languages requiring a team package | [`common_to_core_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/layer_placements.rs#L167), `layer_placements.rs:167`; [`core_to_team_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/layer_placements.rs#L230), `layer_placements.rs:230`; [`language_to_team_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/layer_placements.rs#L235), `layer_placements.rs:235` |
| Dependency integrity | Missing resources, cycles and surviving references to a removed package | [`dangling_references_and_tools_are_refused`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/references.rs#L23), `references.rs:23`; [`dependency_cycles_are_refused`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/references.rs#L78), `references.rs:78`; [`removed_package_dangling_reference_refuses`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/references.rs#L275), `references.rs:275` |
| Reviewed selection | A selected closure containing placeholder, authored or retired resources | [`closure_members_must_be_reviewed_beside_a_reviewed_neighbour`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/references.rs#L107), `references.rs:107`; [`area_roots_require_reviewed_transitive_members`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/area_regressions.rs#L182), `area_regressions.rs:182` |
| Ownership record | An area record without owners, or invalid or duplicate owner/maintainer principals | [`area_owners_maintainers_validate`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/ownership.rs#L13), `ownership.rs:13`; [`registered_area_requires_ownership_record`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/area_ownership.rs#L14), `area_ownership.rs:14` |
| Resource ownership | Resource-level `owner`, `owners` or `maintainers`, with a diagnostic pointing to the area's `package.toml` | [`resource_ownership_is_derived`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/ownership.rs#L76), `ownership.rs:76`; [`legacy_owner_missing_maturity_points_to_area`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/ownership.rs#L454), `ownership.rs:454` |
| Review protection | A later matching rule that removes owners-only protection from a protected path | [`broad_codeowners_rule_cannot_override_descriptor`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/ownership.rs#L167), `ownership.rs:167`; [`last_match_reprotection_accepts`](https://github.com/Orchestration-Maestro/maestro-core/blob/ad5cf89/crates/maestro-catalog/src/source/tests/ownership.rs#L399), `ownership.rs:399` |

Owners-only protection follows the last matching review rule. A later
owners-only rule can restore protection; repairing one file does not protect
an entire tree.

Checking a declaration does not execute its content, grant permission or approve
its publisher. Protected review and runtime authorization are separate controls.

## 📚 Documentation

- [Agent instructions and repository boundaries](AGENTS.md)
- [Engineering rules](docs/standards/engineering.md)
- [Security rules](docs/standards/security.md)
- [Quality targets](docs/standards/northstar.md)
- [Banner credits](.github/assets/CREDITS.md)

## Licence

[MIT](LICENSE). Third-party artwork and font terms are documented in the
[banner credits](.github/assets/CREDITS.md).
