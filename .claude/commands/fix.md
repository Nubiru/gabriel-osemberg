# /fix Command

## Purpose

Execute simple, obvious fixes without full task expansion. For trivial corrections that don't require comprehensive planning.

## Usage

```
/fix [brief description of fix]
```

## Complexity Threshold

**< 10/100** -- Trivial fixes only (typos, broken links, simple linter errors, formatting issues)

## Examples

```
/fix typo in README line 45 (should be "configuration" not "configration")
/fix broken link in docs/API.md pointing to old domain
/fix missing semicolon in src/utils/parser.js:23
/fix import path should be @/models not ./models
/fix indentation in config.yaml
```

## What Happens

### Step 1: Quick Assessment (5 seconds)

1. Identify fix location (file, line number if provided)
2. Validate fix is safe (no breaking changes, no logic changes)
3. Check complexity (must be < 10/100 trivial threshold)
4. If complex (> 10), escalate to `/task` command

### Step 2: Immediate Execution (10-30 seconds)

1. Read target file
2. Apply fix (search and replace or edit)
3. Quick validation (syntax check, no regressions)
4. Conversational confirmation

### Step 3: Validation (5 seconds)

1. Confirm fix applied correctly
2. Check for side effects (imports still valid, references intact)
3. Report success

**Total Time**: < 1 minute for most fixes

## Output

**Chat Response** (conversational, no files created):

```
Fixed typo in README.md line 45
   - Changed: "configration" -> "configuration"
   - Validated: No side effects
```

**No Task Specification** -- Lightweight, immediate execution

## When to Use

- Typo correction (docs, comments, strings)
- Broken link fixes (documentation, config files)
- Simple linter errors (missing semicolon, trailing comma)
- Import path corrections (straightforward updates)
- Formatting fixes (indentation, whitespace)
- Simple variable renames (localized, no breaking changes)
- Comment updates (clarifications, corrections)

## When NOT to Use

- Logic changes (use `/task` for proper analysis)
- Multiple files affected (use `/task` for coordination)
- Breaking changes possible (use `/task` with full testing)
- Architectural changes (use `/plan` for design)
- Requires testing (use `/task` with test strategy)
- Uncertain about impact (use `/task` for safety)

## Decision Logic

```
User requests fix
  |
Is complexity < 10? (trivial)
  | YES
Execute immediately with /fix
  | NO
Escalate to /task (full expansion)
```

## Validation Criteria

**Before Executing Fix**:

- [ ] Fix location identified (file + line/section)
- [ ] Change is trivial (typo, link, formatting only)
- [ ] No logic changes involved
- [ ] No breaking changes possible
- [ ] Single file affected (or localized changes)

**After Executing Fix**:

- [ ] Syntax valid (no errors introduced)
- [ ] References still valid (imports, links)
- [ ] Formatting consistent
- [ ] User confirmation received

## Anti-Patterns

### Anti-Pattern 1: Logic Changes via /fix

**Problem**: Using `/fix` for business logic changes
**Example**: `/fix user authentication to use JWT instead of sessions`
**Why Bad**: Requires architecture design, testing, migration strategy
**Solution**: Use `/task` with full expansion

### Anti-Pattern 2: Multi-File Changes via /fix

**Problem**: Using `/fix` for changes spanning multiple files
**Example**: `/fix import paths across entire src/ directory`
**Why Bad**: Requires coordination, validation across modules
**Solution**: Use `/task` for systematic refactoring

### Anti-Pattern 3: Uncertain Impact

**Problem**: Using `/fix` when unsure of consequences
**Example**: `/fix this error in database migration script`
**Why Bad**: High-risk changes need full analysis
**Solution**: Use `/task` with proper testing strategy

## Quick Reference

| Fix Type     | Example                          | Time | Risk |
| ------------ | -------------------------------- | ---- | ---- |
| Typo         | "configration" -> "configuration"| 10s  | None |
| Broken link  | old.com -> new.com               | 15s  | None |
| Linter error | Add semicolon                    | 10s  | None |
| Import path  | ./models -> @/models             | 20s  | Low  |
| Formatting   | Fix indentation                  | 15s  | None |
| Comment      | Update outdated comment          | 10s  | None |
