Author: AI Agent (Claude Code)
Date: 2026-11-10
Task: TASK-007
Status: In Review

# TASK-007: Enforce rustfmt and Clippy in CI

## Objective

Make the `test` job of `.github/workflows/release.yml` run `cargo fmt --check` and
`cargo clippy --all-targets --all-features --locked -- -D warnings`, so CI enforces what
CONTRIBUTING.md and AGENTS.md already require.

## Background

TASK-001 recorded that CI runs `cargo test`, the script tests, and the formula `--check`,
but never rustfmt or Clippy. Today only the local pre-commit hook (`.githooks/pre-commit`)
runs them, and only for contributors who enabled it. Because `build` needs `test` and
`release` needs `build`, adding the checks to `test` also blocks a release on a format or
lint failure.

## Requirements

- `cargo fmt --check` and `cargo clippy --all-targets --all-features --locked -- -D warnings`
  run in the `test` job, before the tests.
- The Rust toolchain step installs the `rustfmt` and `clippy` components explicitly.
- OS coverage of each check is the one the maintainer approves.
- No change to triggers, `permissions`, `needs`, or any publishing step.
- The maintainer approves the exact workflow diff before it is applied (AGENTS.md section 7).

## Scope

- `.github/workflows/release.yml`, `test` job only.
- Documentation that describes what the `test` job runs.

## Out of Scope

- Pinning the Rust toolchain version.
- Code changes in `src/` beyond what Clippy on a non-Windows host requires (see Implementation Plan step 3).
- `sync-packages.yml` and every other job.
- Pushing, opening a PR, or running the workflow.

## Assigned Roles

- Advisor: Main agent (Claude Code). The plan is a few lines of YAML and is shown verbatim to the maintainer for approval.
- Dev: Main agent (Claude Code), after maintainer approval.
- QA Review: Independent `qa-review` subagent with a fresh context.
- Docs: Main agent (Claude Code).

## Implementation Plan

Approaches compared:

| Approach | Coverage | Cost per CI run | Verdict |
|---|---|---|---|
| A. fmt on ubuntu, Clippy on all three OSes | Every `#[cfg(windows)]` and `#[cfg(not(windows))]` block is linted | about 1 to 2 min extra per runner | Recommended |
| B. fmt and Clippy on ubuntu only | `#[cfg(not(windows))]` code only; the 16 `#[cfg(windows)]` blocks (FFI, Registry) are never linted in CI | lowest | Acceptable fallback |
| C. Separate `lint` job, `needs`-wired into `build` | Same as A or B | extra job | Rejected: changes the `needs` graph for no gain |

rustfmt output does not depend on the host, so it runs once. Clippy only sees code that
compiles on the host, and `src/main.rs` has 16 `#[cfg(windows)]` and 8
`#[cfg(not(windows))]` items, so a Linux-only Clippy misses the Windows FFI and Registry
code. macOS compiles the same `cfg` set as Linux, so it adds little coverage, but keeping
the step condition-free keeps the YAML simple.

Steps:

1. In the `test` job's `Install Rust` step add `with: components: rustfmt, clippy`
   (`dtolnay/rust-toolchain` installs the minimal profile, which has neither).
2. After `Install Python` and before `Run tests`, add `Check formatting`
   (`if: matrix.os == 'ubuntu-latest'`) and `Clippy` (condition per the approved option).
3. `src/main.rs`: add `#[cfg(any(windows, test))]` to `parse_disk_support_from_strings`
   and `parse_ports`. Both are called only inside `#[cfg(windows)]` blocks and by unit
   tests, so Clippy on Linux fails today with two `dead_code` errors (found by the
   Linux-target check below). The attribute keeps them compiled and tested on every OS
   and removes them from non-Windows release binaries; no runtime behaviour changes.
   Rejected alternative: `#[allow(dead_code)]`, which suppresses the lint instead of
   stating where the code is used.
