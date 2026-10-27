# Themes

Themes are JSON color definitions for the terminal interface. A manifest may
supply theme files and a default `theme` setting, but choosing a theme remains the
user's choice. No theme is required for the core workflows to run.

## Complete example

Create `themes/review-dark.json` in an area, or use the same file at
`~/.maestro/agent/themes/review-dark.json` or
`.maestro/themes/review-dark.json`:

```json
{
  "name": "review-dark",
  "vars": {
    "primary": "#00aaff",
    "secondary": 242
  },
  "colors": {
    "accent": "primary",
    "border": "primary",
    "borderAccent": "#00ffff",
    "borderMuted": "secondary",
    "success": "#00ff00",
    "error": "#ff0000",
    "warning": "#ffff00",
    "muted": "secondary",
    "dim": 240,
    "text": "",
    "thinkingText": "secondary",
    "selectedBg": "#2d2d30",
    "userMessageBg": "#2d2d30",
    "userMessageText": "",
    "customMessageBg": "#2d2d30",
    "customMessageText": "",
    "customMessageLabel": "primary",
    "toolPendingBg": "#1e1e2e",
    "toolSuccessBg": "#1e2e1e",
    "toolErrorBg": "#2e1e1e",
    "toolTitle": "primary",
    "toolOutput": "",
    "mdHeading": "#ffaa00",
    "mdLink": "primary",
    "mdLinkUrl": "secondary",
    "mdCode": "#00ffff",
    "mdCodeBlock": "",
    "mdCodeBlockBorder": "secondary",
    "mdQuote": "secondary",
    "mdQuoteBorder": "secondary",
    "mdHr": "secondary",
    "mdListBullet": "#00ffff",
    "toolDiffAdded": "#00ff00",
    "toolDiffRemoved": "#ff0000",
    "toolDiffContext": "secondary",
    "syntaxComment": "secondary",
    "syntaxKeyword": "primary",
    "syntaxFunction": "#00aaff",
    "syntaxVariable": "#ffaa00",
    "syntaxString": "#00ff00",
    "syntaxNumber": "#ff00ff",
    "syntaxType": "#00aaff",
    "syntaxOperator": "primary",
    "syntaxPunctuation": "secondary",
    "thinkingOff": "secondary",
    "thinkingMinimal": "primary",
    "thinkingLow": "#00aaff",
    "thinkingMedium": "#00ffff",
    "thinkingHigh": "#ff00ff",
    "thinkingXhigh": "#ff0000",
    "bashMode": "#ffaa00"
  }
}
```

This defines all 51 required color tokens. `vars` is optional and lets colors
reference a shared value. A color is a `#RRGGBB` string, an ANSI 256-color integer
from 0 to 255, an empty string for the terminal default, or a variable name.
Keep every required token even when its value is the terminal default. A theme
name identifies the definition; use a unique name to avoid accidental collisions.

The tokens cover ordinary UI text and borders, selected/message/tool backgrounds,
Markdown, diffs, syntax highlighting, thinking-level borders and bash mode.
Foreground and background contrast both matter: test success/error output and
selected rows rather than only the accent color. A theme file changes colors,
not policy, keybindings or model behavior.

## Selection and packaging

Select the loaded theme in `/settings` or set a local preference:

```json
{
  "theme": "review-dark"
}
```

A package may list the example in native `package.json`:

```json
{
  "name": "review-colors",
  "version": "0.1.0",
  "maestro": {
    "themes": ["./themes/review-dark.json"]
  }
}
```

Manifest areas can also expose the conventional `themes/` directory. Do not set
resource path arrays in the manifest area's settings. Local `themes` arrays and
explicit `--theme` paths retain their ordinary discovery behavior; `--no-themes`
disables automatic discovery, subject to required-resource rules.

## Overrides and locks

The manifest can offer themes and default `theme`; it cannot lock or bound either
`theme` or `themes`. User and project resources can replace optional same-name
manifest definitions. A required resource cannot be removed by discovery flags
or filters, but offering a theme does not force its selection. Machine-specific
terminal presentation stays local.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). Load the full
JSON with Maestro's theme loader, select `review-dark` and inspect ordinary text,
Markdown, tool results, diffs and all thinking levels. The active custom theme
supports hot reload, so edit a disposable copy to inspect contrast changes.
Check both a truecolor terminal and the color mode your users actually use.
A JSON parse success alone cannot detect a missing token or unreadable contrast.
