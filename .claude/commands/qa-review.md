---
description: AngkorFetch QA Review. Independently review a task's uncommitted changes and return a verdict with severity-ranked findings
argument-hint: "[TASK-NNN]"
disable-model-invocation: true
---

Delegate this to the `qa-review` subagent with the Agent tool (`subagent_type: qa-review`)
so the review runs in a fresh context. Tell it to follow `team/Commands/qa-review.md` with
this input (may be empty):

$ARGUMENTS

When it returns, relay the verdict, the findings by severity, the commands it ran, and
what it could not verify. Do not fix the findings in this command.
