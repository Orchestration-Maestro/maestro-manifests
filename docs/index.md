# Manifest documentation

Read these guides to select a release, understand its composition and author
resources in their native formats. Each artifact guide quotes a real release
file, except the explicitly approved model and extension examples below. The
[README artifact index](../README.md#artifact-index) inventories the release inputs;
this page puts the guides in reading order. All manifest content sits under
`manifests/`, the manifest root; these guides, the checks and the README stay
outside it.

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
- [Scorecard](scorecard.md) - conflicts that block, scores that advise and how to raise them.
- [Schemas and checks](schemas-and-checks.md) - distinguish content parsing, schemas and actual loading.
- [Development](development.md) - pinned tools, hooks, protected review and release automation.
- [Engineering standards](standards/engineering.md) - how this repository follows the organization's engineering rules.
- [Security standards](standards/security.md) - what the repository protects and how its security rules are held.
- [Northstar](standards/northstar.md) - the repository's point and one measured KPI per pillar.

## Example coverage and exceptions

The opt-in `manifests/capabilities/practice/evidence-review/` supplies a real package, role,
skill, instruction, prompt, theme and workflow. Common supplies settings/policy;
core supplies system/persona text and the three implementation/planning workflows.
No optional area is selected by default.

| Exempt kind or path | Reason | Where to read it |
| --- | --- | --- |
| Models (`manifests/core/llm/models.json`) | Not shipped: it would add providers for every release user; keep the full example local instead | [Models](models.md#example) |
| Extensions (`*.wasm`) | Not shipped: no executable components are added merely for documentation; quote the real external greeting source | [Extensions](extensions.md#example) |

These explicit exceptions let the artifact-index check distinguish an intentional
absence from a missing row or file. Models and extensions must not be added to
common or core just to satisfy a documentation count. Generated schemas likewise
come from their consumer, not handwritten examples or empty folders.

## Example

The common settings make the opt-in boundary visible:

Excerpt from [`manifests/settings/settings.json`](../manifests/settings/settings.json):

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

The empty list keeps the real evidence-review capability out of the default
composition. The policy locks `manifestAreas`, so only the organization can add it.

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

For each changed requirement, report its enforcing check or the review evidence:
the input exercised, expected result and observed result. The artifact guides'
Validation and Check a change sections name those checks. Content tests do not
stand in for runtime checks. If a consumer is unavailable, name the check not run
and keep the release claim unverified. For a wording change, compare limits,
defaults, conditions and local customization routes before and after; review
rejects a lost capability even when the new wording passes content checks.
