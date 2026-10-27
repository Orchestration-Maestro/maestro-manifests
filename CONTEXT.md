# Glossary

These terms follow Maestro's manifest configuration vocabulary. They describe
manifest contracts; this repository's tooling checks content, not runtime policy.

- **Artifact kind:** A registered type of manifest resource, such as a skill,
  workflow or instruction, with its own format and validation rules.
- **Manifest area:** One folder of a manifest (common, standard, core, language
  or team package) whose content enters the manifest layer in a fixed order.
- **Lock:** A policy entry fixing a value that local layers cannot change.
- **Manifest release:** A versioned snapshot of the organization's manifest,
  published under a `v`-prefixed tag. Capabilities in this repository share that
  release version.
- **Pin:** The explicit version of a resource's container selected for use.
  Individual resources are selected from that pinned container; files within one
  repository do not have independent versions.
