# Maestro manifests

Author reviewed capabilities for Maestro in one owner-first catalog.

**Status:** layout approved on 2026-09-30; this repository contains documentation
and the MIT licence only. The tree below is the target `maestro-source/2` layout,
not installed content. The seed waits for the checker migration. QA is an
example, delivery arrives with its workflow, collections await S6, and model
cards require owner-approved identities. No vendor or private content lives here.

## Layout

```text
maestro-manifests/
├── core/
│   ├── capability.toml
│   ├── agents/{maestro.agent.md,maestro.maestro.toml}
│   ├── skills/knowledge-evidence/SKILL.md
│   ├── instructions/{knowledge.instructions.md,knowledge.maestro.toml}
│   ├── mcp/maestro.toml
│   └── model-cards/<approved-card>.toml
├── capabilities/engineering/qa/             # example, not seed content
│   ├── capability.toml
│   ├── agents/{reviewer.agent.md,reviewer.maestro.toml}
│   ├── skills/test-planning/SKILL.md
│   ├── instructions/{quality.instructions.md,quality.maestro.toml}
│   ├── mcp/test-runner.toml
│   └── knowledge/collections/qa-public/collection.toml
├── capabilities/engineering/rust/
│   ├── capability.toml
│   └── instructions/{rust.instructions.md,rust.maestro.toml}
├── capabilities/engineering/delivery/       # optional feature-delivery workflow
│   ├── capability.toml
│   ├── workflows/feature-delivery/workflow.md
│   └── agents/                             # required delivery roles only
├── capabilities/orchestration/application-workflow/
│   ├── capability.toml
│   └── knowledge/collections/              # private mount awaits S6
├── presets/{knowledge-client.toml,rust-service.toml,qa.toml}
├── bootstrap/{core.toml,rust.toml}          # template inventories, not presets
├── bootstrap/core/.github/copilot-instructions.md
├── bootstrap/rust/.maestro/recipes.json
├── settings/README.md                      # S1 settings reference only
├── docs/standards/{engineering.md,security.md}
└── {CODEOWNERS,README.md,LICENSE}           # CODEOWNERS is generated
```

Braces mean separate files. Omit unused directories; do not create empty roles.
Every owner root supports `agents/`, `skills/`, `instructions/`, `mcp/` and
`knowledge/`. A folder does not register a resource kind: unsupported nonempty
subtrees refuse until their descriptor exists. Later workflows, contracts,
policies, profiles, hooks and evaluations follow their owner too.

### What each root holds

| Root | Responsibility |
| --- | --- |
| `core/` | Only resources every install needs: Maestro, common knowledge resources, shared policies, the hook and Maestro's agent-session profiles. The canonical persona/system prompt is `core/agents/maestro.agent.md`, with its sidecar; it is not hard-coded in the runtime |
| `capabilities/<domain>/<capability>/` | One self-contained optional capability, with its owner record and resources together. Rust instructions stay with Rust; delivery roles stay with delivery; the generic application workflow, answer contract and evaluation stay with application-workflow |
| `presets/` | Global named selections of exact capability IDs, plus optional template-inventory names. A knowledge-client selection must not carry optional delivery roles |
| `bootstrap/` | Explicit inert template inventories and their files. `core.toml`/`rust.toml` are inventories, not a second set of presets; distinct inventories cannot overwrite one output |
| `docs/`, `settings/` and governance files | Public standards and generated ownership. `settings/README.md` explains the shared S1 registry; it is not another settings schema or authority |

The private overlay is a separate source outside this repository, not another
public root or a folder copied into a release.

## Identity and selection

Resource IDs are typed and owner-qualified:

- `agent:core/maestro`
- `skill:qa/test-planning`
- `instructions:rust/rust`
- `mcp:qa/test-runner`

Owner closure roots use `capability:core` or `capability:qa`; global presets use
`preset:knowledge-client`. Reserve `core`. Each capability leaf is globally
unique across domains: two domains cannot both own `qa`. A domain move preserves
IDs, but changes source paths and therefore needs a fresh authoring preview.
Each segment uses lowercase hyphenated names, at most 64 characters.

Source agent/skill `name` stays local and matches its file stem/directory.
Same local names under different owners are valid; duplicate full IDs, source
paths, namespaces or descriptor placements are not, even for identical bytes.
There are no basename aliases.

Every selection includes reviewed `capability:core` once, with reviewed
`agent:core/maestro`. Presets require capability roots, never directory globs.
Selection does not execute workflows, launch MCP servers, crawl collections or
grant runtime permission. Maturity and ownership are checked declarations;
protected review and runtime authorization remain separate controls.

## Segregation rules

These are checker/CI requirements, not optional filing conventions.

| Rule | Required check |
| --- | --- |
| Separate roots | Keep core, each capability, shared roots and private inputs separate. Refuse misplaced resources, nested owners, unsupported nonempty subtrees, links and path escapes |
| Exactly one owner | Each owner root has one `capability.toml` with one approved GitHub user/team, matching namespace, schema, maturity, rows, workflow labels and exact entry requirements. Resource owner fields must match that record |
| Declared dependencies only | Every local or cross-owner resource edge uses a typed qualified ID in `requires`. No cross-root file paths or includes. Capabilities may require core or each other explicitly; core never requires a capability |
| Removable as one folder | Deleting a capability must expose all surviving dangling references as errors. After its dependants are explicitly changed, unrelated selections still pass; never silently substitute another owner's local name |
| Additive private overlay | Refuse public/core replacement or shadowing, duplicate paths/IDs, owner/trust overrides and public presets whose transitive closure requires a private ID. Missing private input must not disable public presets |
| Generated ownership | Generate anchored CODEOWNERS per owner root, deriving shared-root/governance rules from core's same owner record. CI regenerates and refuses any drift, including stale removed-capability rules |

