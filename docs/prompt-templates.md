# Prompt templates

A prompt template is a Markdown file under `prompts/`. Its filename supplies the
slash-command name, optional YAML frontmatter supplies a description and its body
expands into prompt text. Templates do not execute a workflow unless their text
asks the agent to do so; use [workflow YAML](workflows.md) for a structured run.

## Complete example

Create `prompts/explain-change.md` in a manifest area:

```markdown
---
description: Explain a change for a named audience.
---

Explain the change in $1 for $2.

Use these additional details: ${@:3}

Read the relevant files first. Describe the observable behavior and cite the
source. Separate confirmed behavior from assumptions. Do not change any files.
```

Invoke it as:

```text
/explain-change src/settings.rs "new contributors" focus on policy refusals
```

The first argument is the path, the second is the quoted audience, and the rest
become the additional details. Quotes keep words together as one argument; they
are not a shell command. With the file installed locally at
`.maestro/prompts/explain-change.md`, the same template works without a manifest.

## Argument expansion

| Form | Expansion |
| --- | --- |
| `$1`, `$2`, and other positions | One positional argument |
| `$@`, `$ARGUMENTS` | All arguments |
| `${@:N}` | Arguments starting at the one-based position N |
| `${@:N:L}` | At most L arguments starting at position N |

The template engine performs text substitution, not shell evaluation. Avoid
shell-looking substitutions or an invented expression language. Workflow agent
tasks, commands and asks use these same argument rules, with `{previous}` added
for chain output.

Descriptions can be supplied in frontmatter. Without one, the native loader
uses its usual text-derived description. Templates can be exposed from
`~/.maestro/agent/prompts/`, `.maestro/prompts/`, package prompt paths and selected
manifest areas. A package can declare this example explicitly:

```json
{
  "name": "explain-prompts",
  "version": "0.1.0",
  "maestro": {
    "prompts": ["./prompts/explain-change.md"]
  }
}
```

Keep the file beside the package at that path. Do not put resource path arrays
into the manifest area's settings document.

## Locks and overrides

Optional manifest templates may be replaced by same-name user or project
resources. A `prompts` lock forbids local additions; automatic additions are
ignored with a diagnostic and explicit `--prompt-template` entries are refused.
A required template cannot be disabled by filters, `--no-prompt-templates` or
SDK overrides, and wins a same-name collision. Template text remains guidance:
it does not bypass model, tool or role policy.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks) and load the
pinned revision. List the command and invoke the example with quoted arguments;
confirm the path, audience and tail appear in the intended places. Also invoke
with missing arguments and inspect the expansion before relying on it. If the
template is referenced by a required workflow, check that the reference resolves
from that workflow's release rather than a local same-name file.
