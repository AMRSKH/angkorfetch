Author: AI Agent (Claude Code)
Date: 2026-11-10
Task: TASK-005
Status: Completed

# TASK-005: Role Commands per Tool

## Objective

Provide `/task`, `/advisor`, `/dev`, `/qa-review`, and `/doc` in every configured tool,
in each tool's documented format, with real subagent delegation where the tool supports it.

## Background

Official docs checked on 2026-11-10 by research subagents (sources in Notes) showed:

- Codex, Cursor, and Antigravity all discover `.agents/skills/<name>/SKILL.md` and run it by name.
- OpenCode V2 uses `.opencode/commands/` and `.opencode/agents/`.
- Claude Code supports `.claude/commands/` and `.claude/skills/`; Cursor and OpenCode also
  read `.claude/skills/`, so Claude skills would collide with `.agents/skills/` there.
- Cursor reads `.claude/agents/` as subagents for compatibility.
- Codex custom agents are TOML in `.codex/agents/`.
- Antigravity workflows are deprecated; its subagents are documented for App and CLI only.

## Requirements

- Canonical, tool-neutral command definitions in `team/Commands/`.
- Thin wrappers per tool that point to them; no duplicated procedure text.
- Role subagents with identical names in every tool that supports them.
- Discovery verified where a local CLI exists; limitations documented.

## Scope

`team/Commands/*`, `.claude/commands/*`, `.claude/agents/*`, `.agents/skills/*`,
`.opencode/commands/*`, `.opencode/agents/*`, `.codex/agents/*`, AGENTS.md section 12, this file.

## Out of Scope

Antigravity subagents, `opencode.json`, `.cursor/` files, Codex `agents/openai.yaml`
policies (not needed or not verifiable; see Notes).

## Assigned Roles

- Advisor: Main agent (Claude Code) plus two research subagents for the tool formats.
- Dev: Main agent (Claude Code).
- QA Review: Self-review plus discovery checks below. Independent QA of the whole harness is TASK-006.
- Docs: Main agent (Claude Code).

## Implementation Plan

| Layer | Files | Notes |
|---|---|---|
| Canonical | `team/Commands/{README,task,advisor,dev,qa-review,doc}.md` | Full procedures; `task.md` holds the 9-step flow |
| Claude Code | `.claude/commands/*.md`, `.claude/agents/{advisor,dev,qa-review,docs}.md` | Commands delegate with the Agent tool; agent files are tool-neutral so Cursor can reuse them |
| Shared skills | `.agents/skills/{task,advisor,dev,qa-review,doc}/SKILL.md` | For Codex, Cursor, Antigravity; `name` matches folder |
| OpenCode | `.opencode/commands/*.md`, `.opencode/agents/*.md` | Role commands run as subagents; agents deny `.env` reads |
| Codex | `.codex/agents/*.toml` | `name`, `description`, `developer_instructions` |

Claude Code wrappers use `.claude/commands/` rather than `.claude/skills/` to avoid
duplicate skill names in Cursor and OpenCode.

## Dependencies

TASK-002 (AGENTS.md), TASK-004 (role READMEs).

## Acceptance Criteria

- [x] Five canonical command definitions exist in `team/Commands/` with usage and arguments.
- [x] Every configured tool has a documented location for all five commands.
- [x] Role subagents exist for Claude Code, OpenCode, Cursor (shared), and Codex.
- [x] All Markdown frontmatter parses as YAML; all Codex agent files parse as TOML.
- [x] Discovery verified with Claude Code and OpenCode CLIs.
- [x] Unsupported features and unverified tools are documented.

## Testing Requirements

| Check | Result |
|---|---|
| YAML frontmatter of 23 Markdown wrapper files (PyYAML `safe_load`) | all parse |
| `tomllib.load` on 4 `.codex/agents/*.toml` | all parse, required keys present |
| `claude -p "/advisor ..." --output-format stream-json` (Claude Code 2.1.311) | init event lists slash commands `advisor, dev, doc, qa-review, task` and agents `advisor, dev, docs, qa-review`; the run itself stopped at "Not logged in" |
| `opencode debug agents` (OpenCode 2.2.6) | `advisor`, `dev`, `qa-review`, `docs` listed as `subagent`, each with `read *.env` and `read *.env.*` set to `deny` after the defaults |
| `opencode debug config` | project `.opencode` directory listed as a config source |
| Relative link check over `team/Commands/*`, AGENTS.md, `team/README.md` | recorded in QA Results |

## QA Results

Self-review by the main agent. Link check: all relative links resolve, including the
`team/README.md -> Commands/README.md` link left dangling by TASK-004.

Not verified:

- End-to-end delegation in Claude Code (CLI not logged in; not logged in on the user's behalf).
- OpenCode slash command discovery (no CLI command lists commands).
- Cursor, Codex, Antigravity discovery (no local CLI; based on official docs only).

## Documentation Updates

New: `team/Commands/README.md` and five command definitions. Updated: AGENTS.md section 12.

## Git Commit

- Commit Message: `feat(harness): add role commands and subagents for supported tools [TASK-005]`
- Commit Hash: `3ea7ae6`

## Final Report

All five commands are available in Claude Code, OpenCode, Cursor, Codex, and Antigravity
through one canonical definition each. Real subagent delegation is configured for Claude
Code, OpenCode, Cursor, and Codex.

## Notes and Limitations

- Limitations per tool are listed in `team/Commands/README.md`.
- OpenCode agent `permissions` were limited to the `.env` deny rules, the only form whose
  effect was confirmed with `opencode debug agents`. Edit restrictions for Advisor and QA
  rely on instructions.
- Sources: https://code.claude.com/docs/en/skills.md, https://code.claude.com/docs/en/sub-agents.md,
  https://opencode.ai/docs/commands/, https://opencode.ai/docs/agents/, https://cursor.com/docs/skills,
  https://cursor.com/docs/subagents, https://learn.chatgpt.com/docs/build-skills,
  https://learn.chatgpt.com/docs/agent-configuration/subagents, https://antigravity.google/docs/skills/,
  https://antigravity.google/docs/ide/workflows/, https://antigravity.google/docs/subagents/
