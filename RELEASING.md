# Releasing AngkorFetch

Reference for maintainers cutting a release. Everything below is automated except
the version bump and the two pull request merges.

## Overview

```
 tag vX.Y.Z on main
         │
         ▼
 ┌──────────────────────────── release.yml ────────────────────────────┐
 │  test ──► build (5 targets) ──► packages ──► checksums              │
 │                                     └──────────► release            │
 │                                                     │               │
 │                                                     ▼               │
 │                          sync_packages (calls sync-packages.yml)    │
 └─────────────────────────────────────────────────────────────────────┘
                                                       │
                                                       ▼
                          reads checksums.txt from the published release
                          rewrites Homebrew + winget, opens a pull request
```

## Cutting a release

1. **Bump the version** in a pull request into `main`:

   - `Cargo.toml` and `Cargo.lock` (run a build to refresh the lock)
   - `README.md` sample banner
   - `snap/snapcraft.yaml` — both `version` and `source-tag`
   - `flatpak/io.github.AMRSKH.angkorfetch.yml` — `tag`
   - `linux/rpm/angkorfetch.spec` — `Version` plus a new `%changelog` entry
   - `linux/rpm/build-rpm.sh` and `linux/deb/build-deb.sh` — `VERSION`

   Do **not** touch `HomebrewFormula/` or `winget-pkgs/` here. See
   [Why package definitions lag](#why-package-definitions-lag).

2. **Merge the pull request** once CI is green.

3. **Tag the merge commit** and push:

   ```bash
   git tag -a vX.Y.Z <merge-sha> -m "AngkorFetch vX.Y.Z"
   git push origin vX.Y.Z
   ```

4. **Review the auto-opened package sync pull request** and merge it.

The version must be bumped in step 1 rather than relying on the tag alone,
because the two version sources are independent: `.deb`/`.rpm` versions come
from the tag via `VER="${GITHUB_REF_NAME#v}"`, while the binary's reported
version comes from `Cargo.toml`. Tagging without bumping ships a `1.1.1`-named
package containing a binary that reports `1.1.0`.

## release.yml

Triggered by pushes to `main`/`dev`, `v*` tags, pull requests into `main`, and
manual dispatch. Only the tag path publishes anything.

| job | runs on | purpose |
| --- | --- | --- |
| `test` | ubuntu, windows, macos | `cargo test --release --locked` |
| `build` | 5 targets | builds and uploads each binary |
| `packages` | tags only | builds `.deb` and `.rpm` |
| `checksums` | tags only | `sha256sum` over every artifact |
| `release` | tags only | verifies completeness, publishes |

The `needs` graph is load-bearing, not incidental:

- `build` needs `test`, so a failing test blocks the release transitively.
  Cross-compiled targets cannot execute their own binaries, which is why tests
  run on native hosts in a separate job rather than inside the build matrix.
- `checksums` needs `packages`, otherwise `checksums.txt` is generated from a
  partial artifact set and silently omits the `.deb` and `.rpm`.
- `release` needs `packages`, otherwise it publishes as soon as `checksums`
  finishes and the packages arrive too late to be attached.

That last point was a real bug. On the v1.1.0 run `packages` finished at
01:50:16 while `release` had already published at 01:50:07, so v1.0.1 and v1.1.0
both shipped without their `.deb` and `.rpm`. Ordering alone only fixes the
graph as it exists today, so `release` also asserts the expected asset set
before publishing. If a future refactor reintroduces a gap, the job fails
loudly instead of shipping a short release.

A hyphen in the tag marks the release as a prerelease, so `v1.2.0-rc.1` will not
become "Latest" and will not move the package managers.

### Prerelease versions in packages

`.deb` and `.rpm` versions come from the tag, but neither format accepts a bare
hyphen. rpm rejects it outright with `Illegal char '-'`, and for dpkg a hyphen
separates the upstream version from the Debian revision. The `packages` job
therefore translates hyphens to tildes, which is the packaging convention for a
prerelease and sorts correctly — `1.2.0~rc.1` precedes `1.2.0`. Normal releases
contain no hyphen and are unaffected.

This is done with `tr`, not `"${VER//-/~}"`, because bash applies tilde
expansion to the replacement string in a pattern substitution and the shorthand
silently produces `1.2.0/home/runnerrc.1`.

### Expected assets

A correct release publishes 8 assets:

```
angkorfetch-linux-x86_64.tar.gz
angkorfetch-linux-aarch64.tar.gz
angkorfetch-macos-x86_64.tar.gz
angkorfetch-macos-aarch64.tar.gz
angkorfetch-windows-x86_64.zip
angkorfetch_X.Y.Z_amd64.deb
angkorfetch-X.Y.Z-1.x86_64.rpm
checksums.txt
```

Six assets means the packages were dropped — check the `release` job's
`Verify all expected artifacts are present` step.

## sync-packages.yml

Called by `release.yml` after a release publishes, and also available on manual
dispatch. It reads `checksums.txt` from the release, rewrites the package
definitions via `scripts/sync_package_manifests.py`, and opens a pull request.

```bash
# Re-sync a specific release, for example after fixing something by hand
gh workflow run sync-packages.yml -f tag=v1.1.1

# Preview without pushing a branch or opening a pull request
gh workflow run sync-packages.yml -f tag=v1.1.1 -f dry_run=true

# Sync from a prerelease, which is otherwise skipped
gh workflow run sync-packages.yml -f tag=v1.2.0-rc.1 -f allow_prerelease=true
```

It skips prereleases and drafts, and refuses tags without a leading `v`. The
prerelease and draft state is read from the release itself rather than the event
payload, so the guard behaves identically no matter which trigger fired.

### Why it is chained rather than event-driven

`sync-packages.yml` does declare a `release: published` trigger, but that is only
a fallback for releases published by hand. It does **not** fire for our own
releases.

GitHub suppresses workflow triggers for events raised by `GITHUB_TOKEN`, to stop
workflows recursively triggering themselves. `release.yml` publishes via
`softprops/action-gh-release` using `GITHUB_TOKEN`, so the resulting
`release: published` event never starts a workflow run. This was confirmed
empirically: after publishing `v1.1.2-pmtest.2`, `sync-packages.yml` had zero
runs despite being active on the default branch.

`release.yml` therefore calls it directly as a reusable workflow
(`uses: ./.github/workflows/sync-packages.yml`) in a `sync_packages` job that
needs `release`. That is deterministic, keeps the result visible in the same run,
and does not depend on event delivery.

Note that the calling job must grant `pull-requests: write` explicitly. A
reusable workflow cannot escalate beyond the caller's token permissions, and
`release.yml` only grants `contents: write` at the top level.

### Why package definitions lag

`HomebrewFormula/angkorfetch.rb` and the winget manifests pin a `sha256` of
release artifacts. Those artifacts do not exist until the release is published,
so the definitions cannot be updated in the same pull request that creates the
tag — the URLs would point at missing files with stale hashes, breaking both
package managers. They intentionally continue pointing at the previous working
release until the new hashes exist, which is exactly the window this workflow
closes automatically.

### Idempotency

Rerunning the workflow for the same tag creates no duplicate commits and no
duplicate pull requests. Three properties combine to guarantee that:

1. `sync_package_manifests.py` writes identical bytes when the definitions are
   already current, including preserving original line endings, so `git diff` is
   an authoritative answer to "is there anything to do".
2. The branch name is derived from the tag (`automation/sync-packages-vX.Y.Z`),
   so a rerun reuses the same branch rather than creating a new one.
3. The branch is pushed only when its tree differs from the remote, and a pull
   request is created only when one is not already open. If a pull request
   exists, its description is refreshed instead.

### One-time setup required for full automation

By default GitHub does **not** allow Actions to open pull requests, so the sync
pushes its branch and then stops with a warning:

```
GitHub Actions is not permitted to create or approve pull requests
```

Nothing is lost when this happens — the branch is pushed and the job prints a
compare link — but the last manual step remains. Pick one of:

1. **Enable the repository setting.** Settings → Actions → General → Workflow
   permissions → *Allow GitHub Actions to create and approve pull requests*, or:

   ```bash
   gh api -X PUT repos/AMRSKH/angkorfetch/actions/permissions/workflow \
     -F default_workflow_permissions=read \
     -F can_approve_pull_request_reviews=true
   ```

   Be aware that this single checkbox governs both *creating* and *approving*
   pull requests. If branch protection ever requires reviews, a workflow could
   satisfy that requirement itself. Pair it with a CODEOWNERS review requirement
   if that matters.

2. **Add a `PACKAGE_SYNC_TOKEN` secret** holding a personal access token with
   `contents: write` and `pull-requests: write`. The workflow prefers it
   automatically and falls back to `GITHUB_TOKEN`. This avoids the
   self-approval concern and has the added benefit that CI *does* run on the
   resulting pull requests, which `GITHUB_TOKEN` cannot trigger.

This deliberately fails soft rather than hard. The sync runs inside the release
workflow, so a hard failure would mark an otherwise-successful release as failed.
Any error other than this specific permission refusal still fails the job.

### CI does not run on the auto-opened pull request

Pull requests opened with the default `GITHUB_TOKEN` do not trigger further
workflow runs. This is a deliberate GitHub restriction that prevents recursive
workflows, not a misconfiguration.

The change is limited to version strings and checksums, and the workflow prints
the full diff, so review is usually sufficient. To get CI on these pull requests
anyway, add a personal access token with `contents: write` and
`pull-requests: write` as a repository secret named `PACKAGE_SYNC_TOKEN`. The
workflow uses it automatically when present and falls back to `GITHUB_TOKEN`
otherwise. Alternatively, close and reopen the pull request to trigger checks.

## scripts/sync_package_manifests.py

Standalone and runnable locally:

```bash
gh release download v1.1.1 --pattern checksums.txt
python scripts/sync_package_manifests.py --version 1.1.1 --checksums checksums.txt
git diff
```

Edits are driven by the artifact filename embedded in each existing URL rather
than by matching the old version string, so the script stays correct no matter
how far behind the definitions have drifted. It fails rather than writing a
manifest it cannot verify — if an artifact referenced by a definition is missing
from `checksums.txt`, or if a `url` is not followed by a checksum line, the job
errors out.

## Verifying a release by hand

```bash
gh release view vX.Y.Z --json assets --jq '.assets[].name'

gh release download vX.Y.Z --dir /tmp/rel
cd /tmp/rel && sha256sum -c checksums.txt

tar xzf angkorfetch-linux-x86_64.tar.gz && ./angkorfetch --version
```
