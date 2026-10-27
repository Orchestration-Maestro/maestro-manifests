# Extensions

Extensions add tools, commands and event handlers as built WebAssembly components.
A capability can package a component, but this release ships none: adding executable
code merely to illustrate a guide would change the release's trust requirements.
The example below is Maestro's real greeting extension, linked at a fixed revision.

## Contents

1. [Example](#example)
2. [Locations](#locations)
3. [Format](#format)
4. [Usage](#usage)
5. [Loading rules](#loading-rules)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Validation](#validation)
8. [Check a change](#check-a-change)

## Example

This complete factory registers a `hello` tool. It requires a string `name` and
returns both greeting text and the supplied name in structured details.

From [`examples/extensions/hello.rs`](https://github.com/Orchestration-Maestro/maestro/blob/e2df52128d6ad8a503d4a2002fc20e5b0c32c8b8/examples/extensions/hello.rs):

```rust
//! A greeting tool registered by an extension factory.
#![forbid(
    clippy::pedantic,
    clippy::too_many_arguments,
    clippy::excessive_nesting
)]

use std::rc::Rc;

use maestro_extensions_wasm::{
    AgentToolResult, ExtensionAPI, ExtensionFuture, Presence, TextContent, ToolDefinition,
    ToolMetadata, UserBlock,
};
use serde::Deserialize;
use serde_json::json;

/// The name supplied to the greeting tool.
#[derive(Deserialize)]
struct HelloParams {
    /// Name to greet without trimming or rewriting it.
    name: String,
}

/// A tool that returns a greeting and the supplied name.
fn hello_tool() -> ToolDefinition {
    ToolDefinition {
        render_call: None,
        render_result: None,
        metadata: ToolMetadata {
            name: "hello".into(),
            label: "Hello".into(),
            description: "A simple greeting tool".into(),
            parameters: json!({"type":"object","required":["name"],"properties":{
                "name":{"type":"string","description":"Name to greet"}
            }})
            .into(),
            prompt_snippet: Presence::Missing,
            prompt_guidelines: Presence::Missing,
            render_shell: Presence::Missing,
            execution_mode: Presence::Missing,
        },
        prepare_arguments: None,
        execute: Rc::new(|_, params, _, _, _| {
            Box::pin(async move {
                let params: HelloParams =
                    serde_json::from_value(params).map_err(|error| error.to_string())?;
                Ok(AgentToolResult {
                    content: vec![UserBlock::Text(TextContent::new(format!(
                        "Hello, {}!",
                        params.name
                    )))],
                    details: Presence::Present(json!({"greeted":params.name}).to_string().into()),
                    terminate: Presence::Missing,
                })
            })
        }),
    }
}

/// Registers the greeting tool through the supplied extension host.
#[must_use]
pub fn factory(api: ExtensionAPI) -> ExtensionFuture<'static, ()> {
    Box::pin(async move { api.register_tool(hello_tool()) })
}

#[cfg(target_arch = "wasm32")]
maestro_extensions_wasm::export_extension!(factory);
```

This is an external source example, not a Rust file or compiled component in the
manifest. See [the documented exception](index.md#example-coverage-and-exceptions).
For a complete separate-project build, use Maestro's
[extension guide](https://github.com/Orchestration-Maestro/maestro/blob/main/docs/extensions.md)
and its [example inventory](https://github.com/Orchestration-Maestro/maestro/blob/main/examples/extensions/README.md).
There is no source-language runtime supplied by a manifest.

## Locations

Selected areas may provide `extensions/` or explicit component entries under
`maestro.extensions` in `package.json`. Local extensions live under
`~/.maestro/agent/extensions/` or `.maestro/extensions/`; an explicit `-e` path
loads a built component directly. Helper data stays beside its package, not in
another settings scope.

## Format

Author against the matching Maestro SDK and canonical extension world using
`maestro-extensions-wasm`. Rust's supported build target is `wasm32-wasip2`; a
generic WASI command is not automatically a Maestro extension. JavaScript,
TypeScript and Python are not verified authoring routes.

A capability that ships an extension must commit the built component and all
runtime data, then list its relative `.wasm` entry in `maestro.extensions`.
Each entry creates an instance, so do not list helper files as entry components.
Rust dependencies are already linked. Consumers must not need Cargo or an
install-time compilation step. Use the package owner for acquisition rather than
adding a second installer.

## Usage

Build `hello` in a Maestro checkout matching your installed host, following its
extension guide, and load the resulting artifact with
`maestro --no-extensions -e` followed by its path in internal mode. Ask Maestro
to call `hello` with a name and inspect the greeting. A model-driven call needs
a configured model; registration alone does not exercise the tool.

For distribution, copy the tested component into its capability, include its
native package entry and tag the tested revision. Review the packaged bytes,
source revision and file list. This repository deliberately does not perform
that packaging for the greeting tool.

## Loading rules

Optional manifest resources rank below explicit, project and user resources.
Discovery and reload use the ordinary extension loader. An optional load failure
warns and skips the resource; failure to load a required component refuses the
session. `--no-extensions` disables automatic discovery subject to required rules.

Extensions run with the user's access, including available filesystem, process,
network and environment access. A component is not an untrusted-code sandbox.
Review both its source and the artifact supplied by its publisher.

## What the organization can lock

An `extensions` lock excludes automatic local additions before execution and
refuses explicit `-e` requests. Required components cannot be removed by filters,
discovery flags or SDK overrides. Optional additions remain possible without
those constraints.

Extension requests to change models, thinking, active tools, providers or the
prompt pass ordinary policy first. Provider locks refuse provider registration;
prompt locks block per-turn prompt replacement. Tool registration does not
activate a tool above the main role's ceiling. None of these controls sandbox
the extension's own operating-system access.

## Validation

Check a native package entry's path and the exact component against the host.
JSON or schema validation cannot prove a component is safe or implements the
right interface. The matching SDK's controlled host tests exercise registration
and callbacks without a provider; a model-driven check is a separate test.

## Check a change

Run the [available content checks](schemas-and-checks.md#available-checks). If a
future capability adds a component, require the source/build provenance and the
matching host's loading test, then run the packaged artifact. In a disposable
release, verify a bad required component refuses with the resource and fix.
This release has no component to build or load; report that scope explicitly.
