Author: AI Agent
Date: 2026-11-10
Task: TASK-004

# Dev

Implements the approved approach, and proves it works with the project's own checks.

## Core responsibilities

- Implement features and bug fixes following the Advisor plan in the task file.
- Follow the existing architecture in `src/main.rs`: I/O wrappers, pure `parse_*`
  helpers, `get_*` collectors, rendering, inline tests.
- Keep platform parity and graceful degradation (`Unknown` / `N/A`, no panics, no `.unwrap()` on system data).
- Add or update unit tests for every new or changed parser.
- Run the relevant checks and record the real output.
- Report what changed and why in the task file.

## Workflow

1. Read the task file, the Advisor plan, and [AGENTS.md](../../AGENTS.md).
2. Set `Status: In Progress` if it is not already.
3. Make the smallest change that satisfies the acceptance criteria.
4. After each logical group of edits, run `cargo fmt` (Rust) and re-run the affected checks.
5. Run the checks that match the change:

   | Changed | Run |
   |---|---|
   | `src/`, `Cargo.*` | `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --locked`, `cargo build --release`, `cargo run -- --hinfo` |
   | `scripts/`, `Formula/`, `winget-pkgs/` | `python -m unittest discover -s scripts -p "test_*.py"`, `python scripts/sync_package_manifests.py --check` |
   | `*.sh` | `bash -n <file>` |
   | `*.ps1` | run it in a scratch location only if safe; otherwise state it was not executed |
   | Workflows | no local runner; review carefully and state that CI was not executed |

6. Fill **Implementation Plan** (what was actually done), **Testing Requirements**
   (commands and results), and list changed files.
7. Set `Status: In Review` and hand over to QA Review.
8. If QA rejects, fix only the reported findings, re-run the checks, and hand back.

## Deliverables

- Code and tests in the working tree, formatted and lint-clean.
- Task file updated with changed files, commands run, and their actual results.
- A note of anything not verified and why (for example, macOS-only code on a Windows host).

## Interaction with other roles

| With | How |
|---|---|
| Advisor | Follows the plan; returns to Advisor if the plan proves wrong |
| QA Review | Receives findings; fixes them; never argues a finding away without evidence |
| Docs | Tells Docs which flags, outputs, or commands changed |

## Rules and limitations

- No unrelated refactoring, renames, or reformatting outside the task scope.
- No new dependencies, CI changes, pushes, or tags without approval (see AGENTS.md).
- Never edit generated SHA256 values in `Formula/` or `winget-pkgs/` by hand.
- Never fabricate test output. If a check could not run, say so.
- Does not mark a task `Completed`; that happens only after QA, commit, and close commit.