4. Docs: update the `test` row in `RELEASING.md` and the `test` line in
   `docs/MAINTAINING.md`; add fmt and Clippy to the protected CI gates in AGENTS.md section 7.

Risks:

| Risk | Mitigation |
|---|---|
| `stable` floats, so a new Rust release can add a Clippy lint and fail CI with no code change, blocking a tag | Same toolchain policy as the local hook; fix lints when they appear. Pinning is out of scope |
| Clippy fails on Linux-only code that was never linted | Confirmed: two `dead_code` errors. Fixed by step 3; verified with `cargo clippy --target x86_64-unknown-linux-gnu` |
| Components missing on the runner | Installed explicitly in step 1 |

Approvals required: the workflow diff (AGENTS.md section 7), the OS coverage choice, and
the `src/main.rs` change in step 3, which widens the original scope.

## Dependencies

- TASK-001 (observation that raised this gap), TASK-003 (pre-commit hook with the same commands).

## Acceptance Criteria

- [x] Maintainer approved the exact workflow diff and OS coverage (2026-11-11: "Approve, Clippy 3 OS", option A plus step 3)
- [x] `test` job installs `rustfmt` and `clippy` and runs both checks before the tests
- [x] No change to triggers, `permissions`, `needs`, or publishing steps
- [x] Both checks pass locally for the Windows and Linux `cfg` sets
- [x] Workflow YAML parses
- [x] Docs describing the `test` job updated
- [x] Independent QA accepted (Round 1 code, Rounds 2 and 3 docs)

## Testing Requirements

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features --locked -- -D warnings` (host, Windows)
- `cargo clippy --target x86_64-unknown-linux-gnu --all-targets --all-features --locked -- -D warnings`
- YAML parse of `.github/workflows/release.yml`
- `cargo test --locked`
- `git diff` shows only the planned lines

Pre-approval results (2026-11-11, Windows 11, Rust 1.99.1; target `x86_64-unknown-linux-gnu`
added locally with `rustup target add` for the Linux check):

| Command | Before step 3 | With step 3 (uncommitted) |
|---|---|---|
| `cargo fmt --check` | pass | pass |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` (Windows) | pass | pass |
| same with `--target x86_64-unknown-linux-gnu` | **fail**: `parse_disk_support_from_strings` and `parse_ports` never used | pass |
| `cargo test --locked` (Windows) | not run | 16 passed |

macOS was not checked: `src/main.rs` has no `target_os = "macos"` attributes, so macOS
compiles the same `cfg` set as Linux.

Dev results after applying the approved diff (2026-11-11, same machine):

