# Copilot instructions for Maestro manifests

## Start here

Author reviewed capabilities for Maestro in one owner-first catalog.

Paths below are relative to this repository. Before editing, read
[AGENTS.md](../AGENTS.md) for the rules that bind every change and
[CONTRIBUTING.md](https://github.com/Orchestration-Maestro/.github/blob/main/CONTRIBUTING.md)
for how a change is proposed. The organization's [golden
rules](https://github.com/Orchestration-Maestro/.github/blob/main/golden-rules/engineering.md)
come first: nothing in a specification, a plan or this repository weakens them.

For quality, engineering or security changes, read
[northstar.md](../docs/standards/northstar.md),
[engineering.md](../docs/standards/engineering.md) and
[security.md](../docs/standards/security.md): this repository's map of the
organization's golden rules.

Keep changes scoped to the request, and read historical plans and specifications
as records, not as instructions to start new work.

## Repository tree

Every tracked file, with what it is for. `rust-gate guide` writes this tree at
every commit and keeps each explanation already here, so improve an explanation
in place.

```text
.                                               # Repository root
├── .github/                                    # GitHub metadata, templates and workflows
│   ├── copilot-instructions.md                 # This guide, written by rust-gate guide at every commit
│   └── dependabot.yml                          # The organization merges only conventional titles: "ci(deps): bump ..."; rendered by rust-gate sync
├── docs/                                       # Documentation
│   └── standards/                              # Standards
│       ├── engineering.md                      # Engineering rules in maestro-manifests
│       ├── northstar.md                        # Northstar for maestro-manifests
│       └── security.md                         # Security rules in maestro-manifests
├── .editorconfig                               # Editor settings that survive the editor; rendered by rust-gate sync
├── .gitattributes                              # How Git should treat each kind of file; rendered by rust-gate sync
├── .pre-commit-config.yaml                     # The commit hooks prek runs locally and CI runs over every file; rendered by rust-gate sync
├── AGENTS.md                                   # Rules for coding agents: what to read, what never to weaken, how to verify
├── LICENSE                                     # The licence this repository is distributed under
├── README.md                                   # maestro-manifests
└── typos.toml                                  # The words this repository means, from [typos] words in maestro-quality.toml; rendered by rust-gate sync
```

## Change and verification procedure

1. Read the rules in AGENTS.md that cover the files you change, and keep every
   gate intact: never weaken one to pass.
2. Add an executable regression check for a change in behaviour.
3. The commit hook `rust-gate guide` rewrites this guide when a file is added,
   moved or removed; commit it with the change. The organization's daily drift
   check reports a guide left stale.
4. Run `prek run --all-files`, and report the commands you actually ran.
5. Commits are signed, with a conventional title; the default branch takes only
   squash-merged pull requests.
