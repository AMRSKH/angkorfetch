Author: AI Agent (Claude Code)
Date: 2026-11-10
Task: TASK-004
Status: Completed

# TASK-004: Define Team Roles

## Objective

Define the four harness roles (Advisor, Dev, QA Review, Docs) and the shared workflow
so any AI tool or human can take a role with the same expectations.

## Background

The harness needs role definitions that reflect AngkorFetch specifics: a single-file
Rust CLI with per-OS `cfg` code, generated package manifests, security-sensitive
installers, and CI gates.

## Requirements

- `team/README.md` with the role overview, workflow, and delegation rules.
- One README per role with: core responsibilities, workflow, deliverables, interaction
  with other roles, rules and limitations.
- AGENTS.md summarises team responsibilities and links to the role files.

## Scope

`team/README.md`, `team/Advisor/README.md`, `team/Dev/README.md`,
`team/QA-Review/README.md`, `team/Docs/README.md`, AGENTS.md team section, this file.

## Out of Scope

Command definitions and tool wrappers (TASK-005).

## Assigned Roles

- Advisor: Main agent (Claude Code). No separate advisor agent was used.
- Dev: Main agent (Claude Code).
- QA Review: Self-review. Independent QA of the whole harness is TASK-006.
- Docs: Main agent (Claude Code).

## Implementation Plan

1. Write `team/README.md`: role table, nine-step workflow, delegation rule, shared rules.
2. Write each role README with project-specific checks (check matrix per changed area,
   QA severity scale, doc placement table).
3. Use `team/QA-Review/` instead of `team/QA Review/` (approved in TASK-001).
4. Add a team responsibilities section to AGENTS.md.

## Dependencies

TASK-001 (task tracking), TASK-002 (AGENTS.md).

## Acceptance Criteria

- [x] `team/README.md` exists with roles, workflow, and delegation rules.
- [x] Each role README has all five required sections.
- [x] Role checks reference only commands that exist in this repository.
- [x] AGENTS.md links to the team structure.

## Testing Requirements

Documentation only. Verify every relative link in `team/` resolves to an existing file
(checked with a script listing Markdown links and testing each target path).

## QA Results

Self-review by the main agent. Link check over AGENTS.md, `team/README.md`, the four
role READMEs, and this file: all relative links resolve except
`team/README.md -> Commands/README.md` (two occurrences), which TASK-005 creates.
Commands in the Dev check matrix ran in TASK-001, or in TASK-006 for the release build and `cargo run -- --hinfo` (corrected in TASK-006).

**Process deviation (recorded in TASK-006):** this task was marked `Completed` after
self-review only, before any independent QA ran. That breaks the Definition of Done.
History was not rewritten. Independent QA of this task runs in TASK-006; its QA
Results hold the verdict and how each finding was resolved.

## Documentation Updates

New: `team/README.md` and four role READMEs. Updated: AGENTS.md (team section).

## Git Commit

- Commit Message: `docs(team): define team roles and workflow [TASK-004]`
- Commit Hash: `6c3fc9e`

## Final Report

Roles and the shared workflow are defined. `team/Commands/` references are completed by TASK-005.

## Notes and Limitations

`team/README.md` links to `team/Commands/README.md`, which is created in TASK-005; that
link is dangling between the TASK-004 and TASK-005 commits.
