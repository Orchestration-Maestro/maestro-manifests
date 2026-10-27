# Catalog

The catalog presents manifest entries as cards and a dependency graph. Maestro's
manifest consumer supplies blueprint and relation data; this repository supplies
the authored artifacts and CODEOWNERS that data describes. The catalog is a view,
not another copy of the configuration or an agent-session runtime.

## Complete example

Consider the actual core workflow in this release:

```yaml
apiVersion: maestro.workflow/v1
kind: Workflow
name: implement-and-review
description: Worker implements, reviewer reviews, worker applies feedback
chain:
  - agent: worker
    task: "Implement: $@"
  - agent: reviewer
    task: "Review the implementation from the previous step: {previous}"
  - agent: worker
    task: "Apply the feedback from this review: {previous}"
```

With the actual catch-all ownership rule:

```text
* @FrancoisLDaigneault
```

A compatible catalog can derive the following view without a separate entity
file:

```text
manifest release
  contains -> core area
                contains -> implement-and-review workflow
                              uses -> worker role
                              uses -> reviewer role

implement-and-review
  source: core/workflows/implement-and-review.yaml
  description: Worker implements, reviewer reviews, worker applies feedback
  required: false under the shipped policy
  owner: @FrancoisLDaigneault
```

The worker runs twice but is one referenced role entity. The release field comes
from the selected Git pin, not a workflow-level version. This example describes
derived data; it is not a new catalog serialization format to commit.

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

## Ownership, locks and overrides

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

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). In a build
with the catalog consumer, open the pinned release and inspect the example's
card, owner and role edges. Check a required test policy changes the displayed
state and prevents direct editing. Test CODEOWNERS ordering with a specific rule
followed by a broader one; the last match must win. Verify an unmatched example
shows unowned rather than inheriting invented team metadata. Until that consumer
is present, report this as a documented contract, not a passing UI test.
