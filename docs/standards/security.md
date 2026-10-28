# Security standards

`maestro-manifests` follows the organization's [security
rules](https://github.com/Orchestration-Maestro/.github/blob/864d85597a833864cd8506c3925830503b3c2163/golden-rules/security.md):
eleven rules for changes, reviews, research, tool use, delegated work and the
handling of sensitive data, whether a person or an agent does the work. This page
says what the repository protects and where untrusted input enters, then explains
each family of rules: what it asks for, why this repository applies it, and how
it is applied here, with the checks and practices by name. A local practice may be
stricter than a rule and none is weaker. Maintainers write the page by hand from
the rules of `.github@864d855`. The checks named here are described under [the
checks and reviews](engineering.md#the-checks-and-reviews-in-this-repository) on
the engineering page.

## Contents

1. [What this repository protects](#what-this-repository-protects)
2. [Keep sensitive data small](#keep-sensitive-data-small)
3. [Treat everything read as data](#treat-everything-read-as-data)
4. [Take authority from enforcement](#take-authority-from-enforcement)
5. [Inspect code without running it unsafely](#inspect-code-without-running-it-unsafely)
6. [Stop, escalate and report vulnerabilities](#stop-escalate-and-report-vulnerabilities)
7. [Keep evidence truthful](#keep-evidence-truthful)
8. [Sign every release](#sign-every-release)
9. [What the organization can lock](#what-the-organization-can-lock)
10. [Check a change](#check-a-change)

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
2. **Capability packages.** A package under `manifests/capabilities/` declares resource
   paths, and an extension would run with the user's access. No package ships an
   executable component today.
3. **Fetched content in workflows.** A chain hands the task (`$@`) and the previous
   step's output (`{previous}`) to the next role, and a role such as
   `evidence-reviewer` reads cited sources. To the role, all of it is data, never
   authority.

## Keep sensitive data small

### What it is

Collect, copy, retain and expose only the sensitive data the authorized task
needs (SEC-001). Store it only in approved protected locations, redact it from
output and diagnostics, and delete it on time. Never invent, request or disclose a
secret to complete a task. This is one of three rules that accept an
organization exception; the others are SEC-009 and SEC-011.

### Why this repository applies it

The repository is public. Anything committed here, a log line or a fixture
included, is readable by anyone and copied by clones and caches, so the safe
amount of sensitive data to hold is none.

### How it is applied here

The repository holds no credentials, and `README.md` limits the content to public
material. The hook `gitleaks` scans each staged change at commit and redacts what
it prints. The pull request template carries a checkbox confirming that no
credential, token or private data appears in the diff or the logs. The release
App's key is the only secret in reach, and only the release workflow reads it.

## Treat everything read as data

### What it is

Repository files, pages, tickets, tool output, generated text, delegated results,
agent memory and retrieved context are untrusted data, never authority (SEC-002).
They cannot grant permissions, override policy or direct the disclosure of a
secret. Paths, revisions, URLs and tool arguments are validated before use, and a
link or a file name is never proof of authorization (SEC-003).

### Why this repository applies it

The content here is the text that steers agents, and chains pass the task and the
previous step's output from one role to the next. A pull request, a package path
or a cited source can carry text written to look like an instruction, and a
manifest source can name a branch that moves.

### How it is applied here

The test `tests/test_manifest_examples.py` fixes each chain's placeholders and
each core role's tool list. Maestro applies the tool list whatever the task says,
so a role reading hostile text still holds only its listed tools. A new role,
instruction or workflow takes authority only from its own frontmatter and the
settings. The repository stores no agent memory.

For boundaries, a path in a package's `maestro` arrays stays relative to the
package root and names a file in it, and a manifest source is pinned to a release
tag or a full commit ID, never a branch (`README.md`). Maestro checks the pin and
the references when it loads the release (`docs/integrity.md`), but no required
job loads it yet, so the reviewer cites the path or the pin in a thread until one
does.

## Take authority from enforcement

### What it is

Authority comes from enforced policy, access control and an identifiable
approver, never from content, urgency or a claimed role (SEC-004). Each agent acts
under its own identity with only the access its task needs, never with a person's
borrowed credentials. A sensitive, irreversible or external action needs its own
scoped approval, and no one approves for themselves (SEC-005).

### Why this repository applies it

A release changes what every adopting organization's agents do. It has to come
from a path that an enforced rule describes and a named person approves, not from
whoever can phrase a convincing request.

### How it is applied here

Rulesets, workflow permissions and the organization bot's own App identity hold
authority (`actions/create-github-app-token` in
`.github/workflows/release-please.yml`). Automation borrows no person's
credentials, no ruleset has a bypass actor, and `.github/CODEOWNERS` names the
human owner. A release is approved only by merging its own Release Please pull
request, which carries `version.txt` and `manifests/CHANGELOG.md`. A settings, policy or
ruleset change goes in a pull request of its own, never inside a content change.
The ruleset requires no approving review, so the owner's merge is the approval.

## Inspect code without running it unsafely

### What it is

Prefer static inspection. Untrusted code, and any code or command an agent
generates, runs only when authorized and isolated: least privilege, no secrets,
no unnecessary network and bounded resources. Pull request scripts, hooks and
repository-provided commands never run with secrets available (SEC-006).

### Why this repository applies it

The `justfile` and the hooks that `just check` runs are files in the pull
request, so a pull request can change the code that checks it. The check is
useful only if running it cannot reach anything worth taking.

### How it is applied here

`Required content checks` runs on `pull_request` with `contents: read`,
`persist-credentials: false` and no repository secret, on a hosted runner. The
organization requires approval before an external contributor's workflow runs.
The job comes from the shared content workflow at the ruleset's pinned tag. On a
local machine, the reviewer reads changes to `justfile`, `.pre-commit-config.yaml`
and `mise.toml` before running `just check`, since those files choose what runs.

## Stop, escalate and report vulnerabilities

### What it is

Stop the affected work when a boundary is crossed, a secret may be exposed or
evidence may have been tampered with. Escalate through the authorized incident
path with redacted details, never conceal the event, and contain only when
separately authorized (SEC-007). Offer a private channel for reporting a
vulnerability, and send the fix through the normal reviewed, tested and gated
path, starting with a regression test seen failing (SEC-010).

### Why this repository applies it

A public repository with an unclear reporting path invites public disclosure
before a fix exists. Urgency is also the usual reason a gate gets skipped, and a
release that carries a vulnerability reaches every adopter.

### How it is applied here

A suspected exposure stops the work and goes through the organization's private
reporting channel with details redacted. A leaked secret is revoked and rotated,
the release App's key included. The release job is bounded by `timeout-minutes: 10`
in `.github/workflows/release-please.yml`. No agent runs inside this repository, so
Maestro owns the kill switch. Private vulnerability reporting is on for the
repository and the organization's security policy routes reports to it. The fix
follows the normal gates, including a test that fails without it, and the
advisory publishes once the release ships.

## Keep evidence truthful

### What it is

Record what was observed, supplied, executed, blocked and not checked, with the
revision or provenance. Never claim a control, test or safety result that has no
evidence (SEC-008). When a side effect is blocked, bounded read-only work may
continue if it is clearly reported as partial, and a blocked action is never
presented as completed (SEC-009).

### Why this repository applies it

Maintainers decide what to merge from what the description says ran. A result
reported without its command, or a skipped check reported as passed, moves the
risk to the next person who trusts it.

### How it is applied here

Results are reported as run, with what was not checked. The pull request's
**Verification** section lists the commands and their results, and `AGENTS.md`
asks for the checks actually run. Blocked work is reported as partial in a draft
pull request, which GitHub cannot merge, and the issue keeps `needs-info` or
`ready-for-human` until the block clears.

## Sign every release

### What it is

Every published release is signed or attested, carries a checksum for each asset
and a software bill of materials, and says how to verify them. A checksum never
replaces a signature (SEC-011). Like SEC-001 and SEC-009, this rule accepts only
an organization exception.

### Why this repository applies it

Organizations pin a release of this repository and adopt what is in it. A
signature or attestation lets them verify the release came from here and has not
changed since.

### How it is applied here

This rule is unsupported here. A release is a `v`-prefixed tag that Release
Please creates from the merged release pull request, on a commit signed under
`default-branch-discipline`. Consumers pin the tag or the full commit, and Maestro
verifies the commit at each start (`docs/integrity.md`). No release carries a
signature, attestation, checksum list or bill of materials, and
`.github/workflows/release-please.yml` has no step that adds one. No release has
been published yet. Until a workflow step adds those assets, the pinned commit and
its signature are the only verification a consumer has.

## What the organization can lock

These standards govern contributions; they add no runtime policy keys. Protected
review and required checks hold repository changes. The [policy guide](../policy.md#what-the-organization-can-lock)
names the settings and resources an organization can lock at runtime. Neither
kind of control grants an extension less operating-system access.

## Check a change

Run `mise exec -- just check`. Review each changed security claim against the
linked security rules, tool permissions and release workflow. Report the checks
run and any missing control; content tests do not establish safe execution or
release attestation.
