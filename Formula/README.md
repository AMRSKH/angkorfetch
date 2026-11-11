# Homebrew formula for AngkorFetch

Homebrew is **Planned (homebrew-core)** (see [Package Status](../README.md#package-status--availability)),
so there is no supported `brew install` command yet. To install today, use a quick-install script
or `cargo install angkorfetch` ([Installation](../README.md#installation)).

## What this formula is

`angkorfetch.rb` installs the prebuilt release archives for macOS and Linux (arm and intel).
After each release, `.github/workflows/sync-packages.yml` regenerates it on the
`automation/sync-packages-v<version>` branch, for a pull request into `main`.

The [AMRSKH/homebrew-tap](https://github.com/AMRSKH/homebrew-tap) repository still exists and
holds a copy of this formula, but it is no longer the documented install path and CI does not
update it. Its formula points to v1.1.1 (checked 2026-11-11), behind the v1.2.1 release.

The planned homebrew-core formula builds from source (commit `c40983a`), so it will not be this file.

This README is hand-written; `scripts/sync_package_manifests.py` neither generates nor checks it.

## Do not edit angkorfetch.rb by hand

`angkorfetch.rb` is generated, and the next release's sync pull request overwrites it. Change the
template in `render_homebrew_formula()` in
[`scripts/sync_package_manifests.py`](../scripts/sync_package_manifests.py) and
update `scripts/testdata/angkorfetch.rb.golden`. CI fails otherwise:

```bash
python scripts/sync_package_manifests.py --check
```

## Why this directory is named Formula/

Homebrew resolves a tap's formulae from `Formula/`, `HomebrewFormula/` or the tap
root, but its RuboCop config only applies the `FormulaAudit` cops to paths
matching `**/{Formula,Casks}/**/*.rb`. A formula in `HomebrewFormula/` or at the
root is never reached by `brew audit --strict`, which is how an unauditable shape
went unnoticed here for several releases. See
[RELEASING.md](../RELEASING.md#why-the-formula-lives-in-formula).
