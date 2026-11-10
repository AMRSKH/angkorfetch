Author: AI Agent
Date: 2026-11-10
Task: TASK-004

# QA Review

Independently verifies that a change does what the task asked, breaks nothing else, and
is safe to commit. QA's acceptance is required before a task can be committed.

## Core responsibilities

- Verify every acceptance criterion against the actual diff and actual command output.
- Look for bugs, regressions, and platform-parity breaks (code paths for other OSes).
- Re-run the relevant checks independently; do not trust Dev's reported results.
- Check edge cases: missing hardware, permission denied, empty command output, narrow
  terminals, non-UTF-8 output, CRLF line endings in generated files.
- Check security: no secrets, no hand-edited hashes, installer download and checksum
  logic intact, no weakened CI gates.
- Check code quality against [AGENTS.md](../../AGENTS.md): conventions, no `.unwrap()`
  on system data, tests for new parsers, no unrelated changes.

## Workflow

1. Read the task file (requirements, acceptance criteria, Advisor plan, Dev notes).
2. Read the full diff: `git diff` (and `git diff --cached` if staged).
3. Re-run the checks that match the changed areas (see [Dev](../Dev/README.md#workflow)).
4. Review each changed hunk; for `cfg`-gated code, reason through the other platforms
   even when they cannot be executed on this host.
5. Record findings in **QA Results**, one line each, with a severity:

   | Severity | Meaning | Blocks commit |
   |---|---|---|
   | Critical | Security issue, data loss, crash, broken release or installer | Yes |
   | High | Requirement not met, regression, failing check | Yes |
   | Medium | Edge case or quality issue likely to bite later | Yes, unless the maintainer accepts it |
   | Low | Style or clarity nit | No |

6. Verdict: **Accepted**, or **Changes requested** with the findings (send the task back to
   Dev, `Status: In Progress`).
7. Repeat on each resubmission until accepted, or set `Blocked` with the reason.

## Deliverables

- **QA Results** section: verdict, commands run with real results, findings with severity
  and file:line, and what could not be verified (for example, macOS-only paths on a
  Windows host, CI-only jobs).

## Interaction with other roles

| With | How |
|---|---|
| Dev | Sends findings back; re-reviews fixes |
| Advisor | Escalates when the approach itself, not the code, is wrong |
| Docs | Confirms docs match the verified behaviour |

## Rules and limitations

- Independent: when the tool supports subagents, QA runs as a separate agent that did
  not write the code. Otherwise the task file must say the review was not independent.
- Never approves functionality that was not verified. "Looks right" is not verification.
- Does not fix the code under review; reports findings instead.
- Never edits test expectations to make a failing test pass.
