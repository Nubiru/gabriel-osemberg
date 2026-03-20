# /stream — Experience Stream Orchestrator

**Purpose**: Run a stream orchestrator session. Each stream owns one domain of {PROJECT_NAME} and progresses through: Research -> Roadmap -> Build -> Polish.

**Arguments**: `$ARGUMENTS`

---

## Mode Detection

Parse the first argument:
- `{STREAM_NAME}` (one of your defined streams) -> **RUN STREAM**
- (no args) -> **SHOW HELP** — list available streams

NOTE: Status checking is done by MEGA, not by `/stream`. MEGA reads all status.md files in its own session where file changes are visible.

---

## Run Stream

You are the **$ARGUMENTS** stream orchestrator.

### Startup

1. Read `CLAUDE.md` — project rules
2. Read `SOUL.md` — project spirit and vision (if applicable)
3. Read `.context/streams/PROTOCOL.md` — coordination protocol
4. Read `.context/streams/BOOTSTRAP.md` — stream agent rules
5. Read `.context/streams/$ARGUMENTS/status.md` — your current state
6. Read `.context/streams/$ARGUMENTS/inbox.md` — pending messages
7. Read `.context/streams/$ARGUMENTS/tracks.md` — your owned tracks

### Phase 0: Research

If status shows Phase 0 (RESEARCH):

1. Check which research sections (001-007) are complete
2. Pick the next incomplete section
3. Use the unified research skeleton:
   - 001: Inventory — scan codebase for modules in your domain
   - 002: World Survey — research 5+ best-in-class examples (use web search)
   - 003: Gap Analysis — compare inventory vs vision, categorize gaps
   - 004: Questions — answer open questions from tracks.md (research each)
   - 005: Roadmap — design 4 layers (L0 Foundation -> L3 Perfection)
   - 006: Dependencies — what you need from / provide to other streams
   - 007: Resources — map reference materials + identify gaps
4. Write the section to `.context/streams/$ARGUMENTS/research/00N-*.md`
5. Update `status.md` with section count
6. If you discover cross-stream needs, write to destination's `inbox.md` and your `outbox.md`
7. Continue to next section or exit if 3+ sections complete this session

### Phase 2: Build

If status shows Phase 2 (BUILD):

Follow the standard 6-phase loop from PROTOCOL.md:
1. EVALUATE — read roadmap for unchecked items in your tracks
2. PLAN — write task.md
3. EXECUTE — launch Writer subagent (TDD)
4. VALIDATE — launch Checker subagent
5. MAINTAIN — launch Maintainer subagent
6. DOCUMENT — write report.md, commit + push

### Exit Conditions

- 3+ research sections completed this session
- 3+ code tasks completed this session
- Blocked on external dependency (write to outbox.md, update status.md)
- All work in current phase complete

---

## Status (handled by MEGA)

Status checking is done by MEGA. Use your primary session command to see all stream states. MEGA reads files from its own persistent session where updates are always visible.
