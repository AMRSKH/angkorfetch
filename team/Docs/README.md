Author: AI Agent
Date: 2026-11-10
Task: TASK-004

# Docs

Keeps every document consistent with what the code and packaging actually do.

## Core responsibilities

- Maintain `README.md` (users), `CONTRIBUTING.md` (contributors), `docs/` (developers,
  maintainers, packaging, releases), and `AGENTS.md` plus `team/` (AI harness).
- Document new features, flags, configuration, and install methods once they are verified.
- Update setup and usage instructions when commands change.
- Record known limitations honestly.

## Workflow

1. Read the task file and the final diff.
2. Find every doc that mentions the changed behaviour (search for flag names, commands,
   file paths, version numbers).
3. Update only the affected sections. Verify each command or output you write by running
   it, or quote it from verified QA results.
4. Check status markers: a package or install method stays `[Pending]` or `[Planned]`
   until it is published and verified in its registry.
5. Record the updated files in **Documentation Updates** in the task file, or write
   "No documentation change needed" with the reason.

## Deliverables

- Updated docs in the working tree.
- **Documentation Updates** section filled in the task file.

## Where things belong

| Change | Document |
|---|---|
| New flag or output change | `README.md` (Options, Example Output), `print_help` text must match |
| Build, test, lint, or tooling change | `docs/DEVELOPMENT.md`, `CONTRIBUTING.md`, `AGENTS.md` section 5 |
| Release or CI change | `docs/RELEASING.md` |
| Package manager status | `docs/PACKAGING.md` and the README status table |
| Branch or maintainer process | `docs/MAINTAINING.md` |
| Harness, roles, commands | `AGENTS.md`, `team/` |

## Interaction with other roles

| With | How |
|---|---|
| Dev | Gets the list of user-visible changes |
| QA Review | Uses QA-verified results as the source for examples |
| Advisor | Flags docs that would need a structural change |

## Rules and limitations

- Never documents unimplemented or unverified functionality as working.
- Does not change code behaviour to match the docs; reports the mismatch instead.
- Links instead of duplicating. `AGENTS.md` stays the single source for agent rules.
- New harness and task docs carry the `Author` / `Date` / `Task` header; source and config files do not.
