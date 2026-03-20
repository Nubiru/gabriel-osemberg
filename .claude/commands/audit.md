# /audit Command

## Purpose

Run quality gate validation against the project's quality standards. Reports pass/fail status for each gate and provides actionable recommendations.

## Usage

```
/audit [scope]
```

## Scopes

```
/audit all          # All quality gates
/audit build        # Compilation and build gates only
/audit tests        # Test gates only
/audit lint         # Code quality and linting gates only
/audit phase N      # Check Phase N roadmap criteria
```

## What Happens

### Step 1: Read Quality Standards

Read `{QUALITY_GATES_PATH}` for gate definitions.
Read `{CONVENTIONS_PATH}` for coding standards.

### Step 2: Run Applicable Gates

**Build Gate**:
- [ ] Does the project build successfully with zero warnings?
- [ ] Do all build targets succeed?

**Test Gate**:
- [ ] Do all tests pass?
- [ ] Are critical paths covered by tests?

**Code Quality Gate**:
- [ ] Follows project conventions?
- [ ] No lint errors?
- [ ] Dependencies properly managed?

**Performance Gate** (when applicable):
- [ ] Meets defined performance targets?
- [ ] No known performance regressions?

**Security Gate** (when applicable):
- [ ] No known vulnerabilities in dependencies?
- [ ] Sensitive data properly handled?

### Step 3: Report

```
# Quality Audit

Score: X/Y gates passed

## Build
- [x] Build succeeds: PASS
- [ ] Zero warnings: FAIL (3 warnings found)

## Tests
- [x] All tests pass: PASS (42 tests, 0 failures)

## Code Quality
- [x] Conventions followed: PASS
- [ ] No lint errors: FAIL (2 issues)

## Recommendations
1. Fix 3 compiler warnings in {FILE}
2. Resolve lint errors in {FILE}
```

## When to Use

- Before committing significant changes
- After completing a roadmap phase or milestone
- Periodic quality check
- Before a release or deployment
- After major refactoring

## Output Constraints

- Use checkboxes `[x]` / `[ ]` for each gate
- Show PASS / FAIL clearly for each item
- Include specific file paths and line numbers for failures
- Provide actionable recommendations, not just status
- Keep output concise and scannable

## Tool Permissions

- **Read**: All project files, standards, conventions
- **Execute**: Build commands, test runners, linters (read-only analysis)
- **Write**: None (audit is read-only, reports inline)

---

**Command Type**: Quality Validation
