# Instructions

An area's `instructions/*.instructions.md` files offer guidance a developer may
install. Selecting an area does **not** inject every instruction into the prompt.
The manifest consumer copies the chosen file; the rules system decides when to
load and apply it.

## Complete example

Create `instructions/review-evidence.instructions.md` in an area:

```markdown
---
applyTo: "**"
---

When reviewing a change, cite the affected file and line for each finding.
Distinguish a verified failure from a hypothesis. List the checks you ran and
name any check you could not run. Do not include credentials or private evidence
in public review comments.
```

There is no sidecar. The frontmatter belongs in the Markdown file. `applyTo`
describes matching for the rules consumer; it is not a workflow label, dependency
ID or permission grant. For path-specific guidance, a file may use a pattern such
as `src/**/*.rs`. The manifest contract supplies the file and installation route;
the rules-system contract owns full rule matching, always-apply behavior,
`globs`, `rule://` reads and TTSR. Do not infer those mechanisms from a YAML parse
success.

Once this example is in the selected release, install it interactively:

```text
/manifest install review-evidence
```

The default destination is
`~/.maestro/agent/instructions/review-evidence.instructions.md`. For a project
copy, use:

```text
/manifest install review-evidence -l
```

That destination is `.maestro/instructions/review-evidence.instructions.md`.
The filename above is an example to add, not an instruction already shipped by
this revision. An installation needs the instruction to exist in a selected area.

## Copies, updates and local changes

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

## Locks and requirements

The policy `required` list can name release-relative resources, but how required
standards instructions stay installed and applied belongs to the rules-system
work. This catalog does not implement a new enforcement mechanism. Prompt locks
control the base and appended prompts, not `applyTo` matching. Do not promise
mandatory application merely because an instruction exists under `standards/`.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), including YAML
frontmatter validation when the generated schemas are available. In a compatible
Maestro build, install the example into a disposable project and verify the file
bytes and recorded provenance. Exercise matching and nonmatching files through
the rules consumer. Modify only the disposable installed copy, restart, and
confirm the restore notice and restored bytes. Do not test drift against
uncommitted work you intend to keep.
