Author: AI Agent (Claude Code)
Date: 2026-11-10
Task: TASK-003
Status: In Review

# TASK-003: Pre-commit Gate and Tool Permissions

## Objective

Enforce the project's existing format, lint, and script checks at commit time, and
turn the AGENTS.md safety rules into real tool permissions where a tool supports them.

## Background

`rustfmt` and `clippy` are already the project's formatter and linter (default config,
documented in CONTRIBUTING.md). No hook was installed and CI does not run either tool
(TASK-001 observation 1). No new package is needed.

## Requirements

- A dependency-free `pre-commit` hook that runs only the checks matching staged files and aborts on failure.
- Works with Git for Windows (Git Bash `sh`) and POSIX `sh` on Linux and macOS.
- Hook file stays LF on every checkout.
- Claude Code project permissions deny reading `.env` and key files and ask before push, destructive Git, and dependency changes.

## Scope

`.githooks/pre-commit`, `.gitattributes`, `.claude/settings.json`, `AGENTS.md` (hook and permissions sections), this file.

## Out of Scope

Adding fmt or clippy to CI (separate decision for the maintainer). Permissions for other
tools (added with their commands in TASK-005 where supported).

## Assigned Roles

- Advisor: Main agent (Claude Code). No separate advisor agent was used.
- Dev: Main agent (Claude Code).
- QA Review: Self-review plus the hook tests below. Independent QA of the whole harness is TASK-006.
- Docs: Main agent (Claude Code).

## Implementation Plan

1. `.githooks/pre-commit` (POSIX `sh`, `set -eu`), matching staged paths with `grep -E`
   so file names with spaces are safe:

   | Staged paths | Checks |
   |---|---|
   | `*.rs`, `Cargo.toml`, `Cargo.lock` | `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings` |
   | `scripts/`, `Formula/`, `winget-pkgs/` | `python -m unittest discover -s scripts -p "test_*.py"`, `python scripts/sync_package_manifests.py --check` (tries `python3`, then `python`) |
   | `*.sh` | `bash -n <file>` per staged file |

2. Enable with `git config core.hooksPath .githooks` (local config, once per clone).
3. Add `.githooks/* text eol=lf` to `.gitattributes`.
4. `.claude/settings.json`: `deny` reads of `**/.env`, `**/.env.*`, `**/*.pem`, `**/*.key`;
   `ask` for `git push`, `git reset --hard`, `git clean`, `cargo add/remove/update/publish`,
   `pip install`, `gh pr create`, `gh release`.
5. Document the hook and its activation in AGENTS.md.

## Dependencies

TASK-002 (AGENTS.md exists).

## Acceptance Criteria

- [x] Misformatted Rust aborts a real `git commit`.
- [x] Clippy warnings abort a real `git commit`.
- [x] A hand edit of `Formula/angkorfetch.rb` aborts a real `git commit`.
- [x] A shell syntax error in a staged `.sh` aborts a real `git commit`.
- [x] Clean Rust and Python changes pass the hook (exit 0).
- [x] Docs-only commits skip all checks.
- [x] `.claude/settings.json` is valid JSON.
- [x] Activation is documented in AGENTS.md.

## Testing Requirements

Run on Windows 11, Git 2.56 (Git Bash), Rust 1.99.1, Python 3.14. Each probe edit was
reverted with `git restore --staged` and `git checkout --` on that file only.

| Probe | Result |
|---|---|
| Append `fn  hook_probe( ){let _x=1;}` to `src/main.rs`, `git commit` | aborted at `cargo fmt --check`, exit 1, no commit |
| Append `fn hook_probe() {}`, `git commit` | aborted at clippy (`function hook_probe is never used`), exit 1 |
| Append `# probe` to `Formula/angkorfetch.rb`, `git commit` | aborted at script unit tests, exit 1 |
| Append `if then fi (` to `install.sh`, `git commit` | aborted at `bash -n install.sh`, exit 1 |
| Comment added to `src/main.rs` and `scripts/test_sync_package_manifests.py`, run hook | fmt, clippy, 35 tests, `--check` all pass, exit 0 |
| Only `CLAUDE.md` staged, run hook | no checks, exit 0 |
| `python -c "json.load(...)"` on `.claude/settings.json` | valid |

## QA Results

Self-review by the main agent through the probes above. Not verified: hook behaviour on
Linux and macOS hosts (no such host available; the script uses POSIX `sh` features only).
The Claude Code permission rules were checked for syntax against the Claude Code
permissions documentation, not exercised by attempting a denied action.

## Documentation Updates

`AGENTS.md`: new section on the pre-commit hook and tool permissions.

## Git Commit

- Commit Message: `chore(hooks): add pre-commit gate and Claude Code permissions [TASK-003]`
- Commit Hash: recorded in the close commit

## Final Report

Commits are now gated locally on format, lint, script tests, and shell syntax, scoped to
what is staged. Claude Code enforces the secret-file and approval rules from AGENTS.md.

## Notes and Limitations

- The hook checks the working tree, not the staged snapshot: unstaged edits to the same files are checked too.
- `core.hooksPath` is local Git config; every clone must run `git config core.hooksPath .githooks` once.
- `git commit --no-verify` bypasses the hook; AGENTS.md forbids it without approval.
- Claude Code `Bash(...)` rules match command prefixes; variants such as `git -C . push` are not matched.
- The full `cargo test` suite is not in the hook (kept fast); it remains in final validation and CI.
