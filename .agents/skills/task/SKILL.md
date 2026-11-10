---
name: task
description: Run an AngkorFetch development task end to end (task record, Advisor, Dev, QA Review, Docs, validation, commit, report). Use only when the user explicitly invokes this command.
disable-model-invocation: true
---

Follow `team/Commands/task.md` exactly. The input is the text the user gave with this
command: a request in plain words, or an existing Task ID such as `TASK-007`.

If this tool can start subagents, delegate the Advisor, Dev, QA Review, and Docs stages
to the agents named `advisor`, `dev`, `qa-review`, and `docs` when they are defined for
this tool. Otherwise run the stages yourself in order and write in the task file that no
independent agent was used.
