# Guards

A guard is a rule that checks a command, a path or a piece of text and then
blocks, confirms, redacts or warns. The rules live in four YAML files under
`manifests/core/guards/`, in the `maestro.guards/v1` format of Maestro's
manifest-mode contract. This page is for security reviewers: it explains the
format, what each family of rules does, how to propose, test and review a rule,
and how CODEOWNERS routes a guard change to review.

## Contents

1. [Example](#example)
2. [Format](#format)
3. [Targets and actions](#targets-and-actions)
4. [The four families](#the-four-families)
5. [Propose, test and review a rule](#propose-test-and-review-a-rule)
6. [Review routing](#review-routing)
7. [Usage](#usage)
8. [Validation](#validation)
9. [What the organization can lock](#what-the-organization-can-lock)
10. [Check a change](#check-a-change)

## Example

The protected-paths family blocks the `write` and `edit` tools on three kinds of
path. Each rule carries the examples that prove what it matches:

From [`manifests/core/guards/protected-paths.yaml`](../manifests/core/guards/protected-paths.yaml):

```yaml
apiVersion: maestro.guards/v1
rules:
  - id: environment-files
    description: Environment files hold credentials and are never edited by the agent
    regex: '\.env'
    action: block
    appliesTo: [path]
    examples:
      match:
        - '.env'
        - 'api/.env.local'
        - '/srv/app/.env.production'
        - 'C:\work\app\.env'
        - '.envrc'
        - 'config/prod.env'
        - '.env-backup'
        - 'venv/.env/bin/activate'
        - '.envs/.production/.django'
        - '.env_local'
        - '.env2'
        - '.envrc.local'
      noMatch:
        - 'src/environment.ts'
        - 'docs/env-guide.md'
  - id: git-folder
    description: The Git folder holds history and hooks and is never edited by the agent
    regex: '\.git(?:[/\\]|$)'
    action: block
    appliesTo: [path]
    examples:
      match:
        - '.git/config'
        - 'repo/.git/hooks/pre-commit'
        - 'repo\.git\HEAD'
        - 'src/widget.git/readme.md'
      noMatch:
        - '.github/workflows/ci.yml'
        - '.gitignore'
  - id: installed-dependencies
    description: Installed dependencies are generated and are never edited by the agent
    regex: 'node_modules[/\\]'
    action: block
    appliesTo: [path]
    examples:
      match:
        - 'node_modules/left-pad/index.js'
        - 'web/node_modules/react/package.json'
        - 'my_node_modules/x.js'
      noMatch:
        - 'src/node_modules_notes.md'
```

Redaction rules replace each match with a marker. This rule from the `pii`
family turns a Canadian social insurance number into `[PII:canada-sin]`:

Excerpt from [`manifests/core/guards/pii.yaml`](../manifests/core/guards/pii.yaml):

```yaml
apiVersion: maestro.guards/v1
rules:
  - id: canada-sin
    description: Canadian social insurance number
    regex: '\b(?:\d{3}-\d{3}-\d{3}|\d{3} \d{3} \d{3}|\d{9})\b'
    action: redact
    appliesTo: [tool-output, agent-output, prompt]
    examples:
      match:
        - 'SIN 046-454-286'
        - 'SIN 046 454 286'
        - 'SIN 046454286'
      noMatch:
        - 'order 12345'
        - 'call 416-555-0199'
        - 'date 2026-10-30'
```

## Format

Each `manifests/core/guards/<family>.yaml` file has two members: `apiVersion`,
exactly `maestro.guards/v1`, and `rules`, a non-empty list. The family name is
lowercase and hyphenated, at most 64 characters. Maestro reads files in
file-name order and rules in list order. Every rule member is required:

| Member | Contract |
| --- | --- |
| `id` | Lowercase and hyphenated, at most 64 characters, unique across all guard files |
| `description` | One line. Maestro shows it in every refusal, confirmation and warning the rule causes, so write it to read after `Blocked by guard <id>:` |
| `regex` | A regular expression in the syntax of the workspace's `regex` crate |
| `action` | `block`, `confirm`, `redact` or `warn` |
| `appliesTo` | A non-empty list of targets: `command`, `path`, `tool-output`, `agent-output` or `prompt` |
| `examples` | `match`, a non-empty list of strings the expression must match, and `noMatch`, a non-empty list of strings it must not match |

The expression syntax has no look-around and no back-references, and matching
takes time linear in the length of the text. Inline flags such as `(?i)` set
case behaviour. Classes such as `\b`, `\d` and `\s` are Unicode-aware unless the
pattern turns Unicode off, for example with `(?-u:\b)`. Write patterns in
single-quoted YAML strings so backslashes stay literal.

An unknown or missing member, a duplicate `id`, an expression that does not
compile, an action that does not allow one of the rule's targets, or an example
that does not behave as listed makes the whole manifest invalid.

## Targets and actions

Guards never check images. Each target is checked at one moment:

| Target | Text checked | When |
| --- | --- | --- |
| `command` | The `command` argument of the `bash` tool and the user's `!` and `!!` shell commands | Before the command runs |
| `path` | The `path` argument of the `write` and `edit` tools, as given | Before the tool runs |
| `tool-output` | Each text block of a tool result, the strings in its details and the output of the user's shell commands | Before the result reaches the model, the screen or the session file |
| `agent-output` | Each text and thinking block of an assistant message | When the message ends, before it is shown, saved or sent again |
| `prompt` | Each text block of a user message, from every route, after template and skill expansion | When the message ends, before it is saved or sent |

An action allows only some targets:

| Action | Targets | Effect |
| --- | --- | --- |
| `block` | Every target | Refuses the call with `Blocked by guard <id>: <description>`, or withholds the result, response or prompt with the same id and description |
| `confirm` | `command` and `path` | Shows the description and the command or path, and offers `Yes` and `No`. `No` refuses as `block` does. Print mode, JSON mode and RPC without a user interface refuse as `block` does |
| `redact` | `tool-output`, `agent-output` and `prompt` | Replaces each match with `[<FAMILY>:<id>]`, where `<FAMILY>` is the file's family in capitals |
| `warn` | Every target | Lets the operation go ahead unchanged and shows a warning that names the rule |

When several rules match one target, all of them read the original text. A
matching `block` rule wins, then a matching `confirm` rule, and the first such
rule in file and list order names the refusal or the question. Otherwise
Maestro merges the overlapping `redact` ranges and replaces each merged range
once, with the marker of the first contributing rule. Replacement text is never
matched again.

## The four families

| File | Action | Targets | Rules |
| --- | --- | --- | --- |
| [`destructive-commands.yaml`](../manifests/core/guards/destructive-commands.yaml) | `confirm` | `command` | `recursive-rm`, `sudo`, `world-writable-permissions` |
| [`protected-paths.yaml`](../manifests/core/guards/protected-paths.yaml) | `block` | `path` | `environment-files`, `git-folder`, `installed-dependencies` |
| [`pii.yaml`](../manifests/core/guards/pii.yaml) | `redact` | `tool-output`, `agent-output`, `prompt` | `canada-sin`, `payment-card`, `email-address`, `phone-number`, `iban` |
| [`prompt-injection.yaml`](../manifests/core/guards/prompt-injection.yaml) | `block` | `tool-output` | `ignore-previous-instructions`, `reveal-system-prompt`, `address-to-the-agent`, `send-secrets-to-url`, `chat-template-markers` |

**Destructive commands** start from the three patterns of Maestro's shipped
permission-gate example: recursive `rm`, `sudo` and a mode ending in 777, such
as 1777, 4777 or 0o777, with `chmod` or `chown`. The rules read the command
text and do not parse the shell, so `echo "rm -rf x"` asks too. A `confirm` rule
asks; it does not forbid. The recursive-removal rule also catches flag clusters
such as `-fr` and `-R`, and flags that come before the recursive one.

**Protected paths** start from the paths of Maestro's shipped protected-paths
example: environment files, the Git folder and the installed-dependencies
folder. The environment rule blocks any path that holds `.env`, as the example
does, so `.envrc`, `prod.env`, `.env-backup`, `.env_local`, `.envs/` and a
`.env/` folder are blocked, and so is a template such as `.env.example`. The Git
and installed-dependencies rules match the same paths as the example, plus
backslash separators: `.git/` and `node_modules/` anywhere in a path, including
a bare `widget.git/` folder. Unlike the example, the Git rule also blocks a path
that ends in `.git`. `.github/` and `.gitignore` pass. Path rules check the
`path` argument of `write` and `edit` only; they do not see a path inside a `bash` command, which only the `command`
rules read.

**Personal data** is redacted in tool output, assistant text and prompts. The
rules cover Canadian social insurance numbers, payment card numbers, email
addresses, telephone numbers and IBANs. They match shapes and do not validate
checksums, so some digit strings that are not personal data are redacted too: a
nine-digit number reads as a social insurance number, and a 13 to 19 digit
number that starts with 2 to 6 reads as a card. The email rule also matches an
SSH remote such as `git@github.com`. A false positive costs a marker; a miss
sends the real value on.

**Prompt injection** blocks a tool result when fetched content tries to steer
the agent: telling it to drop its instructions, asking it to reveal its system
prompt, addressing the agent directly, asking it to send secrets to a web
address, or embedding the control tokens of a chat template. The block replaces
the whole result and marks it as an error, so the model learns why. These are
phrase patterns. They raise the cost of a crude attack and are not a boundary.

## Propose, test and review a rule

To propose a rule:

1. Open an issue that names the family and the risk. Give manifest content its
   own issue, apart from schema changes.
2. Add the rule to the family file, or add a file for a new family. Fill in
   every member. Pick the action from [the table above](#targets-and-actions).
3. Write the examples first. Include at least one real match and one near miss
   in `noMatch`. Use synthetic values only: documentation numbers such as the
   test social insurance number and test card numbers, `example.com` addresses
   and `555` telephone numbers.
4. Run `mise exec -- just check`. The guard tests compile the expression with
   the `regex` crate and run every example, so a look-around, a back-reference
   or a wrong example fails and names the file and the rule.
5. Open a pull request that names the issue. CODEOWNERS requests the security
   review described under [review routing](#review-routing).

To review a rule, check these in order:

1. The description says what the rule catches and reads after
   `Blocked by guard <id>:`.
2. The action fits the risk and allows every target.
3. The expression uses the `regex` crate syntax, with no look-around and no
   back-references.
4. The `noMatch` examples are near misses, not strings that could never match.
5. The cost of a false positive is acceptable for the action. A `block` on a
   common word stops real work.
6. No example holds a real credential, person or account.

Review `prompt-injection.yaml` and `pii.yaml` in the pull request diff, not
through an agent. In manifest mode Maestro withholds `prompt-injection.yaml`
when an agent reads it, because the file's own examples match its rules, and
`pii.yaml` comes back redacted the same way.

## Review routing

[CODEOWNERS](../.github/CODEOWNERS) has an explicit entry for the guard folder:

```text
/manifests/core/guards/ @FrancoisLDaigneault
```

GitHub requests review from the last matching pattern, so every pull request that
touches a guard file asks this owner, and the catalog attributes the rules to
them. The ruleset requires no approving review, so the owner's merge is the
approval, as the [security standards](standards/security.md) describe. An
organization with a security team replaces the principal with that team after it
confirms the team has repository access. The entry names the folder, not each
file, so a new family file is routed without a change to CODEOWNERS. Ownership is
review attribution and not runtime permission.

## Usage

Guards run only in manifest mode, in every session that loads the manifest
package. Select the release as [settings](settings.md#usage) describes. The four
files then apply with no further setting, once Maestro's manifest package reads
them ([maestro#2910](https://github.com/Orchestration-Maestro/maestro/issues/2910)). A refusal, confirmation or warning
names the rule by its id and description, and a redaction leaves the marker
`[<FAMILY>:<id>]` in the text. The core system prompt tells the agent to treat a
refusal as final and report it.

## Validation

`just check` parses the YAML, checks spelling and Markdown, and checks every file
against `.editorconfig`. It also runs the guard tests in `tests/guards.rs`, which
read each file in `manifests/core/guards/` and, for every rule, compile the
expression with the `regex` crate Maestro uses and run its `match` and `noMatch`
examples. A failure names the file, the rule id and the example. The tests read the
YAML as an untyped value, so they do not check the file's shape; Maestro's schema
does that.

The contract has Maestro check the same things when it loads the manifest, and a
failure makes the manifest invalid. The release load check, `maestro -p "/manifest status"` on the pull
request's commit, covers this once Maestro's manifest package reads guard files
([maestro#2910](https://github.com/Orchestration-Maestro/maestro/issues/2910)).
The schema check for guard files waits on
[maestro#2812](https://github.com/Orchestration-Maestro/maestro/issues/2812).
Until both ship, no check loads a guard file with Maestro itself. Report the load
check and the schema check as not run.

## What the organization can lock

Guards need no lock. Every rule comes from the manifest, nothing local can add,
remove or disable one, and `guards` is not a lock name. The organization decides
what the rules are by what it merges and releases. When any rule targets
`agent-output` or `tool-output`, the manifest package sets `holdUncheckedText` to
`true` in its base layer and locks it, so unchecked text does not stream. See
[policy](policy.md#what-the-organization-can-lock) for that setting.

## Check a change

Follow [development](development.md#local-checks) and run
`mise exec -- just check`. Then check each changed rule by hand:

1. Confirm the file has only `apiVersion` and `rules`, and that every rule has
   all six members.
2. Confirm every `id` is unique across the four files.
3. Confirm the guard tests ran: `cargo test --locked` reports them under
   `guards::`. An engine other than the `regex` crate can accept patterns that
   Maestro rejects, so do not substitute one.
4. Confirm every action allows every target in `appliesTo`.
5. Confirm any new quote on this page matches its source. `just check` already
   compares the two quotes here.

When the manifest package reads guard files, add the release load check and the
schema check, and report each result with its exit code. Until then, name them as
not run.
