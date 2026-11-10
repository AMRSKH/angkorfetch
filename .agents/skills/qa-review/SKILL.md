---
name: qa-review
description: AngkorFetch QA Review. Review a task's uncommitted changes, re-run checks, and return a verdict with severity-ranked findings. Use only when the user explicitly invokes this command.
disable-model-invocation: true
---

Follow `team/Commands/qa-review.md` exactly. The input is the Task ID the user gave with
this command, or nothing (then review the current uncommitted diff).

If this tool can start a subagent named `qa-review`, run the review there so it is
independent of the code author. Otherwise state in QA Results that the review was not
independent.
