# Northstar

> Automate the guardrails to deliver faster, with higher quality, and more
> securely.

`maestro-manifests` steers by the organization's
[Northstar](https://github.com/Orchestration-Maestro/.github/blob/864d85597a833864cd8506c3925830503b3c2163/golden-rules/northstar.md).
Speed, quality, maintainability and security are not a trade-off: automation is
what lets one repository have all four. Each pillar below explains what it
protects, why this repository holds it and how, and ends with one KPI: its
current value, its target and the job that measures it. Unmeasured is written
`not measured`, never estimated, and a value read by hand carries the date it was
read. The [engineering standards](engineering.md) and the [security
standards](security.md) are how the repository holds this course.

## Contents

1. [The point](#the-point)
2. [Speed](#speed)
3. [Quality](#quality)
4. [Maintainability](#maintainability)
5. [Security](#security)

## The point

An organization that runs Maestro gets its settings, policy, system prompt, roles
and workflows from one pinned, reviewed Git release, so no developer assembles
them alone. When it works, every developer's agent starts from the same reviewed
configuration, a change reaches them as a release they adopt, and nothing outside
the pinned release alters it.

## Speed

### What it is

Speed protects the edit-run loop and the path from commit to release. A
repository is fast when a contributor gets an answer from the checks within the
time it takes to read them, and when a reviewed change reaches a release without
a manual step in between.

### Why this repository applies it

Contributors run the content check before each proposal, and a slow check is
skipped or batched until the problems pile up. A release here is the merge of a
reviewed pull request followed by the merge of its Release Please pull request,
so delay in the checks is delay in every change that follows.

### How it is applied here

One command serves everyone: `just check` runs the same hooks on a contributor's
machine and in `Required content checks`. At commit, the hook `workspace-check`
runs `just pre-commit` on the staged files only, which keeps the commit step
short. The merge queue merges entries as their checks go green. A change made for
speed cites the measured job time before and after (P-017 on the engineering
page).

### KPI

The KPI is the duration of the required content job.

- **Current:** 7 s, the median of the last 8 runs, read by hand on 2026-10-28.
- **Target:** under 30 s.
- **Measured by:** the duration of the `Required content checks` job on each pull
  request and merge group.

## Quality

### What it is

Quality protects the promise that the content does what it claims and keeps doing
it. For this repository that means the roles, workflows, themes and guides agree
with each other and with the contracts they quote.

### Why this repository applies it

Agents act on this content. A role whose tool list or output headings drift, or
a chain that drops its handoff, changes what users' agents do, and the guides
teach contributors from the same files, so a stale quotation misleads them.

### How it is applied here

`just test` runs the tests under `tests/`. The test `tests/test_manifest_examples.py`
fixes the contracts of the core roles, the chains and the review theme, and checks
that each guide quotes its source file. The test `tests/test_quality.py` runs the
contributor commands in disposable Git indexes, and `tests/test_standards_pages.py`
keeps the standards pages explanatory and accurate. A behaviour change starts
with a test seen failing. `just check` runs `just test`, so a pull request that
breaks a test fails `Required content checks` and cannot merge.

### KPI

The KPI is the number of failing tests in `just test`.

- **Current:** 0. The required job runs the tests on every pull request and merge
  group, so it cannot drift without turning red.
- **Target:** 0.
- **Measured by:** `Required content checks`, which runs `just test` through
  `just check`.

## Maintainability

### What it is

Maintainability protects the next reader's ability to change the repository
safely. The next reader is a person or an agent that has not seen the previous
change.

### Why this repository applies it

Many contributors, and many agents acting for them, read and edit the same files.
Spelling drift, broken Markdown structure or a guide without a purpose paragraph
costs each of them time, and the cost grows with the catalog.

### How it is applied here

`just check` runs the hooks of `.pre-commit-config.yaml` on every tracked file:
spelling with `typos`, Markdown structure with `rumdl`, editor settings with
`editorconfig-checker`, and the hygiene and format checks for YAML, JSON and TOML.
`CONTEXT.md` keeps one definition per term. Authored pages open with a purpose
paragraph and numbered contents, and the check list under Check a change in
`docs/index.md` keeps the reading list and the artifact index in step.

### KPI

The KPI is the number of findings the content checks report on the default
branch.

- **Current:** 0. The gate verifies it on every run, so it cannot drift without
  turning red.
- **Target:** 0.
- **Measured by:** `Required content checks`, which runs `just check` on every
  pull request and merge group.

## Security

### What it is

Security protects the rule that nothing reaches a machine without passing the
gates. For this repository the machine is a developer's agent, and the thing that
arrives is a release.

### Why this repository applies it

The content is public and steers agents. A secret in it, an unpinned dependency
in its tooling or an unreviewed change to a role would reach every organization
that pins the release. The [security standards](security.md) explain the assets
and the places untrusted input enters.

### How it is applied here

The hook `gitleaks` scans each staged change. On the platform, secret scanning
with push protection and validity checks blocks a push that carries a secret,
Dependabot proposes updates and raises alerts, and code scanning (default setup,
weekly, for Actions and Python) reports findings. Required signed commits and
pull requests keep every change attributable. The release itself carries no
signature or bill of materials yet, which the security standards state.

### KPI

The KPI is the number of open code-scanning, secret-scanning and Dependabot
alerts.

- **Current:** 0, read by hand on 2026-10-28.
- **Target:** 0.
- **Measured by:** code scanning (default setup), secret scanning and Dependabot
  alerts on the repository's Security tab.