Workflow labels describe usage, never dependencies, selection or permission.
Core resources and its owner manifest may omit `workflows` or use an empty list;
any label they do carry must name a core workflow, never a capability workflow.
The minimal seed has no core workflow. Capabilities declare their forward
`requires` instead of adding reverse labels to core, so removing a capability
leaves core valid and unchanged. Capability resources keep namespaced usage
labels; graph checks derive required resources from declared closures.

Preset `templates` is the sole shared-root inventory selector. It accepts only
inventory names such as `core` and `rust`, never paths/includes. Each inventory
reads explicit files only inside its declared `bootstrap/` directory. This is
not a resource dependency or an exception for owner-root references, and it
adds no template resource kind.

## Add a capability: QA

Use this sequence **after the `/2` checker migration**. Add only resources with
a real workflow need; this README does not authorize a QA seed or new kinds.

1. Create `capabilities/engineering/qa/capability.toml`. Declare namespace `qa`,
   one approved owner, `maestro-source/2`, honest maturity, architecture rows,
   namespaced workflow labels and exact qualified entry `requires`.
2. Add the needed local resources: `agents/reviewer.agent.md` and its
   `reviewer.maestro.toml`, `skills/test-planning/SKILL.md`, and
   `instructions/quality.instructions.md` with `quality.maestro.toml`.
   Keep the six agent sections: Purpose, Responsibilities, Inputs, Working
   sequence, Outputs, Boundaries. Skill metadata stays inside `SKILL.md`.
3. Declare resource dependencies such as `skill:qa/test-planning` and
   `instructions:qa/quality` in `requires`. Add `presets/qa.toml` requiring
   `capability:qa`; use `templates = ["core"]` only if that preset needs the
   core inventory. Do not point to another root's files.
4. Generate CODEOWNERS from the approved owner records. Run the migrated
   checker and drift check below, then obtain protected owner review. A label
   alone is not review evidence; missing approved owner identity blocks content.
5. Test deletion of the QA folder in a disposable fixture. A remaining
   `capability:qa` reference must refuse; core and other independent selections
   must still pass when the QA dependants are explicitly removed.

With `MANIFESTS` set to the reviewed checkout, the planned commands are:

```sh
maestro catalog codeowners --catalog-dir "$MANIFESTS" > "$MANIFESTS/CODEOWNERS"
maestro catalog check --catalog-dir "$MANIFESTS"
maestro catalog codeowners --catalog-dir "$MANIFESTS" --check
```

Generation prints rules; `--check` refuses drift without writing. These commands
do not configure GitHub protection or approve their own owner identities.

## Register an MCP server

One server is **one file plus one qualified dependency reference**, not a Rust
change or a new plugin.

1. Add `capabilities/engineering/qa/mcp/test-runner.toml` with the supported MCP
   fields, common metadata, the QA owner mirror and explicit allowed tools.
   Use named credential bindings, never credentials or machine-specific paths.
2. Add `mcp:qa/test-runner` to the consuming agent or capability's `requires`.
   An agent using it names `qa/test-runner` in `mcp-servers` and
   `qa/test-runner/run_tests` in its tools; the checker verifies the approved
   server/tool and its declared requirement. It splits the tool at the last slash.
3. Check the catalog. Projection maps names, paths and references together to
   host-safe aliases and refuses length/normalization collisions and user
   shadows. Native agent alias `maestro` is reserved for core. Registration
   itself never launches the server or authorizes a tool call.

## Private collections plug in from outside

S6 supplies an explicit opt-in additive source adapter, not a second public
catalog or a last-wins overlay. A private source can provide:

```text
ctm-collection/catalog/
├── presets/ctm-private.toml
└── capabilities/orchestration/application-workflow/
    └── knowledge/collections/ctm/collection.toml
```

The private preset requires `capability:application-workflow` and
`collection:application-workflow/ctm`. The public capability owns the subtree;
the private source cannot replace its `capability.toml` or authorize its own
publisher. Pin both source identities, revisions and digests; check public alone
first, then combined inputs under aggregate limits and source-aware diagnostics
and locks. A missing or unauthorized overlay disables only private selection.
Public CI and releases never fetch, package or index private inputs.

The private descriptor carries URL approval rules, versions/types, crawl bounds
and credential/storage binding references, not corpus/evidence bytes. Empty URL
approval means no fetch. S6 must check seeds, discovered links and every redirect;
exclusions win. Keep admitted originals and URL/version/digest/transformation
provenance privately. This layout authorizes no crawler, private access,
relevance-based passage deletion or deletion of existing evidence. Checking is
offline, installation does not crawl, and runtime collection ACLs remain separate.

## Migration boundary

The target contracts are `maestro-source/2`, `maestro-cli/catalog-check/2`,
`maestro-project/2` and `maestro-authoring-lock/2`, with incremented changed
source-descriptor versions. Reject old or mixed layouts with a migration
diagnostic. Old source-bound locks need a fresh preview, not silent rebinding.
Kernel model-card identities and the S1 preference schema do not change.

## Licence

[MIT](LICENSE).
