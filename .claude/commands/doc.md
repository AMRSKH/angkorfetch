---
description: AngkorFetch Docs. Update documentation affected by a task's verified changes, or do docs-only work
argument-hint: "TASK-NNN | <what to document>"
disable-model-invocation: true
---

Delegate this to the `docs` subagent with the Agent tool (`subagent_type: docs`).
Tell it to follow `team/Commands/doc.md` with this input:

$ARGUMENTS

When it returns, relay the documents it changed, or why no change was needed.
