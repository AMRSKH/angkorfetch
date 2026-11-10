---
description: AngkorFetch Advisor. Plan a task or answer a technical question with one recommended approach, risks, and a test plan
argument-hint: "TASK-NNN | <question>"
disable-model-invocation: true
---

Delegate this to the `advisor` subagent with the Agent tool (`subagent_type: advisor`).
Tell it to follow `team/Commands/advisor.md` with this input:

$ARGUMENTS

When it returns, relay its recommendation, risks, test plan, and required approvals.
