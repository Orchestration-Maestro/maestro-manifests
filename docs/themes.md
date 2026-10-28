# Themes

Themes are JSON color definitions for the terminal interface. A manifest may
supply theme files and a default `theme` setting, but choosing a theme remains the
user's choice. No theme is required for the core workflows to run.

## Contents

1. [Locations](#locations)
2. [Example](#example)
3. [Format](#format)
4. [Usage](#usage)
5. [Loading rules](#loading-rules)
6. [What the organization can lock](#what-the-organization-can-lock)
7. [Validation](#validation)
8. [Check a change](#check-a-change)

## Locations

Manifest theme files live under an area's `themes/` or explicit package paths.
Local folders are `~/.maestro/agent/themes/` and `.maestro/themes/`. Local
`themes` settings and repeatable `--theme` paths can also expose files.

## Example

The evidence-review capability includes a complete dark theme, not a default
selection:

From [`manifests/capabilities/practice/evidence-review/themes/review-dark.json`](../manifests/capabilities/practice/evidence-review/themes/review-dark.json):

```json
{
  "name": "review-dark",
  "vars": {
    "accent": "#D9A066",
    "border": "#5E7383",
    "success": "#FF6A1A",
    "error": "#FF6A1A",
    "warning": "#D9A066",
    "muted": "#D9A066",
    "panel": "#2A1F1A",
    "text": "#F2E8DC",
    "live": "#FF6A1A"
  },
  "colors": {
    "accent": "accent",
    "border": "border",
    "borderAccent": "accent",
    "borderMuted": "border",
    "success": "success",
    "error": "error",
    "warning": "warning",
    "muted": "muted",
    "dim": "muted",
    "text": "",
    "thinkingText": "muted",
    "selectedBg": "panel",
    "userMessageBg": "panel",
    "userMessageText": "",
    "customMessageBg": "panel",
    "customMessageText": "",
    "customMessageLabel": "accent",
    "toolPendingBg": "panel",
    "toolSuccessBg": "panel",
    "toolErrorBg": "panel",
    "toolTitle": "",
    "toolOutput": "muted",
    "mdHeading": "accent",
    "mdLink": "accent",
    "mdLinkUrl": "muted",
    "mdCode": "accent",
    "mdCodeBlock": "text",
    "mdCodeBlockBorder": "border",
    "mdQuote": "muted",
    "mdQuoteBorder": "border",
    "mdHr": "border",
    "mdListBullet": "accent",
    "toolDiffAdded": "success",
    "toolDiffRemoved": "error",
    "toolDiffContext": "muted",
    "syntaxComment": "muted",
    "syntaxKeyword": "accent",
    "syntaxFunction": "accent",
    "syntaxVariable": "text",
    "syntaxString": "success",
    "syntaxNumber": "warning",
    "syntaxType": "live",
    "syntaxOperator": "text",
    "syntaxPunctuation": "text",
    "thinkingOff": "border",
    "thinkingMinimal": "muted",
    "thinkingLow": "accent",
    "thinkingMedium": "accent",
    "thinkingHigh": "live",
    "thinkingXhigh": "live",
    "bashMode": "success"
  }
}
```

All 51 required tokens are present. Colors use Maestro's Forge dark roles:
Brass for accents and muted text, Ember for activity and status, Bone for code,
Steel for borders and Oxide for panels. Success and error share Ember; their
labels and diff markers distinguish them. The four empty values retain the
terminal's default text color.

This is the full file used by the capability, so a color change needs a matching
documentation update. The palette and role mapping come from Maestro's
[brand pack](https://github.com/Orchestration-Maestro/maestro/blob/main/assets/brand/brand.json)
and [identity guide](https://github.com/Orchestration-Maestro/maestro/blob/main/docs/identity.md#terminal-roles).

## Format

`name` and `colors` are required. Optional `vars` defines reusable colors;
optional `$schema` enables editor schema support, and `export` supplies export
colors in the native theme format. Each color is `#RRGGBB`, an ANSI 256-color
integer from 0 to 255, an empty string for the terminal default or a variable
name. Resolve aliases without missing names or cycles.

The 51 tokens cover UI text/borders, message and tool backgrounds, Markdown,
diffs, syntax highlighting, thinking-level borders and bash mode. Six tokens
are backgrounds and the other 45 are foregrounds. Test readable contrast for
success, failure and selected rows, not just the accent color.

## Usage

Select `capabilities/practice/evidence-review` in `manifestAreas`, then choose
`review-dark` in `/settings`. The capability's native package manifest explicitly
lists this file. No `theme` preference is shipped, and selecting the capability
does not switch your active theme.

For local use without a manifest, copy this exact file to a local theme folder
and select its name. Do not edit an installed release checkout for customization.

## Loading rules

Manifest areas can expose conventional folders or native package entries.
Manifest settings cannot set a `themes` resource-path array; local settings can.
`--no-themes` disables automatic discovery within required-resource rules.
Optional same-name local definitions can replace a manifest theme. The active
custom theme supports hot reload for local color edits.

## What the organization can lock

The manifest can offer themes and default `theme`; it cannot lock or bound either
`theme` or `themes`. User and project resources can replace optional same-name
manifest definitions. A required resource cannot be removed by discovery flags
or filters, but offering a theme does not force its selection. Machine-specific
terminal presentation stays local.

## Validation

The native theme loader checks required tokens and resolves variables. Editor
and runtime schemas have different tolerance for unknown fields; neither checks
readability on your terminal palette. A JSON parse alone cannot detect missing
tokens, bad aliases or unusable contrast.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). Load the full
JSON with Maestro's theme loader, select `review-dark` and inspect ordinary text,
Markdown, tool results, diffs and all thinking levels. The active custom theme
supports hot reload, so edit a disposable copy to inspect contrast changes.
Check both a truecolor terminal and the color mode your users actually use.
A JSON parse success alone cannot detect a missing token or unreadable contrast.
