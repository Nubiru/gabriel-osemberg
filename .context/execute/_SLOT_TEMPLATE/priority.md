# {SLOT} Priority Queue -- Task Pipeline

{Written by MEGA to direct agent focus. Agent reads this on startup.}
{If this file exists and is non-empty, it overrides self-directed task selection.}

---

## Priority Levels

- **P0 -- CRITICAL**: Drop everything, do this now
- **P1 -- HIGH**: Next task after current completes
- **P2 -- NORMAL**: Standard queue order
- **P3 -- LOW**: Backlog, do when nothing else available

---

## Active Queue

1. **CURRENT** (P{N}): {task title}
   - {Brief description of what to build}
   - Files: `{SOURCE_DIR}/{area}/{name}`
   - Depends on: {dependencies or "None"}
   - Status: IN-PROGRESS | QUEUED

2. **NEXT** (P{N}): {task title}
   - {Brief description}
   - Files: `{SOURCE_DIR}/{area}/{name}`
   - Depends on: {dependencies or "None"}
   - Status: QUEUED

3. **AFTER** (P{N}): {task title}
   - {Brief description}
   - Files: `{SOURCE_DIR}/{area}/{name}`
   - Depends on: {dependencies or "None"}
   - Status: QUEUED

4. **BACKLOG** (P3): {task title}
   - {Brief description}
   - Status: QUEUED

---

## Domain Boundaries

{SLOT} operates within these directories:
- `{SOURCE_DIR}/{domain_area_1}/`
- `{SOURCE_DIR}/{domain_area_2}/`

{SLOT} does NOT touch:
- {list of directories owned by other agents}

---

## Notes from MEGA

{Any strategic context, warnings, or guidance from MEGA.}
{E.g., "Wait for ALPHA to deliver X before starting Y."}
{E.g., "This task has a research dependency -- check manifest.json first."}

---

**Last Updated**: {DATE}
**Updated By**: MEGA
