# Multi-Session Coordination Protocol

**Version**: 1.0

---

## Architecture

This project runs N parallel Claude Code terminal sessions simultaneously. Each session is a full Claude instance with Agent tool access, enabling a multi-level hierarchy.

```
{HUMAN_NAME} (human, launches N terminal sessions)
|
+-- Session: MEGA (primary -- planning, mentoring, vision, SOUL.md)
|   +-- Subagent: Researcher
|   +-- Subagent: Planner
|   +-- Subagent: Auditor
|
+-- Session: ORCHESTRATOR ALPHA (self-directing from roadmap)
|   +-- Subagent: Writer (TDD implementation)
|   +-- Subagent: Checker (validation, compile gate)
|   +-- Subagent: Maintainer (regression, cleanup, metrics)
|
+-- Session: ORCHESTRATOR BETA (self-directing from roadmap)
|   +-- Subagent: Writer
|   +-- Subagent: Checker
|   +-- Subagent: Maintainer
|
+-- ... (more orchestrators as needed)
```

**Coordination**: File system. All sessions share the project directory.
**Identity**: Each session reads `CLAUDE.md` + `SOUL.md` on startup.
**Work discovery**: Orchestrators read `docs/ORCHESTRATOR_ROADMAP.md`.
**Conflict prevention**: Claim protocol via `.context/execute/{SLOT}/task.md`.

---

## Session Roles

| Role | Invocation | Reads | Writes |
|------|-----------|-------|--------|
| MEGA | Conversation, `/mega` | Everything | Plans, SOUL.md, roadmap edits |
| ORCHESTRATOR | `/execute ALPHA` (or BETA, GAMMA, ...) | Roadmap, protocol, standards | Code, tests, reports |

MEGA is the primary Claude session. It plans, mentors, writes SOUL.md, manages the roadmap, integrates agent output into the build system, and resolves cross-session conflicts.

Orchestrators are autonomous execution sessions. They find work, claim it, build it (via Writer subagent), validate it (via Checker subagent), and report. They never wait for MEGA.

---

## Work Claiming Protocol

**Problem**: N sessions reading the same roadmap could grab the same item.

**Solution**: Claim-before-work via task.md.

### Steps

1. Orchestrator scans `docs/ORCHESTRATOR_ROADMAP.md` for unchecked `[ ]` items in its domain
2. Before starting, reads ALL `.context/execute/*/task.md` files to check what's claimed
3. Writes its own `.context/execute/{SLOT}/task.md` with the selected roadmap reference
4. Only then begins the PLAN phase

### Conflict Resolution

If two sessions race to the same item, the second one detects it when reading task.md files and picks a different item. Files are the lock.

If two sessions have already started the same item (race condition), the later-finishing session detects the conflict when reading the other's report.md and discards its own work. First to report wins.

---

## Self-Governing Execution Loop

Each orchestrator runs continuously:

```
LOOP:
  1. EVALUATE  -- scan roadmap, check claims, select work
  2. PLAN      -- design module (write task.md with claim marker)
  3. EXECUTE   -- launch Writer subagent (TDD: tests -> implementation)
  4. VALIDATE  -- launch Checker subagent (compile, test, standards)
  5. MAINTAIN  -- launch Maintainer subagent (regression, cleanup, metrics)
  6. DOCUMENT  -- write report.md + {BUILD_ADDITIONS_FILE} + maintenance.md
  7. GOTO 1 (immediately -- no waiting for MEGA)
```

**Key**: Steps 3, 4, and 5 are SEPARATE subagents. The writer creates; the checker validates; the maintainer sweeps. Three perspectives.

---

## Writer Subagent

Dispatched by orchestrator. Receives task.md as input.

1. Writes test file first (RED phase)
2. Writes interface/header with declarations
3. Writes implementation (GREEN phase)
4. Compiles with `{COMPILE_COMMAND}` (zero warnings)
5. Runs tests (zero failures)

Returns: files created + test output.

---

## Checker Subagent

Dispatched by orchestrator AFTER writer completes.

1. Reads writer's output files
2. Runs quality checks ({QUALITY_CHECK_RULES})
3. Verifies compilation independently
4. Runs tests independently
5. Checks naming conventions, style compliance
6. Checks: does this module duplicate existing functionality?

Returns: PASS/FAIL with specific findings.

If Checker returns FAIL: orchestrator instructs Writer to fix, re-runs Checker.
If Checker returns PASS: orchestrator proceeds to Maintainer.

---

## Maintainer Subagent

Dispatched by orchestrator AFTER checker passes.

