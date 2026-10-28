# Catalog

The catalog presents manifest entries as cards and a dependency graph. Maestro's
manifest consumer supplies blueprint and relation data; this repository supplies
the authored artifacts and CODEOWNERS that data describes. The catalog is a view,
not another copy of the configuration or an agent-session runtime.

## Contents

1. [Example](#example)
2. [Blueprints and relations](#blueprints-and-relations)
3. [What the organization can lock](#what-the-organization-can-lock)
4. [Usage](#usage)
5. [Validation](#validation)
6. [Check a change](#check-a-change)

## Example

A workflow card can be derived from this actual core file:

From [`core/workflows/implement-and-review.yaml`](../core/workflows/implement-and-review.yaml):

```yaml
apiVersion: maestro.workflow/v1
kind: Workflow
name: implement-and-review
description: Implement a task, check the change and apply the review
chain:
  - agent: worker
    task: "Complete this implementation task: $@"
  - agent: reviewer
    task: "Check this implementation for defects: {previous}"
  - agent: worker
    task: "Update the implementation to address these review findings: {previous}"
```

It uses the worker and reviewer roles. The worker runs twice but is one role
entity; the catalog should not create a second role card for the last step.
With this release's CODEOWNERS, the owner is `@FrancoisLDaigneault`. The common
policy does not require the workflow, so its card is optional. The selected Git
pin supplies its release property; no workflow-level version is needed.

The evidence-review capability similarly provides its role, skill, prompt and
theme. Those relations come from real paths and native package
entries, not a second catalog entity file committed beside each resource.

## Blueprints and relations

The manifest consumer declares blueprints for manifest release, capability group,
capability, policy, role, workflow and instruction, alongside package, skill and
prompt kinds. Each describes properties such as name, description, source area,
release, required/locked state and whether an installed copy is modified locally.

| Source | Relation | Target |
| --- | --- | --- |
| Release | requires; contains | Packages; areas |
| Capability group | contains | Capabilities |
| Capability | provides | Skills, prompts, roles, instructions, workflows |
| Workflow | uses | Roles and prompts |
| Policy | locks, bounds, requires | Governed entries |
| Every entry | owned by | Its CODEOWNERS principal |

Cards show those properties and relations. The graph draws the same relations so
users can inspect what a capability provides and what a required workflow needs.
The catalog reads the installed release when a page opens and keeps no duplicate
content store. Blueprint implementation belongs to the catalog/manifest
consumers; do not add a second hand-maintained ownership or dependency index here.

## What the organization can lock

The release's `.github/CODEOWNERS` supplies ownership using GitHub's last matching
pattern. An unmatched entry is unowned. The catch-all in this repository avoids
that; explicit group and capability rules document their ownership too. Teams
can replace that approved principal only after the organization authorizes real
team names. See [capabilities and ownership](capabilities-and-ownership.md).

Locked entries are read-only in the catalog; edits go through the owning
maintainers and pull requests. A catalog card does not grant permission to bypass
policy or update a pin. Optional artifacts can still be customized through their
ordinary local routes; required workflows retain their release-local closure.
Ownership is attribution and review responsibility, not permission to execute
unreviewed extension code.

The browser application remains in internal mode and has no package installer.
Browsing a manifest in the catalog is not selecting it as the browser agent's
runtime configuration.

## Usage

In a build with the catalog consumer, open the installed release, inspect a
workflow card, then follow its role relations. Browse a capability to see what
it provides and who owns it. Changes still go through the owning maintainers;
browsing a card neither installs the capability nor changes the session pin.

## Validation

Check the source file and CODEOWNERS match before attributing a card. Blueprint
and relation implementation belongs to the catalog/manifest consumers. Current
content checks can validate inputs but cannot prove rendered cards or graph
edges; report that distinction when those consumers are unavailable.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). In a build
with the catalog consumer, open the pinned release and inspect the example's
card, owner and role edges. Check a required test policy changes the displayed
state and prevents direct editing. Test CODEOWNERS ordering with a specific rule
followed by a broader one; the last match must win. Verify an unmatched example
shows unowned rather than inheriting invented team metadata. Until that consumer
is present, report this as a documented contract, not a passing UI test.
