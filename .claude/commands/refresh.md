# /refresh -- Update Project Metrics & Docs

**Purpose**: Scan the codebase, update `docs/METRICS.md` with current counts, refresh stale docs, and report project health.

**Arguments**: `$ARGUMENTS`

---

## Mode Detection

```
Parse $ARGUMENTS:
- empty or "all"    -> Full refresh (metrics + docs + health check)
- "metrics"         -> Update METRICS.md only
- "docs"            -> Check doc freshness only
- "health"          -> Run health verification only
```

---

## Step 1: Scan Codebase

Run these scans (all read-only). Adapt the file extensions and paths to your project:

```bash
# Source file counts (adjust extensions: *.py, *.ts, *.go, *.rs, *.c, etc.)
find src -name "{SOURCE_EXT}" | wc -l
find src -name "{SOURCE_EXT}" | xargs wc -l | tail -1

# Test file counts
find tests -name "{TEST_PATTERN}" | wc -l
find tests -name "{TEST_PATTERN}" | xargs wc -l | tail -1

# Test results (adapt to your test runner: pytest, jest, go test, cargo test, etc.)
{TEST_COMMAND} 2>&1 | grep -E "{TEST_SUMMARY_PATTERN}"

# Dependencies
{DEPENDENCY_COUNT_COMMAND}   # e.g., wc -l < requirements.txt, jq '.dependencies | length' package.json

# Build artifact size (if applicable)
ls -la {BUILD_OUTPUT} 2>/dev/null | awk '{print $5}'

# TODOs in code
grep -rn "TODO\|FIXME\|HACK\|XXX" src/ tests/ --include="{SOURCE_EXT}" | wc -l
```

---

## Step 2: Update docs/METRICS.md

Write the scanned data into `docs/METRICS.md`. Include these sections:

1. **Codebase** -- file counts, line counts, module counts
2. **Testing** -- test count, pass/fail, coverage percentage (if available)
3. **Dependencies** -- direct dependency count, outdated count
4. **Build** -- build status, artifact sizes, build time
5. **Technical Debt** -- TODO count, known issues, deprecation warnings
6. **Architecture** -- pure vs stateful module ratio, dependency graph health

Add a timestamp and comparison to previous values where possible.

---

## Step 3: Check Doc Freshness

Read key documentation files and flag any that seem stale:

- `docs/ROADMAP.md` -- does current phase match actual progress?
- `docs/ARCHITECTURE.md` -- does it describe the actual current structure?
- `docs/DECISIONS.md` -- are recent decisions recorded?
- `README.md` -- are setup instructions still accurate?
- `CHANGELOG.md` -- is it up to date? (if used)
- Any other project-specific docs that track living state

Report mismatches found between docs and reality.

---

## Step 4: Health Verification

Quick sanity checks:

```bash
# Build succeeds
{BUILD_COMMAND}

# Tests pass
{TEST_COMMAND}

# Linter clean (if applicable)
{LINT_COMMAND}

# No secrets in tracked files
grep -rn "password\|secret\|api_key\|token" src/ --include="{SOURCE_EXT}" | grep -v "test\|mock\|example"
```

---

## Step 5: Output Summary

Print a compact dashboard:

```
# Project Refresh -- YYYY-MM-DD

Code:       N source files | N lines
Tests:      N tests | N pass | N fail
Deps:       N direct dependencies
Build:      {STATUS} ({SIZE} artifact)
TODOs:      N
Debt:       [LOW | MEDIUM | HIGH]

Stale docs: [none | list of issues]
Health:     [CLEAN | list of issues]
```

---

## What This Tracks (and Why)

| Metric | Why it matters |
|--------|---------------|
| Lines of code | See the project grow; catch bloat early |
| Test count | Confidence grows with tests |
| Test failures | Catch regressions immediately |
| TODO count | Technical debt visibility |
| Dependency count | Supply chain awareness |
| Build artifact size | Deployment performance |
| Doc freshness | Docs that lie are worse than no docs |

---

## When to Use

- Start of every session (quick health check)
- After completing a milestone (measure progress)
- Before committing (verify clean state)
- When onboarding someone (show project overview)

## When NOT to Use

- Mid-implementation (just keep coding)
- Nothing has changed since last refresh
