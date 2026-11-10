Author: AI Agent
Date: 2026-11-10
Task: TASK-005

# /doc: Update Documentation

Brings documentation in line with a task's verified changes.

**Usage**

```text
/doc TASK-NNN                update docs affected by that task
/doc <what to document>      docs-only work; creates a task record first if non-trivial
```

**Input**: `$ARGUMENTS` is a Task ID or a description.

## Steps

1. Read [AGENTS.md](../../AGENTS.md), [Docs/README.md](../Docs/README.md), and the task
   file (especially QA Results).
2. Search the docs for every mention of what changed: flags, commands, paths, versions,
   package statuses.
3. Update only the affected sections. Verify every command and output you write.
4. Keep `[Pending]` / `[Planned]` markers until a package is published and verified.
5. Record changed docs in **Documentation Updates**, or "No documentation change needed"
   with the reason.

## Output

Updated docs and the filled **Documentation Updates** section.

## Limits

No code or behaviour changes. Never documents unverified functionality as working.
