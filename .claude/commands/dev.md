---
description: AngkorFetch Dev. Implement an existing task's approved plan, or fix its QA findings, with real check results
argument-hint: "TASK-NNN"
disable-model-invocation: true
---

Delegate this to the `dev` subagent with the Agent tool (`subagent_type: dev`).
Tell it to follow `team/Commands/dev.md` with this input:

$ARGUMENTS

When it returns, relay the files changed, the commands it ran with their results, and
anything it could not verify. Do not commit; commits happen in the `/task` flow after QA.
