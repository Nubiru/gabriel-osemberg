# Maintainer Health Sweep -- {TASK_TITLE}

**Date**: {DATE}
**Agent**: {SLOT}
**Verdict**: PASS | WARN | CRITICAL

---

## Regression Gate

{TEST_COMMAND} result:
{N}/{N} tests pass, 0 failures. {Clean rebuild | Incremental build}.

{If FAIL: describe which tests broke and why. Return CRITICAL immediately.}

---

## Quality Audit

{Results of quality checks on all files created/touched by Writer.}

| Check | Result | Details |
|-------|--------|---------|
| {QUALITY_RULE_1} | PASS/FAIL | {details} |
| {QUALITY_RULE_2} | PASS/FAIL | {details} |
| {QUALITY_RULE_3} | PASS/FAIL | {details} |

{If any FAIL: CRITICAL}

---

## Naming/Style Compliance

- {NAMING_CONVENTION_1}: PASS/FAIL
- {NAMING_CONVENTION_2}: PASS/FAIL
- Interface guards / module boundaries: PASS/FAIL

---

## Dead Code Scan

{Every function declared in the interface is called somewhere (implementation or test).}
- Orphan functions found: {list or "None"}
{If orphan found: WARN}

---

## TODO Audit

{Search for TODO/FIXME/HACK/XXX in new files.}
- Naked TODOs (without ticket/issue reference): {count or "None"}
{If naked TODOs found: WARN}

---

## Duplication Check

{Search docs/STATE.md for modules with similar names/signatures.}
- Potential overlaps found: {list or "None"}
{If potential overlap found: WARN}

---

## Build System Sync

- New source files registered: YES/NO/N-A
- New test targets registered: YES/NO/N-A
- Phantom files (registered but missing on disk): {list or "None"}
{If phantom files found: CRITICAL}

---

## Attribution Check

{If module references an external algorithm, verify contributor is in {CONTRIBUTORS_FILE}.}
- External sources used: {list or "None"}
- Attribution status: COMPLETE / MISSING
{If MISSING: WARN}

---

## Metrics Snapshot

| Metric | Count |
|--------|-------|
| Total test suites | {N} |
| Total test functions | {N} |
| Source files ({SOURCE_DIR}/) | {N} |
| Interface files ({SOURCE_DIR}/) | {N} |

---

## Findings Summary

**Blocking (CRITICAL)**:
{List or "None"}

**Non-blocking (WARN)**:
{List or "None"}

**Clean (PASS)**:
{List of checks that passed cleanly}

---

## Recommendations

{Any non-blocking improvements for future tasks.}
{E.g., "Consider extracting helper function X for reuse."}
{E.g., "Test coverage could be improved for edge case Y."}
