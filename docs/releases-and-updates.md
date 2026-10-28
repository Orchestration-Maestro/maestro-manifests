# Releases and updates

The versioned unit is a container: this manifest release, an ordinary package or
a separately published capability package. A skill, role or workflow inside a
Git release has no independent version. Select an item from a pinned container
with filters; do not invent a per-file pin.

## Contents

1. [Example](#example)
2. [Usage](#usage)
3. [Checks, floors and automatic updates](#checks-floors-and-automatic-updates)
4. [Permission growth and repair](#permission-growth-and-repair)
5. [What the organization can lock](#what-the-organization-can-lock)
6. [Publishing this repository](#publishing-this-repository)
7. [Validation](#validation)
8. [Check a change](#check-a-change)

## Example

The common policy sets `updateCheckMinutes` to 30 and imposes no minimum release
floor; its other members are in [policy](policy.md#example). The common settings
leave optional automatic updates off, and the policy locks that choice. An update
target must exist as a release of the selected Git source.

## Usage

Select the manifest as described in [settings](settings.md#usage), then run
`/manifest status`. It shows the active release, selecting file, newest known
release, minimum and intervening changelog entries. `/manifest update` chooses
the newest release; append an existing version to move forward or back within
the same identity, never below the newest known minimum.

The command shows current and target pins, changed manifest files, changelog and
permission growth before applying. Interactive users confirm; scripts can run
`maestro -p "/manifest status"` or `maestro -p` with the explicit update command.
A model credential is needed for noninteractive startup even though status
never calls the model. A headless update with growth refuses rather than treating
silence as approval.

Application writes the pin through the settings owner, installs through ordinary
package acquisition and reloads. For project settings, submit the pin change by
pull request so teammates take it at their next start.

## Checks, floors and automatic updates

At every session start, then every policy `updateCheckMinutes` minutes, the
manifest consumer checks the selected source for releases. The default is 30;
`0` means session start only. Checks obey `update.versionCheck` and
`MAESTRO_OFFLINE`. Offline keeps the pin with a warning. No live push or
per-prompt check exists. A running session sees a notice; required releases
apply at its next start, or through `/manifest update` now.

Only common and core can set `minimumRelease` or `updateCheckMinutes`. A floor
must not exceed the release you publish. Candidate inspection reads the optional
common and core policies and the candidate `manifests/CHANGELOG.md` in temporary scope. The higher
minimum wins. Inspection does not execute a lifecycle script, build step or
component.

If the newest release's minimum exceeds the current pin, the next start must
apply it. Without permission growth it does so without asking. With growth it
asks, and declining refuses the session with the fix; without a terminal it
refuses. Setting `manifestAutoUpdate: true` in the common settings also allows
optional newer releases to apply without asking, but never bypasses growth
confirmation. Otherwise an optional pinned release does not move automatically.

## Permission growth and repair

Growth means adding a role tool, widening models or a bound, removing a lock or
required entry, or adding an extension, package or MCP server. A rollback may
grow permissions and needs the same comparison. Rejecting or cancelling leaves
files, pin and settings byte-identical. A failed installation reports its actual
state; there is no rollback checkout hidden behind it.

A broken pinned release has one startup repair exception: the consumer may move
to a clean newer release of the same identity under the same growth rules. It
compares against the last accepted composition; with none, the candidate's whole
permission set counts as growth. If no candidate can apply, load fails with the
fix. Drift recovery is separate; see [integrity](integrity.md).

## What the organization can lock

A `manifest` lock fixes identity but still permits `/manifest update` to change
the same identity's ref. `maestro update` remains the ordinary engine/package
updater and skips pinned packages; it is not the manifest update route. When
unlocked, hand-editing a pin remains possible within the minimum-release rules.

## Publishing this repository

The existing Release Please workflow opens a release pull request from
conventional commits. It updates `version.txt`, `manifests/CHANGELOG.md` and its version
manifest. Merging that pull request creates a protected `v`-prefixed tag,
starting at `v0.1.0`. The common `manifests/package.json` is private and omits an independent
npm version; the Git tag is the release identity. In-tree capabilities share
that release. Keep separately versioned capabilities in separate containers.

The release configuration is a real contributor-side file, not runtime policy:

From [`.github/release-please/config.json`](../.github/release-please/config.json):

```json
{
  "release-type": "simple",
  "initial-version": "0.1.0",
  "include-component-in-tag": false,
  "include-v-in-tag": true,
  "bump-minor-pre-major": true,
  "changelog-sections": [
    { "type": "ci", "section": "Continuous Integration" },
    { "type": "feat", "section": "Features" },
    { "type": "fix", "section": "Bug Fixes" },
    { "type": "perf", "section": "Performance" }
  ],
  "packages": {
    ".": { "version-file": "version.txt", "changelog-path": "manifests/CHANGELOG.md" }
  }
}
```

See [development](development.md) for App credentials and merge-queue setup.
Never place those credentials in the catalog. Before tagging, load the exact
release candidate and validate every selected capability combination; current
content checks alone do not establish runtime compatibility.

## Validation

Semantic versions determine minimum floors after an optional leading `v`.
Candidate composition, not just tag order, determines permission growth. Content
checks cannot prove adoption/refusal behavior; exercise published fixture tags
with the manifest consumer before a release.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). In a disposable
setup with published test tags, verify an optional notice, a required floor,
rollback above the floor, rejection below it and growth refusal without a
terminal. Inspect the selecting settings file after acceptance and compare it
byte-for-byte after cancellation. Confirm a running session keeps its accepted
composition until the next start or explicit update.
