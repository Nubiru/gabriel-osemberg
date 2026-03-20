# Stream Coordination Protocol

**Version**: 1.0
**Date**: {DATE}

---

## Architecture

{PROJECT_NAME} is built through parallel EXPERIENCE STREAMS, each owning a domain of the final product. The primary session (MEGA) coordinates without operating. The project owner launches streams in terminal sessions.

```
{OWNER} (launches terminal sessions)
|
+-- MEGA (primary — coordination, vision, synthesis)
|
+-- Stream: {STREAM_1}   — {STREAM_1_DESCRIPTION}
+-- Stream: {STREAM_2}   — {STREAM_2_DESCRIPTION}
+-- Stream: {STREAM_3}   — {STREAM_3_DESCRIPTION}
+-- Stream: ...
```

## Stream Phases

```
PHASE 0: RESEARCH   — Produce 7 research sections (no code)
PHASE 1: ROADMAP    — Refine layered roadmap from research
PHASE 2: BUILD      — Code production (Writer/Checker/Maintainer)
PHASE 3: POLISH     — Integration, refinement, cross-stream testing
```

All streams start at Phase 0. Streams advance independently. MEGA reviews after transitions.

---

## Phase 0: Research Loop

During research, each stream produces 7 documents using the unified skeleton:

```
LOOP:
  1. ORIENT      — Read status.md, inbox.md, check progress
  2. SURVEY      — Pick next incomplete section (001-007)
  3. INVESTIGATE  — Research: read codebase, references, web searches
  4. VERIFY      — Check thoroughness, gaps, accuracy
  5. SYNTHESIZE  — Integrate findings, note cross-stream needs
  6. PUBLISH     — Write section, update status.md, write outbox.md
  GOTO 1
```

### Research Sections

| # | File | Content | Done When |
|---|------|---------|-----------|
| 1 | 001-inventory.md | What exists in the codebase for this stream | Every relevant module listed |
| 2 | 002-world-survey.md | Best-in-class examples from the world | 5+ examples analyzed |
| 3 | 003-gap-analysis.md | What's missing between codebase and vision | Gaps categorized by severity |
| 4 | 004-questions.md | Open questions researched and answered | No unanswered blocking questions |
| 5 | 005-roadmap.md | Layered plan (L0-L3) | L0 concrete enough for tasks |
| 6 | 006-dependencies.md | Cross-stream needs and provides | All dependencies documented |
| 7 | 007-resources.md | References, knowledge sources, resource needs | Actionable resource list |

---

## Phase 2: Build Loop

```
LOOP:
  1. EVALUATE    — Read roadmap, check claims
  2. PLAN        — Write task.md (claim lock)
  3. EXECUTE     — Launch Writer subagent (TDD)
  4. VALIDATE    — Launch Checker subagent (compile, test, lint)
  5. MAINTAIN    — Launch Maintainer subagent (regression, metrics)
  6. DOCUMENT    — Write report.md, commit + push
  GOTO 1
```

---

## File-Based Coordination

Each stream has:
- `status.md` — current phase + section progress
- `task.md` — active work claim (same lock mechanism)
- `report.md` — last completed work
- `inbox.md` — messages FROM other streams
- `outbox.md` — messages TO other streams
- `tracks.md` — which roadmap tracks this stream owns

Global files:
- `CROSS.md` — master cross-stream dependency index

---

## Cross-Stream Communication

**Direct routing** — streams write DIRECTLY to the destination stream's `inbox.md`:
1. Stream writes message to destination's `.context/streams/{TARGET_STREAM}/inbox.md`
2. Stream ALSO copies the message to its own `outbox.md` (as a sent-log)
3. Destination reads its `inbox.md` at start of each ORIENT phase
4. MEGA reviews all outbox.md files during coordination cycles to update CROSS.md

MEGA does NOT need to be in the middle. Streams communicate peer-to-peer.

**CROSS.md is stream-maintained.** When a stream discovers a new cross-stream dependency, it appends to `.context/streams/CROSS.md` directly. MEGA reviews CROSS.md but does NOT need to be the one updating it.

**Each stream's truth is in its own `status.md`.** Streams MUST update their status.md header (Phase, readiness, blockers) at session start and end. MEGA reads all status.md files directly.

---

## Startup Sequence (every stream session)

1. Read `CLAUDE.md` — project rules
2. Read `SOUL.md` — project spirit (if applicable)
3. Read `.context/streams/PROTOCOL.md` — this file
4. Read `.context/streams/{STREAM_NAME}/status.md` — current state
5. Read `.context/streams/{STREAM_NAME}/inbox.md` — pending messages
6. Read `.context/streams/{STREAM_NAME}/tracks.md` — owned tracks
7. Resume from where last session left off

---

## Git Protocol

- Commit only files this stream created/modified
- `git add` specific files by name — NEVER `git add -A`
- Commit format: `{COMMIT_PREFIX}(stream-{NAME}): {module} — {description}`
- Pull before push: `git pull --rebase origin {DEFAULT_BRANCH} && git push origin {DEFAULT_BRANCH}`
- NEVER force push, amend, reset, or branch

---

## Phase Transitions — Self-Advancing (with Completion Test)

Streams advance their own phase. No MEGA approval needed. BUT: before advancing, pass the Completion Test from BOOTSTRAP.md:
1. **User Test** — does this serve a real user?
2. **Coverage Test** — are most domains in your area covered with real content?
3. **Depth Test** — can you explain what the output MEANS, not just output a value?

If you pass all three:
- Research complete -> update status.md Phase to ROADMAP -> continue
- Roadmap refined -> update status.md Phase to BUILD -> continue
- Build complete -> update status.md Phase to POLISH -> continue

**Roadmap is not the mission.** Your roadmap (L0-L3) is a plan. Your mission is to serve the project. If L0-L3 are done but half your domain has zero content, you are NOT complete. Go back and build the content.

MEGA reviews work AFTER delivery. If MEGA finds premature completion claims, MEGA writes to your inbox with the gap analysis.

**When blocked:** Streams do NOT go idle. They research, write docs, answer inbox, or help other streams. See BOOTSTRAP.md fallback rules.

---

## Reporting Priority

**Files first, terminal second.** Everything a stream produces MUST be written to files:
1. `status.md` — always current (phase, section count, blockers)
2. `report.md` — completion report with details
3. `outbox.md` — cross-stream messages (write DIRECTLY to destination inbox.md too)
4. Research sections in `research/` directory

The terminal summary is a COURTESY. MEGA reads FILES, not terminals.

## Stream Session End Summary

When a stream finishes a session, its LAST action must be updating `status.md` with readiness:
- GREEN: ready for next run, no blockers
- YELLOW: can run but some items blocked on another stream
- RED: cannot proceed, needs MEGA decision or owner action

---

## Quality Gates

| Gate | Criteria |
|------|----------|
| Build | {BUILD_TOOL} succeeds with zero warnings |
| Tests | All tests pass |
| Lint | {LINT_TOOL} reports no violations |
| Quality | Maintainer subagent PASS before commit |
