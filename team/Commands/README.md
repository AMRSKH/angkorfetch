Author: AI Agent
Date: 2026-11-10
Task: TASK-005

# Commands

The files in this folder are the **canonical, tool-neutral definitions** of the harness
commands. Every tool-specific command file is a thin wrapper that says "follow
`team/Commands/<name>.md`" and adds only what that tool needs (argument placeholder,
how to delegate). Change behaviour here, not in the wrappers.

| Command | Definition | Purpose |
|---|---|---|
| `/task` | [task.md](task.md) | Main entry point: full task lifecycle from request to commit and report |
| `/advisor` | [advisor.md](advisor.md) | Technical plan, risks, test plan |
| `/dev` | [dev.md](dev.md) | Implementation and checks |
| `/qa-review` | [qa-review.md](qa-review.md) | Independent verification and verdict |
| `/doc` | [doc.md](doc.md) | Documentation updates |

## Where each tool finds them

| Tool | Commands | Invoke | Role subagents | Discovery verified |
|---|---|---|---|---|
| Claude Code | `.claude/commands/*.md` | `/task ...` | `.claude/agents/*.md` | Yes: `claude -p` init lists all 5 commands and 4 agents (CLI 2.1.311) |
| OpenCode | `.opencode/commands/*.md` | `/task ...` | `.opencode/agents/*.md` (`mode: subagent`) | Agents yes (`opencode debug agents`, v2.2.6); commands no (no CLI lists them) |
| Cursor | `.agents/skills/*/SKILL.md` | `/task ...` | `.claude/agents/*.md` (Cursor reads `.claude/agents/` for compatibility) | No local CLI; from Cursor docs |
| Codex | `.agents/skills/*/SKILL.md` | `$task ...` or `/skills` | `.codex/agents/*.toml` | No local CLI; from Codex docs |
| Antigravity | `.agents/skills/*/SKILL.md` | `/task ...` | Not configured (see limitations) | No local CLI; from Antigravity docs |

Role agent names are the same everywhere: `advisor`, `dev`, `qa-review`, `docs`.

## Delegation behaviour

| Tool | `/task` stages | `/advisor`, `/dev`, `/qa-review`, `/doc` |
|---|---|---|
| Claude Code | Delegated to the role subagents with the Agent tool | Delegated to the matching subagent |
| OpenCode | Delegated with the subagent tool | Run as the matching subagent (`subagent: true`) |
| Cursor | The skill asks the agent to delegate to the shared subagents | Run in the current agent |
| Codex | The skill asks Codex to delegate to the custom agents | Run in the current agent |
| Antigravity | Sequential in one agent | Run in the current agent |

When a stage does not run in a separate agent, the task file must say so.

## Limitations

- **Argument passing**: Claude Code and OpenCode substitute `$ARGUMENTS`. Skills (Cursor,
  Codex, Antigravity) receive the user's text as part of the message, so the skill
  tells the agent to treat that text as the input.
- **Antigravity subagents** (`.agents/agents/`) are documented for the Antigravity App
  and CLI only, not the IDE, so none are configured. Antigravity workflows
  (`.agents/workflows/`) are deprecated in favour of skills, so none are used.
- **Codex** custom prompts (`~/.codex/prompts/`) are deprecated and user-level only, so
  skills are used instead. Project `.codex/` files load only for trusted projects.
- **Cursor** `.cursor/commands/` is superseded by skills; none are created. Cursor
  subagents come from `.claude/agents/`; if `.cursor/agents/` is ever added, files there
  win on name conflicts.
- **OpenCode** also loads `.agents/skills/` as model-loadable skills, next to its own
  `/task` commands. The skills point to the same definitions, so both paths behave the same.
- **`disable-model-invocation: true`** is honoured by Claude Code and Cursor. Other tools
  ignore it; the skill descriptions say to use them only on explicit request.
- **Permissions**: Claude Code (`.claude/settings.json`) and OpenCode agents
  (`permissions` deny `.env` reads) enforce part of AGENTS.md section 7. The other tools
  rely on their own approval prompts and the written rules.
