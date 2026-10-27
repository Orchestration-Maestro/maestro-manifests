# Integrity

Manifest selection is pinned Git only so the consumer can verify exactly which
files it is loading. At every session start it checks the installed release
checkout against the pinned commit and refreshes installed instruction copies.
Drift is repaired with a notice, not silently retained and not treated as a
reason to crash an otherwise recoverable session.

## Complete example

Select a published release in project `.maestro/settings.json`:

```json
{
  "manifest": "git:github.com/Orchestration-Maestro/maestro-manifests@v0.1.0",
  "manifestAreas": []
}
```

Start a manifest-capable Maestro build, then run:

```text
/manifest status
```

Use the source and pin it reports when inspecting the installed checkout, not a
separate development clone. To inspect a checkout without changing it:

```sh
git status --porcelain --untracked-files=all
git rev-parse HEAD
git rev-parse 'v0.1.0^{commit}'
```

Run these inside that installed Git repository. The two commit IDs should agree
and status should be empty. A clean tree alone is insufficient: it may be clean
at the wrong commit. These inspection commands are read-only; they do not stand
in for Maestro's startup verification.

## What is restored

Modified, added, deleted and untracked files, or a checkout moved away from its
pin, count as drift. Startup restores the pinned commit through the ordinary
package owner's Git operations, then prints
`Manifest release restored: <release>` and lists reset or removed files. A
changed pin committed by a team therefore takes effect at the next start even
if an old checkout already exists.

Installed instructions carry source commit and digest provenance in manifest
package state. An edited copy no longer matches its release file and is
refreshed at startup, with the same restore notice. A release update also
refreshes an installed copy when that source file changes. Keep personal
guidance in a separate file instead of editing a managed copy.

**Do not develop in an installed release checkout.** Restoration can discard
edits and remove untracked files there. Author in a normal clone, commit and
publish a ref, then select that ref. Before deliberately testing drift, use a
disposable settings scope and checkout with no work to preserve.

## Recovery and offline behavior

When the pinned commit cannot be restored locally, the consumer installs the
release again through ordinary package operations. If that is also impossible,
for example offline, it uses its last accepted composition with a warning naming
the drift and retries repair at the next start. It never trusts changed release
bytes as a new accepted composition. With no accepted composition, there is no
prior safe configuration to run; a missing/unloadable initial release follows
the ordinary load-failure rules.

A malformed release and a dirty checkout are different failures. A broken
pinned release may be repaired with a newer clean same-identity release under
[update permission-growth rules](releases-and-updates.md). If no candidate can
apply, the manifest load refuses with source, selecting settings file, problem
and fix. There is no silent fallback to internal mode. Help, version, package
commands and editing unlocked settings remain repair routes.

`/reload` recomposes the release too. A failed reload reports the error and keeps
the previously accepted composition so the existing session can continue.
A fresh start must still pass before a release is considered usable.

## Locks and trust limits

Integrity applies to the selected release regardless of whether prompt or
resource locks are set. It does not make the manifest publisher trustworthy,
protect every user file or sandbox an extension. A `manifest` lock fixes the
identity, and the update command remains the permitted same-identity ref route.
Local optional artifacts remain customizable under their own policy rules.

## Check a change

Run the [available checks](schemas-and-checks.md#available-checks). In an isolated
runtime fixture, start once to establish an accepted composition. Test a changed
file, deleted file, untracked file and moved HEAD separately, confirming the
restored bytes/commit and notice. Install an instruction and test copy refresh.
Simulate failed restoration and acquisition only in that fixture; confirm the
warning and last accepted composition, then successful repair on the next start.
Content parsing alone cannot prove these startup behaviors.
