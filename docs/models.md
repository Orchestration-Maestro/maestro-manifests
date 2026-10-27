# Models

Only core may supply `llm/models.json`. It uses the ordinary models format:
providers with endpoint/API configuration and model definitions. The manifest
model file loads before `~/.maestro/agent/models.json`; there is no project model
file. Model credentials stay local.

## Complete example

For a running Ollama service with `llama3.1:8b` already available, this is a
complete `core/llm/models.json`:

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
but the client expects a key. The manifest stores the variable name, never the
value. This is a local-service example, not a model endpoint shipped by this
repository. No connection to that service is needed for content checks.

To choose it as a default, pair it with core `settings/settings.json`:

```json
{
  "defaultProvider": "ollama",
  "defaultModel": "llama3.1:8b"
}
```

## Provider and model fields

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

## Credentials and constraints

Every manifest `apiKey` and credential header must be either an uppercase
letters/digits/underscores environment variable name or a command reference
beginning with `!`. Literal secret values invalidate the manifest. Credential
headers are Authorization and Proxy-Authorization, plus the case-insensitive
names in policy `credentialHeaders`. A noncredential header such as
`Content-Type: application/json` can remain literal. Command references execute
locally under ordinary permissions; review them as code.

A user provider's partial override keeps manifest fields it does not set. A user
model with the same provider and ID replaces the manifest model. Refresh and
dynamic provider removal must reveal the correct underlying layer. A `providers`
lock ignores the user model file with a diagnostic and refuses extension provider
registration.

Policy `models` constrains every selection path: defaults, flags, `/model`,
cycling, RPC, SDK, extension calls and restored sessions. `enabledModels` is
intersected with that allowed set. Stored disallowed choices resolve with a
diagnostic; explicit forbidden choices refuse before activation, history or
settings changes. Thinking bounds apply to the requested level, then the model
capability clamp applies; this example's non-reasoning model runs with thinking
off even under a policy minimum.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks), inspect every
credential-bearing field and load the pinned revision with credentials provided
locally. Open `/model` and confirm the provider/model identity. A real prompt
checks endpoint compatibility; listing alone does not. In a disposable setup,
try an allowed selection and an explicit disallowed one, then a stored default
outside the allowed set. Confirm refusal versus diagnostic resolution and that
credentials never appear in repository files or public logs.
