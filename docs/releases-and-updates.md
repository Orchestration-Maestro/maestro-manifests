# Releases and updates

The versioned unit is a container: this manifest release, an ordinary package or
a separately published capability package. A skill, role or workflow inside a
Git release has no independent version. Select an item from a pinned container
with filters; do not invent a per-file pin.

## Complete selection and update example

After the tags exist, this project `.maestro/settings.json` selects a release and
keeps optional updates as proposals:

```json
{
  "manifest": "git:github.com/Orchestration-Maestro/maestro-manifests@v0.1.0",
  "manifestAutoUpdate": false,
  "manifestAreas": []
}
```

In a manifest-capable Maestro session:

```text
/manifest status
/manifest update 0.1.1
```

`status` shows the active release, selecting settings file, newest known release,
minimum and intervening changelog entries. The explicit update can move forward
or back within the same identity, never below the newest known minimum. For
example, `/manifest update 0.1.0` rolls back only when that floor permits it.
`/manifest update` without a version chooses the newest release.

Those version numbers illustrate a release sequence; the target tag must exist.
The command shows the current and target versions, changed manifest files,
changelog and permission growth before applying. An interactive user confirms.
Application writes the new pin through the settings owner, acquires the release
through ordinary package operations and reloads. For project settings, commit
the pin change by pull request so teammates take it at their next start.

Scripts can use the command in print mode:

```sh
maestro -p "/manifest status"
maestro -p "/manifest update 0.1.1"
```

A configured model credential is needed for non-interactive startup even though
`status` never calls the model. Noninteractive updates with permission growth
refuse rather than treating silence as approval.

## Checks, floors and automatic updates

At every session start, then every policy `updateCheckMinutes` minutes, the
manifest consumer checks the selected source for releases. The default is 30;
`0` means session start only. Checks obey `update.versionCheck` and
`MAESTRO_OFFLINE`. Offline keeps the pin with a warning. No live push or
per-prompt check exists. A running session sees a notice; required releases
apply at its next start, or through `/manifest update` now.

This complete common `settings/policy.json` marks a security release's floor and
sets its notice interval:

```json
{
  "minimumRelease": "0.1.1",
  "updateCheckMinutes": 15
}
```

Use that policy in release `0.1.1` or later, not in `0.1.0`. Only common and core
can declare these members. Candidate inspection reads both optional common and
core policies and the candidate `CHANGELOG.md` in temporary scope. The higher
minimum wins. Inspection does not execute a lifecycle script, build step or
component.

If the newest release's minimum exceeds the current pin, the next start must
apply it. Without permission growth it does so without asking. With growth it
asks, and declining refuses the session with the fix; without a terminal it
refuses. Setting `manifestAutoUpdate: true` also allows optional newer releases
to apply without asking, but never bypasses growth confirmation. Otherwise an
optional pinned release does not move automatically.

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

A `manifest` lock fixes identity but still permits `/manifest update` to change
the same identity's ref. `maestro update` remains the ordinary engine/package
updater and skips pinned packages; it is not the manifest update route. When
unlocked, hand-editing a pin remains possible within the minimum-release rules.

## Publishing this repository

The existing Release Please workflow opens a release pull request from
conventional commits. It updates `version.txt`, `CHANGELOG.md` and its version
manifest. Merging that pull request creates a protected `v`-prefixed tag,
starting at `v0.1.0`. The root `package.json` is private and omits an independent
npm version; the Git tag is the release identity. In-tree capabilities share
that release. Keep separately versioned capabilities in separate containers.

See [development](development.md) for App credentials and merge-queue setup.
Never place those credentials in the catalog. Before tagging, load the exact
release candidate and validate every selected capability combination; current
content checks alone do not establish runtime compatibility.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). In a disposable
setup with published test tags, verify an optional notice, a required floor,
rollback above the floor, rejection below it and growth refusal without a
terminal. Inspect the selecting settings file after acceptance and compare it
byte-for-byte after cancellation. Confirm a running session keeps its accepted
composition until the next start or explicit update.
