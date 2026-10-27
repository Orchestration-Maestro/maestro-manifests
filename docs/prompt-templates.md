# Prompt templates

A prompt template is a Markdown file under `prompts/`. Its filename supplies the
slash-command name, optional YAML frontmatter supplies a description and its body
expands into prompt text. Templates do not execute a workflow unless their text
asks the agent to do so; use [workflow YAML](workflows.md) for a structured run.

## Contents

1. [Locations](#locations)
2. [Example](#example)
3. [Format](#format)
4. [Usage](#usage)
5. [Argument expansion](#argument-expansion)
6. [Loading rules](#loading-rules)
7. [What the organization can lock](#what-the-organization-can-lock)
8. [Validation](#validation)
9. [Check a change](#check-a-change)

## Locations

Templates load from `~/.maestro/agent/prompts/*.md`, `.maestro/prompts/*.md`,
package prompt paths and selected manifest areas. Local `prompts` settings can
add files or directories; `--prompt-template` is repeatable.

## Example

The evidence-review capability provides this explanation template:

From [`capabilities/practice/evidence-review/prompts/explain-change.md`](../capabilities/practice/evidence-review/prompts/explain-change.md):

```markdown
---
description: Explain a change for a named audience.
argument-hint: "<file> <audience> [details...]"
---

Explain the change in $1 for $2.

Use these additional details: ${@:3}

Read the relevant files first. Describe the observable behavior and cite the
source. Separate confirmed behavior from assumptions. Do not change any files.
```

## Format

The filename supplies the command name, so `explain-change.md` becomes
`/explain-change`. Optional `description` supplies completion text; without it,
the loader takes the first nonempty line. Optional `argument-hint` puts required
`<arguments>` and optional `[arguments]` before the description in completion.
The body is Markdown expanded as text, not shell code.

## Usage

After selecting the capability, invoke this from a normal clone of the release:
`/explain-change settings/settings.json "new contributors" focus on area selection`.
The first argument is a real file in this release, the second is one quoted
audience argument, and the remaining words fill the details tail. No file is
changed. The same template works unchanged when copied into a local prompt folder.

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

## Loading rules

Conventional `prompts/` discovery is nonrecursive. To expose a nested file, list
it explicitly in the native package manifest or local prompt settings. The
capability's package file lists the example by its exact relative path.
`--no-prompt-templates` disables automatic discovery; explicit entries retain
ordinary loading behavior within policy. Manifest settings cannot supply a
`prompts` path array.

## What the organization can lock

Optional manifest templates may be replaced by same-name user or project
resources. A `prompts` lock forbids local additions; automatic additions are
ignored with a diagnostic and explicit `--prompt-template` entries are refused.
A required template cannot be disabled by filters, `--no-prompt-templates` or
SDK overrides, and wins a same-name collision. Template text remains guidance:
it does not bypass model, tool or role policy.

## Validation

Check frontmatter and relative package paths. Invoke the template to verify
quoted arguments and slicing: parsing Markdown cannot prove the final text says
what you intended. A valid template is guidance, not a grant of tool access.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks) and load the
pinned revision. List the command and invoke the example with quoted arguments;
confirm the path, audience and tail appear in the intended places. Also invoke
with missing arguments and inspect the expansion before relying on it. If the
template is referenced by a required workflow, check that the reference resolves
from that workflow's release rather than a local same-name file.
