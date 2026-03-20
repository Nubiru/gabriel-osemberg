# /refactor Command

## Purpose

Analyze code for refactoring opportunities using evidence-based metrics. Identify code smells, propose safe transformations, execute refactors with full test protection.

## Usage

```
/refactor <target>            # Analyze file or function
/refactor <target> --execute  # Analyze AND apply transformations
```

## Targets

```
/refactor src/services/auth.py                    # Analyze entire file
/refactor src/services/auth.py:validate_token     # Analyze specific function
/refactor src/services/                           # Analyze directory
```

---

## Phase 1: MEASURE -- Collect Metrics

Read the target. Compute these metrics for every function/method and for the file overall:

### Function-Level Metrics

| Metric | GREEN | YELLOW | RED |
|--------|-------|--------|-----|
| Lines (body) | < 30 | 30-60 | > 60 |
| Parameters | <= 3 | 4-5 | > 5 |
| Cyclomatic complexity | < 10 | 10-15 | > 15 |
| Nesting depth | <= 3 | 4 | > 4 |
| Local variables | <= 7 | 8-10 | > 10 |

### File-Level Metrics

| Metric | GREEN | YELLOW | RED |
|--------|-------|--------|-----|
| Total lines | < 300 | 300-500 | > 500 |
| Number of functions | < 15 | 15-25 | > 25 |
| Import/include count | <= 5 | 6-8 | > 8 |
| Coupling (external refs) | <= 3 | 4-6 | > 6 |

### Output Format

```
# Refactor Analysis: <target>

## Metrics Summary
File: src/services/auth.py (480 lines -- YELLOW)
Imports: 7 -- YELLOW
Functions: 18

| Function | Lines | Params | Complexity | Depth | Verdict |
|----------|-------|--------|------------|-------|---------|
| validate | 85    | 6      | 18         | 5     | RED     |
| create   | 25    | 2      | 3          | 2     | GREEN   |
| ...      |       |        |            |       |         |
```

---

## Phase 2: SMELL -- Identify Code Smells

Check for these patterns:

### Structural Smells
- **God File**: > 500 lines, does too many things
- **God Function**: > 60 lines, too many responsibilities
- **Long Parameter List**: > 5 params -- candidate for parameter object / config struct
- **Deep Nesting**: > 4 levels -- candidate for guard clauses or extraction
- **Switch/If Chains**: > 5 branches on same variable -- candidate for dispatch table / strategy pattern
- **Repeated Code**: 3+ similar blocks -- candidate for extraction
- **Feature Envy**: Function uses another module's data more than its own
- **Shotgun Surgery**: One change requires touching 5+ files

### General Smells
- **Magic Numbers/Strings**: Literal constants without named definitions
- **Dead Code**: Unreachable code, unused functions, commented-out blocks
- **Excessive Comments**: Comments explaining "what" instead of "why" (code should be self-documenting)
- **Inconsistent Naming**: Mixed conventions within same scope
- **Hidden Dependencies**: Implicit coupling not visible in signatures
- **Primitive Obsession**: Using raw types where a domain type would add safety

### Output Format

```
## Code Smells Found

1. GOD FUNCTION: validate() -- 85 lines, 18 complexity
   Location: src/services/auth.py:120-205
   Impact: Untestable, hard to modify, merge conflict magnet

2. DEEP NESTING: process_request() -- 5 levels deep at line 340
   Location: src/services/auth.py:320-365
   Impact: Cognitive load, error-prone

3. MAGIC NUMBERS: 3600 appears 4 times
   Location: lines 45, 122, 340, 567
   Impact: Maintenance risk, no semantic meaning
```

---

## Phase 3: PRESCRIBE -- Propose Transformations

For each RED or YELLOW smell, propose a specific transformation:

### Extract Function
**When**: Code block does one coherent thing inside a larger function.
**How**: Cut block, create new function with minimal parameters, call it.
**Risk**: LOW -- purely mechanical if tests exist.

### Extract File (Split God File)
**When**: File has 2+ unrelated responsibilities.
**How**: Group related functions, move to new file, update imports.
**Risk**: MEDIUM -- changes dependency graph, may affect build.

### Introduce Guard Clause
**When**: Deep nesting from validation/early-return checks.
**How**: Invert condition, return/continue early, reduce nesting by 1+ levels.
**Risk**: LOW -- logic-preserving.

### Introduce Parameter Object
**When**: 4+ related parameters passed together to multiple functions.
**How**: Bundle into a struct/class/object, pass as single argument.
**Risk**: LOW -- mechanical transformation.

### Replace Magic Number/String
**When**: Literal constant with no semantic name.
**How**: Define as a named constant, enum, or configuration value.
**Risk**: TRIVIAL.

### Introduce Dispatch Table / Strategy Pattern
**When**: Long switch/if chain mapping values to actions.
**How**: Array/map of handlers indexed by key, or strategy objects.
**Risk**: MEDIUM -- changes control flow pattern.

### Output Format

```
## Proposed Transformations (priority order)

### T1: Extract validation logic from validate() (RISK: LOW)
Smell: God Function (85 lines)
Pattern: Extract Function
Action: Move token parsing (lines 130-165) to parse_token()
Result: validate() drops from 85 to 50 lines
Files touched: src/services/auth.py (edit)

### T2: Split auth.py into auth.py + token.py (RISK: MEDIUM)
Smell: God File (480 lines)
Pattern: Extract File
Action: Move token-related functions to src/services/token.py
Result: auth.py drops to ~280 lines, token logic is isolated
Files touched: auth.py (edit), token.py (create), imports in callers
```

---

## Phase 4: EXECUTE (only with --execute flag)

For each approved transformation, follow this exact loop:

```
1. VERIFY    -- Run tests. All pass? Proceed. If not, STOP.
2. SNAPSHOT  -- Note current test count and pass rate.
3. TRANSFORM -- Apply ONE transformation. Smallest possible change.
4. BUILD     -- Compile / lint / type-check. Zero errors? Proceed. If not, REVERT.
5. LINK      -- All imports/references resolve? Proceed. If not, REVERT.
6. TEST      -- Run full test suite. Same pass count? Proceed. If not, REVERT.
7. REPORT    -- Log what changed, metrics before/after.
```

**Rules**:
- ONE transformation at a time. Never batch.
- If any step fails, REVERT immediately. Do not debug during refactor.
- Behavior-preserving ONLY. No new features. No bug fixes. No cleanups beyond the transformation.
- Do not rename functions in public APIs unless introducing a parameter object.
- Keep import/dependency changes minimal -- add what's needed, remove what's orphaned.

---

## Phase 5: REPORT -- Summary

```
## Refactor Report: <target>

### Before
- Lines: 480 | Functions: 18 | Imports: 7
- RED metrics: 3 | YELLOW: 4

### Transformations Applied
1. T1: Extract validation logic -- DONE (validate: 85 -> 50 lines)
2. T2: Split into auth + token -- DONE (auth.py: 480 -> 280 lines)

### After
- Lines: 280 | Functions: 12 | Imports: 5
- RED metrics: 0 | YELLOW: 2

### Tests
- Before: N pass, 0 fail
- After: N pass, 0 fail

### Remaining Opportunities
- process_request() still 45 lines (acceptable, monitor)
```

---

## When NOT to Refactor

- No tests exist for the target code -- write tests FIRST
- Mid-feature development -- finish the feature, then refactor
- Code is scheduled for deletion or major rewrite
- The smell is YELLOW and the code is stable / rarely touched
- Refactoring would change public API used by other modules without clear benefit
