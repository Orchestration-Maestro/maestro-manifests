# Extensions

Extensions add tools, commands and event handlers. A manifest carries an
**already-built WebAssembly component** and a native package manifest. It does
not compile Rust or run JavaScript/TypeScript source as an extension. Authoring
belongs in an extension project; this repository is a content catalog.

Extensions run with the user's access. Governing registered tools, providers or
prompt changes does not sandbox filesystem, process or network access. Review
both the component's source and the artifact you distribute.

## Complete packaging example

Use a Maestro checkout matching the host revision. Its greeting example provides
a complete source implementation, so the following builds a real artifact rather
than an empty component. From that checkout:

```sh
rustup target add wasm32-wasip2
~/.local/bin/capped mise exec -- cargo build --locked \
  -p maestro-extensions-wasm --example hello --target wasm32-wasip2 --release
mkdir -p ../hello-package/dist
cp target/wasm32-wasip2/release/examples/hello.wasm ../hello-package/dist/index.wasm
```

Create `../hello-package/package.json`:

```json
{
  "name": "maestro-hello-example",
  "version": "0.1.0",
  "private": true,
  "files": ["dist/index.wasm"],
  "maestro": {
    "extensions": ["./dist/index.wasm"]
  }
}
```

The complete package now contains that file and the built `dist/index.wasm`.
From `hello-package`, try the component explicitly in internal mode:

```sh
maestro --no-extensions -e ./dist/index.wasm
```

Ask Maestro to call `hello` with a name. The example returns a greeting and
structured result. No provider call is needed to validate loading/registration,
but a normal model-driven invocation needs a configured model.

For a manifest area, place this package content at the area's root, or put the
built component under `extensions/` and declare its path in the area's
`package.json`. If integrating into a capability, merge the `maestro` entry into
that capability's existing package file rather than creating nested area
descriptors. A separately released package can be listed in manifest settings
`packages` with a pin; then it is required and installs through ordinary package
acquisition.

## Authoring and distribution

The supported source route is Rust with `maestro-extensions-wasm`, built for
`wasm32-wasip2`. The SDK implements Maestro's extension world and native async
component interface; a generic WASI command is not automatically an extension.
Use Maestro's [extension guide](https://github.com/Orchestration-Maestro/maestro/blob/main/docs/extensions.md)
for the complete separate-project manifest, factory, callbacks and build recipe.
JavaScript, TypeScript and Python are not verified authoring routes merely
because a loader recognizes source-looking paths.

Distribute the component with every runtime data file it needs. `maestro.extensions`
contains entry components only, not helper files: each entry creates an instance.
Rust dependencies are linked into the artifact. Consumers must not need Cargo or
an install-time build script. For Git distribution, commit the package file and
built component and tag the tested revision. For npm, inspect the package's file
list before publishing. Never add lifecycle scripts that compile on a consumer's
machine.

## Locks and overrides

Optional manifest extensions rank below explicit, project and user resources.
An `extensions` lock excludes automatic local additions before execution and
refuses explicit `-e` requests. A required extension cannot be dropped by filters,
`--no-extensions` or SDK resource overrides; loading failure refuses the session.
Without those constraints, developers can try local components and add optional
extensions.

Extension calls that change a model, thinking level, active tools or providers
pass the same policy as application requests. A provider lock refuses provider
registration; a prompt lock prevents per-turn prompt replacement. Registration
alone does not activate a tool outside the main role's tool ceiling.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). JSON parsing
alone cannot inspect a component's behavior. Build against the matching SDK,
load the exact packaged bytes, verify registration and run the greeting. Inspect
package paths and file inclusion, then load the pinned manifest revision. For a
required extension, test a missing/bad component in a disposable release and
confirm refusal names the resource and fix rather than silently skipping it.
No extension component is shipped by this repository revision.
