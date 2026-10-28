# Engineering rules in `maestro-manifests`

`maestro-manifests` follows the organization's [engineering
rules](https://github.com/Orchestration-Maestro/.github/blob/864d85597a833864cd8506c3925830503b3c2163/golden-rules/engineering.md).
This page is its rule map (C-001): for every rule, the gate, check or review step
that holds it here, and the evidence that step leaves. A row may name a stricter
local rule; none weakens one. Maintainers edit the page by hand, from the golden
rules of `.github@864d855`.

## How a rule is held

A row opens with its kind of holder: a gate, a review, the organization or
GitHub. A gate gives the same answer on every run. A review is a judgement and is
recorded as one.

- **Content check:** `just check` runs the hooks of `.pre-commit-config.yaml` on
  every tracked file. The `hygiene-central` ruleset requires its job,
  `Required content checks`, on every pull request and merge group.
- **Test:** `just test` runs `tests/`. No required job runs it; run it before
  proposing a change.
- **Commit hooks:** `just setup` installs them. The hooks `workspace-check` and
  `gitleaks` run at commit; the hooks `conventional-commit-header` and
  `eighty-columns` check the message.
- **Review:** the reviewer records each finding as a thread on the pull request.
  The `default-branch-discipline` ruleset refuses the merge while a thread is open.

## Rule map

| Rule | Held here by | Evidence |
| --- | --- | --- |
| FND-001 Think before coding | Review: the pull request states its assumptions, the alternatives weighed and what stays unclear | The **Change** section of the pull request; an unstated assumption is a review thread |
| FND-002 Simplicity first | Review: the pull request names the requirement each change serves and links the issue that states it | The linked issue; a review thread on any change that serves no requirement in it |
| FND-003 Surgical changes | Review: every changed line traces to the issue's goal, and `AGENTS.md` keeps a change within the issue's scope | The diff read against the issue; each adjacent fix listed in the description |
| FND-004 Goal-driven execution | Gate: `Required content checks` runs `just check` on every pull request. Review: the description's **Verification** section holds the check that means done | The job result, and the commands run with their output, or a note that one did not run |
| P-001 YAGNI | Review: a new artifact, area or setting arrives with the reader that needs it and its row in the README artifact index (`docs/index.md`, Check a change); an empty folder is refused | The README row, and a review thread on any file nothing reads |
| P-002 KISS | Review: an artifact keeps the native format of its kind, as its guide under `docs/` defines it, with no private syntax layered on | A review thread citing the guide section the file departs from |
| P-003 DRY | Gate: `just test` fails when a guide's quoted workflow or theme differs from the file it quotes (`tests/test_manifest_examples.py`) | The failing test's name and the guide that drifted |
| P-004 WET | Review: a second copy of a role, chain or passage is allowed; the reviewer refuses a shared abstraction built from two cases | A review thread on the premature shared file, naming the two cases |
| P-005 Rule of three | Review: on the third copy of a passage, one source replaces the copies and the others link to it | The thread naming the three copies, then the link that replaces them |
| P-006 Chesterton's fence | Review: a deletion states why the thing existed, from the issue or commit that added it (`git log -S`) | The reason in the description, with the introducing commit or issue |
| P-007 Boy Scout rule | Review: an improvement outside the goal stays inside a file the change already edits, and the description lists it | The list of adjacent fixes in the description |
| P-008 Least astonishment | Gate: hook `typos` checks spelling against `typos.toml`. Review: a term matches its `CONTEXT.md` entry and keeps one spelling | Hook output naming the word; a thread naming the term and its glossary entry |
| P-009 Single responsibility | Review: a role, chain or guide does one job, and a guide opens with one purpose paragraph and numbered Contents (`docs/index.md`, Check a change) | A thread naming the second job and where it moves |
| P-010 Composition over inheritance | Review: new behaviour is a new area or chain step composed from existing roles, not an existing role copied and edited (`README.md`, How composition works) | The chain or area in the diff that composes the roles |
| P-011 Fail fast | Gate: hooks `check-yaml`, `check-json` and `check-toml` refuse a malformed structured file at commit and in `Required content checks`; `tests/test_quality.py` proves each refusal | `- hook id: check-yaml` in the failing output; the `just test` run |
| P-012 Make illegal states unrepresentable | Gate: `just test` fixes each core role's tools and sections, each chain's `$@` and `{previous}` handoffs and the theme's 51 tokens (`tests/test_manifest_examples.py`). Review: the guide's Format table for the rest, as no schema check exists yet (`docs/schemas-and-checks.md`) | The failing test; a thread citing the Format row |
| P-013 Parse, don't validate | Review: a test or script added here parses a file into a value before it asserts on it, as `tests/test_manifest_examples.py` does with `json.loads` and anchored patterns; Maestro parses the release when it loads it (`docs/schemas-and-checks.md`) | A thread on a check that string-matches a raw file |
| P-014 Principle of least privilege | Gate: the release workflow sets `permissions: contents: read` and mints a token for `contents`, `issues` and `pull-requests` only (`.github/workflows/release-please.yml`); `tests/test_manifest_examples.py` fixes the tool list of each core role and of the evidence reviewer. Organization: only selected actions may run | The workflow file, the failing role test and the organization's Actions settings |
| P-015 Separation of concerns | Review: prompt, persona, instruction, role, workflow and settings stay in their own artifact kinds (`README.md`, Layout), and `.github/CODEOWNERS` names the owner of each area | A thread naming the mixed concerns; the CODEOWNERS entry of the area |
| P-016 Zero, one or many | Gate: `tests/test_manifest_examples.py` fixes the shipped `manifestAreas` as an empty list. Review: a setting that can repeat is a list | The test's name; a thread on a numbered pair |
| P-017 Premature optimisation | Review: a change made for speed cites the measured job time; the Speed KPI on the [Northstar page](northstar.md) is the baseline | The job durations before and after, in the description |
| P-018 Broken windows | Gate: `just check` runs `prek run --all-files`, so existing spelling, Markdown and hygiene debt fails the next pull request, not only the lines it touches | A red `Required content checks` run naming the file |
| ENF-001 No machine-named paths | Review: the reviewer searches the branch with `git grep -n -e '/h[o]me/' -e '/U[s]ers/'`, and examples keep synthetic paths | The empty output recorded in the review; a hit is a finding |
| ENF-002 Every claimed platform is tested | Review: the repository claims no platform, because its content is text that Maestro reads on any system; `Required content checks` runs on one hosted Linux runner, so a claim arrives with its own merge-blocking job | The required checks listed on the pull request; a claim without a job is a thread |
| ENF-003 English only | Review: prose and identifiers are English; hook `typos` flags misspellings but does not prove the language | Hook output, and the reviewer's note on the language |
| ENF-004 Conventional commits | Gate: the organization's `commits-are-conventional` ruleset refuses any other title on the default branch; hooks `conventional-commit-header` and `eighty-columns` check the message at commit; Release Please builds `CHANGELOG.md` from the commit types (`.github/release-please/config.json`) | A refused title, the hook output and the release pull request's changelog |
| ENF-005 Failing test first | Review: a behaviour change lands as a test commit seen failing, then the change that passes it (`AGENTS.md`); the pull request template asks for a test that fails without the change | Two commits in the history, and both runs in the issue's evidence |
| ENF-006 Never weaken a gate | Organization: the `hygiene-central` ruleset requires the content job, no ruleset has a bypass actor, and a gate changes only in its own reviewed pull request. Review: a diff that edits `.pre-commit-config.yaml`, `justfile` or `mise.toml` states why | The rulesets' empty bypass lists; the pull request that edits the gate |
| ENF-007 Pull requests only | Organization: `default-branch-discipline` requires a pull request, signed commits and squash merges; `floor-no-destruction` refuses deletion and force-push of the default branch; the `merge-queue` ruleset merges only green entries | The rulesets; a refused direct push, force-push or unsigned commit |
| ENF-008 Tiered checks | Gate: hook `workspace-check` runs `just pre-commit` on the staged files at commit, and `just check` runs the same hooks on every file in `Required content checks`, which blocks the merge. Code scanning's weekly scan is the heavy tier. No pre-push tier exists because no check is heavier than the commit tier | `.pre-commit-config.yaml`, `justfile` and the job log |
| ENF-009 Allowlists that cannot rot | Review: the allowlists are the `FND` word in `typos.toml`, the rules disabled and the HTML elements allowed in the `rumdl` entry, and `-disable-indent-size` in the `editorconfig-checker` entry of `.pre-commit-config.yaml`; each is added with its reason in the pull request. No check fails when one goes stale, so review alone holds this rule | The entries, and the pull request that added each |
| ENF-010 Configuration is the authority | Organization: rulesets and settings live as code in the `.github` repository, and the Organization drift workflow compares them with GitHub weekly. Review: a pull request that renames a required check updates the ruleset that requires it in the same change | The workflow run, and the paired pull requests |
| ENF-011 Instructions grant nothing | Gate: authority comes from the rulesets, `permissions:` in `.github/workflows/release-please.yml` and each role's `tools:` list that Maestro applies, never from `AGENTS.md` or an instruction file | The rulesets, the workflow file and the role tests |
| ENF-012 Pinned inputs | Gate: `mise.toml` pins each tool to an exact version and `mise.lock` records its checksum and provenance; action references in `.github/workflows/release-please.yml` use full commit SHAs, which the organization's Actions policy requires; `.github/dependabot.yml` proposes updates weekly after a seven-day cooldown. No dependency needs a lockfile: `package.json` lists none and the tests use only the Python standard library | The lock file, a refused unpinned action and the Dependabot pull requests |
| ENF-013 No secret in history | Gate: hook `gitleaks` scans each staged change at commit; it is a local hook, and no CI job runs it. Organization: secret scanning with push protection and validity checks (`maestrolabs-baseline`) blocks a push that carries a secret | The hook output, a blocked push, and the rotation record of any leak (SEC-007) |
| ENF-014 Multi-factor authentication | Organization: two-factor authentication is required of every member and outside collaborator (`two_factor_requirement_enabled` in the organization settings) | The enforced setting in the `.github` repository's organization settings, compared weekly by the drift workflow |
| C-001 Map every rule | Gate: `tests/test_standards_pages.py` fails on a row with no holder or evidence, a holder shared by two rules, or a gate named that this repository lacks. Review: a maintainer maps a rule added upstream when moving the revision linked above | The test run, and the pull request that moves the revision |
| C-004 Detect drift | Organization: the Organization drift workflow checks this repository daily against the file baseline (these three pages among its files), the merge queue, the tool pins and the metadata, and keeps a `Drift: maestro-manifests` issue open in the `.github` repository while any differs | The workflow run and the issue |
| C-005 Keep the evidence | GitHub: pull requests keep the description and review threads; Actions keeps run logs for 30 days (organization setting), so the description carries the command output that matters | The pull request, the `Required content checks` run and the drift issue |
| C-006 Controlled exceptions | Review: an exception is recorded in the pull request that makes it, with its decision, scope, rationale and expiry; the organization approves it, never the author or an agent | The exception section of that pull request |
