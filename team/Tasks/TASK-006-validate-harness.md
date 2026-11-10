Author: AI Agent (Claude Code)
Date: 2026-11-10
Task: TASK-006
Status: In Review

# TASK-006: Validate Harness

## Objective

Independently review the complete harness (TASK-001 to TASK-005), fix confirmed
findings, and run the full project validation on the harness branch.

## Background

TASK-001 to TASK-005 were self-reviewed by the implementing agent. This task supplies
the independent QA those records deferred.

## Requirements

- An independent reviewer that did not write the harness reviews every harness file.
- Confirmed findings are fixed or explicitly accepted with a reason.
- Full validation from AGENTS.md section 5 passes; the binary still runs.

## Scope

All files added or changed on `chore/ai-harness` relative to `main`.

## Out of Scope

Project code, CI, and the TASK-001 observations (CI fmt/clippy gap, duplicate RELEASING.md,
`improve plan.md`, `dev` behind `main`).

## Assigned Roles

- Advisor: Main agent (Claude Code).
- Dev: Main agent (Claude Code), fixing QA findings.
- QA Review: Independent Claude Code general-purpose subagent with a fresh context and a read-only brief.
- Docs: Main agent (Claude Code).

## Implementation Plan

1. Run the independent QA review (read-only).
2. Fix confirmed findings; record any finding not fixed with the reason.
3. Re-run the full validation and the affected harness checks.

## Dependencies

TASK-001 to TASK-005.

## Acceptance Criteria

- [x] Independent QA review completed (four rounds, same independent subagent).
- [x] Every Critical, High, and Medium finding fixed (none accepted without a fix).
- [x] Full validation passes.

## Testing Requirements

Full validation before fixes (2026-11-10, Windows 11, Rust 1.99.1, Python 3.14):

| Command | Result |
|---|---|
| `cargo fmt --check` | pass |
| `cargo clippy --all-targets --all-features -- -D warnings` | pass |
| `cargo test --locked` | 16 passed |
| `cargo build --release --locked` | pass |
| `target/release/angkorfetch.exe -v`, `--hinfo` | exit 0 |
| `python -m unittest discover -s scripts -p "test_*.py"` | 35 tests OK |
| `python scripts/sync_package_manifests.py --check` | OK |
| `bash -n get.sh`, `bash -n install.sh`, `sh -n .githooks/pre-commit` | OK |

After round 1 fixes:

| Check | Result |
|---|---|
| `cargo build --locked` | pass |
| `cargo run --locked -- -v`, `-- --hinfo`, default | exit 0 |
| `sh -n .githooks/pre-commit` | OK |
| `git rm Formula/angkorfetch.rb`, real `git commit` | aborted by script tests, exit 1; file restored |
| Staged `tést-probe.sh` with a syntax error, hook run directly | aborted at `bash -n`, exit 1; probe removed |
| `.claude/settings.json` JSON parse | valid, 37 ask rules, 4 deny rules |
| `claude -p "/qa-review"` init event (CLI 2.1.311) | 5 commands and 4 agents discovered; no settings warnings on stderr |
| `opencode debug agents` (2.2.6) | 4 role subagents, each denies `*.env`, `*.env.*`, `*.pem`, `*.key` |

## QA Results

### Round 4

Verdict: Accepted

Narrow re-check of the round 3 fix in `team/Commands/task.md` (row 5) and
`team/Commands/qa-review.md` (step 7): a rejected docs re-check now resolves to the
Step 6 documentation re-check row. The reviewer's optional wording for row 5 ("the newest
round is a code round (plain `Verdict:`) and it is not `Accepted`") was applied verbatim.

### Round 3

Verdict: Accepted

All round 2 findings confirmed fixed. Resume table walked against this file and against
a fixed task in `In Review` after one rejected round; both resolve correctly. One Low:

| Severity | Finding | Resolution |
|---|---|---|
| Low | A rejected docs re-check resumed at Dev or full QA instead of Docs | Row 5 limited to code rounds; `Verdict (docs): Changes requested` leaves `In Review` (verified in round 4) |

### Round 2

Verdict: Changes requested

Same independent subagent, read-only. Every round 1 finding confirmed fixed in the files.
Hook re-tested by the reviewer in a scratch repo with `sh` and `dash`: deleted-only `.sh`
exit 0; deleted file plus broken `.sh` exit 1; non-ASCII broken `.sh` exit 1; `py -3` as
the only Python exit 0; no Python or Python 2 only exit 1. All project checks pass.

