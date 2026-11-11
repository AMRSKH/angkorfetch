Author: AI Agent
Date: 2026-11-10
Task: TASK-005

# /advisor: Technical Plan and Risks

Analyses a task and recommends the simplest correct approach. Does not edit project files.

**Usage**

```text
/advisor TASK-NNN            plan an existing task and write the plan into its file
/advisor <question>          advice only, no task file (for example, "should X use sysinfo?")
```

**Input**: `$ARGUMENTS` is a Task ID or a free-form question.

## Steps

1. Read [AGENTS.md](../../AGENTS.md) and [Advisor/README.md](../Advisor/README.md).
2. If a Task ID was given, read `team/Tasks/TASK-NNN-*.md`.
3. Inspect the relevant code and history (`git log -- <path>`).
4. Compare up to three approaches: change size, affected platforms, risk.
5. Recommend one, with the exact files and functions to change and the tests that prove it.
6. List risks with mitigations and any approval the maintainer must give.
7. With a Task ID: write the result into **Implementation Plan** and **Dependencies**.
   If a maintainer decision is needed, set `Status: Blocked` and state the question.

## Output

- Recommendation (one approach) and why.
- Files and functions to change.
- Risks and mitigations.
- Test plan.
- Approvals required, or "none".

## Limits

Read-only for everything except the task file. No code, config, or doc edits.