| Command | Result |
|---|---|
| `cargo fmt --check` | pass |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` (Windows) | pass |
| same with `--target x86_64-unknown-linux-gnu` | pass |
| `cargo test --locked` | 16 passed |
| `python -m unittest discover -s scripts -p "test_*.py"` | 35 tests OK |
| `python scripts/sync_package_manifests.py --check` | pass |
| `bash -n get.sh && bash -n install.sh` | pass |
| `cargo build --release --locked`, then `angkorfetch -v` | build OK, exit 0 |
| PyYAML `safe_load` of `release.yml` | parses; `test` steps in the planned order; `on`, `permissions`, `build.needs` unchanged |

## QA Results

### Round 3

Verdict (docs): Accepted

Independent documentation re-check: ran as the `qa-review` subagent in Claude Code (Claude
Agent SDK); the reviewer did not write the docs. Scope: the fixes for the four Round 2 Low
findings (`git diff -- RELEASING.md docs/MAINTAINING.md AGENTS.md` plus this task file),
2026-11-11.

Findings: none.

| Round 2 finding | Now | Matches workflow |
|---|---|---|
| docs/MAINTAINING.md:101 | "Checks formatting on Ubuntu, then runs Clippy and the tests on Ubuntu, Windows, and macOS." | yes: `Check formatting` has `if: matrix.os == 'ubuntu-latest'` and runs before `Clippy`; `Clippy` and `Run tests` have no condition (release.yml:63-71) |
| RELEASING.md:173 | "a failing format, Clippy, or test check blocks the release transitively" | yes: all three steps are in `test`, which `build` needs |
| AGENTS.md:139 | gates separated by semicolons; wording otherwise unchanged | still lists every protected gate and weakens none |
| Task file:104 | `- [x] Docs describing the `test` job updated`; Documentation Updates table lists the Round 2 rewording | yes |

Commands run: `git diff -- RELEASING.md docs/MAINTAINING.md AGENTS.md` (1 line changed in
AGENTS.md and docs/MAINTAINING.md, 2 in RELEASING.md, nothing staged); `git diff --check` on the
three docs (clean); `git diff -- .github src | md5sum` (same hash as Round 2, so the code diff
is unchanged); `wc -c AGENTS.md` (16,265 bytes, under 32 KiB).

Could not be verified: CI has not run, so the docs are checked against the workflow file only.

### Round 2

Verdict (docs): Accepted

Independent documentation re-check: ran as the `qa-review` subagent in Claude Code (Claude
Agent SDK); the reviewer did not write the docs. Reviewed
`git diff -- RELEASING.md docs/MAINTAINING.md AGENTS.md` (1 line changed in each, nothing
staged) against `.github/workflows/release.yml` lines 59-71 on 2026-11-11.

Findings (none blocking):

| Severity | Location | Finding |
|---|---|---|
| Low | docs/MAINTAINING.md:101 | "Checks formatting, runs Clippy and the tests across Ubuntu, Windows, and macOS" can be read as formatting running on all three OSes; the workflow runs `cargo fmt --check` on ubuntu only (RELEASING.md:165 says so precisely). |
| Low | RELEASING.md:173 | "a failing test blocks the release transitively" is still true but no longer complete: a fmt or Clippy failure now blocks a tag too, which is the main effect of this task; "a failing check" would say so. |
| Low | AGENTS.md:139 | Nested commas in "(`test` with its fmt, Clippy, and test steps, `version-check`, formula `--check`)" make the gate list hard to parse; semicolons between gates would fix it. |
| Low | team/Tasks/TASK-007-ci-fmt-clippy-gate.md:104 | The "Docs describing the `test` job updated" criterion is still unchecked although the docs are done; tick it before Step 7. |

Checked:

- Accuracy: RELEASING.md:165 matches the workflow (fmt only on `ubuntu-latest`, Clippy with
  `-D warnings` on all three, `cargo test --release --locked`). docs/MAINTAINING.md:101 and
  AGENTS.md:139 match apart from the wording notes above. AGENTS.md:139 extends the
  protected-gate rule and weakens nothing.
- Missed docs: searched every `*.md` for CI, workflow, `test` job, fmt, and Clippy mentions.
  CONTRIBUTING.md and docs/DEVELOPMENT.md give local commands only. docs/RELEASING.md:39 sums up
  `release.yml` without listing `test` job steps. RELEASING.md:94 ("Multi-platform tests"),
  RELEASING.md:368-379, Formula/README.md:18, and AGENTS.md:32 are still accurate. No doc claims
  CI skips fmt or Clippy. `improve plan.md` is a planning note, not a CI description.
  The RELEASING.md:165 row also leaves out script tests, the formula check, and the smoke
  test; that gap was already there and is covered at RELEASING.md:377.
- AGENTS.md section 11: the right documents were updated (release/CI, maintainer process,
  harness rules). Nothing unimplemented is described as working. Docs link instead of
  duplicating. No new files, so the header rule does not apply. AGENTS.md is 16,265 bytes,
  under the 32 KiB Codex limit.
- Task file: the Round 1 Low about the stale "Blocked" note is resolved in Notes. The other
  Round 1 Low is recorded there as a follow-up.
- The code diff (`.github/`, `src/`) is the same as in Round 1. `git diff --check` on the three
  docs is clean. The index stores LF, and only the working tree is CRLF via `core.autocrlf`.

Could not be verified: CI itself has not run, so the documented `test` job steps are checked
against the workflow file, not against a real Actions run.

### Round 1

Verdict: Accepted

Independent review: ran as the `qa-review` subagent in Claude Code (Claude Agent SDK) with a
fresh context; the reviewer did not write the code. Reviewed the uncommitted `git diff`
(`.github/workflows/release.yml` +13 lines, `src/main.rs` +2 lines, nothing staged) on
2026-11-11, Windows 11, Rust 1.99.1 (`stable-x86_64-pc-windows-gnu`).

Findings (none blocking):

| Severity | Location | Finding |
|---|---|---|
| Low | team/Tasks/TASK-007-ci-fmt-clippy-gate.md:157 | Notes still says "Blocked (2026-11-11)" on maintainer approval, while line 99 records the approval; mark it resolved so the record does not contradict itself. |
| Low | src/main.rs:208, src/main.rs:217 | Pre-existing test-only parsers `parse_package_count` and `validate_version_tag` use `#[allow(dead_code)]`, the pattern this task rejected, so the file now has two conventions for test-only code. Out of scope here; possible follow-up. |