1. Runs `{CLEAN_AND_TEST_COMMAND}` (full regression gate)
2. Runs quality audit ({QUALITY_CHECK_RULES}) on all files the Writer created/touched
3. Checks naming/style compliance ({NAMING_CONVENTIONS})
4. Scans for dead code (every declaration must be called somewhere)
5. Audits for naked TODOs/FIXMEs without ticket references
6. Checks for duplication against `docs/STATE.md`
7. Counts total tests and source files (metrics)
8. Verifies external algorithm attribution in `{CONTRIBUTORS_FILE}`
9. **DEPENDENCY CHAIN CHECK**: If this module added new dependencies, verify ALL test targets that link this module include the new dependencies in their configuration.
10. **PHANTOM FILE CHECK**: Every file registered in the build system MUST exist on disk. If ANY phantom file found: CRITICAL -- do NOT commit until the file exists or the registration is removed.
11. **INTERFACE/IMPLEMENTATION MATCH CHECK**: Every public function declared in the interface file must be implemented with MATCHING signatures.

Returns: PASS, WARN (non-blocking), or CRITICAL (blocking).

Gate definitions: `.context/standards/QUALITY_GATES.md`

If Maintainer returns CRITICAL: orchestrator sends Writer back to fix, re-runs Maintainer.
If Maintainer returns WARN: noted in `maintenance.md`, pipeline continues.
If Maintainer returns PASS: orchestrator writes report.md, claims next work.

---

## Research Protocol

### Web Access Reality

| Source Type | Accessible? | Method |
|------------|-------------|--------|
| Open data (public APIs, government data) | YES | WebFetch |
| Open catalogs (public datasets) | YES | WebSearch + WebFetch |
| Preprints (arXiv, open access) | YES | WebFetch |
| GitHub repos/data | YES | WebFetch |
| Paywalled journals/books | NO | {HUMAN_NAME} downloads manually |

### Reference Library Access

Reference materials are accessible via `.context/library/manifest.json`:
1. Read manifest.json to discover available materials by category/priority
2. Read relevant pages/sections from available files
3. Extract data into `.context/research/digested/NNN-topic.md`
4. Update manifest.json extraction status
5. Update `{CONTRIBUTORS_FILE}` with author attribution

### Research Task Flow

```
1. Check .context/library/manifest.json for relevant materials
2. If material exists and priority allows:
   -> Read relevant sections
   -> Extract data into .context/research/digested/NNN-topic.md
   -> Update manifest.json extraction status
   -> Update {CONTRIBUTORS_FILE} with author
3. If data available via open web:
   -> WebFetch from public sources
   -> Extract into digest file
4. If paywalled:
   -> Write request to .context/execute/{SLOT}/escalation.md
   -> "BLOCKED: Need [title]. {HUMAN_NAME}: download from [URL]"
   -> Move to next available task
5. Use digested data to implement module (normal TDD flow)
```

### Paywalled Content Escalation

When an orchestrator needs a paywalled resource:
1. Write `.context/execute/{SLOT}/escalation.md` with the title, URL, and what data is needed
2. Do NOT stall -- move to the next available task
3. {HUMAN_NAME} downloads the material and places it in `{INBOX_PATH}`
4. On next evaluation cycle, orchestrator checks manifest and finds it available

---

## Knowledge Pipeline (End-to-End)

The full pipeline from "we need data X" to shipped code:

```
PHASE A: NEED IDENTIFICATION
  MEGA or orchestrator identifies missing data for a roadmap item
  -> Adds unchecked item to ORCHESTRATOR_ROADMAP.md (if MEGA)
  -> Or claims existing research item (if orchestrator)

PHASE B: RESEARCH
  Orchestrator claims research task
  -> Checks manifest.json for relevant materials
  -> WebSearches for open data
  -> WebFetches from open sources
  -> If paywalled: escalation.md -> {HUMAN_NAME} downloads
  -> Writes digest: .context/research/digested/NNN-topic.md
  -> Updates {CONTRIBUTORS_FILE}

PHASE C: DEVELOPMENT (standard pipeline)
  -> Phase 2: PLAN (design from digest)
  -> Phase 3: EXECUTE (Writer TDD)
  -> Phase 4: VALIDATE (Checker audit)
  -> Phase 5: MAINTAIN (Maintainer sweep)
  -> Phase 6: DOCUMENT (report + build additions + maintenance)

PHASE D: INTEGRATION (MEGA)
  -> Applies build system changes
  -> Wires module into application
  -> {TEST_COMMAND}
  -> {HUMAN_NAME} tests in target environment
```

---

## Knowledge Feedback Loop

A closed loop connecting agent work, knowledge gaps, references, and roadmap growth.

```
Agent builds module
  -> encounters missing data/algorithm
  -> reports "Knowledge Gaps" in report.md
  -> MEGA reads report, translates gaps into:
     1. {BOOKS_CATALOG} entries
     2. .context/library/manifest.json updates (when material acquired)
     3. docs/ORCHESTRATOR_ROADMAP.md new research tasks
  -> {HUMAN_NAME} acquires material (or agent WebFetches open data)
  -> Agent claims research task, extracts data
  -> Agent writes .context/research/digested/NNN-topic.md
  -> Agent builds module using extracted data
  -> LOOP
```

