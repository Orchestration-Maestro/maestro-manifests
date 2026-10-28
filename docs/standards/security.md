# Security rules in `maestro-manifests`

`maestro-manifests` follows the organization's [security
rules](https://github.com/Orchestration-Maestro/.github/blob/864d85597a833864cd8506c3925830503b3c2163/golden-rules/security.md).
This page is its rule map (C-001): for every rule, the gate, check or review step
that holds it here, and the evidence that step leaves. A row may name a stricter
local rule; none weakens one. Maintainers edit the page by hand, from the golden
rules of `.github@864d855`. The checks the rows name are described under [how a
rule is held](engineering.md#how-a-rule-is-held).

## What this repository protects

The content that reaches every developer's agent: the settings, policy, system
prompt, agent roles and workflows that Maestro loads from a pinned release. A
change to a prompt or to a role's tool list changes what each of those agents may
do, so the content is the asset. The repository holds no credentials, models or
executable extensions. The one secret near it is the release App's private key,
held in GitHub and used only by `.github/workflows/release-please.yml`.

Untrusted input enters in three places:

1. **Pull requests.** Their files become the next release, and `just check` runs
   the `justfile` and hooks they carry. The required job runs them with a
   read-only token and no secret.
2. **Capability packages.** A package under `capabilities/` declares resource
   paths, and an extension would run with the user's access. No package ships an
   executable component today.
3. **Fetched content in workflows.** A chain hands the task (`$@`) and the previous
   step's output (`{previous}`) to the next role, and a role such as
   `evidence-reviewer` reads cited sources. To the role, all of it is data, never
   authority.

## Rule map

| Rule | Held here by | Evidence |
| --- | --- | --- |
| SEC-001 Minimise sensitive data | Gate: hook `gitleaks` scans each staged change. Review: the pull request checklist confirms that no credential, token or private data appears in the diff or the logs, and `README.md` limits the content to public material | Redacted hook output; the ticked checklist item |
| SEC-002 Treat input as data | Gate: `tests/test_manifest_examples.py` fixes each chain's placeholders and each core role's tool list, which Maestro applies whatever the task says. Review: a new role, instruction or workflow takes authority only from its own frontmatter and settings. The repository stores no agent memory | The failing test; a thread naming the line where input acts as authority |
| SEC-003 Validate boundaries | Review: a path in a package's `maestro` arrays stays relative to the package root and names a file in it; a manifest source is pinned to a release tag or a full commit ID, never a branch (`README.md`). Maestro checks the pin and the references when it loads the release (`docs/integrity.md`), but no required job loads it yet | A thread citing the path or the pin; the load result once a job runs it |
| SEC-004 Use real authority | Organization: rulesets, workflow permissions and the organization bot's own App identity (`actions/create-github-app-token` in `.github/workflows/release-please.yml`); automation borrows no person's credentials, and `.github/CODEOWNERS` names the human owner | The workflow file; the rulesets, none with a bypass actor |
| SEC-005 Scope sensitive approvals | Review: a release is approved only by merging its own Release Please pull request, and a settings, policy or ruleset change in a pull request of its own, never inside a content change. The ruleset requires no approving review, so the owner's merge is the approval | The release pull request (`version.txt`, `CHANGELOG.md`) and the scope in the description |
| SEC-006 Inspect code safely | Gate: `Required content checks` runs on `pull_request` with `contents: read`, `persist-credentials: false` and no repository secret, on a hosted runner, and the organization requires approval before an external contributor's workflow runs. Review: the reviewer reads changes to `justfile`, `.pre-commit-config.yaml` and `mise.toml` before running `just check` on a local machine | The shared content workflow at the ruleset's pinned tag, the Actions settings and the review thread |
| SEC-007 Stop and escalate incidents | Review: a suspected exposure stops the work and goes through the organization's private reporting channel with details redacted; a leaked secret is revoked and rotated, the release App's key included. Gate: `timeout-minutes: 10` bounds the release job. No agent runs inside this repository; Maestro owns the kill switch | The private report, the rotation record and the timeout in `.github/workflows/release-please.yml` |
| SEC-008 Keep truthful evidence | Review: results are reported as run, with what was not checked; the description's **Verification** section lists the commands and their results, and `AGENTS.md` asks for the checks actually run | The **Verification** section and the job results it cites |
| SEC-009 Preserve safe progress | Review: blocked work is reported as partial in a draft pull request, which GitHub cannot merge, and the issue keeps `needs-info` or `ready-for-human` | The draft pull request and the description naming what was blocked |
| SEC-010 Report vulnerabilities privately | Organization: private vulnerability reporting is on (`maestrolabs-baseline`) and the organization's SECURITY.md routes reports to it. Review: a fix follows the normal gates and starts with a failing test (ENF-005) | The Security tab's report form, the advisory and the fix pull request |
| SEC-011 Sign every release | Review: a release is a `v`-prefixed tag that Release Please creates from the merged release pull request, on a commit signed under `default-branch-discipline`; consumers pin the tag or the full commit, and Maestro verifies the commit at each start (`docs/integrity.md`). No release carries a signature, attestation, checksum or bill of materials, and `.github/workflows/release-please.yml` has no step that adds one, so this rule is unsupported here | The release pull request and the tag; no signed asset, checksum list or bill of materials exists |
