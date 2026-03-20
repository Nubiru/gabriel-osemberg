# Agent Bootstrap -- Multi-Session Autonomous Pipeline

You are an agent on this project. You operate autonomously using a self-governing loop with Writer + Checker + Maintainer subagent separation.

**Your command**: `/execute {YOUR_SLOT}` (ALPHA, BETA, GAMMA, ...)

**This project runs N parallel Claude sessions.** See `.context/execute/PROTOCOL.md` for coordination rules.

---

## How You Work

You are NOT given micro-tasks. You **find your own work** from the orchestrator's roadmap, **claim it** (preventing other sessions from duplicating), **design your own modules** using TDD, **validate independently** via Checker subagent, and **document the results**.

The MEGA session (primary Claude) provides macro-level direction in `docs/ORCHESTRATOR_ROADMAP.md`. You provide micro-level expansion: API design, test planning, implementation, validation, documentation.

---

## Your Self-Governing Loop

```
LOOP:
  1. EVALUATE  -- scan roadmap, check all claims, select unclaimed work
  2. PLAN      -- write task.md (this IS your claim marker)
  3. EXECUTE   -- launch Writer subagent (TDD: tests -> interface -> implementation)
  4. VALIDATE  -- launch Checker subagent (compile, test, quality, standards)
  5. MAINTAIN  -- launch Maintainer subagent (regression, quality, cleanup, metrics)
  6. DOCUMENT  -- write report.md + {BUILD_ADDITIONS_FILE} + maintenance.md
  GOTO 1 (immediately -- no waiting, no asking permission)
```

Exit only when: no work remains, 3+ tasks completed, or unrecoverable blocker.

Full protocol: `.claude/commands/execute.md`

---

## Claim Protocol (MANDATORY)

Before starting any work:
1. Read ALL `.context/execute/*/task.md` files
2. If another slot has claimed the item you want, pick a different item
3. Write your task.md FIRST -- this is your claim
4. Only then begin Phase 3

Files are the lock. First to write task.md wins.

---

## Writer + Checker + Maintainer Separation

**Phase 3 (EXECUTE)**: Launch a Writer subagent. It writes tests, interface, implementation, compiles, runs tests.

**Phase 4 (VALIDATE)**: Launch a SEPARATE Checker subagent. It independently compiles, runs tests, checks quality ({QUALITY_CHECK_RULES}), checks naming/style, checks for duplication. Returns PASS/FAIL.

**Phase 5 (MAINTAIN)**: Launch a SEPARATE Maintainer subagent. It runs regression tests, re-checks quality on all touched files, scans for dead code, naked TODOs, duplication, and missing attribution. Returns PASS/WARN/CRITICAL. See `.context/standards/QUALITY_GATES.md` for gate definitions.

**Phase 5 also includes PROJECT HEALTH** (every task, not just per-file):
- Count total tests + suites from `{TEST_COMMAND}` output -> write to `.context/execute/{SLOT}/maintenance.md`
- Verify new source files are registered in the build system
- Check `{CONTRIBUTORS_FILE}` for any new external algorithm sources used

Three perspectives. Writer creates, Checker validates, Maintainer sweeps.

---

## Research Task Type

When a roadmap item requires research before implementation:

1. Check `.context/library/manifest.json` for relevant materials
2. If material exists: Read relevant sections, extract to `.context/research/digested/NNN-topic.md`
3. If open web data available: WebFetch directly
4. If paywalled: write `.context/execute/{SLOT}/escalation.md` and move to next task
5. Use digested data for normal TDD flow

Research produces a digest file AND a code module. Update `{CONTRIBUTORS_FILE}` for all sources.

---

## Universal Rules

1. **Read SOUL.md + CLAUDE.md first** -- Every session, every agent, every time
2. **Read interfaces only** -- When researching existing code, read interface/header files, not implementation files (unless debugging)
3. **Quality standards** -- Follow all rules defined in {QUALITY_CHECK_RULES}
4. **Strict compilation** -- `{COMPILE_COMMAND}` -- zero warnings, zero failures
5. **Git after Maintainer PASS** -- Commit + push your files in Phase 6. NEVER `git add -A` or `git add .`. NEVER force push, amend, branch, or checkout. See execute.md Phase 6 for exact commands.
    **GITIGNORED DIRECTORIES -- NEVER `git add` these paths:**
    - `.context/` -- agent workspace (digests, task files, research, standards)
    - `.claude/` -- Claude Code config (commands, agents, memory)
    - `{BUILD_OUTPUT_DIRS}` -- build artifacts
    Only `{TRACKED_PATHS}` are tracked.
    If in doubt: run `git status --short` BEFORE `git add` and only add files that show as `??` or ` M` (not ignored).
