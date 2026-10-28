# Integrity

Manifest selection is pinned Git only so the consumer can verify exactly which
files it is loading. At every session start it checks the installed release
checkout against the pinned commit and refreshes installed instruction copies.
Drift is repaired with a notice, not silently retained and not treated as a
reason to crash an otherwise recoverable session.

## Contents

1. [Example](#example)
2. [Usage](#usage)
3. [What is restored](#what-is-restored)
4. [Recovery and offline behavior](#recovery-and-offline-behavior)
5. [What the organization can lock](#what-the-organization-can-lock)
6. [Validation](#validation)
7. [Check a change](#check-a-change)

## Example

This is the actual installable instruction in the evidence-review capability:

From [`manifests/capabilities/practice/evidence-review/instructions/cite-code-claims.instructions.md`](../manifests/capabilities/practice/evidence-review/instructions/cite-code-claims.instructions.md):

```markdown
---
applyTo: "docs/**/*.md"
---

Cite the file and line for each claim about the code.
```

To exercise copy integrity in a disposable project, select the capability and
run `/manifest install cite-code-claims -l`. The destination is
`.maestro/instructions/cite-code-claims.instructions.md`. Its recorded commit
and digest identify the source above; a matching copy must be byte-for-byte equal.

## Usage

Use `/manifest status` to identify the selected source, pin and settings file.
Inspect the installed Git checkout, not your development clone. Read-only
`git status --porcelain --untracked-files=all` should be empty, and
`git rev-parse HEAD` should equal the commit resolved from that pin. A clean
checkout at a different commit is still drift. These inspection commands do not
replace the consumer's startup verification.

## What is restored

Modified, added, deleted and untracked files, or a checkout moved away from its
pin, count as drift. Startup restores the pinned commit through the ordinary
package owner's Git operations, then prints
`Manifest release restored: <release>` and lists reset or removed files. A
changed pin committed by a team therefore takes effect at the next start even
if an old checkout already exists.

Installed instructions carry source commit and digest provenance in manifest
package state. An edited copy no longer matches its release file and is
refreshed at startup, with the same restore notice. A release update also
refreshes an installed copy when that source file changes. Keep personal
guidance in a separate file instead of editing a managed copy.

**Do not develop in an installed release checkout.** Restoration can discard
edits and remove untracked files there. Author in a normal clone, commit and
publish a ref, then select that ref. Before deliberately testing drift, use a
disposable settings scope and checkout with no work to preserve.

## Recovery and offline behavior

When the pinned commit cannot be restored locally, the consumer installs the
release again through ordinary package operations. If that is also impossible,
for example offline, it uses its last accepted composition with a warning naming
the drift and retries repair at the next start. It never trusts changed release
bytes as a new accepted composition. With no accepted composition, there is no
prior safe configuration to run; a missing/unloadable initial release follows
the ordinary load-failure rules.

A malformed release and a dirty checkout are different failures. A broken
pinned release may be repaired with a newer clean same-identity release under
[update permission-growth rules](releases-and-updates.md). If no candidate can
apply, the manifest load refuses with source, selecting settings file, problem
and fix. There is no silent fallback to internal mode. Help, version, package
commands and editing unlocked settings remain repair routes.

`/reload` recomposes the release too. A failed reload reports the error and keeps
the previously accepted composition so the existing session can continue.
A fresh start must still pass before a release is considered usable.

## What the organization can lock

Integrity applies to the selected release regardless of whether prompt or
resource locks are set. It does not make the manifest publisher trustworthy,
protect every user file or sandbox an extension. A `manifest` lock fixes the
identity, and the update command remains the permitted same-identity ref route.
Local optional artifacts remain customizable under their own policy rules.

## Validation

The consumer verifies both commit identity and tracked/untracked file bytes at
start. Instruction provenance belongs to package state, not editable frontmatter.
An offline fallback must use its accepted composition, never changed checkout
bytes. Parsing an instruction or policy cannot establish those guarantees.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). In an isolated
runtime fixture, start once to establish an accepted composition. Test a changed
file, deleted file, untracked file and moved HEAD separately, confirming the
restored bytes/commit and notice. Install an instruction and test copy refresh.
Simulate failed restoration and acquisition only in that fixture; confirm the
warning and last accepted composition, then successful repair on the next start.
Content parsing alone cannot prove these startup behaviors.
