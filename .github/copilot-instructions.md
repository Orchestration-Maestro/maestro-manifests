# Copilot instructions for maestro-manifests

Read [AGENTS.md](../AGENTS.md) before editing. It links to the shared rules and
describes this repository's setup, checks and manifest content requirements.
[CONTEXT.md](../CONTEXT.md) defines the manifest terms;
[README.md](../README.md) describes the catalog and its public/private boundary.

Before you propose a change, run `mise exec -- just check`, which runs the
content checks, the Rust checks and the tests. Report its result with its exit
code. The required content workflow runs `just check` on every pull request.

Give manifest content and schema changes their own issues. Checked by: review
compares the diff with the issue.
