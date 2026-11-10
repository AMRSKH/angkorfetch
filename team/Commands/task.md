Author: AI Agent
Date: 2026-11-10
Task: TASK-005

# /task: Run a Task End to End

Main entry point for development work. Takes a request, creates its task record, and
drives it through Advisor, Dev, QA Review, Docs, validation, and commit.

**Usage**

```text
/task <request in plain words>
/task TASK-NNN            resume an existing task from its recorded status
```

Examples: `/task Fix battery detection on Linux laptops with BAT1`,
`/task Add a --json output flag`, `/task Audit the installers for security issues`.

**Input**: `$ARGUMENTS` is the request, or an existing Task ID to resume.

Read [AGENTS.md](../../AGENTS.md) and [team/README.md](../README.md) before starting.

## Step 1: Understand

- Restate the request in one or two sentences.
- Run `git branch --show-current` and `git status --short`. Note both; Step 8 must leave
  any pre-existing changes untouched. If the branch is `main` or `dev`, create a topic
  branch `<type>/<short-description>` before editing, unless the maintainer said to work
  on the current branch.
- Inspect only the files relevant to it. Never open `.env` or secret files.
- Identify affected platforms, packaging or release impact, and risks.
- If the request is ambiguous in a way that changes the result, or needs an approval
  listed in AGENTS.md section 7, ask the maintainer before going further.
- If the request is trivial (see [Tasks/README.md](../Tasks/README.md#when-a-task-file-is-required)),
  do it directly without a task file, commit it without a Task ID, and say so.

## Step 2: Create the task record

- Resuming (`TASK-NNN` given): open that file, skip creating, and continue at the first
  row below that matches:

  | Task file state | Resume at |
  |---|---|
  | `Status: Completed` | Nothing to do; report the recorded result |
  | `Status: Blocked` | Report the blocker; continue only after the maintainer answers it |
  | Implementation Plan empty | Step 3 (Advisor) |
  | `Status: In Progress` | Step 4 (Dev); open findings of the newest QA round come first |
  | `In Review`, and no QA round yet, or the newest round is a code round (plain `Verdict:`) and it is not `Accepted` | Step 5 (QA Review) |
  | Newest `Verdict:` is `Accepted`, Documentation Updates empty | Step 6 (Docs) |
  | Docs changed files, newest `Verdict (docs):` line missing or not `Accepted` | Step 6, documentation re-check |
  | Docs accepted or unchanged, `Commit Hash` empty, no commit in `git log --grep "TASK-NNN"` | Step 7 (Final validation) |
  | Work commit exists in `git log --grep "TASK-NNN"` but hash not recorded | Step 8.3 (close commit) |

  QA Results holds one `### Round N` subsection per review, **newest first**. Each round's
  first line is exactly `Verdict: Accepted` or `Verdict: Changes requested`
  (`Verdict (docs): ...` for a documentation re-check). Only the newest round counts.

- Otherwise list `team/Tasks/`, take the highest `TASK-NNN` and add one.
- Create `team/Tasks/TASK-NNN-short-kebab-description.md` from `task_template.md`.
  Never overwrite an existing file.
- Fill Objective, Background, Requirements, Scope, Out of Scope, Acceptance Criteria.
- Set `Status: In Progress`, `Author` to your tool name, `Date` to today.

## Step 3: Advisor

Delegate to the **advisor** agent if your tool supports subagents; otherwise follow
[advisor.md](advisor.md) yourself. Output goes into the task file (Implementation Plan,
Dependencies, risks). If Advisor sets `Blocked`, stop and report the question.

## Step 4: Dev

Delegate to the **dev** agent, or follow [dev.md](dev.md). Dev implements, runs the
matching checks, records real results, and sets `Status: In Review`.

## Step 5: QA Review

Delegate to the **qa-review** agent so the reviewer did not write the code, or follow
[qa-review.md](qa-review.md) and record that the review was not independent.

- **Changes requested**: send the findings to Dev (Step 4), then QA again. Repeat until
  accepted. After three rejected rounds, set `Blocked` and report to the maintainer.
- Never skip or override QA to finish a task.

## Step 6: Docs

Delegate to the **docs** agent, or follow [doc.md](doc.md). Update only documentation the
change affects, or record why none was needed.

If Docs changed any file, run QA Review again on the documentation diff only before
Step 7, so no change reaches a commit unreviewed. This matters most for `AGENTS.md` and
`team/`, which hold the agent and security rules. QA records it as a new round with
`Verdict (docs): ...`. A rejection goes back to Docs (this step), not to Dev; the
three-round limit from Step 5 applies.

## Step 7: Final validation

Run the full validation from AGENTS.md section 5 for the areas the task touched, then:

```bash
git status
git diff
```

Confirm: every acceptance criterion is checked, no unrelated files changed, no secrets.
If any check fails, return to Step 4. Do not commit a failing state.

## Step 8: Commit

1. Stage the task's files by path, including the task file.
2. Commit: `type(scope): summary [TASK-NNN]` (Conventional Commits, see AGENTS.md section 8).
   The pre-commit hook runs; if it fails, fix and retry. Never use `--no-verify` without
   maintainer approval.
3. Record the hash: set `Commit Message`, `Commit Hash`, `Status: Completed` in the task
   file, then commit only that file as `chore(tasks): close TASK-NNN [TASK-NNN]`.
4. Run `git status` and confirm the tree is clean apart from changes that were already
   there before the task. Do not push.

## Step 9: Final report

Reply to the maintainer with:

| Item | Content |
|---|---|
| Task | ID and title |
| What changed | Short implementation summary |
| Files | Changed files |
| QA | Verdict, findings fixed, independent or not |
| Validation | Commands run and results |
| Commits | Work commit hash, close commit hash |
| Limitations | What was not verified and why, follow-ups |
