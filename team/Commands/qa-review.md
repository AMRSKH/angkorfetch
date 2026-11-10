Author: AI Agent
Date: 2026-11-10
Task: TASK-005

# /qa-review: Independent Verification

Reviews a task's changes, re-runs its checks, and returns a verdict with findings.

**Usage**

```text
/qa-review TASK-NNN          review the working-tree changes for that task
/qa-review                   review the current uncommitted diff without a task record
```

**Input**: `$ARGUMENTS` is a Task ID, or empty.

## Steps

1. Read [AGENTS.md](../../AGENTS.md), [QA-Review/README.md](../QA-Review/README.md), and the
   task file if given.
2. Read the full diff: `git status`, `git diff`, `git diff --cached`.
3. Re-run the checks for the changed areas yourself. Do not rely on Dev's reported results.
4. Check each acceptance criterion against the diff and the output you just produced.
5. Review for regressions, platform parity, edge cases, security, conventions, and
   unrelated changes (checklist in [QA-Review/README.md](../QA-Review/README.md)).
6. Add a new `### Round N` subsection at the **top** of **QA Results** (newest first).
   Its first line is exactly `Verdict: Accepted` or `Verdict: Changes requested`, or
   `Verdict (docs): ...` for a documentation-only re-check; the `/task` resume table
   reads the newest one. Then: commands and results, findings as
   `Severity | file:line | finding`, and what could not be verified.
7. Verdict:
   - **Accepted**: leave `Status: In Review` for the `/task` flow to commit.
   - **Changes requested** (code review): set `Status: In Progress` so Dev picks up the findings.
   - **`Verdict (docs): Changes requested`**: leave `Status: In Review`; the `/task` flow
     sends the findings back to Docs, not Dev.

## Output

Verdict, findings by severity, commands run with results, unverified areas.

## Limits

Does not modify code under review. Never accepts unverified behaviour. Never edits
tests to make them pass. Must state whether the review ran in an independent agent.
