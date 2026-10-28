# Instructions

An area's `instructions/*.instructions.md` files offer guidance a developer may
install. Selecting an area does **not** inject every instruction into the prompt.
The manifest consumer copies the chosen file; the rules system decides when to
load and apply it.

## Contents

1. [Locations](#locations)
2. [Format](#format)
3. [Example](#example)
4. [Usage](#usage)
5. [Loading rules](#loading-rules)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Validation](#validation)
8. [Check a change](#check-a-change)

## Locations

A selected area offers `instructions/*.instructions.md`. Installed user copies
live under `~/.maestro/agent/instructions/`; project copies live under
`.maestro/instructions/`. The rules consumer reads those local folders.

## Format

An instruction contains `applyTo` YAML frontmatter and a Markdown body. No
sidecar, resource ID or ownership field is needed. Matching belongs to the rules
system, including always-apply behavior, `globs`, `rule://` reads and TTSR; the
manifest contract only supplies and installs files. A YAML parse does not prove
that a rule will match a particular request.

## Example

This release ships no instruction. No `applyTo` pattern selects review work, so
the evidence-review rule lives in its role and skill and applies only when
evidence is reviewed. See the explicit
[artifact-index exception](index.md#example-coverage-and-exceptions).

Not shipped: format sample, for illustration only:

```markdown
---
applyTo: "docs/**/*.md"
---

Cite the file and line for each claim about the code.
```

`applyTo` is the rules consumer's matching input, not permission to execute or a
declaration that selecting a capability automatically applies the text.

## Usage

When a selected area offers an instruction, run `/manifest install <name>` in a
compatible Maestro session. It copies the file to the user instructions folder.
Use `/manifest install <name> -l` for a project copy. The basename remains
`<name>.instructions.md`. This release offers none. See
[settings](settings.md#usage) for selection.

## Loading rules

The manifest consumer records the source release commit and file digest in its
own package state. At each session start it compares installed copies with their
release files. A changed copy is refreshed and listed in the restore notice;
updating the manifest also updates an installed instruction when its source file
changes. An edited installed copy is therefore not a durable personal override.
Write separate local guidance instead of editing a managed copy. See
[integrity](integrity.md) for recovery and offline behavior.

Local instruction folders are also the rules system's input. Local
`APPEND_SYSTEM.md` and context files remain available for ordinary project
guidance; they are not installed manifest copies.

## What the organization can lock

The policy `required` list can name release-relative resources, but how required
standards instructions stay installed and applied belongs to the rules-system
work. This catalog does not implement a new enforcement mechanism. Prompt locks
control the base and appended prompts, not `applyTo` matching. Do not promise
mandatory application merely because an instruction exists under `standards/`.

## Validation

Validate frontmatter structure with the generated schemas when available, then
exercise actual matching with the rules consumer. Installation provenance and
startup refresh belong to the manifest consumer, not Markdown validation.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks) and validate
frontmatter against generated schemas when they are available. In a compatible
Maestro build, install an offered instruction into a disposable project and verify the file
bytes and recorded provenance. Exercise matching and nonmatching files through
the rules consumer. Modify only the disposable installed copy, restart, and
confirm the restore notice and restored bytes. Do not test drift against
uncommitted work you intend to keep.
