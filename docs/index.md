# Manifest documentation

Read these guides to select a release, understand its composition and author
resources in their native formats. Each artifact guide quotes a real release
file, except the explicitly approved model and extension examples below. The
[README artifact index](../README.md#artifact-index) inventories the release inputs;
this page puts the guides in reading order.

## Contents

1. [Start here](#start-here)
2. [Author artifacts](#author-artifacts)
3. [Operate and contribute](#operate-and-contribute)
4. [Example coverage and exceptions](#example-coverage-and-exceptions)
5. [Example](#example)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Check a change](#check-a-change)

## Start here

- [Settings](settings.md) - selection, area order and local preferences.
- [Policy](policy.md) - locks, bounds, required entries and governed keys.
- [System prompt and persona](system-prompt-and-persona.md) - base text, appends and context.
- [Capabilities and ownership](capabilities-and-ownership.md) - opt-in bundles and last-match ownership.

## Author artifacts

- [Instructions](instructions.md) - install selected guidance and track managed copies.
- [Roles](roles.md) - delegated prompts, model preferences and tool ceilings.
- [Skills](skills.md) - discover and invoke reusable procedures.
- [Prompt templates](prompt-templates.md) - expand slash-command text and positional arguments.
- [Workflows](workflows.md) - run the three core chains and understand step contracts.
- [Themes](themes.md) - offer a complete color file without choosing it for users.
- [Models](models.md) - configure providers locally and keep credentials out of releases.
- [Extensions](extensions.md) - package built components using Maestro's real greeting example.

## Operate and contribute

- [Releases and updates](releases-and-updates.md) - pins, required floors, permission growth and rollback.
- [Integrity](integrity.md) - repair checkout drift and installed instruction copies.
- [Catalog](catalog.md) - inspect cards, owners and dependency relations.
- [Schemas and checks](schemas-and-checks.md) - distinguish content parsing, schemas and actual loading.
- [Development](development.md) - pinned tools, hooks, protected review and release automation.

## Example coverage and exceptions

The opt-in `capabilities/practice/evidence-review/` supplies a real package, role,
skill, prompt, theme and workflow. Common supplies settings/policy;
core supplies system/persona text and the three implementation/planning workflows.
No optional area is selected by default.

| Exempt kind or path | Reason | Where to read it |
| --- | --- | --- |
| Instructions (`*.instructions.md`) | Not shipped: no `applyTo` pattern selects review work, so the review rule lives in the evidence-reviewer role and skill | [Instructions](instructions.md#example) |
| Models (`core/llm/models.json`) | Not shipped: it would add providers for every release user; keep the full example local instead | [Models](models.md#example) |
| Extensions (`*.wasm`) | Not shipped: no executable components are added merely for documentation; quote the real external greeting source | [Extensions](extensions.md#example) |
| `docs/standards/*` | Generated rule maps owned by rust-gate; do not hand-edit for authored-page formatting | [Engineering](standards/engineering.md), [security](standards/security.md), [quality](standards/northstar.md) |

These explicit exceptions let the artifact-index check distinguish an intentional
absence from a missing row or file. Models and extensions must not be added to
common or core just to satisfy a documentation count. Generated schemas likewise
come from their consumer, not handwritten examples or empty folders.

## Example

The common settings make the opt-in boundary visible:

From [`settings/settings.json`](../settings/settings.json):

```json
{
  "manifestAreas": [],
  "manifestAutoUpdate": false
}
```

The empty list does not prevent a user from selecting the real evidence-review
capability in their own settings. It keeps that selection out of the default
composition.

## What the organization can lock

Documentation and its contents links impose no runtime locks. Policy can lock
area selection and govern resources as described in [policy](policy.md), while
CODEOWNERS and protected review govern changes to these files. A model/extension
example does not grant permission to add a provider or execute code.

## Check a change

Follow [development](development.md#local-checks). Verify every authored page has
an H1, purpose paragraph and ordered Contents links, and that every quoted file
matches its linked source. Check both directions of the README artifact index:
every row points to a file and every authored artifact has one row. Keep this
reading list and the explicit exceptions in sync with any new artifact kind.
