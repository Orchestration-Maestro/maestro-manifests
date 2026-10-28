# Capabilities and ownership

A capability is one opt-in package at `manifests/capabilities/<group>/<name>/`, bundling a
team's authored resources. The group is navigation, not a settings layer, and
ownership comes from the release's CODEOWNERS. Maestro reads these paths, not
GitHub repository properties.

## Contents

1. [Locations](#locations)
2. [Example](#example)
3. [Format](#format)
4. [Usage](#usage)
5. [Loading rules](#loading-rules)
6. [Ownership](#ownership)
7. [What the organization can lock](#what-the-organization-can-lock)
8. [Validation](#validation)
9. [Check a change](#check-a-change)

## Locations

Both group and capability names are lowercase, hyphenated and at most 64
characters. The example lives at `manifests/capabilities/practice/evidence-review/`.
The separately maintained security capability is not included here; this
revision does not copy that lane's branch or claim its migration is complete.

Only core may contain `SYSTEM.md`, the main `maestro` role, models or MCP
configuration. A capability can include skills, instructions, prompts, roles,
workflows, themes and built extensions once the corresponding consumer supports
them. Do not create empty artifact folders or a `knowledge/` format without a
consumer.

## Example

The evidence-review capability is one package with explicit native resource paths:

From [`manifests/capabilities/practice/evidence-review/package.json`](../manifests/capabilities/practice/evidence-review/package.json):

```json
{
  "name": "maestro-evidence-review",
  "private": true,
  "description": "Read-only evidence review guidance and a terminal theme.",
  "license": "MIT",
  "maestro": {
    "skills": [
      "./skills/review-evidence"
    ],
    "prompts": [
      "./prompts/explain-change.md"
    ],
    "themes": [
      "./themes/review-dark.json"
    ]
  }
}
```

Its conventional `agents/`, `instructions/` and `workflows/` files are discovered
by their owning consumers, not by an invented ownership descriptor. It ships one
[evidence-reviewer role](../manifests/capabilities/practice/evidence-review/agents/evidence-reviewer.md),
[skill](../manifests/capabilities/practice/evidence-review/skills/review-evidence/SKILL.md),
[instruction](../manifests/capabilities/practice/evidence-review/instructions/cite-code-claims.instructions.md),
[prompt](../manifests/capabilities/practice/evidence-review/prompts/explain-change.md),
[theme](../manifests/capabilities/practice/evidence-review/themes/review-dark.json) and
[workflow](../manifests/capabilities/practice/evidence-review/workflows/review-evidence.yaml).

## Format

`package.json` is the native package record. Paths in its `maestro` arrays are
relative to the package root, and every entry above has a real file or directory.
This private in-tree package has no independent npm version: its Git release is
the versioned container. It has no lifecycle scripts, credentials or executable
components.

## Usage

In your own user/project settings, select this pinned manifest. The policy locks
`manifestAreas`, so the organization adds `capabilities/practice/evidence-review`
(a path under the manifest root) to it in `manifests/settings/settings.json`. See [settings usage](settings.md#usage). The
common file leaves the list empty, so this capability is not active merely because
its files exist in the checkout. An absent area or the short name
`evidence-review` is not a valid selection.

Once selected, invoke `/skill:review-evidence`, use `/explain-change`, delegate to
`evidence-reviewer`, run the `review-evidence` workflow, or choose `review-dark`.
Supply the workflow with the answer and cited source passages to check; its
single role returns its report. The review rule lives in that role and the
skill, so it applies only when evidence is reviewed. The instruction still needs
explicit installation; selecting the capability does not inject it
automatically.

## Loading rules

Selected capabilities load after selected languages and after common, standards
and core. A group folder contributes no settings. Each capability contributes
its own native resources, settings and policy when present; this example adds
no settings or policy. Optional local resources can replace optional same-name
resources under their kind's ordinary rules.

Capabilities in this repository share one release pin. A capability needing an
independent release cycle must live in a separate Git/npm package, required by
pinned `packages` settings. Package-owned role/workflow filters use the owning
consumer and ordinary package filter semantics. A manifest selection itself is
always pinned Git, even though ordinary packages can use npm.

## Ownership

Review and catalog principals come from this file:

From [`.github/CODEOWNERS`](../.github/CODEOWNERS):

```text
# The last matching pattern supplies review and catalog ownership.
* @FrancoisLDaigneault

# Explicit ownership for the capability group and capability paths under the
# manifest root.
/manifests/capabilities/security/ @FrancoisLDaigneault
/manifests/capabilities/security/maestro-security/ @FrancoisLDaigneault

/manifests/capabilities/practice/ @FrancoisLDaigneault
/manifests/capabilities/practice/evidence-review/ @FrancoisLDaigneault
```

The approved principal is `@FrancoisLDaigneault`; no unverified team is assigned.
A group rule precedes its capability rule because GitHub's last matching pattern
wins. The catch-all covers all other files, including documentation and tooling.
A later broad rule can override a specific rule, so review ordering explicitly.

An organization with real teams can map a group folder to its workspace team and
a capability folder to its workzone team. For example, an authorized
`@example-org/practice` team could own the practice group and
`@example-org/evidence-review` its capability, with the specific rule last.
Those example teams are not configured here. Confirm repository access before
assigning any real principal. Ownership is review attribution, not runtime
permission or folder-level confidentiality.

Sessions do not read CODEOWNERS. The [catalog](catalog.md) derives ownership from
it; individual artifacts must not invent their own ownership fields.

## What the organization can lock

Capabilities and languages can only add or narrow earlier constraints. They
cannot loosen a lock or bound, or set/constrain `manifest`, `manifestAreas`,
`minimumRelease` or `updateCheckMinutes`. Common/core can lock area selection.
Required resources and required workflow closures cannot be shadowed locally.
See the full [policy table](policy.md#what-the-organization-can-lock).

## Validation

Check group/name spelling, native resource paths, selected-area existence and
CODEOWNERS principals/order. Schema validation checks structure when available;
loading the release checks policy combinations and reference closure. A package
file alone does not prove an instruction was installed or a workflow can run.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). Confirm every
new artifact appears in the README index and every quoted example matches its
file. Load the pin once without optional areas and once with evidence-review;
verify the resources appear only in the selected case and the theme is not
chosen automatically. Check catalog ownership against the last matching rule.
