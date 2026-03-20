# /development — Load Current Work State

## Purpose

Load the current development state -- what phase we're in, what's done, what's next, what's blocking. Use at the start of a session or when resuming after a break.

## Usage

```
/development
```

## What Happens

1. Read `{ROADMAP_PATH}` - current phase and progress
2. Read `{ACTIVE_CONTEXT_PATH}` - any active task specs, issues, or work-in-progress notes
3. Read `{PENDING_WORK_PATH}` - pending items (commits, PRs, reviews)
4. Scan source and test directories - what code exists
5. Check build state:
   - Does the project build successfully?
   - Do tests pass?
   - Any build warnings or errors?
6. Check git state:
   - `git status --short` - uncommitted changes
   - `git log --oneline -5` - recent commits
   - Any stashed work?

## Output

```
# Development State

Phase: [N] - [Phase Name] [IN PROGRESS / COMPLETE]
Progress: [X/Y] criteria met

## What Exists
- Source files: [summary or count]
- Test files: [summary or count]
- Key modules: [list of main components]

## Recent Activity
- Last commit: [hash] [message]
- Uncommitted changes: [list or "none"]

## Current Focus
- [Active task or next unchecked roadmap criterion]

## Blockers
- [Any blocking issues, or "None"]

## Pending
- [Pending commits, PRs, reviews, or "None"]

## Suggested Next Steps
1. [Most logical next action based on roadmap + current state]
2. [Alternative if blocked]
```

## When to Use

- Starting a new work session
- Resuming after a break
- Need orientation on where we are
- Before deciding what to work on next

## What This Is NOT

- Not a task executor (use implementation commands for that)
- Not a roadmap updater (use `/roadmap update` for that)
- Not a quality checker (use `/audit` for that)

This command is purely informational: it reads state and reports it.

## Tool Permissions

- **Read**: All project docs, source code, git state
- **Execute**: Build commands and test runners (for status checks only)
- **Write**: None (this is a read-only orientation command)

---

**Command Type**: Session Orientation
