# System prompt and persona

`core/SYSTEM.md` is the manifest's base system prompt. The body of
`core/agents/maestro.md` is the main agent's persona, appended to that base.
Neither file is an instruction selector or a replacement for project context.
Only core may supply these two artifacts.

## Complete example

A small complete `core/SYSTEM.md`:

```markdown
You are Maestro, a coding assistant working in the user's project.

Read the relevant files and repository instructions before editing. Preserve
unrelated work. Make the smallest complete change and run the relevant checks.
Report what changed, what you verified and what remains uncertain. Never claim a
check passed without its result. Keep credentials out of public files.

For organization configuration, read the active manifest's README.md and docs/.
For extension authoring, read Maestro's docs/extensions.md and its examples.
```

Pair it with this complete `core/agents/maestro.md`:

```markdown
---
name: maestro
description: Main coding assistant persona
---

Explain decisions in plain language. Keep routine answers short and give enough
detail to reproduce a change. Distinguish observed results from assumptions.
```

The repository's [base prompt](../core/SYSTEM.md) and
[persona](../core/agents/maestro.md) provide the actual defaults. They deliberately
do not declare a model or tool list. The main model and thinking level come from
settings. Adding `tools` to the persona role imposes a ceiling; it does not merely
suggest tools. See [roles](roles.md).

## Which prompt wins

The base prompt candidates, from highest to lowest priority, are an explicit flag
or SDK replacement, project `SYSTEM.md`, user `SYSTEM.md`, manifest
`core/SYSTEM.md`, then the engine default. A custom base replaces the engine's
default documentation block, so include useful documentation pointers in a custom
prompt. Ordinary date, working directory, skill block and tool assembly still
follow their existing rules.

The persona's body is always first in the appended text. Ordinary appends follow:
every explicit append in order, otherwise the discovered `APPEND_SYSTEM.md`.
Local prompt locations are `~/.maestro/agent/SYSTEM.md`,
`.maestro/SYSTEM.md` and the corresponding `APPEND_SYSTEM.md` files.

Context remains separate. The user context file and ancestor `AGENTS.md` or
`CLAUDE.md` files keep their normal discovery and `--no-context-files` behavior.
Do not put manifest context files into an area and assume they become project
instructions. Installable instructions have their own [format](instructions.md).

## Locks and overrides

A `systemPrompt` lock ignores local base files with a diagnostic and refuses an
explicit replacement, including an extension's per-turn replacement. An
`appendSystemPrompt` lock likewise ignores local append files and refuses
explicit appends. Those locks belong in `settings/policy.json`:

```json
{
  "locked": ["systemPrompt", "appendSystemPrompt"]
}
```

Without the locks, users can customize those prompts. Prompt text is guidance,
not a security boundary; locking it does not sandbox extension code or revoke
filesystem access.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). Load the
release in Maestro and inspect the effective prompt with its session/prompt
inspection facilities. Confirm the persona appears once before local appends.
With a locked test policy, try an explicit replacement and confirm refusal;
without it, confirm a project `SYSTEM.md` takes precedence. Check that prompt
documentation paths point to actual files, because a custom base removes the
engine's documentation list.
