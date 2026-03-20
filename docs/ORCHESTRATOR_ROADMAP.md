# Orchestrator Roadmap — Agent Work Plan

**Owner**: Primary session (orchestrator)
**Last Updated**: {DATE}
**Philosophy**: {DELEGATION_PHILOSOPHY}

---

## Delegation Model

```
MEGA (primary):   Architecture, vision, integration, mentoring.
                  Defines WHAT needs building in this roadmap.

{AGENT_SLOT_1}:   {AGENT_1_FOCUS}
                  /execute {AGENT_SLOT_1}

{AGENT_SLOT_2}:   {AGENT_2_FOCUS}
                  /execute {AGENT_SLOT_2}

OMEGA:            Codebase health, maintenance, quality sweeps.
                  No new modules. /execute OMEGA
```

Customize agent slots to match your project's domain boundaries. Each agent should own a clear, non-overlapping area of responsibility.

---

## Priority Levels

- **P0**: Blocks current phase — do first
- **P1**: Needed for next phase — queue next
- **P2**: Enriches existing work — when capacity allows
- **P3**: Nice to have — backlog

---

## TRACK 1: {TRACK_1_NAME}

{TRACK_1_DESCRIPTION}

### 1.1 {SUBTASK_NAME}
- [ ] {TASK_ITEM}
- [ ] {TASK_ITEM}

**Assigned**: {AGENT_SLOT}
**Priority**: P0
**Status**: Not started

### 1.2 {SUBTASK_NAME}
- [ ] {TASK_ITEM}

**Assigned**: {AGENT_SLOT}
**Priority**: P0
**Status**: Not started

---

## TRACK 2: {TRACK_2_NAME}

{TRACK_2_DESCRIPTION}

### 2.1 {SUBTASK_NAME}
- [ ] {TASK_ITEM}

**Assigned**: {AGENT_SLOT}
**Priority**: P1
**Status**: Not started

---

<!--
Add more tracks as needed. Each track follows this template:

## TRACK N: {TRACK_NAME}

{DESCRIPTION}

### N.1 {SUBTASK}
- [ ] {TASK}

**Assigned**: {AGENT}
**Priority**: P0-P3
**Status**: Not started | In progress | Complete
**Dependencies**: Track X.Y

Track naming convention:
- Group related work into tracks
- Number subtasks within tracks (N.1, N.2, N.3)
- Cross-reference dependencies between tracks
-->

---

## Cross-Track Dependencies

| Blocked Track | Depends On | Notes |
|---------------|------------|-------|
| {TRACK_N} | {TRACK_M} | {WHY} |

---

## Completed Tracks

<!-- Move completed tracks here to keep the active section clean -->