Acceptance criteria checked against the diff:

- Workflow diff matches Implementation Plan steps 1 and 2 exactly: `components: rustfmt, clippy`
  on `Install Rust`; `Check formatting` (`if: matrix.os == 'ubuntu-latest'`) and an
  unconditional `Clippy` step after `Install Python` and before `Run tests` (option A).
- `dtolnay/rust-toolchain` `action.yml` (stable branch, fetched read-only) installs with
  `--profile minimal` and splits `components` on commas and whitespace
  (`for c in ${components//,/ }`), so `rustfmt, clippy` becomes two `--component` flags.
- No change outside the `test` job's steps: PyYAML comparison against `HEAD` shows `on`,
  `permissions`, the job list, the `test` job's non-step keys (`strategy`, `runs-on`), and
  every other job (`version-check`, `test-installer-containers`, `build` incl. `needs`/`if`,
  `packages`, `checksums`, `release`, `sync_packages`) identical.
- `src/main.rs`: `parse_disk_support_from_strings` and `parse_ports` are called only at
  src/main.rs:1653 and src/main.rs:1668 (both inside `#[cfg(windows)]` blocks) and in tests.
  `#[cfg(any(windows, test))]` leaves Windows unchanged, keeps both compiled and tested on
  every OS, and drops never-called code from non-Windows binaries. No runtime change on any
  OS. `.unwrap()` count is still 0. The file has no `#[cfg(target_os = ...)]` attributes,
  so macOS compiles the same `cfg` set as Linux.

Commands run:

| Command | Result |
|---|---|
| `cargo fmt --check` | pass (exit 0) |
| `rustfmt --edition 2021 --check` on an LF copy of `src/main.rs` (as ubuntu checks it out) | pass |
| `cargo clippy --all-targets --all-features --locked -- -D warnings`, clean target dir (Windows cfg) | pass, crate re-linted |
| same with `--target x86_64-unknown-linux-gnu`, clean target dir (non-Windows cfg) | pass, crate re-linted |
| same Linux Clippy on a `git archive HEAD` copy (own target dir, before the change) | fail, exit 101: `parse_disk_support_from_strings` and `parse_ports` never used (confirms step 3 is required) |
| `cargo test --locked` | 16 passed |
| `cargo test --release --locked`, clean target dir (as CI runs it) | 16 passed |
| `python -m unittest discover -s scripts -p "test_*.py"` | Ran 35 tests, OK |
| `python scripts/sync_package_manifests.py --check` | pass |
| `bash -n get.sh && bash -n install.sh` | pass |
| `cargo build --release --locked`, then `angkorfetch -v`, `angkorfetch`, `angkorfetch --hinfo` | build OK; all exit 0; `--hinfo` still prints Disk Type and Ports through the two parsers |
| PyYAML `safe_load` of `release.yml` vs `HEAD` | parses; only the three planned step changes |
| `git diff --check` | clean |

