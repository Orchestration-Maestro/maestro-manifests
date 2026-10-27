# Capabilities and ownership

A capability is one package at `capabilities/<group>/<name>/`. It bundles the
team's roles, skills, instructions, prompts, workflows and other supported
resources. A group is a navigation folder, not a settings area. Both group and
name are lowercase, hyphenated and at most 64 characters.

Maestro reads folder paths. An organization may map them to its GitHub structure,
but Maestro never queries repository properties to discover areas.

## Complete capability example

This small capability needs exactly two files. It illustrates a new public
capability; it does not claim the separately maintained security capability is
already present.

`capabilities/practice/evidence-review/package.json`:

```json
{
  "name": "maestro-evidence-review",
  "version": "0.1.0",
  "private": true,
  "description": "Review an answer against its cited passages.",
  "maestro": {
    "skills": ["./skills/review-evidence"]
  }
}
```

`capabilities/practice/evidence-review/skills/review-evidence/SKILL.md`:

```markdown
---
name: review-evidence
description: Check an answer against its cited source passages.
---

Read the answer and each cited passage. For each factual claim, identify its
supporting passage. Mark unsupported claims and return a corrected answer with
citations. Never invent a source or publish private evidence.
```

After those files are included in a published release, select the capability in
project settings:

```json
{
  "manifest": "git:github.com/Orchestration-Maestro/maestro-manifests@v0.1.0",
  "manifestAreas": ["capabilities/practice/evidence-review"]
}
```

Use the actual tag or commit containing the two files; the example path is not
part of this revision. Selecting only `evidence-review` is invalid. An absent
selected area invalidates the manifest. Do not create an empty folder to satisfy
selection.

Other real resources can be added under the same root: `agents/*.md`,
`instructions/*.instructions.md`, `prompts/*.md`, `workflows/*.yaml`, `extensions/`
and `themes/`. Add settings and policy only when needed. `SYSTEM.md`, the main
`maestro` role, model files and MCP configuration are core-only. Do not add
`knowledge/` until a package actually reads that artifact kind.

## Ownership

This repository's real approved owner is `@FrancoisLDaigneault`.
[CODEOWNERS](../.github/CODEOWNERS) covers every path with that principal, and
explicitly covers `capabilities/security/` and its future `maestro-security/`
capability. No unverified team is assigned to a real path.

An organization **with these example teams** could instead map group and
capability ownership as follows. These are illustrative team names, not teams
configured in this repository:

```text
* @example-org/manifest-maintainers
/capabilities/practice/ @example-org/practice
/capabilities/practice/evidence-review/ @example-org/evidence-review
```

GitHub applies the last matching pattern. The more specific capability rule must
follow its group rule; a later broad rule can override both. The catch-all covers
all other files, including documentation, policy and tooling. Use valid
principals with repository access before adopting a team example. CODEOWNERS
supplies review and catalog attribution, not runtime permissions or folder-level
secrecy. Protected review settings remain a separate repository control.

Sessions never read CODEOWNERS. The [catalog](catalog.md) reads it from the
release and derives entity ownership with the same last-match rule. Do not add
owner frontmatter or a second ownership descriptor to an artifact.

## Versions, locks and overrides

Capabilities inside this repository share its Git release version. Their files
do not have independent refs. A capability needing a separate release cycle must
live in its own Git or npm package; the manifest can require that package with a
pin in settings `packages`. Ordinary package sources support both Git and npm;
**the manifest selection itself remains pinned Git only**.

Language and capability areas may add constraints or narrow existing ones. They
cannot loosen earlier locks or bounds and cannot set or constrain `manifest`,
`manifestAreas`, `minimumRelease` or `updateCheckMinutes`. Optional resources may
be replaced locally according to their kind. Required resources and workflow
closures cannot be shadowed. See [policy](policy.md).

The security lane must integrate its content at
`capabilities/security/maestro-security/`, with one `package.json`, one Markdown
file per role, native skill frontmatter and instruction files. Sidecars and old
source metadata are not part of the v0.1 format. This change does not copy that
lane's branch or claim its migration is complete.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), check every
package path and load the exact selected area in a pinned revision. Confirm its
resources appear once, under the expected source. Review CODEOWNERS ordering for
each new group and capability and verify every real principal has access. Check
selected combinations against earlier policy: a capability must not unlock an
organization constraint. In catalog builds, inspect the capability's owner and
provided-resource relations.
