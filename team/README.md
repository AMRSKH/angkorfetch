Author: AI Agent
Date: 2026-11-10
Task: TASK-004

# Team Structure

AngkorFetch work is split into four roles. Any AI agent (or human) can play any role,
but one role at a time, with the task file in [Tasks/](Tasks/README.md) as the shared
state. Project rules for every role are in [AGENTS.md](../AGENTS.md).

| Role | Folder | One-line purpose |
|---|---|---|
| Advisor | [Advisor/](Advisor/README.md) | Decide the simplest correct approach before code is written |
| Dev | [Dev/](Dev/README.md) | Implement the approved approach and prove it with checks |
| QA Review | [QA-Review/](QA-Review/README.md) | Independently verify the result and accept or reject it |
| Docs | [Docs/](Docs/README.md) | Keep documentation true to the implementation |

[Commands/](Commands/README.md) holds the tool-neutral definition of each command
(`/task`, `/advisor`, `/dev`, `/qa-review`, `/doc`). [Tasks/](Tasks/README.md) holds one
record per task.

## Workflow

```text
/task <request>
  1. Understand        read request and relevant files, decide if clarification is needed
  2. Task record       next Task ID, new file from template, Status: In Progress
  3. Advisor           approach, risks, affected platforms        -> task file
  4. Dev               implement, run checks                       -> task file
  5. QA Review         independent verification                    -> task file
        reject -> back to 4 (Status: In Progress), repeat until accepted or Blocked
  6. Docs              update docs that the change affects         -> task file
  7. Final validation  full validation from AGENTS.md, review git diff
  8. Commit            work commit [TASK-NNN], then close commit with the hash
  9. Final report      summary to the maintainer
```

Each step appends to the task file before the next one starts, so any tool can pick up
a task where another stopped.

## Delegation

- When the tool supports subagents, `/task` delegates each stage to the matching role
  agent so QA Review runs in a fresh context that did not write the code.
- When it does not, the same agent runs the stages in order and the task file states
  that no independent agent was used. Never claim delegation that did not happen.
- Which tools support what is listed in [Commands/README.md](Commands/README.md).

## Shared rules

- The task file is the record. If it is not in the task file, it did not happen.
- Stay in role: Advisor does not edit code, QA Review does not fix what it reviews, Docs
  does not change behaviour.
- Approval rules in [AGENTS.md](../AGENTS.md#7-security-requirements) apply to every role.