| Severity | Finding | Resolution |
|---|---|---|
| Medium | Resume table sent a fixed task back to Dev because the old `Changes requested` verdict was still present; multi-round verdict rule undefined; this file broke it | Rows keyed on `Status` and the newest verdict; rounds are newest first with an exact `Verdict:` first line; this file restructured |
| Low | Resume skipped the docs re-check; docs rejections went to Dev | Row for `Verdict (docs):` added; docs rejections return to Docs |
| Low | `--locked` claim still too broad | AGENTS.md: "`cargo test` and the CI test job use `--locked`" |
| Low | This task was `In Progress` during QA; deviation notes said findings "were fixed" before acceptance | Status set to `In Review`; notes now point to this file for the verdict |

### Round 1

Verdict: Changes requested

Independent Claude Code subagent, read-only brief, fresh context. It confirmed
AGENTS.md matches the code, all links and anchors resolve, all project checks pass,
and `sh -n` / `dash -n` accept the hook. Findings and resolution:

| Severity | Finding | Resolution |
|---|---|---|
| High | TASK-001 to 005 marked `Completed` with self-review only | Process deviation recorded in each file's QA Results; history not rewritten; this task is their independent QA |
| High | TASK-005 overstated discovery verification | Criterion and Final Report reworded to what was verified |
| Medium | TASK-002 / 004 claimed all AGENTS.md commands ran in TASK-001 | Missing commands run here (see Testing); records corrected |
| Medium | Enforcement overstated; only `Bash(...)` rules on a PowerShell-first host; missing ask rules | `PowerShell(...)` twins, `git branch -D`, `rebase`, `commit --amend`, `tag`, `gh pr merge`, `gh workflow run`, `pip3`, `python -m pip`, `Edit(.github/workflows/**)` added; AGENTS.md now says "subset" and lists unenforced cases |
| Medium | Docs ran after QA, so doc edits were committed unreviewed | `task.md` Step 6 and `team/README.md`: QA re-checks the documentation diff when Docs changed files |
| Medium | Resume from status was ambiguous | Resume table in `task.md` Step 2; QA Results first line is now `Verdict: ...`; `In Review` definition updated |
| Medium | No branch check or starting `git status` | `task.md` Step 1 records both and moves work off `main` / `dev` |
| Medium | Hook ignored staged deletions | Area matching uses `--diff-filter=ACMRD`; `bash -n` loop keeps ACMR |
| Medium | Hook not documented in CONTRIBUTING.md / DEVELOPMENT.md | Short sections added to both |
| Low | Non-ASCII paths quoted by Git, `.sh` check skipped | `git -c core.quotePath=false` |
| Low | Python probe accepted Python 2; `py -3` not tried | Version check `>= 3`; tries `python3`, `python`, `py -3` |
| Low | No commit format for trivial changes | AGENTS.md and `task.md`: trivial commits omit the Task ID |
| Low | Rules disagreed (3-round limit, `--no-verify`) | Aligned in QA-Review README and `task.md` |
| Low | Delegation table contradicted the `qa-review` skill | Table aligned |
| Low | `dev.md` told a subagent to run `/advisor` | Now stops and reports the missing Advisor stage |
| Low | Branch model citation inaccurate; PR base rule missing | AGENTS.md section 8 corrected |
| Low | Checksum claim covered all installers; `--locked` inconsistent | Scoped to `get.*`; `--locked` added to release build commands |
| Low | OpenCode denied only `.env`; primary agent not covered | `*.pem`, `*.key` denies added; Commands README states primary agent keeps defaults |
| Low | Header rule had no exemption; example reused a real commit | Exemption added; neutral example in Tasks README |

## Documentation Updates

Updated: `AGENTS.md` (stack, commands, security enforcement, Git workflow, hook, header
exemption), `CONTRIBUTING.md` and `docs/DEVELOPMENT.md` (pre-commit hook),
`team/README.md`, `team/Tasks/README.md`, `team/Dev/README.md`, `team/QA-Review/README.md`,
`team/Commands/{README,task,dev,qa-review}.md`, and TASK-001 to TASK-005 records.

## Git Commit

- Commit Message: `chore(harness): apply independent QA fixes [TASK-006]`
- Commit Hash: recorded in the close commit

## Final Report

The harness passed independent QA after fixes to the hook (deletions, non-ASCII names,
Python 3 detection), Claude Code permissions (PowerShell rules and missing approval
rules), the `/task` resume and docs re-review flow, and several overstated claims in
the task records. Project code and CI are unchanged; all project checks pass.

## Notes and Limitations

- Not verified: hook on real Linux and macOS hosts (Git Bash `sh` and Cygwin `dash` only);
  Claude Code end-to-end delegation (CLI not logged in); Cursor, Codex, and Antigravity
  discovery (no local CLI); OpenCode command discovery and glob matching of nested paths.
- The independent reviewer ran as a Claude Code general-purpose subagent with a read-only
  brief, so the main agent recorded its verdicts in this file.
- Out-of-scope observations from TASK-001 remain open for the maintainer.