**MEGA responsibilities**:
- Maintain reference catalog as living document
- Translate agent knowledge gaps into acquisition suggestions
- Identify gaps proactively by scanning roadmap for data needs
- Update ORCHESTRATOR_ROADMAP.md when new data enables new work

**Agent responsibilities**:
- Always check `.context/library/manifest.json` before starting data-heavy tasks
- Report ALL knowledge gaps in report.md (even if task succeeded with approximations)
- Suggest specific sources when they know what's missing
- Write digests when extracting substantial data from references

---

## Escalation Protocol

When an orchestrator is blocked:
1. Write `.context/execute/{SLOT}/escalation.md` describing the blocker
2. Continue to next available task (don't stall)
3. MEGA resolves escalations during its planning cycles

---

## Build System Integration Protocol

Orchestrators do NOT edit the build configuration directly.

1. Writer produces `{BUILD_ADDITIONS_FILE}` with exact lines to add
2. MEGA or {HUMAN_NAME} applies changes to the build system
3. `{TEST_COMMAND}` verifies no regressions after integration

Format for `{BUILD_ADDITIONS_FILE}`:
```markdown
## Build system additions for: {module_name}

### Source files to register:
{SOURCE_DIR}/{area}/{name}.{SOURCE_EXT}

### Test target:
{TEST_DIR}/{area}/test_{name}.{SOURCE_EXT}

### Dependencies:
[list any module dependencies]
```

---

## Git Protocol

Orchestrators commit and push their own work after Maintainer PASS.

1. `git add` ONLY files created/modified by the current task (specific paths, NEVER `-A` or `.`)
2. Commit: `feat(agent-{SLOT}): {module} -- {description}`
3. Before push, handle other sessions' unstaged changes:
   ```bash
   git stash --include-untracked && git pull --rebase origin main && git push origin main && git stash pop
   ```
4. If stash pop has conflicts: `git checkout --theirs .` on non-agent files, keep agent's own files
5. If push still fails after 2 attempts: write `escalation.md`, skip push, continue to next task
6. NEVER force push, amend, reset, branch, or checkout

**Why stash**: MEGA may be editing docs while agents work. Those unstaged changes block `git pull --rebase`. Stash preserves them safely.

MEGA session uses passive git only (status, log, diff). Drafts commits for {HUMAN_NAME} to review.

---

## Attribution Protocol

Every external knowledge source used in this project must be attributed.

**Rule**: When a session adds a new module that implements an algorithm from a named author, book, or paper, the session MUST also add the contributor to `{CONTRIBUTORS_FILE}`.

This applies to:
- Algorithms from named authors or papers
- Data from external sources or references
- Any new data source or reference material

---

## SOUL.md Protocol

- Every session reads `SOUL.md` on startup alongside `CLAUDE.md`
- Only the MEGA session may update SOUL.md
- Orchestrators and their subagents read it but never modify it
- It is tracked in git as part of the project's permanent record

---

## Directory Structure

```
.context/execute/
  PROTOCOL.md              -- This file (multi-session coordination rules)
  BOOTSTRAP.md             -- Universal agent rules
  _SLOT_TEMPLATE/          -- Template files for new agent slots
    task.md
    report.md
    priority.md
    maintenance.md
    escalation.md
  ALPHA/                   -- Orchestrator Alpha's workspace
    task.md                -- Current work (claim marker)
    report.md              -- Last completion report
    priority.md            -- MEGA override (optional)
    {BUILD_ADDITIONS_FILE} -- Exact lines to add to build system
    maintenance.md         -- Maintainer health sweep report
    escalation.md          -- Blocker description (optional)
  BETA/                    -- Orchestrator Beta's workspace
    task.md
    report.md
    priority.md
    {BUILD_ADDITIONS_FILE}
    maintenance.md
    escalation.md
  ...                      -- More slots created as needed
```

---

## Rules Summary

1. **Read CLAUDE.md + SOUL.md on startup** -- every session, every time
2. **Claim before work** -- write task.md before starting
3. **Check all claims** -- read all */task.md before claiming
4. **Writer + Checker + Maintainer separation** -- three perspectives on every deliverable
5. **Attribute contributors** -- update {CONTRIBUTORS_FILE} for new sources
6. **Escalate, don't stall** -- write escalation.md and move on
7. **Git after Maintainer PASS** -- Orchestrators commit+push in Phase 6. NEVER `git add -A`, force push, amend, branch, or checkout
8. **Loop continuously** -- don't ask permission to continue, just GO
9. **Update downstream docs immediately** -- When MEGA makes architectural changes, update CLAUDE.md, PROTOCOL.md, ORCHESTRATOR_ROADMAP.md, and agent priority.md files so changes propagate to all agents on their next startup
