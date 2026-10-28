# Northstar for `maestro-manifests`

> Automate the guardrails to deliver faster, with higher quality, and more
> securely.

`maestro-manifests` steers by the organization's
[Northstar](https://github.com/Orchestration-Maestro/.github/blob/864d85597a833864cd8506c3925830503b3c2163/golden-rules/northstar.md):
one KPI per pillar, each with its measurement. Unmeasured is written `not
measured`, never estimated; a value read by hand carries the date it was read.

## The point

An organization that runs Maestro gets its settings, policy, system prompt, roles
and workflows from one pinned, reviewed Git release, so no developer assembles
them alone. When it works, every developer's agent starts from the same reviewed
configuration, a change reaches them as a release they adopt, and nothing outside
the pinned release alters it.

## KPIs

| Pillar | KPI | Current | Target | Measured by |
| --- | --- | --- | --- | --- |
| Speed | Duration of the required content job | 7 s, median of the last 8 runs, read by hand on 2026-10-28 | under 30 s | The duration of the `Required content checks` job on each pull request and merge group |
| Quality | Failing tests in `just test` | 0, read by hand on 2026-10-28 | 0 | `just test`, which runs `tests/`; no required job runs it yet |
| Maintainability | Findings from the content checks on the default branch | 0 | 0 | `Required content checks`, which runs `just check` on every pull request and merge group |
| Security | Open code-scanning, secret-scanning and Dependabot alerts | 0, read by hand on 2026-10-28 | 0 | Code scanning (default setup), secret scanning and Dependabot alerts on the repository's Security tab |
