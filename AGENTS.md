# Working in maestro-manifests

Read [README.md](README.md) for the catalog layout and public/private boundary,
and [CONTEXT.md](CONTEXT.md) for the vocabulary. This repository holds public
manifest resources; private collections plug in from outside it.

## Before editing

1. Start from a GitHub issue. Triage it with `needs-triage`, `needs-info`,
   `ready-for-agent`, `ready-for-human`, `wontfix` or `cleanup` as appropriate.
2. Read the files the issue names and check their callers before choosing where
   to change behaviour. Keep the change within the issue's scope.
3. Add a failing check at the affected public interface, make the smallest fix,
   and run the check again. Save both runs in the issue's evidence.

## Checks and documentation

- Run `just setup` once to install the pinned tools and prek hooks. Run commands
  through `mise exec --` when mise is not active in your shell.
- Run `just check` for the content checks CI uses, and `just test` for the
  contributor-command regression tests. Report the checks actually run.
- `.pre-commit-config.yaml` owns the checks. The Git hook calls `just pre-commit`,
  which runs those checks on staged files through prek's `manual` stage to avoid
  calling the Git hook again. Do not duplicate checks in CI.
- Run the humanizer skill on every document you edit. Keep one spelling per
  concept and update the glossary when an issue establishes a new term.
- No private or vendor content belongs here. Never commit credentials or private
  evidence. Manifest content and schema changes need their own issues.

## Delivery

Use signed conventional commits with a subject of at most 71 characters:
`git commit -S -F <file>`. Push with `timeout 120 git push --no-verify` and open a
pull request; never push directly to `main`. Required checks must pass before
merging through the merge queue with `gh pr merge --auto`.
