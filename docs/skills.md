# Skills

A skill is a directory containing `SKILL.md` with YAML frontmatter and a procedure.
Maestro can expose its description for discovery and read its full instructions
when needed. Use a skill for a reusable procedure; use a workflow for a run with
explicit steps and an instruction for guidance selected by rule matching.

## Contents

1. [Example](#example)
2. [Format](#format)
3. [Locations](#locations)
4. [Loading rules](#loading-rules)
5. [Usage](#usage)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Validation](#validation)
8. [Check a change](#check-a-change)

## Example

This skill is part of the opt-in evidence-review capability:

From [`capabilities/practice/evidence-review/skills/review-evidence/SKILL.md`](../capabilities/practice/evidence-review/skills/review-evidence/SKILL.md):

```markdown
---
name: review-evidence
description: Review a proposed answer against its cited source passages and report the evidence truthfully.
---

# Review evidence

Use this procedure only when you review an answer against its sources.
It holds the organization's truthful-evidence rule (SEC-008) as rules a reviewer can check.

1. Read the proposed answer and each cited passage.
2. For every factual claim, locate the passage that supports it.
3. Mark each unsupported claim and say what evidence is missing.
4. Return the report below.

## Report

Return four sections: `Corrected answer`, `Findings`, `Checks run` and `Not run`.
A review passes a rule when its report shows the evidence in the last column.

| Rule | Required | Checked by |
| --- | --- | --- |
| Cite | Each finding opens with `path:line`, or `no source` when no passage supports the claim. | Every entry in `Findings` opens with one of the two. |
| Classify | Each finding is a `Verified failure` or a `Hypothesis`. A `Verified failure` quotes the passage or output that shows it. | Every entry carries one label, and each `Verified failure` carries a quote. |
| Report runs | `Checks run` lists each file you read and each other check, with its result. | The section exists and every entry names a result. |
| Report gaps | `Not run` names each check you could not run, with the reason. It reads `None` when every check ran. | The section exists. |
| Source honesty | You cite only sources you read, and claim no result you did not observe. | Every `path:line` in `Findings` names a file listed in `Checks run`. |
| Private evidence | Credentials and private evidence stay out of the report. | The reviewer reads the report for credentials and private evidence before sharing it. |
```

No script or private checkout is needed. The caller supplies the proposed answer
and source passages; missing evidence is part of the result, not a reason to
invent a citation.

## Format

`name` matches the containing directory: 1 to 64 lowercase letters, digits and
hyphens, without leading, trailing or consecutive hyphens. `description` tells
the agent when to load the procedure and is at most 1024 characters. The body is
Markdown. Optional fields include `license`, `compatibility` (at most 500
characters), `metadata`, `allowed-tools` and `disable-model-invocation`.
`allowed-tools` is an experimental space-delimited hint, not a permission grant.
`disable-model-invocation: true` hides the skill from model discovery while
retaining explicit `/skill:name` use.

A skill may include referenced scripts, examples and supporting files. Keep
references relative to its directory and include those files in the package.
Native metadata is not a second configuration or ownership system.

## Locations

Manifest skills live under an area's `skills/`. The native local locations are
`~/.maestro/agent/skills/`, `~/.agents/skills/`, `.maestro/skills/` and ancestor
`.agents/skills/` folders up to the repository root (filesystem root outside a
repository). Local settings may add `skills` paths, and explicit `--skill` is
repeatable. `--no-skills` disables automatic discovery, not explicit additions
unless policy forbids them.

## Loading rules

Discovery recurses into directories containing `SKILL.md`. Direct Markdown files
are also accepted at the native Maestro skill roots, but not at the `.agents`
alias roots. The capability's native package manifest explicitly lists its skill
directory. Manifest-area settings must not set resource-path arrays.

At startup the loader collects names and descriptions for the prompt. When a
task matches, the agent reads the full skill; it is not guaranteed to choose the
skill solely from a description. Explicit invocation provides a reliable route.
Optional collisions warn and keep the first skill at the applicable resource
precedence; required manifest resources win collisions.

## Usage

Select `capabilities/practice/evidence-review`, then use
`/skill:review-evidence` with a proposed answer and source passages. To use the
same procedure without a manifest, copy its complete directory to a local skill
location. Relative references continue to resolve from that directory.

When skill commands are enabled, users can invoke a skill explicitly through
`/skill:review-evidence`. Otherwise ask Maestro to use the procedure by name.
`enableSkillCommands` controls command exposure and may itself be defaulted or
locked by policy; it does not turn the skill text into executable code.

## What the organization can lock

Optional manifest skills rank below user and project resources. A same-name local
skill can replace an optional manifest skill; replacement is a whole definition.
A `skills` lock disallows local additions: automatic folders and settings entries
are ignored with a diagnostic, and explicit `--skill` requests refuse. Required
skills cannot be disabled by filters, `--no-skills` or loader overrides and win
name collisions. Requiring a skill is not approval of every action it suggests.
Review any skill capable of directing tool execution.

## Validation

The native loader warns about most Agent Skills naming/description violations
and ignores unknown frontmatter; a missing description prevents loading. Fix
warnings before publishing. Those lenient native diagnostics are distinct from
manifest schema or required-resource failures. A procedure can direct tool use,
so review its body and any executable helper, not just the frontmatter.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), inspect the
frontmatter and open every relative reference. Load the pinned revision, confirm
the description appears in skill discovery and invoke the example with an answer
and a real source passage. Check both a supported claim and an unsupported one, then check the report
against each row of the rule table.
If policy requires the skill, confirm a local same-name skill and discovery flags
cannot remove the release's definition.
