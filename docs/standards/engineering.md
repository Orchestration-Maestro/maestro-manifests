# Engineering standards

`maestro-manifests` follows the organization's [engineering
rules](https://github.com/Orchestration-Maestro/.github/blob/864d85597a833864cd8506c3925830503b3c2163/golden-rules/engineering.md):
four foundations, eighteen named principles, fourteen hard mandates and four
rules for adopting them. This page explains each family of rules in turn: what
the rules ask for, why a repository of manifest content applies them, and how
this repository meets them, with the checks and practices by name. A local
practice may be stricter than a rule and none is weaker. Maintainers write the
page by hand from the rules of `.github@864d855` and update it when they move
that revision. Rule IDs such as P-011 or ENF-005 point back to the organization's
text. The [security standards](security.md) and the
[Northstar](northstar.md) complete the set.

## Contents

1. [The checks and reviews in this repository](#the-checks-and-reviews-in-this-repository)
2. [Think first, then change only what the goal needs](#think-first-then-change-only-what-the-goal-needs)
3. [Keep the design small and plain](#keep-the-design-small-and-plain)
4. [Say a thing once, but not too early](#say-a-thing-once-but-not-too-early)
5. [Respect what is already here](#respect-what-is-already-here)
6. [Refuse bad input and bad states](#refuse-bad-input-and-bad-states)
7. [Grant only the authority a task needs](#grant-only-the-authority-a-task-needs)
8. [Let tests and gates carry the rules](#let-tests-and-gates-carry-the-rules)
9. [Deliver through signed pull requests](#deliver-through-signed-pull-requests)
10. [Pin inputs and keep secrets out](#pin-inputs-and-keep-secrets-out)
11. [Write portable plain text](#write-portable-plain-text)
12. [Adopt the rules and prove it](#adopt-the-rules-and-prove-it)
13. [What the organization can lock](#what-the-organization-can-lock)
14. [Check a change](#check-a-change)

## The checks and reviews in this repository

Every practice below is held by a gate or by a review. A gate gives the same
answer on every run. A review is a judgement, and the page says so where a
review is all that holds a practice. Where neither exists, the rule is
unsupported here and the page says that too.

- **Content check.** `just check` runs `just content`, which runs the hooks of
  `.pre-commit-config.yaml` on every tracked file, then `just rust` and `just test`. The job
  `Required content checks` runs it on each pull request and merge group, on a
  hosted `ubuntu-24.04` runner with `contents: read` and no secret. The
  `hygiene-central` ruleset requires that job.
- **Rust gate.** `just rust` runs `cargo fmt --check` and Clippy with warnings
  denied. A pull request with an unformatted file or a warning fails the required job.
- **Tests.** `just test` runs the Rust test crate with `cargo test --locked`, then
  the standard-library tests under `tests/`. Because `just check` runs them, a
  pull request that breaks one fails the required job.
- **Commit hooks.** `just setup` installs them. At commit, the hook
  `workspace-check` runs `just pre-commit` on the staged files and the hook
  `gitleaks` scans them for secrets. The hooks `conventional-commit-header` and
  `eighty-columns` check the commit message.
- **Review.** The reviewer records each finding as a thread on the pull request,
  and the `default-branch-discipline` ruleset refuses the merge while a thread is
  open.

## Think first, then change only what the goal needs

### What it is

The four foundations come before every other rule. Think before coding: state
assumptions, surface every reading of the request, and stop when something is
unclear (FND-001). Keep it simple: choose the least complex solution that meets
the requirement, with nothing speculative (FND-002). Make surgical changes: touch
only what the goal requires, with no drive-by refactor (FND-003). Execute toward
a goal: define what done looks like, run that check, and report what it said
(FND-004).

### Why this repository applies it

The content here reaches every developer's agent. A drive-by edit to a prompt or
to a role's tool list ships to all of them in the next release, and a reviewer
cannot weigh a change whose goal is unstated. A short, single-purpose change is
the cheapest one to review and to revert.

### How it is applied here

Work starts from an issue, and `AGENTS.md` asks the contributor to keep the
change within the issue's scope. The pull request description follows the
organization's template. Its **Change** section states the assumptions and links
the issue, and its **Verification** section lists the commands run with their
results, or says that one did not run. The reviewer reads the diff against the
issue, opens a thread on any line that serves no requirement in it, and expects
each adjacent fix to be listed. `Required content checks` gives every content
change the same observable meaning of done: `just check` exits 0.

## Keep the design small and plain

### What it is

Eight principles keep a design easy to read. Build for the requirement in front
of you, not a possible future (YAGNI, P-001). Prefer the boring construct the
next reader understands (KISS, P-002). Make the reader's first guess correct
(least astonishment, P-008). Give each unit one reason to change (single
responsibility, P-009), assemble behaviour instead of inheriting it (composition
over inheritance, P-010), and keep distinct jobs apart (separation of concerns,
P-015). If something can happen twice, design for any count (zero, one or many,
P-016), and measure before optimising (P-017).

### Why this repository applies it

Every artifact here has two readers, a maintainer and an agent, and Maestro
defines the native format of each kind. A private syntax, a guide that does three
jobs or a role copied and edited makes both readers guess. Areas load in a
documented order and later areas narrow earlier ones, so a role or setting built
from parts is easier to predict than one that carries its own variant.

### How it is applied here

A new artifact, area or setting arrives with the reader that needs it and its
row in the README artifact index (`docs/index.md`, Check a change); the reviewer
refuses an empty folder. Each artifact keeps the native format of its kind, as
its guide under `docs/` defines it. A role, chain or guide does one job, and a
guide opens with one purpose paragraph and numbered contents. New behaviour is a
new area or chain step composed from existing roles, as `README.md` describes
under How composition works, and prompt, persona, instruction, role, workflow
and settings stay in their own artifact kinds (`README.md`, Layout).

The hook `typos` checks spelling against `typos.toml`, and the reviewer checks
that a term matches its `CONTEXT.md` entry and keeps one spelling. A setting
that can repeat is a list, and `tests/test_manifest_examples.py` fixes the
shipped `manifestAreas` as an empty list. A change made for speed cites the
measured job time, with the Speed KPI on the [Northstar page](northstar.md) as
the baseline. These practices are reviews, except the spelling hook and the test
named here.

## Say a thing once, but not too early

### What it is

Three principles govern repetition. Share what is genuinely one idea, not what is
merely similar text (DRY, P-003). Write a thing twice before guessing an
abstraction (WET, P-004). Consider extraction on the third occurrence (rule of
three, P-005).

### Why this repository applies it

Guides quote real release files, and a quotation that falls behind its source
teaches the wrong thing to both readers. Roles and chains that look alike often
change for different reasons, and an abstraction built from two cases couples
them before anyone knows what they share.

### How it is applied here

The test `tests/test_manifest_examples.py` fails when a guide's quoted workflow
or theme differs from the file it quotes, so the copy in the guide cannot drift.
For everything else the reviewer decides. A second copy of a role, chain or
passage is allowed, and a shared abstraction built from two cases is refused.
On the third copy, the reviewer asks for one source and links from the others,
and the thread names the three copies.

## Respect what is already here

### What it is

Understand why something exists before removing it (Chesterton's fence, P-006).
Improve within the diff you already have reason to touch (Boy Scout rule,
P-007). Fix small neglect before it becomes permission for more (broken windows,
P-018). The last of these is non-negotiable.

### Why this repository applies it

A line in a prompt or a restriction in a tool list often exists because of an
incident, and deleting it changes what every user's agent does without failing
any check. In a shared catalog, small debt also gets copied into the next
artifact.

### How it is applied here

A deletion states why the thing existed, from the issue or commit that added it;
`git log -S` finds that commit. Necessary adjacent fixes stay within the affected
diff, and the description lists them. Broken windows have a gate: `just check`
runs `prek run --all-files`, so spelling, Markdown and hygiene debt anywhere in the repository fails the next pull request, not only
the lines that pull request touches.

## Refuse bad input and bad states

### What it is

Refuse bad input at the boundary (fail fast, P-011). Encode constraints so that
invalid states cannot be constructed (P-012). Turn unstructured input into a
value that carries the checked guarantee, instead of validating and passing the
raw input on (parse, don't validate, P-013). Fail fast and parse, don't validate
are non-negotiable.

### Why this repository applies it

The release is data that Maestro loads. A malformed file, a role with the wrong
tool list or a chain that drops its handoff would otherwise be found by every
developer's agent at startup, not by one contributor in review.

### How it is applied here

The hooks `check-yaml`, `check-json` and `check-toml` refuse a malformed
structured file at commit and in `Required content checks`. The test
`tests/test_quality.py` stages a broken file of each kind in a disposable index
and requires the failing run to name the hook. The test
`tests/test_manifest_examples.py` fixes each core role's tools and section
headings, each chain's `$@` and `{previous}` handoffs and the 51 tokens of the
review theme. No schema check exists yet (`docs/schemas-and-checks.md`), so the
Format table in each guide holds the rest by review.

Tests and scripts added here parse a file into a value before they assert on it,
as `tests/test_manifest_examples.py` does with `json.loads` and anchored
patterns, and the reviewer opens a thread on a check that string-matches a raw
file. Maestro parses the release when it loads it; no required job loads it yet.

## Grant only the authority a task needs

### What it is

Give each token, workflow and actor only the access it needs (least privilege,
P-014, non-negotiable). Instruction prose and links never grant tools,
permissions, execution authority or security exemptions; authority comes only
from enforced configuration and access control (ENF-011).

### Why this repository applies it

Role files tell an agent what it may do, and a line in a prompt that says an
agent may push grants nothing. A tool list wider than the job gives that reach to
every user's agent, so the limit has to live where Maestro applies it, and the
same holds for the workflows that build the release.

### How it is applied here

The release workflow sets `permissions: contents: read` and mints a token for
`contents`, `issues` and `pull-requests` only
(`.github/workflows/release-please.yml`). The required job reads the repository
with no secret, and the organization allows only selected actions to run. The
test `tests/test_manifest_examples.py` fixes the tool list of each core role and
of the evidence reviewer, and Maestro applies the `tools:` list whatever the
prompt says. `AGENTS.md` and instruction files grant nothing: authority comes
from the rulesets, the `permissions:` block and the `tools:` lists.

## Let tests and gates carry the rules

### What it is

Five mandates make the rules mechanical. A behaviour change starts with a test
seen failing (ENF-005). A gate is never weakened or bypassed to pass a change; a
gate that blocks something correct is reported and fixed in its own change
(ENF-006, non-negotiable). Cheap checks run at commit, the local aggregate runs
the same commands as CI, and heavy checks report weekly (tiered checks, ENF-008).
Every allowlist entry records its reason, and a check fails when it stops being
true (ENF-009). Enforced configuration beats settings applied by hand (ENF-010).

### Why this repository applies it

A rule that only a person remembers is dropped on the busiest day. A gate answers
the same way each time, and a gate weakened to let one change through protects
nothing afterwards. Because the catalog is content, the gates are also the only
thing that looks at every file on every pull request.

### How it is applied here

A behaviour change lands as a test commit seen failing, then the change that
passes it, as `AGENTS.md` asks, and the organization's template asks for a test
that fails without the change. The `hygiene-central` ruleset requires the content
job and no ruleset has a bypass actor. A diff that edits `.pre-commit-config.yaml`,
`justfile` or `mise.toml` states why, and a gate changes only in a pull request
of its own.

The tiers are the hook `workspace-check`, which runs `just pre-commit` on the
staged files at commit, and `just check`, which runs the hooks on every file in
the merge-blocking job and then Clippy and the Rust and Python tests, which the
commit tier does not run. Code scanning's weekly scan is the heavy tier. No
pre-push tier exists: `just check` is the heavier tier before merge.
Rulesets and organization settings live as code in the `.github` repository, and
the Organization drift workflow compares them with GitHub every week. A pull
request that renames a required check updates the ruleset that requires it in the
same change.

The allowances are the `FND` word in `typos.toml`, the rules disabled and the
HTML elements allowed in the `rumdl` entry, and `-disable-indent-size` in the
`editorconfig-checker` entry of `.pre-commit-config.yaml`. Each is added with its
reason in the pull request. Review covers the recorded reasons; stale-entry
detection remains unsupported here.

## Deliver through signed pull requests

### What it is

Commit titles follow Conventional Commits, the organization refuses any other
form, and changelogs come from the commit types (ENF-004). Every change to the
default branch arrives through a signed pull request, maintainers included, and
the platform refuses a direct push, a force-push and the deletion of the default
branch (ENF-007).

### Why this repository applies it

A release of this repository is what organizations pin and adopt, so the history
behind it has to be attributable and readable. The commit type decides the next
version and the changelog entry, and a signed, reviewed path means no one,
human or agent, changes shipped content unseen.

### How it is applied here

The organization's `commits-are-conventional` ruleset refuses any other title on
the default branch. The hooks `conventional-commit-header` and `eighty-columns`
check the message at commit. Release Please builds `manifests/CHANGELOG.md` from the commit
types in `.github/release-please/config.json`. The `default-branch-discipline`
ruleset requires a pull request, signed commits, squash merges and resolved
review threads. It requires no count of approving reviews, so the owner's merge
is the approval. The `floor-no-destruction` ruleset refuses deletion and
force-push of the default branch, and the `merge-queue` ruleset merges only
entries that are green.

## Pin inputs and keep secrets out

### What it is

Every dependency resolves from a committed lockfile, every external action is
pinned to a full commit SHA, and every downloaded tool is checked against a
pinned checksum (ENF-012). A secret never enters version control; every push is
scanned, and a leaked secret is revoked and rotated, not just deleted (ENF-013).
Every member and outside collaborator signs in with a second factor (ENF-014).

### Why this repository applies it

A mutable tag or an unverified download lets someone else change what runs in the
checks and in the release. A secret in history stays readable after a delete. The
accounts that can merge a release are the last link in that chain.

### How it is applied here

`mise.toml` pins each tool to an exact version and `mise.lock` records its
checksum and provenance. Action references in
`.github/workflows/release-please.yml` use full commit SHAs, which the
organization's Actions policy requires, and `.github/dependabot.yml` proposes
updates weekly after a seven-day cooldown. `Cargo.lock` pins the test crate's
dependencies and `manifests/package.json` lists none. `rust-toolchain.toml` pins
Rust, but rustup downloads the toolchain and the repository pins no checksum for
it, so ENF-012 has a gap here.

The hook `gitleaks` scans each staged change at commit. It is a local hook and no
CI job runs it. On the platform, secret scanning with push protection and
validity checks blocks a push that carries a secret, and a leak is revoked and
rotated as the security standards describe. The organization requires two-factor
authentication of every member and outside collaborator, and the weekly drift
workflow compares that setting with the enforced value.

## Write portable plain text

### What it is

Code, configuration, task runners and workflows never write an absolute path
that names a machine, such as a home directory or a drive letter (ENF-001). Each
platform a repository claims is covered by merge-blocking checks on every pull
request, and a platform without that coverage is not claimed (ENF-002). Prose and
identifiers are English (ENF-003).

### Why this repository applies it

The release is text that Maestro reads on whatever system a developer uses. A path
that names one machine works on one machine, and a promise of support that no
check backs misleads the people who rely on it. One language keeps the catalog
readable to every contributor and every agent.

### How it is applied here

The reviewer searches the branch with `git grep -n -e '/h[o]me/' -e '/U[s]ers/'`
and records the empty output; examples keep synthetic paths, and a hit is a
finding. The repository claims no platform, because its content is text, and
`Required content checks` runs on one hosted Linux runner. A claim of support
would arrive with its own merge-blocking job. The hook `typos` flags
misspellings, but it does not prove the language, so the reviewer notes that
prose and identifiers are English.

## Adopt the rules and prove it

### What it is

Four rules govern adoption. Each repository records what holds every rule, or
why a rule does not apply (map every rule, C-001). An automated check detects
when the repository stops holding the rules and escalates what it finds (detect
drift, C-004). Evidence of compliance is kept, and so is the record of every
control that was unavailable (C-005). An exception exists only as an authorized,
scoped and expiring decision (C-006, non-negotiable).

### Why this repository applies it

A rule nobody can find the holder of is not held, and a page that drifts from the
rules misleads the next contributor. The record of a gap is as useful as the
record of a pass: it tells the owner where to spend the next change.

### How it is applied here

This page and the [security standards](security.md) are the map. Each family
explains how its rules are held, and the gaps are stated where they occur. The
test `tests/test_standards_pages.py` fails when a family loses its explanation of
what, why or how, when a golden rule is no longer named, when a table appears or
when a page names a file, recipe or hook this repository lacks. A maintainer
maps a rule added upstream when moving the revision linked above.

The Organization drift workflow checks this repository daily against the file
baseline, the merge queue, the tool pins and the metadata, and keeps a
`Drift: maestro-manifests` issue open in the `.github` repository while any
differs. It checks that these pages exist, not what they say. Pull requests keep
the description and the review threads, and Actions keeps run logs for 30 days,
so the description carries the command output that matters. An exception is
recorded in the pull request that makes it, with its decision, scope, rationale
and expiry. The organization approves it, never the author or an agent, and a
non-negotiable rule accepts none.

## What the organization can lock

These standards govern contributions; they add no runtime policy keys. Protected
review and required checks hold repository changes. The [policy guide](../policy.md#what-the-organization-can-lock)
names the settings and resources an organization can lock at runtime. Neither
kind of control grants an extension less operating-system access.

## Check a change

Run `mise exec -- just check`. The standards-page tests check the rule families,
explanations, named checks and Contents. Review each changed practice against the
linked engineering rules and the repository configuration. Report command exit
codes and any unavailable control in the pull request.
