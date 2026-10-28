# Workflows

A workflow is one `workflows/<name>.yaml` file with `apiVersion:
maestro.workflow/v1` and `kind: Workflow`. The same file works in a manifest area,
`~/.maestro/agent/workflows/` or `.maestro/workflows/`. Maestro's subagent
capability supplies the runner; the manifest supplies files, not another engine.

## Contents

1. [Example](#example)
2. [Format](#format)
3. [Chain text, commands and questions](#chain-text-commands-and-questions)
4. [The three defaults](#the-three-defaults)
5. [Loading rules](#loading-rules)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Usage](#usage)
8. [Validation](#validation)
9. [Check a change](#check-a-change)

## Example

The core chain implements the task, reviews it, then applies the feedback:

From [`manifests/core/workflows/implement-and-review.yaml`](../manifests/core/workflows/implement-and-review.yaml):

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

The run's arguments become the first task. The reviewer's task receives the
worker's final output; the last worker receives the reviewer's final output.
Each role runs in its own context. The result is the last step's output, not a
merged transcript. A handoff should therefore include paths and enough context
for the next role.

The optional evidence-review capability supplies a read-only chain. It is
available only after selecting `capabilities/practice/evidence-review` in
`manifestAreas`:

From [`manifests/capabilities/practice/evidence-review/workflows/review-evidence.yaml`](../manifests/capabilities/practice/evidence-review/workflows/review-evidence.yaml):

```yaml
apiVersion: maestro.workflow/v1
kind: Workflow
name: review-evidence
description: Check factual claims against sources with the evidence-reviewer role
chain:
  - agent: evidence-reviewer
    task: "Check the claims against the cited source passages supplied here and return your report: $@"
```

Supply an answer and its cited source passages as the run's task. The single
step returns the role's report: corrected answer, findings, checks run and checks
not run. It makes no implementation handoff.

## Format

| Member | Contract |
| --- | --- |
| `apiVersion` | Exactly `maestro.workflow/v1` |
| `kind` | Exactly `Workflow` |
| `name` | Lowercase, hyphenated, at most 64 characters, equal to the file stem |
| `description` | Human-readable purpose |
| `agentScope` | Optional: `user`, `project` or `both`; defaults to `user` |
| `chain` or `tasks` | Exactly one nonempty step list |

Unknown members, the wrong version or kind, a mismatched filename, an empty list
or both `chain` and `tasks` are errors. The default agent scope does not authorize
arbitrary project roles; project-role confirmation still applies when used.

Agent steps contain `agent`, `task` and optional `cwd`. `chain` runs in order.
`tasks` runs agent tasks in parallel, with at most eight tasks and at most four
running at once, and returns every result. Do not rely on previous-output passing
between parallel tasks.

## Chain text, commands and questions

Agent tasks expand `$1`, `$@`, `$ARGUMENTS`, `${@:N}` and `${@:N:L}` using
[prompt-template rules](prompt-templates.md). In a chain, `{previous}` expands to
the preceding step's final output. There is no other expression language or
field interpolation. Literal text is fixed; run arguments supply the variable
part.

A chain step may instead contain `command`, a literal argument vector run
without a shell, or `ask`, a message whose output is the user's typed answer.
Arguments and `{previous}` expand within each string. A command returns standard
output and fails on a nonzero exit. `ask` fails on cancellation or without an
interactive terminal.

Without a shell, pipes, redirects and `&&` are not interpreted. Every step still runs within the
ordinary model, tool and role policy. A failed, refused or cancelled step stops
the chain; later steps do not run. The diagnostic is
`Workflow stopped at step <n> (<step>): <message>`.

## The three defaults

| File | Ordered roles | Passing and final result |
| --- | --- | --- |
| [implement-and-review.yaml](../manifests/core/workflows/implement-and-review.yaml) | worker, reviewer, worker | Implement arguments; review previous implementation; apply previous feedback |
| [implement.yaml](../manifests/core/workflows/implement.yaml) | scout, planner, worker | Scout arguments; plan for original arguments with previous context; implement previous plan |
| [scout-and-plan.yaml](../manifests/core/workflows/scout-and-plan.yaml) | scout, planner | Scout arguments; plan for original arguments with previous context; return plan without implementation |

These chains have no approval, test, commit or release step of their own. The
planner and reviewer only read; the worker implements and commits its failing
test first. Their [role definitions](roles.md) specify the output passed to the
next step.

## Loading rules

Definitions load from manifest areas in area order, then user, then project. A
later same-name optional definition replaces the whole earlier workflow. New
local workflows are always permitted; policy cannot lock all workflow additions.

## What the organization can lock

A workflow named under `required` cannot be disabled or shadowed. Its roles,
prompt files and other references resolve only from the same release. A local
role with the same name cannot replace a role inside that closure. Literal text
in a required workflow stays fixed, but declared positional arguments remain
per-run inputs. Role model and thinking choices remain within role policy.

## Usage

Ask Maestro to list workflows with sources, show `implement`, check a workflow
file, or run `scout-and-plan` with a concrete task. The contract does not define
a separate CLI flag or slash-command spelling for the runner; use the installed
runner's listing/help rather than inventing one.

## Validation

The YAML parser checks syntax; schema validation checks members when delivered.
The runner and manifest loader must check role resolution and required-reference
closure. A parse success does not show that an agent or command can run.

## Check a change

Run the [available repository checks](schemas-and-checks.md#available-checks).
In a compatible build, check every default with the runner, verify roles resolve,
and run `scout-and-plan` on a disposable project. It must return a plan without
editing. Run the implementation chains on a disposable change and inspect
handoffs. Without the optional capability, confirm `review-evidence` is absent;
select the capability and run it with an answer and source passages. It must
return the report without editing files. Exercise a deliberately
failing command in a separate local workflow and confirm no later step runs.
Requiring a workflow also needs the loader's
same-release closure check; parsing YAML alone is insufficient.
