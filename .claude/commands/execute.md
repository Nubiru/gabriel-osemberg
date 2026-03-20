# /execute -- Multi-Session Autonomous Agent Pipeline

**Purpose**: Self-governing orchestrator pipeline. Each orchestrator finds work, claims it, builds it (Writer subagent), validates it (Checker subagent), sweeps it (Maintainer subagent), reports, and loops. No waiting for external validation.

**Arguments**: `$ARGUMENTS`

---

## Mode Detection

- `ALPHA`, `BETA`, `GAMMA`, ...: **RUN** -- Orchestrator enters continuous self-governing loop
- `status`: **STATUS** -- Show all slot states and available work

---

## Agent Domains

Orchestrators have distinct domains to prevent overlap. Each picks work within its domain.

{Define your project's agent domains here. Examples:}

**ALPHA -- {ALPHA_DOMAIN_NAME}**
{Description of ALPHA's focus area, typical directories, and concerns.}

**BETA -- {BETA_DOMAIN_NAME}**
{Description of BETA's focus area, typical directories, and concerns.}

**GAMMA -- {GAMMA_DOMAIN_NAME}**
{Description of GAMMA's focus area, typical directories, and concerns.}

**OMEGA -- Codebase Health & Maintenance**
Quality guardian. Does NOT create new modules. Audits, refactors, measures, alerts. Runs quality checks, refactor scans, git health, build system sync, metrics updates, dead code detection, attribution audit, style compliance, test quality. See `.context/execute/OMEGA/priority.md` for full task list.

**Other slots** -- Domain assigned by MEGA when slot is created. Check `.context/execute/{SLOT}/priority.md` for domain definition.

---

## Mandatory Reads (Before ANY Phase)

1. `SOUL.md` -- Claude's perspective on this partnership
2. `.context/execute/PROTOCOL.md` -- Multi-session coordination rules
3. `.context/execute/BOOTSTRAP.md` -- Universal agent rules
4. `docs/ORCHESTRATOR_ROADMAP.md` -- Where to find work
5. `docs/STATE.md` -- Existing module inventory

---

## The Self-Governing Loop

```
LOOP:
  Phase 1: EVALUATE  -- scan roadmap, check claims, select work
  Phase 2: PLAN      -- design module, write task.md (claim marker)
  Phase 3: EXECUTE   -- launch Writer subagent (TDD)
  Phase 4: VALIDATE  -- launch Checker subagent (independent audit)
  Phase 5: MAINTAIN  -- launch Maintainer subagent (regression, quality, cleanup)
  Phase 6: DOCUMENT  -- write report.md + {BUILD_ADDITIONS_FILE} + maintenance.md
  GOTO Phase 1 (immediately)
```

**Exit conditions** (only these):
- No unchecked items remain in your domain
- 3+ tasks completed in this session (context window management)
- Unrecoverable blocker after 3 fix attempts

---

## Phase 1: EVALUATE

### Read (in order)
1. `docs/ORCHESTRATOR_ROADMAP.md` -- the macro work plan
2. `docs/STATE.md` -- existing module inventory
3. `.context/execute/{SLOT}/report.md` -- your last completed work

### Check Claims (MANDATORY)
Read ALL `.context/execute/*/task.md` files to see what other agents have claimed.
If another slot has claimed an item, skip it. Pick a different item.

### Priority Override Check
If `.context/execute/{SLOT}/priority.md` exists and is non-empty:
- This is a MEGA-assigned task. Use it directly.
- Skip claim checking for this specific item.
- Delete priority.md after copying to task.md.

### Self-Directed Task Selection
Scan `docs/ORCHESTRATOR_ROADMAP.md` for unchecked `[ ]` items. For each:
1. **Is it in my domain?** Match against domain keywords above.
2. **Does it need a module that doesn't exist yet?** Check source directory for existing interfaces.
3. **Is it agent-appropriate?** Pure computation, data modules = YES. Integration, configuration, main entry point changes = NO (MEGA work).
4. **Is it already claimed?** Check all task.md files.

Select the **highest-priority unclaimed candidate** (earlier tracks first, lower-numbered items first).

**If NO work found**: Report "Slot {SLOT} idle -- no unchecked items in my domain." and stop.

---

## Phase 2: PLAN

### Write `.context/execute/{SLOT}/task.md` (THIS IS YOUR CLAIM)

```markdown
# Task: [Descriptive Name]

**Agent**: {SLOT}
**Roadmap Reference**: Track X.Y -- "[exact item text from roadmap]"
**Date**: {today}
**Status**: CLAIMED

## Goal
[1-3 sentences: what this module does, why the roadmap needs it]

## READ FIRST
[List interface files that inform your API design]

## Files to Create
- `{SOURCE_DIR}/{area}/{name}.{HEADER_EXT}`
- `{SOURCE_DIR}/{area}/{name}.{SOURCE_EXT}`
- `{TEST_DIR}/{area}/test_{name}.{SOURCE_EXT}`

## API
[Types and function signatures. Pure functions. Return values, not pointers.]

## DONE WHEN
- [ ] All functions declared in interface and implemented in source
- [ ] >= {MIN_TESTS_PER_MODULE} tests covering: [list specific scenarios]
- [ ] All tests pass with zero warnings
- [ ] Quality checks pass
- [ ] Compiles: `{COMPILE_COMMAND}`

## Constraints
- {LANGUAGE_STANDARD}
- {COMPILE_FLAGS}
- {QUALITY_CHECK_RULES}
```

---

## Phase 3: EXECUTE (Writer Subagent)

Launch a **Writer subagent** via the Agent tool. Provide it with:
- The full task.md content
- BOOTSTRAP.md rules
- Relevant existing interface files

**Writer subagent instructions**:

```
You are a Writer agent for this project. Your job: TDD implementation.

MANDATORY READS FIRST:
- SOUL.md (project root)
- .context/execute/BOOTSTRAP.md (universal rules)

STEPS:
1. Read task.md for your assignment
2. Read existing interfaces listed in READ FIRST section
3. Write test file FIRST (RED phase) -- minimum {MIN_TESTS_PER_MODULE} tests
4. Write interface file with all declarations
5. Write implementation file -- make all tests GREEN
6. Compile: {COMPILE_COMMAND}
7. Run tests: {TEST_COMMAND}
8. GATE: 0 warnings, 0 failures. If not, fix and re-run.

RESEARCH (before writing code):
- Check .context/library/manifest.json for relevant materials
- If material exists: Read relevant sections for algorithms/data/constants
- If open data needed: WebFetch from public sources
- If paywalled: note in Knowledge Gaps section of report
- Write digest to .context/research/digested/ if substantial extraction

RULES:
- Follow all quality rules from {QUALITY_CHECK_RULES}
- Follow naming conventions from {NAMING_CONVENTIONS}
- No active git commands (orchestrator handles git, not you)
- When using external algorithms, note the source author for attribution

RETURN: List of files created + full test output + compile command used + any knowledge gaps encountered.
```

---

## Phase 4: VALIDATE (Checker Subagent)

After the Writer completes, launch a **Checker subagent** via the Agent tool. Provide it with the Writer's output files.

**Checker subagent instructions**:

```
You are a Checker agent for this project. Your job: independent validation.

MANDATORY READS FIRST:
- SOUL.md (project root)
- .context/execute/BOOTSTRAP.md (universal rules)
- .context/standards/CONVENTIONS.md (naming, style)

VALIDATE THESE FILES: [list from Writer output]

CHECKS (all must pass):
1. COMPILE: {COMPILE_COMMAND}
2. TEST: {TEST_COMMAND} -- 0 failures, >= {MIN_TESTS_PER_MODULE} tests
3. QUALITY: {QUALITY_CHECK_RULES}
   Run all project quality checks on source files.
   All must pass clean.
4. NAMING: {NAMING_CONVENTIONS}
5. INTERFACE GUARDS: Proper module boundaries and include protection
6. API: Functions follow project conventions (return values, const params, etc.)
7. TEST QUALITY: Arrange-Act-Assert pattern, descriptive names, focused assertions
8. DUPLICATION: Does this module duplicate existing functionality? Check docs/STATE.md
9. FULL BUILD: {TEST_COMMAND} -- existing tests still pass (regression check)

RETURN: PASS or FAIL with specific findings for each check.
```

### On FAIL:
- Orchestrator sends findings back to a new Writer subagent with fix instructions
- Re-run Checker after fixes
- After 3 FAIL cycles on the same issue: write escalation.md and move on

### On PASS:
- Proceed to Phase 5 (MAINTAIN)

---

## Phase 5: MAINTAIN (Maintainer Subagent)

After the Checker passes, launch a **Maintainer subagent** via the Agent tool. It performs a comprehensive health sweep on the Writer's output and the overall codebase.

**Maintainer subagent instructions**:

```
You are a Maintainer agent for this project. Your job: post-delivery health sweep.

MANDATORY READS FIRST:
- SOUL.md (project root)
- .context/execute/BOOTSTRAP.md (universal rules)
- .context/standards/QUALITY_GATES.md (gate definitions)
- docs/STATE.md (existing module inventory)

FILES TO AUDIT: [list from Writer output]

CHECKS (in order):

1. REGRESSION GATE: {CLEAN_AND_TEST_COMMAND}
   - Full rebuild from scratch, every test must pass
   - If FAILS: return CRITICAL immediately

2. QUALITY AUDIT on ALL files the Writer created/touched:
   {QUALITY_CHECK_RULES}
   All must pass clean. If VIOLATES: CRITICAL

3. NAMING/STYLE:
   {NAMING_CONVENTIONS}

4. DEAD CODE SCAN:
   - Every function declared in the interface is called somewhere (implementation or test file)
   - If orphan found: WARNING

5. TODO AUDIT:
   - Search for TODO/FIXME/HACK/XXX in new files
   - Naked TODOs without reference: WARNING

6. DUPLICATION CHECK:
   - Search docs/STATE.md for modules with similar names/signatures
   - If potential overlap found: WARNING

7. METRICS:
   - Count total tests ({TEST_COMMAND} output)
   - Count source files

8. ATTRIBUTION CHECK:
   - If module references an external algorithm, verify contributor is in {CONTRIBUTORS_FILE}
   - If missing: WARNING

RETURN:
- PASS (all clear), WARN (non-blocking issues), or CRITICAL (blocking issues)
- Maintenance report content for maintenance.md
- List all findings per check
```

### Gate Behavior

- **CRITICAL** (tests fail, quality violation, missing module boundary): orchestrator sends Writer back to fix, then re-runs Maintainer
- **WARNING** (naked TODOs, potential duplication, missing attribution): noted in maintenance.md, not blocking
- **PASS**: orchestrator proceeds to Phase 6 (DOCUMENT)

After 3 CRITICAL cycles on the same issue: write `escalation.md` and move on.

---

## Phase 6: DOCUMENT

### Write `.context/execute/{SLOT}/report.md`

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
{full test output: N tests, 0 failures}

## Compile Command
{exact command used}

## Checker Result
PASS -- [summary of validation]

## Maintainer Result
PASS | WARN -- [summary of maintenance findings]

## Build System Additions
See {BUILD_ADDITIONS_FILE}

## Attribution
[If this module uses an external algorithm, name the contributor]

## Knowledge Gaps
[Data or references this task needed but couldn't find. Be specific:]
- "Need [what] from [source type] for [purpose]"
- "Reference suggestion: [title] by [author] -- would enable [roadmap item]"
[If none: "No gaps -- all data available."]

## Next Candidate
[What you'd pick next from the roadmap]
```

### Write `.context/execute/{SLOT}/{BUILD_ADDITIONS_FILE}`

```markdown
## Build system additions for: {module_name}

### Source files to register:
{SOURCE_DIR}/{area}/{name}.{SOURCE_EXT}

### Test target:
{TEST_DIR}/{area}/test_{name}.{SOURCE_EXT}

### Dependencies:
[list module dependencies]
```

### Git Commit + Push

After writing report.md and {BUILD_ADDITIONS_FILE}, commit and push:

```bash
# Stage ONLY files this task created/modified (NEVER git add -A or git add .)
git add {SOURCE_DIR}/{area}/{name}.{HEADER_EXT} {SOURCE_DIR}/{area}/{name}.{SOURCE_EXT} {TEST_DIR}/{area}/test_{name}.{SOURCE_EXT}

# Commit with agent convention
git commit -m "feat(agent-{SLOT}): {module_name} -- {short description}

{N} tests, 0 failures. Checker: PASS. Maintainer: PASS.
Track X.Y: {roadmap item text}"

# Push (handle concurrent agents + MEGA's unstaged edits)
git stash --include-untracked && git pull --rebase origin main && git push origin main && git stash pop
```

**Rules**:
- NEVER `git add -A` or `git add .`
- NEVER force push, amend, reset, branch, or checkout
- Stash before pull -- MEGA may have unstaged doc edits that block rebase
- If stash pop conflicts on non-agent files: `git checkout --theirs <file>` (MEGA's version wins for docs)
- If push fails after 2 attempts: write escalation.md, skip push, continue to next task

### Auto-Continue
Print summary and immediately loop to Phase 1:
```
Task complete: [title] (N tests, 0 failures, Checker: PASS, Maintainer: PASS). Committed + pushed.
Claiming next task...
```

No "Execute? (y/n)" prompt. Just GO.

---

## Research Task Type

When a roadmap item requires research data before implementation, follow this flow:

1. **Check `.context/library/manifest.json`** for relevant materials
2. **If material exists and priority allows**:
   - Read relevant sections via Read tool
   - Extract data into `.context/research/digested/NNN-topic.md`
   - Update manifest.json extraction status
   - Update `{CONTRIBUTORS_FILE}` with author
3. **If data available via open web** (public APIs, open access):
   - WebFetch from source directly
   - Extract into digest file
4. **If paywalled**:
   - Write request to `.context/execute/{SLOT}/escalation.md`
   - "BLOCKED: Need [title]. {HUMAN_NAME}: download from [URL] to {INBOX_PATH}"
   - Move to next available task
5. **Use digested data** to implement module (normal TDD flow)

Research tasks produce a digest file AND a code module. Both count as deliverables.

---

## STATUS Mode

When `$ARGUMENTS` is `status`:

1. Read all `*/task.md` and `*/report.md` files in `.context/execute/`
2. Scan `docs/ORCHESTRATOR_ROADMAP.md` for unchecked items
3. Report:

```
ALPHA: [COMPLETE|ACTIVE|IDLE] -- [task title]
BETA:  [COMPLETE|ACTIVE|IDLE] -- [task title]
GAMMA: [COMPLETE|ACTIVE|IDLE] -- [task title]
OMEGA: [COMPLETE|ACTIVE|IDLE] -- [task title]

Roadmap items available: N unchecked
  ALPHA domain: [list unclaimed items]
  BETA domain:  [list unclaimed items]
  GAMMA domain: [list unclaimed items]
```

---

## Directory Structure

```
.context/execute/
  PROTOCOL.md              -- Multi-session coordination rules
  BOOTSTRAP.md             -- Universal agent rules
  _SLOT_TEMPLATE/          -- Template files for new agent slots
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
  ...                      -- More slots as needed
```
