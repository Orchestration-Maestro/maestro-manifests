# Skills

A skill is a directory containing `SKILL.md` with YAML frontmatter and a procedure.
Maestro can expose its description for discovery and read its full instructions
when needed. Use a skill for a reusable procedure; use a workflow for a run with
explicit steps and an instruction for guidance selected by rule matching.

## Complete example

Create `skills/review-evidence/SKILL.md`:

```markdown
---
name: review-evidence
description: Review a proposed answer against its cited source passages.
---

# Review evidence

1. Read the proposed answer and each cited passage.
2. For every factual claim, locate the passage that supports it.
3. Mark unsupported claims and explain what evidence is missing.
4. Return a corrected answer with source citations, followed by any uncertainty.

Do not invent sources. Keep private passages out of public review comments.
```

`name` matches the containing directory and uses lowercase letters, digits and
hyphens, at most 64 characters. The description tells the agent when to use the
skill. Optional frontmatter supported by the native skill format includes
`license`, `compatibility`, `allowed-tools`, `metadata` and
`disable-model-invocation`. Do not use metadata strings as a second Maestro
configuration format; nothing here needs a source-schema or maturity envelope.

A skill can hold referenced scripts, examples and supporting documents. Use
paths relative to its directory and include every referenced file in the package.
Do not silently depend on a private checkout. Tool hints in a skill do not grant
permissions beyond the host and policy.

## Locations and selection

Manifest skills live under an area's `skills/`. Local locations include
`~/.maestro/agent/skills/` and `.maestro/skills/`, plus the existing supported
aliases. Packages may declare explicit paths in their `maestro.skills` array:

```json
{
  "name": "review-guidance",
  "version": "0.1.0",
  "maestro": {
    "skills": ["./skills/review-evidence"]
  }
}
```

This package example is complete when paired with the `SKILL.md` above. A manifest
area's settings must not set a `skills` path array; its folders or native package
manifest supply resources. The selected manifest's object filters can narrow
optional resources, using paths relative to the release root.

When skill commands are enabled, users can invoke a skill explicitly through
`/skill:review-evidence`. Otherwise ask Maestro to use the procedure by name.
`enableSkillCommands` controls command exposure and may itself be defaulted or
locked by policy; it does not turn the skill text into executable code.

## Locks and overrides

Optional manifest skills rank below user and project resources. A same-name local
skill can replace an optional manifest skill; replacement is a whole definition.
A `skills` lock disallows local additions: automatic folders and settings entries
are ignored with a diagnostic, and explicit `--skill` requests refuse. Required
skills cannot be disabled by filters, `--no-skills` or loader overrides and win
name collisions. Requiring a skill is not approval of every action it suggests.
Review any skill capable of directing tool execution.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), inspect the
frontmatter and open every relative reference. Load the pinned revision, confirm
the description appears in skill discovery and invoke the example with an answer
and a real source passage. Check both a supported claim and an unsupported one.
If policy requires the skill, confirm a local same-name skill and discovery flags
cannot remove the release's definition.
