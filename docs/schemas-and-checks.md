# Schemas and checks

Content checks validate syntax and file hygiene; generated schemas and loading
Maestro validate artifact structure and runtime composition. This page separates
the checks available in this repository from the schema/runtime integration
required by the v0.1 contract, so a passing parser is not mistaken for policy
enforcement.

## Contents

1. [Available checks](#available-checks)
2. [Generated schemas](#generated-schemas)
3. [Example](#example)
4. [Usage](#usage)
5. [Validation](#validation)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Check a change](#check-a-change)

## Available checks

From a clean clone, trust the mise configuration and run `mise exec -- just setup`,
then `mise exec -- just check`. The commands are defined in the real
[justfile](../justfile), not a separate CI implementation.

`just setup` installs pinned tools and prek hooks. `.pre-commit-config.yaml`
owns the content checks. The `rust` recipe in the `justfile` owns the Rust
checks. The shared required CI invokes the same `just check`, which runs the
hooks through `just content`, then `just rust` and `just test`. Do not create a
second copy of those checks in a workflow. `just test` runs the
contributor-command tests in disposable indexes and covers malformed YAML, JSON,
TOML and staged-versus-untracked behavior. It also checks the authored role
contracts, chain handoffs, Forge theme tokens, opt-in workflow and their quoted
documentation examples. It also checks the layout: every manifest artifact sits
under `manifests/`, CODEOWNERS and the release changelog name paths under it, and
every documentation link resolves. Those are content assertions, not runtime
execution.
`just rust` fails on an unformatted file or a Clippy warning. `just test` runs only
`cargo test --locked` for the repository's one Rust test crate,
`maestro-manifests-tests`.

The content checks parse YAML/JSON/TOML and check spelling, Markdown, EditorConfig,
file hygiene and conflict markers. Staged commit hooks also scan secrets and
validate commit headers. A check that rewrites a file requires review and
restaging. See [development](development.md) for delivery and CI details.

## Generated schemas

The manifest-mode contract requires these schema subjects:

| Artifact | Schema input |
| --- | --- |
| Policy | `manifests/settings/policy.json` |
| Workflow | Parsed `maestro.workflow/v1` YAML converted to JSON |
| Role | Role frontmatter converted to JSON |
| Instruction | Instruction frontmatter converted to JSON |
| Capability package | Its native `package.json` |

The manifest consumer's Rust types are the source of truth. Maestro generates
schemas with its pinned `schemars` and validates with its pinned `jsonschema`.
Published generated files belong under `schemas/`, with valid and invalid
fixtures for each member, wrong JSON kind and unknown member. Required checks
must fail if generated schemas are stale, and a pre-commit hook must validate
the artifacts under `manifests/` using the same definitions.

Do not handwrite replacement schemas, invent generation commands or create empty
schema folders while waiting for that consumer. This revision has no generated
schemas or schema-validation hook. The existing check configuration parses
structured content but is not a schema validator. A release must add the real
consumer integration before claiming schema enforcement.

## Example

This actual workflow is an input to content parsing, future schema validation
and the manifest loader's required-reference checks:

From [`manifests/core/workflows/implement-and-review.yaml`](../manifests/core/workflows/implement-and-review.yaml):

```yaml
apiVersion: maestro.workflow/v1
kind: Workflow
name: implement-and-review
description: Implement a task, check the change and apply the review
chain:
  - agent: worker
    task: "Complete this implementation task: $@"
  - agent: reviewer
    task: "Check this implementation for defects: {previous}"
  - agent: worker
    task: "Update the implementation to address these review findings: {previous}"
```

A YAML parser alone cannot decide whether worker and reviewer resolve from the
correct source. Generated schemas validate the allowed members and field kinds;
the manifest loader validates reference closure.

## Usage

Load the exact proposed Git commit in an isolated settings scope using a
manifest-capable Maestro build. Set the `manifest` source/pin as described in
[settings](settings.md#usage), then run `maestro -p "/manifest status"` from that
project. The revision must be fetchable before it can be loaded.

There is no `--check-manifest` flag or separate validator executable. A broken
release fails extension load with its selecting file, source, problem and fix,
exits nonzero and makes no model request. Noninteractive startup still requires
one model credential; CI supplies it as a secret, never in a release file.

## Validation

| Check | Proves | Does not prove |
| --- | --- | --- |
| Content parsers and linters | Basic syntax, style and file hygiene | Artifact semantics or permissions |
| Generated schema validation | Field types, allowed members and structure | Model availability or required-reference closure |
| Manifest load at pinned commit | Layout, policy combination and references | That an extension is safe or an endpoint answers |
| Focused runtime exercise | The tested workflow, override or repair path | Every selected capability combination |
| Protected review | An approved change by its owners | A sandbox for executable content |

## What the organization can lock

Schema rules are format rules, not runtime locks. Valid user preferences still
pass policy; an invalid constraint cannot be overridden locally to make the
release acceptable. Do not weaken parsing to accept a forbidden override.
Generated schemas, once they exist, remain generated; change their source and
regenerate them rather than hand-editing them.

## Check a change

Run the available commands above. Keep the boundary explicit: report actual
content/test results separately from schema and runtime checks not yet available.
When the consumer lands, use its own commands and generated outputs, test a valid
and invalid artifact through each schema, then require the same check in hooks
and CI. Do not claim these future checks passed based on documentation examples.