Could not be verified:

- macOS Clippy and tests: no `x86_64-apple-darwin` target installed, and installing is out of
  bounds. Covered only by reasoning (same `cfg` set as Linux).
- Linux tests were type-checked by Clippy `--all-targets` but not executed.
- Windows CI uses the MSVC host; this host is GNU. `src/main.rs` has no `target_env` gating,
  so the linted `cfg` set is the same.
- The workflow on GitHub Actions itself: no `actionlint` locally; the first CI run is the
  end-to-end check. A future `stable` release can add Clippy lints (accepted risk in the plan).
- The maintainer's approval message itself; it is taken from line 99 of this file.
- Docs criterion is still open; it belongs to the Docs step, not this round.

## Documentation Updates

Docs role: main agent (Claude Code).

| File | Change |
|---|---|
| `RELEASING.md` (CI architecture table) | `test` row lists `cargo fmt --check` (ubuntu), Clippy with `-D warnings`, and the tests |
| `RELEASING.md` (`needs` graph notes) | a failing format, Clippy, or test check blocks the release (QA Round 2) |
| `docs/MAINTAINING.md` (Step 4) | `test` line: formatting on Ubuntu, Clippy and tests on all three OSes |
| `AGENTS.md` section 7 | fmt and Clippy steps named as part of the protected `test` gate; gates separated by semicolons (QA Round 2) |

Not changed: `CONTRIBUTING.md` and `docs/DEVELOPMENT.md` already give the same commands as
local requirements and do not describe CI contents; `docs/RELEASING.md` links to the root
`RELEASING.md` for the job table.

## Git Commit

- Commit Message: `ci(test): enforce rustfmt and Clippy in the test job [TASK-007]`
- Commit Hash:

## Final Report

| Item | Content |
|---|---|
| Task | TASK-007: Enforce rustfmt and Clippy in CI |
| What changed | `test` job installs `rustfmt` and `clippy`, runs `cargo fmt --check` on ubuntu and Clippy with `-D warnings` on all three OSes before the tests; two Windows-only parsers gated with `#[cfg(any(windows, test))]` so Clippy passes off Windows |
| Files | `.github/workflows/release.yml`, `src/main.rs`, `RELEASING.md`, `docs/MAINTAINING.md`, `AGENTS.md`, this file |
| QA | Independent `qa-review` subagent: Round 1 `Accepted` (2 Low: stale note fixed, `#[allow(dead_code)]` convention recorded as follow-up); docs Round 2 `Accepted` (4 Low, all fixed); docs Round 3 `Accepted`, no findings |
| Validation | Full AGENTS.md section 5 validation passed on 2026-11-11, plus Linux-target Clippy; see Testing Requirements |
| Limitations | Workflow not run on GitHub Actions; macOS Clippy and Linux test execution not run locally; floating `stable` toolchain can add lints |

## Notes and Limitations

- The workflow itself cannot run before a push; the first CI run is the end-to-end check.
- Was `Blocked` (2026-11-11) on maintainer approval of the workflow diff, the OS coverage
  option, and the `src/main.rs` step 3 change. Resolved the same day: option A plus step 3
  approved (see Acceptance Criteria). Without step 3, any Clippy step on ubuntu or macOS
  would fail on the first run.
- Follow-up (QA Round 1, Low, out of scope): `parse_package_count` and `validate_version_tag`
  in `src/main.rs` still mark test-only code with `#[allow(dead_code)]`, while this task
  used `#[cfg(any(windows, test))]`. Pick one convention in a separate task.
- macOS Clippy and Linux test execution were not run locally; the first CI run covers them.
- Local environment: `rustup target add x86_64-unknown-linux-gnu` was run for the Linux check.
