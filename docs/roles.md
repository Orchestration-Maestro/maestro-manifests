# Roles

A role is `agents/<name>.md`: YAML frontmatter describes an agent and the body is
its prompt. Manifest roles use the same format as user roles and project roles.
The subagent consumer owns discovery and execution; this repository supplies
content rather than an agent runner.

## Complete example

`core/agents/planner.md` can contain:

```markdown
---
name: planner
description: Creates implementation plans from context and requirements
tools: read, grep, find, ls
model: claude-sonnet-4-5
thinking: medium
---

You are a planning specialist. Read the supplied context and requirements.
Do not change files. Return a concrete implementation plan.

For each step name the file and function to change, the expected behavior and
how to check it. List risks and any unresolved decision before handing the plan
to a worker.
```

This is a complete shorter authoring example. The actual [planner](../core/agents/planner.md)
preserves the reference role's detailed output contract and leaves thinking to
the consumer's defaults. `thinking` above illustrates the optional v0.1 field.

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

The reviewer prompt forbids modifications and builds. That is an instruction, not
a shell sandbox. Scout defaults to `claude-haiku-4-5`; the other delegated roles
default to `claude-sonnet-4-5`, as in the reference chains. Supply a compatible
model configuration or an allowed role preference before running them if those
models are unavailable in your installation.

## Discovery and overrides

The subagent consumer discovers selected manifest-area roles, user `agents/`
under `~/.maestro/agent` and the nearest project `agents/` folder. User and project
roles may replace an optional manifest role by name. Replacing a role replaces
the whole definition; fields do not merge. Project roles retain the confirmation
before execution.

`roles.<name>.model` and `roles.<name>.thinking` in user settings are preferences
within policy. For example:

```json
{
  "roles": {
    "planner": {
      "model": "claude-sonnet-4-5",
      "thinking": "high"
    }
  }
}
```

Policy may lock `model`, `thinking` or `tools`, bound thinking, or restrict models
for a role. Tools are always narrow-only. A required role cannot be shadowed; a
required workflow resolves its roles from its own release. Local additions of
new roles are not globally forbidden by a role lock.

The main `maestro` role is special: its body appends the persona and its declared
`tools` limits the main agent's active tools. Main model and thinking defaults
come from settings, not from that role's frontmatter. Only core may define it.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). Check the name
against the stem, tool names against the installed host and model availability.
Load the pinned revision and list/show roles through the subagent consumer. Run
a small delegated request, check its output contract, and try an allowed model
preference. For required workflows, add a same-name local role in a disposable
project and verify the required chain still uses the release's role.
