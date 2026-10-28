# Copilot instructions for maestro-manifests

Read [AGENTS.md](../AGENTS.md) before editing. It owns the issue-first workflow,
the checks and the delivery rules. [CONTEXT.md](../CONTEXT.md) defines the
manifest terms; [README.md](../README.md) describes the catalog and its
public/private boundary.

This file is written by hand and changed through a pull request.

Before you propose a change, run `mise exec -- just check` and
`mise exec -- just test`. Report both results with their exit codes. The required
content workflow runs `just check` on every pull request; `just test` is yours to
run.

Give manifest content changes their own issues. Checked by: review compares the
diff with the issue.
