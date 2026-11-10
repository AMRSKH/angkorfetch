Author: AI Agent (Claude Code)
Date: 2026-11-10
Task: TASK-002
Status: Completed

# TASK-002: Central Agent Instructions

## Objective

Create `AGENTS.md` as the single source of truth for every AI coding tool, and add only
the tool entry points that a tool needs in order to load it.

## Background

No AI instruction file existed. Project rules were spread across CONTRIBUTING.md and
`docs/`. Tools in use: Claude Code, Codex, OpenCode, Cursor, Antigravity (TASK-001).

## Requirements

- AGENTS.md covers: overview, stack, architecture, setup, dev/build/test/format/lint
  commands, coding standards, naming, error handling, security, Git workflow, task
  management, Definition of Done, documentation requirements.
- Every command and convention in it exists in this repository.
- Tool entry files reuse AGENTS.md instead of duplicating it.

## Scope

`AGENTS.md`, `CLAUDE.md`, this file.

## Out of Scope

Hook and permission sections (TASK-003), team section (TASK-004), commands and
subagents (TASK-005).

## Assigned Roles

- Advisor: Main agent (Claude Code), with two research subagents that checked each tool's official docs for file locations.
- Dev: Main agent (Claude Code).
- QA Review: Self-review. Independent QA of the whole harness is TASK-006.
- Docs: Main agent (Claude Code).

## Implementation Plan

1. Write AGENTS.md from the existing docs and the code (`src/main.rs` structure, zero
   `.unwrap()` calls, `run_stdout` / `run_powershell` patterns, installer `set -euo pipefail`
   and checksum verification, CI gates).
2. Link to CONTRIBUTING.md and `docs/` instead of copying them.
3. `CLAUDE.md`: one `@AGENTS.md` import plus a comment. Claude Code reads `AGENTS.md`
   only when no `CLAUDE.md` exists, so the import keeps it loaded either way.
4. No entry files for Codex, OpenCode, Cursor, Antigravity: all read root `AGENTS.md`
   natively per their docs (sources below).

## Dependencies

TASK-001.

## Acceptance Criteria

- [x] AGENTS.md contains every required section.
- [x] Every command listed in AGENTS.md section 5 was run successfully during TASK-001 inspection.
- [x] Claims about the code were checked against `src/main.rs`, `get.sh`, `get.ps1`, and the workflows.
- [x] CLAUDE.md is a thin import, not a copy.
- [x] AGENTS.md is under Codex's 32 KiB limit.

## Testing Requirements

Documentation only. Checked: AGENTS.md size, relative links resolve, commands match the
TASK-001 baseline run.

## QA Results

Self-review by the main agent. Sources used for tool discovery rules:

| Tool | Source |
|---|---|
| Claude Code | https://code.claude.com/docs/en/memory.md |
| Codex | https://learn.chatgpt.com/docs/agent-configuration/agents-md |
| OpenCode | https://opencode.ai/docs/instructions/ , https://opencode.ai/docs/migrate-v1/ |
| Cursor | https://cursor.com/docs/rules |
| Antigravity | https://antigravity.google/docs/rules/ |

Not verified by running each tool: discovery is taken from the docs above. TASK-006
verifies discovery where a local CLI exists.

## Documentation Updates

New: `AGENTS.md`, `CLAUDE.md`.

## Git Commit

- Commit Message: `docs(agents): add central AGENTS.md and Claude Code entry point [TASK-002]`
- Commit Hash: `568880a`

## Final Report

All five tools load the same instructions from AGENTS.md; only Claude Code needs `CLAUDE.md`.

## Notes and Limitations

- If someone adds a `CLAUDE.local.md` without `@AGENTS.md`, Claude Code stops reading AGENTS.md.
- Antigravity reads both `AGENTS.md` and `GEMINI.md` with no documented precedence; do not add a `GEMINI.md` to this repo.
