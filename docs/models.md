# Models

A models document defines providers, protocol adapters and model limits. Only
core may supply manifest `llm/models.json`; this release deliberately supplies
none, because it would add providers for every user, even without an optional
capability. Credentials remain local and the complete inline example below is
not shipped as an artifact.

## Contents

1. [Locations](#locations)
2. [Example](#example)
3. [Usage](#usage)
4. [Format](#format)
5. [What the organization can lock](#what-the-organization-can-lock)
6. [Loading rules](#loading-rules)
7. [Validation](#validation)
8. [Check a change](#check-a-change)

## Locations

The manifest path is `core/llm/models.json`, read before the user file at
`~/.maestro/agent/models.json`. There is no project models file. To try the
example without changing the release, put it in your own user models file.
See the explicit [artifact-index exception](index.md#example-coverage-and-exceptions).

## Example

For a running Ollama service with `llama3.1:8b` already available, the following
complete document uses an environment reference instead of a literal key.

Not shipped: complete user `models.json` example (approved inline exception):

```json
{
  "providers": {
    "ollama": {
      "baseUrl": "http://localhost:11434/v1",
      "api": "openai-completions",
      "apiKey": "OLLAMA_API_KEY",
      "compat": {
        "supportsDeveloperRole": false,
        "supportsReasoningEffort": false
      },
      "models": [
        {
          "id": "llama3.1:8b",
          "name": "Llama 3.1 8B (Local)",
          "reasoning": false,
          "input": ["text"],
          "contextWindow": 128000,
          "maxTokens": 32000,
          "cost": {
            "input": 0,
            "output": 0,
            "cacheRead": 0,
            "cacheWrite": 0
          }
        }
      ]
    }
  }
}
```

Set `OLLAMA_API_KEY` locally before starting Maestro; Ollama ignores the value,
but the client expects a key. A manifest stores the variable name, never the
value. This is a local-service example, not a model endpoint shipped by this
repository. No connection to that service is needed for content checks.

## Usage

This release allows only three Claude models and locks `defaultProvider` and
`defaultModel`, so choosing `ollama/llama3.1:8b` and making it a local default
both need a manifest without those limits.

Set `OLLAMA_API_KEY` locally and start Maestro with the service running. Open
`/model` and choose `ollama/llama3.1:8b`. To make it a local default, set
`defaultProvider` to `ollama` and `defaultModel` to `llama3.1:8b` in your user
settings.

## Format

`baseUrl` selects the endpoint; `api` selects the protocol adapter. The native
format supports `openai-completions`, `openai-responses`, `anthropic-messages` and
`google-generative-ai`. Set an API at provider level or override per model.
`headers` supplies provider request headers; `compat` describes endpoint-specific
protocol support. Do not assume every OpenAI-compatible service accepts the
`developer` role or reasoning effort, hence the explicit compatibility values in
the example.

A minimal model needs an `id`; optional values describe display name, reasoning,
input modalities, context window, maximum output and per-million-token costs.
Use actual service limits rather than copying the example for another model.
Consult Maestro's model-format documentation for adapter-specific overrides.

## What the organization can lock

Every manifest `apiKey` and credential header must be either an uppercase
letters/digits/underscores environment variable name or a command reference
beginning with `!`. Literal secret values invalidate the manifest. Credential
headers are Authorization and Proxy-Authorization, plus the case-insensitive
names in policy `credentialHeaders`. A noncredential header such as
`Content-Type: application/json` can remain literal. Command references execute
locally under ordinary permissions; review them as code.

A `providers` lock ignores the user model file with a diagnostic and refuses extension provider
registration.

Policy `models` constrains every selection path: defaults, flags, `/model`,
cycling, RPC, SDK, extension calls and restored sessions. `enabledModels` is
intersected with that allowed set. Stored disallowed choices resolve with a
diagnostic; explicit forbidden choices refuse before activation, history or
settings changes. Thinking bounds apply to the requested level, then the model
capability clamp applies; this example's non-reasoning model runs with thinking
off even under a policy minimum.

## Loading rules

The manifest file precedes the user's model file and built-in models remain
available. Partial provider overrides retain omitted manifest fields; a same-ID
user model replaces its manifest model. Refresh and provider removal must reveal
the correct underlying layer rather than erase it. Credentials and commands are
resolved locally, never from a secret embedded in the release.

## Validation

The ordinary models parser validates the document. Manifest validation also
rejects literal credential values. Neither parser proves that the endpoint is
reachable, its advertised model exists or its compatibility settings are right.
Test a real prompt separately; no network request is made by content checks.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), inspect every
credential-bearing field and load the pinned revision with credentials provided
locally. Open `/model` and confirm the provider/model identity. A real prompt
checks endpoint compatibility; listing alone does not. In a disposable setup,
try an allowed selection and an explicit disallowed one, then a stored default
outside the allowed set. Confirm refusal versus diagnostic resolution and that
credentials never appear in repository files or public logs.
