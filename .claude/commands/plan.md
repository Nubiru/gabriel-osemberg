# /plan Command

## Purpose

Design architecture for a new module or system, producing an Architecture Decision Record and implementation plan.

## Usage

```
/plan [module/system to design]
```

## Examples

```
/plan authentication module
/plan database migration system
/plan notification pipeline
/plan caching layer
/plan file upload handler
```

## What Happens

### Step 1: Context Gathering

1. Read `docs/ROADMAP.md` -- which phase does this belong to?
2. Read architecture docs (e.g., `docs/ARCHITECTURE.md`) -- how does it fit the existing structure?
3. Read relevant existing source files in the affected area
4. Check `docs/DECISIONS.md` for related past decisions

### Step 2: Architecture Design

1. Define the module's responsibility (single responsibility principle)
2. Identify layer placement -- where does it fit in the project's architecture?
3. Design data structures (types, schemas, models)
4. Define public API (function/method signatures, interfaces, contracts)
5. Identify dependencies (which other modules are required)
6. Plan resource strategy (memory, connections, file handles, cleanup)

### Step 3: Test Strategy

Before designing implementation, define:
- What unit tests prove the design works?
- What reference values or fixtures will be used?
- What edge cases must be handled?
- What integration points need testing?

### Step 4: ADR Creation

Write a decision record in `docs/DECISIONS.md`:
- **Title**: Short description of the decision
- **Status**: Proposed / Accepted / Deprecated / Superseded
- **Context**: What prompted this decision
- **Decision**: What was decided and why
- **Alternatives considered**: Other approaches evaluated
- **Consequences**: Positive and negative impacts

### Step 5: Implementation Plan

Break into ordered subtasks:
- Each subtask specifies exact file paths to create or modify
- Each subtask specifies function/method signatures
- Each subtask has a test criterion (how to verify it works)
- Dependencies between subtasks are explicit

## Output

- ADR entry appended to `docs/DECISIONS.md`
- Implementation plan written to `.context/active/right-now/PLAN_{name}.md` (or shared in chat if `.context/` is not used)

## When to Use

- Designing a new module from scratch
- Making significant architecture choices
- Starting a new roadmap phase
- Introducing a new dependency or integration

## When NOT to Use

- Simple function additions (use `/task`)
- Bug fixes (use `/fix`)
- Refactoring existing code (use `/refactor`)
