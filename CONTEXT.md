# Glossary

These terms follow Maestro's manifest-mode v0.1 contract. Content checks in this
repository validate files; they do not execute policy or approve a publisher.

- **Authored artifact:** A settings document, prompt, instruction, role, skill,
  prompt template, workflow, extension, theme or model file. Each kind has one
  native format, whether supplied locally or by a manifest where supported.
- **Manifest:** An organization's optional configuration layer, selected by a
  pinned Git source in the `manifest` setting.
- **Internal mode:** Maestro with no manifest selected.
- **Manifest area:** The common release root, a standard domain, core, a language
  or a capability. Areas load in the order documented in [settings](docs/settings.md).
- **Capability group:** The navigation folder under `capabilities/` that groups
  capabilities. It is not a settings area and Maestro reads no GitHub properties.
- **Capability:** One package at `capabilities/<group>/<name>/`, selected by that
  full path in `manifestAreas`.
- **Package:** A versioned resource container with a native `package.json` and
  `maestro` resource declarations, or conventional resource directories. Code
  integrations ship already-built extension components.
- **Role:** A Markdown agent definition with frontmatter and a prompt body.
  The main `maestro` role supplies persona and a tool ceiling, not model defaults.
- **Instruction:** An offered `*.instructions.md` file copied into a local
  instructions folder by choice. Matching and loading belong to the rules system.
- **Lock:** A policy entry fixing a governed value or forbidding local resource
  additions. A `manifest` lock fixes identity; its update route may change the pin.
- **Bound:** A policy minimum, maximum or allowed model set that limits choices.
- **Required resource:** A release-relative resource that local discovery,
  shadowing or filters cannot remove. Required workflows resolve references from
  their own release.
- **Manifest release:** A versioned Git snapshot, normally published under a
  `v`-prefixed tag. Capabilities in this repository share its release version.
- **Pin:** The explicit ref selecting a versioned container. Individual files
  inside a container have no independent versions.
- **Minimum release:** The lowest allowed release version, declared by common or
  core policy. A newer release can require adoption at the next session start.
- **Permission growth:** A change that adds role tools, widens a bound or model
  set, removes a lock or required entry, or adds an extension, package or MCP
  server. It needs confirmation even during rollback or automatic updates.
- **Last accepted composition:** Manifest-package state retained for failed
  reloads, drift recovery and repair permission comparisons; not a second checkout.
- **Blueprint:** Catalog data defining an entry kind's properties and relations.
  Entries derive their owning principal from the last matching CODEOWNERS pattern.
