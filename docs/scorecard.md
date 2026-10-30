# Scorecard

A scorecard rates each manifest artifact. Conflicts and duplicates fail the
check; every other score is advice shown on the card and never fails it. This
page defines the card, the blocking rules, each score and how to improve it. It
also separates what runs today from what waits on another component.

## Contents

1. [The card](#the-card)
2. [Conflicts that block](#conflicts-that-block)
3. [Scores that advise](#scores-that-advise)
4. [Improve a score](#improve-a-score)
5. [Workspace and workzone alignment](#workspace-and-workzone-alignment)
6. [What runs today](#what-runs-today)
7. [What the organization can lock](#what-the-organization-can-lock)
8. [Check a change](#check-a-change)

## The card

Each pull request gets one card per changed artifact: role, skill, instruction,
prompt, workflow, guard rule and capability. The release gets a card for every
artifact. A card lists the blocking result first, then the scores, each with its
value and the next action that raises it.

| Part | Content | Effect |
| --- | --- | --- |
| Conflicts | Each conflict found, naming both artifacts and the rule | Fails the check |
| Checkability | Share of rules that say how they are checked | Advice |
| Alignment | Golden rules, workspace and workzone, CODEOWNERS | Advice |
| Voice | Maestro's voice lint | Advice |
| Size | Length against the artifact's budget | Advice |
| Benchmark | One headless test per changed skill or role | Advice |

## Conflicts that block

The check reads the whole manifest root, so a pull request that adds one side of
a conflict fails and the message names both artifacts. A name is unique per kind:
a role `review` and a workflow `review` do not conflict. It reads roles, skills,
instructions, prompts, workflows, themes, guard rules and each capability's
`package.json`.

Each kind is found where the loaders find it, at the root of an area: `agents/`,
`instructions/`, `prompts/`, `workflows/`, `themes/`, `guards/` and `skills/`
under `core/` or under `capabilities/<group>/<name>/`. A file below a skill's
folder, such as `skills/audit/agents/grader.md`, belongs to the skill and is
not another artifact. A skill is a `SKILL.md` that no outer `SKILL.md` encloses,
or a Markdown file directly in `skills/` when `skills/` has no `SKILL.md`. Hidden
folders and files, and `node_modules`, are skipped. A skill is named by its
`name`, or by its folder when it has none or an empty one. A file extension must
match in case: `explain.MD` is not a prompt. A guard rule is named by its `id` and
reported as `<file>#<id>`.

| Rule | Fails when | Example |
| --- | --- | --- |
| `same-name` | One kind and name appears at two places of the release, in one area or in two, whether or not either is required | A capability role named `reviewer` beside the core `reviewer` |
| `same-capability-name` | Two capabilities have one `package.json` name | Two groups each holding a package named `maestro-review` |
| `same-content` | Two artifacts of one kind are equal after the name is removed; Markdown bodies also have whitespace collapsed | Two roles with the same prompt under two names, or two guard rules that differ only in their `id` |

One organization authors the whole release, and selection uses unique names.
Run-time shadowing, where a later definition replaces an earlier one, applies to
layers outside the release, such as a user's or a project's local files. It never
applies between areas of one release. The Maestro specification records this in
[maestro#3680](https://github.com/Orchestration-Maestro/maestro/issues/3680).

Instructions with different `applyTo` scopes are not duplicates. Two capability
folders with one folder name in different groups are not a conflict, because
selection uses `capabilities/<group>/<name>`.

Contradicting rules do not block yet: two instructions or roles requiring opposite
things in one scope. This waits on a definition, tracked in
[issue 54](https://github.com/Orchestration-Maestro/maestro-manifests/issues/54).

## Scores that advise

| Score | Measures | Source of the value |
| --- | --- | --- |
| Checkability | Rules that state how they are checked, divided by all rules | The artifact's own text |
| Golden rules | Rules that cite an engineering, security or Northstar golden rule | The [standards pages](standards/engineering.md) |
| Alignment | Whether the capability matches the organization's workspace, workzone and CODEOWNERS teams | The folder path and `.github/CODEOWNERS` |
| Voice | Findings of Maestro's voice lint | The artifact's text |
| Size | Length against its budget | The artifact's text |
| Benchmark | Deterministic score of one test per changed skill or role, kept per release so a drop is visible | A headless Maestro run on a local model |

## Improve a score

1. Checkability: add the check to each rule, in the form "Checked by: ...". A
   rule with no check is advice a reviewer cannot confirm.
2. Golden rules: name the golden rule each rule serves, as the
   [system prompt](system-prompt-and-persona.md) does.
3. Alignment: see the next section.
4. Voice: put the result first and then the next action; cut words that add no
   fact.
5. Size: move detail into a linked page and keep the artifact to one job.
6. Benchmark: make the skill or role state its expected output, so the test can
   score it deterministically.

## Workspace and workzone alignment

Maestro reads folders only, never GitHub properties. The organization maps its
structure onto the folders: `capabilities/<group>/<name>/` is a workspace and a
workzone. `.github/CODEOWNERS` gives the group folder to the workspace team and
the capability folder to the workzone team, so access and ownership come from
GitHub. The card flags a capability when:

- its group is not a workspace of the organization;
- its name is not a workzone of that workspace;
- CODEOWNERS does not give the group to the workspace team;
- CODEOWNERS does not give the capability to the workzone team.

This release's CODEOWNERS names one approved principal for every path. Until the
organization authorizes real team names, every capability shows the CODEOWNERS
flag. See [capabilities and ownership](capabilities-and-ownership.md#ownership).

## What runs today

| Part | State |
| --- | --- |
| Conflict check | Runs in `just check` (the `scorecard` tests) and in the required content workflow |
| Card rendering | Waits on the catalog ([catalog](catalog.md)) |
| Advisory scores | Defined here; computed once the card renders |
| Benchmark | Waits on in-process inference in Maestro, which the local model needs |
| Contradicting rules | Waits on a definition |

Report a part that waits as a documented contract, not as a passing check.

## What the organization can lock

The scorecard changes no setting and grants no permission. Only conflicts fail a
check, and the repository rulesets decide who may merge. Scores never override
policy: a high score does not select a capability, and a low score does not
remove one. Required resources stay required.

## Check a change

Run `mise exec -- just check`; it runs the conflict check on the whole manifest
root and on the cases under `tests/scorecard_cases/`. Add a case to that folder
for each new conflict rule: one `expected.json` with the status and the artifacts
the message must name, and a release that holds them. Confirm a failing case
names both artifacts and the rule. Keep the table of rules above in step with
the cases.
