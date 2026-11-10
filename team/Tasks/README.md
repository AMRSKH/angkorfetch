Author: AI Agent
Date: 2026-11-10
Task: TASK-001

# Task Records

Every non-trivial change to this repository is tracked as one task file in this
directory. The file is the shared record that every role (Advisor, Dev,
QA Review, Docs) and every AI tool reads and updates.

## When a task file is required

| Needs a task file | Does not |
|---|---|
| Code changes in `src/` | Answering a question |
| Packaging, CI, installer, or script changes | Fixing a single typo |
| New or restructured documentation | Running tests or checks only |
| Harness or tooling changes | Reading or reviewing without edits |

When in doubt, create one. A task file costs less than an untraceable change.

## Task ID format

```text
TASK-{NNN}-{short-description}.md
```

- `NNN` is three digits with leading zeros, starting at `001`, strictly sequential.
- `short-description` is lowercase kebab-case, 2 to 6 words.
- One logical task, one ID, one file. Never reuse an ID. Never overwrite an existing task file.
- Subtasks are only for genuinely separable work: `TASK-005-A-short-description.md`, `TASK-005-B-...`.

### Finding the next ID

Always inspect this directory first. Take the highest existing number and add one.

```bash
ls team/Tasks | grep -oE '^TASK-[0-9]{3}' | sort -u | tail -1
```

```powershell
Get-ChildItem team/Tasks -Filter 'TASK-*.md' | ForEach-Object { $_.Name.Substring(5, 3) } | Sort-Object | Select-Object -Last 1
```

## File header

Every task file starts with this header, then the title. Copy
[task_template.md](task_template.md) and fill every section.

```markdown
Author: AI Agent
Date: YYYY-MM-DD
Task: TASK-001
Status: Pending

# TASK-001: Audit Full Workflow
```

`Author` is the person or the tool that created the record (for example
`AI Agent (Claude Code)` or `AMRSKH`).

## Status values

| Status | Meaning |
|---|---|
| Pending | Recorded, not started |
| In Progress | Advisor or Dev is working on it |
| In Review | Implementation done, QA Review is verifying |
| Blocked | Cannot continue; the blocker is written in **Notes and Limitations** |
| Completed | Verified, committed, and the commit hash is recorded |

Typical flow: `Pending -> In Progress -> In Review -> Completed`. QA findings send a
task from `In Review` back to `In Progress`. Never mark a task `Completed` while any
acceptance criterion is unchecked or any required check failed.

## Commits for a task

1. **Work commit**: all changes for the task plus its task file (status `In Review`),
   in one Conventional Commit that ends with the Task ID:
   `fix(installer): refuse glibc asset on musl [TASK-004]`.
2. **Close commit**: a commit cannot contain its own hash, so the hash is recorded
   afterwards. Update only the task file (commit hash, `Status: Completed`) and commit it as
   `chore(tasks): close TASK-004 [TASK-004]`.

`git log --grep "TASK-004"` then shows the full history of a task. See
[AGENTS.md](../../AGENTS.md) for the full Git rules.

## Worked example

[TASK-001-bootstrap-task-tracking.md](TASK-001-bootstrap-task-tracking.md) is the first real
record. It created this directory and doubles as the reference example.
