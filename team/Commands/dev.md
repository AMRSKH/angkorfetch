Author: AI Agent
Date: 2026-11-10
Task: TASK-005

# /dev: Implement a Task

Implements the approved plan of a task and verifies it with the project's checks.

**Usage**

```text
/dev TASK-NNN                implement (or fix QA findings for) an existing task
```

**Input**: `$ARGUMENTS` is a Task ID. Without one, ask for it, or suggest `/task` to
create the record first. Dev does not work without a task record for non-trivial changes.

## Steps

1. Read [AGENTS.md](../../AGENTS.md), [Dev/README.md](../Dev/README.md), and the task file.
2. If **QA Results** lists open findings, fix those only. Otherwise follow the
   **Implementation Plan**. If there is no plan, run `/advisor TASK-NNN` first.
3. Set `Status: In Progress`.
4. Implement the smallest change that meets the acceptance criteria. Add tests for new
   or changed `parse_*` functions.
5. Run `cargo fmt` after each logical group of Rust edits.
6. Run the checks for the changed areas (table in [Dev/README.md](../Dev/README.md#workflow))
   and record the commands and their real results in **Testing Requirements**.
7. List changed files and what was done in **Implementation Plan**.
8. Set `Status: In Review`.

## Output

- Working-tree changes, formatted and lint-clean.
- Updated task file with files changed, commands run, and results.
- Anything not verified, with the reason.

## Limits

No commits (the `/task` flow commits after QA), no dependency changes, no pushes, no
edits outside the task scope.
