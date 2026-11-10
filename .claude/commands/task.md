---
description: Run an AngkorFetch task end to end (task record, Advisor, Dev, QA Review, Docs, validation, commit, report)
argument-hint: "<request> | TASK-NNN"
disable-model-invocation: true
---

Follow `team/Commands/task.md` exactly, with this input:

$ARGUMENTS

Claude Code specifics:

- Delegate each role stage with the Agent tool: `subagent_type: advisor` (Step 3),
  `dev` (Step 4), `qa-review` (Step 5), `docs` (Step 6). Pass the Task ID and the task
  file path in every delegation prompt. QA must always run in the `qa-review` subagent.
- Keep orchestration, final validation, commits, and the final report in this main
  session. Subagents do not commit.
- Record in the task file which stages ran in subagents.
