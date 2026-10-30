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

Install [mise](https://mise.jdx.dev/) and [rustup](https://rustup.rs/), trust the checkout with `mise trust` and
run `mise exec -- just setup` once. Then run `mise exec -- just check`, which
runs the content checks, the Rust checks and the tests, before proposing a
change.

## Example

The complete contributor command file is shared by local work and CI:

From [`justfile`](../justfile):

```just
# Install pinned tools and Git hooks.
setup:
    mise install
    mise exec -- prek install

# Run the content checks, the Rust checks and the tests, as CI does.
check: content rust test

# Run the repository-owned content checks.
content:
    prek run --all-files --stage manual

# Use the same checks on the staged files without calling the Git hook again.
pre-commit:
    prek run --stage manual

# Check Rust formatting and lints, with warnings denied.
rust:
    cargo fmt --check
    cargo clippy --all-targets --locked -- -D warnings

# Run the Rust tests, then the Python tests that exercise contributor commands in
# disposable Git indexes.
test:
    cargo test --locked
    python3 -m unittest discover -s tests
```

`mise.toml` pins the tools; `mise.lock` records their download checksums.
`rust-toolchain.toml` pins Rust with Clippy and rustfmt, which rustup installs on
first use, and `Cargo.lock` pins the crates of the test crate. Prek
installs the pre-commit, pre-merge-commit and commit-msg hooks. The content checks
live in `.pre-commit-config.yaml`: spelling, EditorConfig, Markdown structure,
YAML/JSON/TOML parsing and basic file hygiene. `just content` runs them on every
tracked file. `just rust` fails on an unformatted file or a Clippy warning, and
`just check` runs `just content`, `just rust` and then `just test`. The shared
content workflow invokes `just check` through mise, so a pull request that breaks a
test, a lint or a format fails the required job.

`just pre-commit` runs the content checks on staged files. It uses prek's `manual`
stage, so the hook can call it without recursion. The Git hook also scans staged
changes for secrets; the commit-msg hooks check conventional headers and line
lengths. Check failures stop the commit. If a check fixes a file, review and stage
that fix before retrying.

`just test` exercises these commands in disposable Git indexes: valid content
passes, malformed YAML, JSON and TOML fail, and the staged hook ignores unrelated
untracked files. Those tests run `just content`, not `just check`, which would
run the tests again, and they read `just check` only as a dry run. Tests do not
change your index.

## Required CI

The organization's `hygiene-central` ruleset requires the shared
`content.yml@v0.16.1` workflow on pull requests and merge groups targeting the
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
It updates `version.txt`, `manifests/CHANGELOG.md` and its version manifest. Merging that
pull request creates a `v`-prefixed release tag, starting at `v0.1.0`. The
organization's `floor-release-tags` ruleset protects those tags from changes and
deletion.

The release workflow uses the organization's `RELEASE_APP_CLIENT_ID` variable
and `RELEASE_APP_PRIVATE_KEY` secret. The App token lets release pull requests
trigger required checks. Keep those credentials in GitHub, never in this catalog.

## What the organization can lock

Repository rulesets require checks, signed changes and protected review. These
are contributor controls, not entries in manifest policy. A runtime policy lock
does not replace repository review, and a maintainer must not edit a file to
bypass the check or the owner it names.

## Check a change

Run the check and test recipes above, review any automatic formatting and restage
it. Commit through the installed hooks with a signed conventional header. Report
which commands actually passed and which runtime/schema checks remain unavailable.
Do not duplicate `.pre-commit-config.yaml` in CI. The pages under
`docs/standards/` are written by hand and follow the authored-guide format.
