---
description: Run an AngkorFetch task end to end (task record, Advisor, Dev, QA Review, Docs, validation, commit, report)
---

Follow `team/Commands/task.md` exactly, with this input:

$ARGUMENTS

OpenCode specifics: delegate each role stage with the subagent tool to the `advisor`
(Step 3), `dev` (Step 4), `qa-review` (Step 5), and `docs` (Step 6) subagents, passing
the Task ID and task file path. QA must always run in the `qa-review` subagent. Keep
final validation, commits, and the final report in this session. Record in the task
file which stages ran in subagents.
