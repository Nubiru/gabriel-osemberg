# /roadmap — Track Development Progress

**Purpose**: View, update, plan, or audit progress against the project roadmap.

**Arguments**: `$ARGUMENTS`

---

## Mode Detection

```
Parse $ARGUMENTS for mode keyword:
- "status" or empty   -> MODE: STATUS (default)
- "update"            -> MODE: UPDATE
- "plan"              -> MODE: PLAN
- "audit"             -> MODE: AUDIT
```

---

## Mandatory Reading

Before any mode:
1. Read `{ROADMAP_PATH}` - The roadmap itself
2. Read `{PROJECT_HUB_PATH}` - Project identity and rules

---

## Mode 1: STATUS (default)

**When**: No arguments, or `$ARGUMENTS` contains "status".

### Steps

1. Read `{ROADMAP_PATH}` - get phase definitions and "done when" criteria
2. Scan the codebase to determine actual progress:
   - `ls src/` - what source directories/files exist?
   - `ls tests/` - what tests exist?
   - Check if build system has all targets
   - Review other key directories as defined by project
3. For each phase/milestone, check "done when" criteria against actual codebase state
4. Determine current active phase (first phase with incomplete criteria)

### Output

```
# Roadmap Status

Phase 0: {PHASE_NAME}     [COMPLETE / IN PROGRESS / NOT STARTED]
  [x] {criterion_1}
  [ ] {criterion_2}
  ...

Phase 1: {PHASE_NAME}     [NOT STARTED]
  ...

---
Active Phase: {N}
Next milestone: {DESCRIPTION}
```

---

## Mode 2: UPDATE

**When**: `$ARGUMENTS` contains "update".

### Steps

1. Run `git log --oneline -20` - see recent work
2. Run `git diff --stat` - see uncommitted changes
3. Read `{ROADMAP_PATH}` - current checkbox state
4. Compare recent commits/changes against roadmap criteria
5. Update checkboxes in `{ROADMAP_PATH}` for completed criteria
6. Update "Current Status" section
7. Update "Last Updated" date

### Output

```
Roadmap updated:
- Phase {N}: marked [x] for "{criterion}"
- Current Status updated to reflect progress
```

---

## Mode 3: PLAN

**When**: `$ARGUMENTS` contains "plan".

### Steps

1. Run STATUS mode first (determine where we are)
2. Identify the active phase and remaining unchecked criteria
3. Break remaining work into concrete, ordered tasks
4. For each task, specify:
   - What to do (action)
   - Which files to create/modify
   - What concepts/skills are involved
   - Estimated complexity (small/medium/large)
5. Output as a numbered task list

### Output

```
# Phase {N} - Remaining Tasks

Current progress: X/Y criteria met

## Next Tasks (in order)

1. {TASK_NAME}
   - Action: {DESCRIPTION}
   - Files: {AFFECTED_FILES}
   - Complexity: {small/medium/large}

2. {TASK_NAME}
   - Action: {DESCRIPTION}
   - Verification: {HOW_TO_VERIFY}
   - Complexity: {small/medium/large}

3. ...
```

---

## Mode 4: AUDIT

**When**: `$ARGUMENTS` contains "audit".

### Steps

1. Read `{ROADMAP_PATH}` - marked progress
2. Read `git log --oneline -30` - recent commit history
3. Cross-reference: do commits match roadmap phases?
4. Check for scope creep:
   - Are there files/features that don't belong to the active phase?
   - Are we working on future-phase things while current phase is incomplete?
5. Check for unmarked progress:
   - Did we complete criteria but forget to update the roadmap?
6. Report findings

### Output

```
# Roadmap Audit

## Alignment
- Commits align with Phase {N}: YES/NO
- Scope creep detected: YES/NO

## Unmarked Progress
- Phase {N} criterion "{CRITERION}" appears complete but not checked

## Recommendations
- Update {ROADMAP_PATH} checkboxes
- Stay focused on Phase {N} before moving to Phase {M}
```

---

## Output Constraints

- Always show phase number + name
- Use checkboxes `[x]` / `[ ]` for criteria
- Keep output concise: status in <30 lines, plan in <50 lines
- Evidence-based: check actual files, don't guess
- Reference specific files when claiming criteria are met

---

## When to Use

**Use `/roadmap` when**:
- Starting a new session (quick orientation)
- Completing a milestone (update progress)
- Planning next work session (what to do next)
- Periodic check (are we on track?)

**Don't use `/roadmap` when**:
- In the middle of implementing something (just keep coding)
- Need detailed task specs (use dedicated task planning)

---

**Command Type**: Progress Tracking
