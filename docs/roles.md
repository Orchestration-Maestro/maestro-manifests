# Roles

A role is `agents/<name>.md`: YAML frontmatter describes an agent and the body is
its prompt. Manifest roles use the same format as user roles and project roles.
The subagent consumer owns discovery and execution; this repository supplies
content rather than an agent runner.

## Contents

1. [Locations](#locations)
2. [Example](#example)
3. [Format](#format)
4. [Roles supplied here](#roles-supplied-here)
5. [Loading rules](#loading-rules)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Usage](#usage)
8. [Validation](#validation)
9. [Check a change](#check-a-change)

## Locations

Manifest roles live at `agents/*.md` in selected areas. Local roles live in the
user's `agents/` folder under `~/.maestro/agent` and the nearest project `agents/`
folder, not a project model file or settings descriptor.

## Example

The evidence-review capability includes a read-only role with no model override:

From [`capabilities/practice/evidence-review/agents/evidence-reviewer.md`](../capabilities/practice/evidence-review/agents/evidence-reviewer.md):

```markdown
---
name: evidence-reviewer
description: Check factual claims against their cited source passages without editing.
tools: read, grep, find, ls
---

Read the supplied answer and its cited source passages. Do not modify files.
For each factual claim, identify the passage that supports it. Report unsupported
claims with the affected file and line when available. Return a corrected answer
with citations and state what evidence is still missing.

Do not invent sources or publish private evidence. Distinguish a verified
contradiction from uncertainty. List the files you read and any validation you
could not perform.
```

This role is separate from the core `reviewer`, so selecting the capability does
not replace a default-chain role. Its tool list excludes writing and shell use;
the host and policy still own actual tool availability.

## Format

| Field | Meaning |
| --- | --- |
| `name` | Local role name, matching the file stem |
| `description` | Short purpose used when listing or selecting roles |
| `tools` | Comma-separated tool names; an explicit list narrows the available tools |
| `model` | Model selector consumed by the subagent runner, subject to allowed-model policy |
| `thinking` | Requested thinking level, subject to role bounds and model capability |

The body is the role prompt, not a metadata envelope. Do not add source schema,
resource owner or workflow dependency metadata. Ownership comes from CODEOWNERS.

## Roles supplied here

| Role | Responsibility | Tools |
| --- | --- | --- |
| [maestro](../core/agents/maestro.md) | Main-agent persona | No additional ceiling declared |
| [scout](../core/agents/scout.md) | Gather compressed context with precise source ranges | read, grep, find, ls, bash |
| [planner](../core/agents/planner.md) | Produce a plan without editing | read, grep, find, ls |
| [reviewer](../core/agents/reviewer.md) | Review quality, correctness and security | read, grep, find, ls, bash; prompt limits bash to read-only inspection |
| [worker](../core/agents/worker.md) | Complete the delegated task in isolated context | All available tools within policy |
| [evidence-reviewer](../capabilities/practice/evidence-review/agents/evidence-reviewer.md) | Opt-in factual review with source citations | read, grep, find, ls |

The reviewer prompt forbids modifications and builds. That is an instruction, not
a shell sandbox. The core scout defaults to `claude-haiku-4-5`; core planner,
reviewer and worker default to `claude-sonnet-4-5`. Supply a compatible model
configuration or an allowed role preference before running them if those
models are unavailable in your installation.

## Loading rules

The subagent consumer discovers selected manifest-area roles, user `agents/`
under `~/.maestro/agent` and the nearest project `agents/` folder. User and project
roles may replace an optional manifest role by name. Replacing a role replaces
the whole definition; fields do not merge. Project roles retain the confirmation
before execution.

`roles.<name>.model` and `roles.<name>.thinking` in user settings are preferences
within policy. A role without a model field uses the consumer's ordinary model
resolution; no endpoint is implied by its prompt.

## What the organization can lock

Policy may lock `model`, `thinking` or `tools`, bound thinking, or restrict models
for a role. Tools are always narrow-only. A required role cannot be shadowed; a
required workflow resolves its roles from its own release. Local additions of
new roles are not globally forbidden by a role lock.

The main `maestro` role is special: its body appends the persona and its declared
`tools` limits the main agent's active tools. Main model and thinking defaults
come from settings, not from that role's frontmatter. Only core may define it.

## Usage

Ask Maestro to list roles with sources, then delegate a small evidence check to
`evidence-reviewer` after selecting its capability. Supply both a proposed answer
and the source passage. Expect citations or an explicit evidence gap, not edits.
Use [workflows](workflows.md) for the three core chains.

## Validation

Check frontmatter, role names and tool names. Generated schemas check structure
when available; only the authored-agent consumer can prove role discovery,
model availability, project confirmation and required-workflow closure.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). Check the name
against the stem, tool names against the installed host and model availability.
Load the pinned revision and list/show roles through the subagent consumer. Run
a small delegated request, check its output contract, and try an allowed model
preference. For required workflows, add a same-name local role in a disposable
project and verify the required chain still uses the release's role.
