# Homebrew formula for AngkorFetch

Install from the tap:

```bash
brew install AMRSKH/tap/angkorfetch
```

## This is a mirror, not the source of truth

The user-facing formula lives in [AMRSKH/homebrew-tap](https://github.com/AMRSKH/homebrew-tap)
at `Formula/angkorfetch.rb`. The copy here exists so this repository can also be
tapped directly, and is kept **byte-identical** to the tap's.

Both are generated. Do not edit `angkorfetch.rb` by hand — the next release will
overwrite it. Change the template in `render_homebrew_formula()` in
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
