# Working in maestro-manifests

This repository holds public manifest resources; private collections plug in
from outside it. [README.md](README.md) describes the catalog layout and the
public/private boundary, and [CONTEXT.md](CONTEXT.md) defines the vocabulary.

This file and [.github/copilot-instructions.md](.github/copilot-instructions.md)
are written by hand and changed through a pull request. Each rule below states
what is required and how it is checked.

## Setup

Run `mise trust` and `mise exec -- just setup` once. It installs the tools pinned
in `mise.toml` and the prek hooks for pre-commit, pre-merge-commit and
commit-msg. Run later commands through `mise exec --` when mise is not active in
your shell.

Checked by: the pull request reports that setup ran. No check verifies that the
hooks are installed.

## Before editing

1. Start from a GitHub issue. Triage it with `needs-triage`, `needs-info`,
   `ready-for-agent`, `ready-for-human`, `wontfix` or `cleanup` as appropriate.
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
  `mise exec -- just check` on every pull request and merge group, and the merge
  queue waits for it.
- `just rust` runs `cargo fmt --check` and `cargo clippy --all-targets --locked
  -- -D warnings`. Checked by: the required workflow runs it through
  `just check`; report its result in the pull request.
- `just test` runs `cargo test --locked` and then
  `python3 -m unittest discover -s tests`. The Python tests run the contributor
  commands in disposable Git indexes, the manifest examples and the retired
  generator check. Checked by: the required workflow runs it through
  `just check`; report its result in the pull request.
- Report the checks you actually ran, with exit codes. Do not claim a schema or
  runtime check; this repository has none yet.
- `.pre-commit-config.yaml` owns the content checks and the `rust` recipe in the
  `justfile` owns the Rust checks. The Git hooks call `just pre-commit`, which
  runs the content checks on staged files through prek's `manual` stage to avoid
  calling the Git hook again, and scan the staged change with `gitleaks`. Do not
  duplicate the checks in CI. Checked by: the required workflow calls only
  `just check`.

## Documentation

- Run the humanizer skill on every document you edit. Checked by: the pull
  request reports it.
- Keep one spelling per concept, and update [CONTEXT.md](CONTEXT.md) when an issue
  establishes a new term. Checked by: review, and `typos` for spelling.
- Put no private or vendor content here, and never commit credentials or private
  evidence. Checked by: `gitleaks` on credentials at commit time; review for the
  rest.
- Give manifest content and schema changes their own issues. Checked by: review
  compares the diff with the issue.

## Delivery

- Sign each commit with `git commit -S -F <file>`. Give it a conventional header
  of at most 71 characters and body lines of at most 80. Checked by: the
  commit-msg hooks check the header and the line width; the repository rulesets
  require the conventional header and signed commits on `main`.
- Push with `timeout 120 git push --no-verify` and open a pull request. Never
  push directly to `main`. Checked by: the repository rulesets require a pull
  request, resolved review threads and the content workflow.
- Merge through the queue with `gh pr merge --auto` once the required checks
  pass. Checked by: the merge queue.
