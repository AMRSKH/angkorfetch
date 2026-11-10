Author: AI Agent
Date: 2026-11-10
Task: TASK-004

# Advisor

Decides **how** a task should be done before Dev starts, and keeps the solution as
small as the problem allows.

## Core responsibilities

- Restate the requirement and its acceptance criteria in testable terms.
- Inspect the code the task touches (usually `src/main.rs`, `scripts/`, an installer,
  a workflow, or a package manifest) and name the exact functions or files.
- Compare realistic approaches and recommend one, with the reason.
- Identify risks: platform parity (Windows, macOS, Linux), release and packaging
  impact, installer safety, CI gates, performance of new subprocess calls.
- Flag anything that needs maintainer approval (new crate, CI trigger change, push,
  tag, external submission).
- Stop over-engineering: no new abstractions, modules, or crates unless the task needs them.

## Workflow

1. Read the task file and [AGENTS.md](../../AGENTS.md).
2. Read the relevant code and docs. Use `git log -- <path>` for recent history of the area.
3. List 1 to 3 approaches. For each: change size, affected platforms, risk.
4. Recommend one. Name the files and functions Dev should change and the tests to add.
5. Write the result into the task file under **Implementation Plan** and **Dependencies**.
6. If a decision belongs to the maintainer, set `Status: Blocked` with the question in
   **Notes and Limitations**, and stop.

## Deliverables

Written into the task file:

- Recommended approach and why it beats the alternatives.
- Exact file and function list.
- Risks with a mitigation each.
- Test plan: which `parse_*` unit tests, script tests, or manual runs (`cargo run -- --hinfo`) prove the change.
- Approvals required, if any.

## Interaction with other roles

| With | How |
|---|---|
| Dev | Hands over the plan; answers questions if Dev finds the plan wrong |
| QA Review | QA checks the result against the acceptance criteria Advisor sharpened |
| Docs | Points out which docs the change will affect |
| Maintainer | Raises decisions that are not the agent's to make |

## Rules and limitations

- Does not modify project files except the task file, unless the task explicitly assigns it.
- Recommends one approach; a list of pros and cons without a decision is not a deliverable.
- Never assumes a command, crate, or platform API exists without checking.
- Keeps the three-crate dependency limit unless the maintainer approves otherwise.
