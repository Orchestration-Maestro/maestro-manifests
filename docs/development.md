# Development

Use the pinned contributor commands to validate catalog changes, install hooks
and submit signed changes through protected review. This page describes the
existing tooling and release setup; it does not add a separate validator or
claim that runtime manifest loading is already a required check.

## Contents

1. [Local checks](#local-checks)
2. [Example](#example)
3. [Required CI](#required-ci)
4. [Ownership and delivery](#ownership-and-delivery)
5. [Releases](#releases)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Check a change](#check-a-change)

## Local checks

Install [mise](https://mise.jdx.dev/), trust the checkout with `mise trust` and
run `mise exec -- just setup` once. Then run `mise exec -- just check` and
`mise exec -- just test` before proposing a change.

## Example

The complete contributor command file is shared by local work and CI:

From [`justfile`](../justfile):

```just
# Install pinned tools and Git hooks.
setup:
    mise install
    mise exec -- prek install

# Run the repository-owned content checks used by CI.
check:
    prek run --all-files --stage manual

# Use the same checks on the staged files without calling the Git hook again.
pre-commit:
    prek run --stage manual

# Exercise contributor commands in disposable Git indexes.
test:
    python3 -m unittest discover -s tests
```

`mise.toml` pins the tools; `mise.lock` records their download checksums. Prek
installs the pre-commit, pre-merge-commit and commit-msg hooks. The content checks
live in `.pre-commit-config.yaml`: spelling, EditorConfig, Markdown structure,
YAML/JSON/TOML parsing and basic file hygiene. `just check` runs them on every
tracked file. The shared content workflow invokes that same command through mise.

`just pre-commit` runs the content checks on staged files. It uses prek's `manual`
stage, so the hook can call it without recursion. The Git hook also scans staged
changes for secrets; the commit-msg hooks check conventional headers and line
lengths. Check failures stop the commit. If a check fixes a file, review and stage
that fix before retrying.

`just test` exercises these commands in disposable Git indexes: valid content
passes, malformed YAML, JSON and TOML fail, and the staged hook ignores unrelated
untracked files. Tests do not change your index.

## Required CI

The organization's `hygiene-central` ruleset requires the shared
`content.yml@v0.16.0` workflow on pull requests and merge groups targeting the
default branch. Its job is `Required content checks`; this repository needs no
caller workflow. The repository's `merge-queue` ruleset uses Maestro's queue
settings: squash merges, all checks green, up to seven entries building and
five entries merging, with a six-hour check timeout.

`hygiene-central` targets only `maestro-manifests` by repository name. GitHub
accepts either repository-name or repository-property selection, not both; the
repository's `stack=other` property remains unchanged. Add `.github` to the
include list when it provides `just check`; add `maestro-packages` when it does
the same if it remains non-Rust. Neither repository is included yet.

## Ownership and delivery

Start with a GitHub issue and the triage labels described in [AGENTS.md](../AGENTS.md).
[CODEOWNERS](../.github/CODEOWNERS) supplies review ownership and the catalog's
owner lookup. Submit signed conventional commits through a pull request and use
`gh pr merge --auto` to enter the merge queue after required checks pass.

## Releases

Release Please opens a release pull request from conventional commits on `main`.
It updates `version.txt`, `CHANGELOG.md` and its version manifest. Merging that
pull request creates a `v`-prefixed release tag, starting at `v0.1.0`. The
organization's `floor-release-tags` ruleset protects those tags from changes and
deletion.

The release workflow uses the organization's `RELEASE_APP_CLIENT_ID` variable
and `RELEASE_APP_PRIVATE_KEY` secret. The App token lets release pull requests
trigger required checks. Keep those credentials in GitHub, never in this catalog.

## What the organization can lock

Repository rulesets require checks, signed changes and protected review. These
are contributor controls, not entries in manifest policy. A runtime policy lock
does not replace repository review, and a maintainer must not hand-edit generated
rust-gate files to bypass their owner.

## Check a change

Run the check and test recipes above, review any automatic formatting and restage
it. Commit through the installed hooks with a signed conventional header. Report
which commands actually passed and which runtime/schema checks remain unavailable.
Do not duplicate `.pre-commit-config.yaml` in CI. The generated rule maps under
`docs/standards/` stay generated and are exempt from authored-guide formatting.
