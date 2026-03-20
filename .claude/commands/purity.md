# /purity -- Architecture Purity Audit

**Purpose**: Verify that architectural boundaries are respected. Pure zones stay pure, dependency direction is correct, no side effects leak into pure code.

**Arguments**: `$ARGUMENTS`

---

## Concept

Most well-structured projects separate **pure logic** (deterministic, no side effects, easily testable) from **stateful/impure code** (I/O, global state, external services, UI). This command audits that separation.

Configure the zones and rules below for your project:

- **`{PURE_ZONES}`**: Directories containing pure, side-effect-free code (e.g., `src/utils/`, `src/domain/`, `src/lib/`)
- **`{STATEFUL_ZONES}`**: Directories containing stateful/impure code (e.g., `src/api/`, `src/db/`, `src/ui/`, `src/main/`)
- **`{FORBIDDEN_PATTERNS}`**: Patterns that must not appear in pure zones (e.g., database imports, HTTP calls, global state access, filesystem I/O)

---

## Mode Detection

```
Parse $ARGUMENTS:
- empty or "all"  -> Full audit (all rules)
- "quick"         -> Core purity rules only (fast)
- "fix"           -> Audit + suggest fixes for violations
```

---

## Step 1: Pure Zone Checks

For each rule, search `{PURE_ZONES}` for violations:

### P1: No external service dependencies in pure code
Search for imports/references to databases, HTTP clients, message queues, file systems, or other external services.

### P2: No mutable global state
Search for global/module-level mutable variables, singletons with mutable state, or shared state not passed through function parameters.

### P3: No direct I/O
Search for filesystem reads/writes, network calls, stdin/stdout (except logging at debug level), or any operation that reaches outside the process boundary.

### P4: No environment coupling
Search for environment variable access, system clock reads, random number generation, or any source of non-determinism not injected through parameters.

### P5: No dependency direction violations
Pure code must not import from stateful zones. Verify import/include direction flows correctly: stateful depends on pure, never the reverse.

Each check: **PASS** if no violations, **FAIL** with file:line if violations found.

---

## Step 2: Stateful Zone Checks

### S1: State is centralized
Count mutable global/singleton state instances. Flag if state is scattered across many files instead of centralized in designated locations.

### S2: Side effects are explicit
Verify that functions with side effects are clearly named or documented (e.g., `save_`, `send_`, `write_`, `delete_` prefixes, or documented in signatures/docstrings).

---

## Step 3: Test Coverage Check

### T1: Pure modules have tests
For each file in `{PURE_ZONES}`, verify a corresponding test file exists.

### T2: Zero test failures
Run the test suite and confirm zero failures.

---

## Step 4: Code Hygiene

### H1: Naked TODOs
Search for `TODO`, `FIXME`, `HACK`, `XXX` in source and test files. Report count and locations.

---

## Step 5: Score and Report

Count violations per category and output:

```
# Purity Audit -- YYYY-MM-DD

## Pure Zones ({PURE_ZONES})
P1 No external deps:     PASS
P2 No mutable globals:   PASS
P3 No direct I/O:        PASS (or FAIL: N occurrences)
P4 No env coupling:      PASS
P5 No direction errors:  PASS

## Stateful Zones ({STATEFUL_ZONES})
S1 State centralized:    PASS
S2 Side effects named:   PASS

## Tests
T1 Coverage:             N/N pure modules have tests
T2 Failures:             0

## Hygiene
H1 Naked TODOs:          N

## Score: N/10 rules passing
```

---

## When to Use

- Before committing (quick sanity check)
- After creating new modules (verify correct placement)
- After refactoring (verify nothing leaked across boundaries)
- Periodic architecture health checks

## When NOT to Use

- Mid-implementation of a single function
- When you just want metrics (use `/refresh` instead)
