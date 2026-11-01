# Working in maestro-manifests

Follow [Maestro's shared rules](https://github.com/Orchestration-Maestro/maestro/blob/main/AGENTS.md)
for work in this repository. They take precedence wherever this file disagrees.
The sections below cover this repository's tooling, manifest content and
contributor requirements.

This repository holds public manifest resources; private collections plug in
from outside it. [README.md](README.md) describes the catalog layout and the
public/private boundary, and [CONTEXT.md](CONTEXT.md) defines the vocabulary.

## Setup

Install [rustup](https://rustup.rs/), which installs the Rust toolchain that
`just check` needs and that `just setup` does not. Run `mise trust` and
`mise exec -- just setup` once. It installs the tools pinned
in `mise.toml` and the prek hooks for pre-commit, pre-merge-commit and
commit-msg. Run later commands through `mise exec --` when mise is not active in
your shell.

Checked by: the pull request reports that setup ran. No check verifies that the
hooks are installed.

## Before editing

1. Start from a GitHub issue and triage it with the shared labels or `cleanup`.
   Checked by: the pull request names its issue.
2. Read the files the issue names and check their callers before choosing where
   to change behaviour. Keep the change within the issue's scope. Checked by:
   review compares the diff with the issue.
3. Add a failing check at the affected public interface, make the smallest fix,
   and run the check again. Save both runs in the issue's evidence. Checked by:
   the pull request reports the failing run and the passing run.

## Checks

- `just check` runs `just content`, `just rust` and `just test`, as CI does.
  `just content` runs `prek run --all-files --stage manual`. It checks merge
  markers, YAML, JSON and TOML syntax, final newlines, trailing whitespace, line
  endings, files over 500 KB, case conflicts, shebangs, spelling (`typos`), every
  file against `.editorconfig` (`editorconfig-checker`) and Markdown structure
  (`rumdl`). The organization's required content workflow runs
  `mise exec -- just check` on every pull request and merge group.
- `just rust` runs `cargo fmt --check` and `cargo clippy --all-targets --locked
  -- -D warnings`. Checked by: the required workflow runs it through
  `just check`; report its result in the pull request.
- `just test` runs `cargo test --locked`: the Rust checks of the guides, roles,
  policy and manifest examples, and of the contributor commands in disposable Git
  indexes. Checked by: the required workflow runs it through `just check`; report
  its result in the pull request.
- Report the checks you actually ran, with exit codes. Do not claim a schema or
  runtime check; this repository has none yet.
- `.pre-commit-config.yaml` owns the content checks. The `rust` recipe in the
  `justfile` owns the Rust checks. The Git hooks call `just pre-commit`, which
  runs the content checks on staged files through prek's `manual` stage to avoid
  calling the Git hook again, and scan the staged change with `gitleaks`. The
  required workflow calls only `just check`.

## Documentation

Follow [Maestro's documentation rules](https://github.com/Orchestration-Maestro/maestro/blob/main/AGENTS.md#documentation).

- Keep one spelling per concept, and update [CONTEXT.md](CONTEXT.md) when an issue
  establishes a new term. Checked by: review, and `typos` for spelling.
- Put no private or vendor content here, and never commit credentials or private
  evidence. Checked by: `gitleaks` on credentials at commit time; review for the
  rest.
- Give manifest content and schema changes their own issues. Checked by: review
  compares the diff with the issue.

## Delivery

Follow [Maestro's Git discipline](https://github.com/Orchestration-Maestro/maestro/blob/main/AGENTS.md#test-and-git-discipline).

Checked by: the commit-msg hooks check the header and line width; the repository
rulesets require conventional, signed commits on `main`.

- Push with `timeout 120 git push`. Checked by: no check verifies this command.