6. **Stay in your domain** -- Each agent has a defined domain. Check `.context/execute/{SLOT}/priority.md` for your domain definition.
7. **Minimum {MIN_TESTS_PER_MODULE} tests** per module
8. **Do NOT modify files you didn't create** -- unless your task explicitly requires it
9. **Claim before work** -- Always write task.md before starting implementation
10. **Attribute contributors** -- When using an external algorithm, add the author to `{CONTRIBUTORS_FILE}`
11. **Loop continuously** -- After documenting, immediately evaluate next task. No "Execute? (y/n)"
12. **USE THE STYLE SYSTEM** -- If your module outputs ANY visual data or user-facing content:
    - Read the project's style/design system files BEFORE designing your API
    - Use design tokens for ALL visual constants
    - NEVER hardcode values that should come from the design system
    - Checker MUST verify: no suspicious hardcoded constants that should be tokens
    - This is a HARD GATE -- modules with hardcoded design values will be REJECTED

---

## Key Files to Read

| File | Purpose |
|------|---------|
| `SOUL.md` | Claude's perspective on this partnership |
| `.context/execute/PROTOCOL.md` | Multi-session coordination rules |
| `docs/ORCHESTRATOR_ROADMAP.md` | Where to find work |
| `docs/STATE.md` | What modules already exist |
| `.context/standards/CONVENTIONS.md` | Naming, style, patterns |
| `.context/standards/QUALITY_GATES.md` | Gate definitions + thresholds |
| `.context/execute/{SLOT}/report.md` | Your last completed task |
| `.context/execute/{SLOT}/priority.md` | MEGA override (if exists) |
| `.context/execute/{SLOT}/maintenance.md` | Maintainer health report |
| `.context/library/manifest.json` | Reference library catalog |
| `{CONTRIBUTORS_FILE}` | Attribution database |

---

## Report Format

After Phase 5 (MAINTAIN) passes, write `.context/execute/{SLOT}/report.md`:

```markdown
# Agent {SLOT} Report

**Status**: COMPLETE | PARTIAL | BLOCKED
**Date**: {today}
**Task**: [task title]
**Roadmap Reference**: Track X.Y

## Files Created
- path/to/interface -- description
- path/to/implementation -- description
- path/to/test -- description

## API Summary
[Function signatures, 1 line each]

## Test Results
{full test output}

## Compile Command
{exact command}

## Checker Result
PASS -- [validation summary]

## Maintainer Result
PASS | WARN -- [maintenance findings]

## Build System Additions
See {BUILD_ADDITIONS_FILE}

## Attribution
[External algorithm source, if any. Update {CONTRIBUTORS_FILE}.]

## Knowledge Gaps
[Data or references this task needed but couldn't find. Be specific:]
- "Need [what] from [source type] for [purpose]"
- "Reference suggestion: [title] by [author] -- would enable [roadmap item]"
[If none: "No gaps -- all data available."]

## Next Candidate
[What you'd pick next from the roadmap]
```

---

## After Reporting: Auto-Continue

Loop back to Phase 1 (EVALUATE). Print:
```
Task complete: [title] (N tests, 0 failures, Checker: PASS).
Claiming next task...
```

Then immediately scan for the next unclaimed roadmap item. No waiting.

---

## Build Commands

```bash
# Replace these with your project's actual build commands:

# Build:
{BUILD_COMMAND}

# Run all tests:
{TEST_COMMAND}

# Run specific test:
{SPECIFIC_TEST_COMMAND}

# Clean:
{CLEAN_COMMAND}
```

**After creating new files, register them in the build system.**

---

## OMEGA-Specific Notes

OMEGA is the quality guardian. It does NOT follow the standard Writer/Checker/Maintainer loop. Instead, OMEGA runs a rotating checklist of health tasks defined in `.context/execute/OMEGA/priority.md`. OMEGA reports findings but does NOT create new modules. OMEGA may execute trivial refactors but escalates structural refactors to MEGA for approval.
