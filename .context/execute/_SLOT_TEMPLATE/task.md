# Task: {TASK_TITLE}

**Agent**: {SLOT}
**Roadmap Reference**: Track {X.Y} -- "{exact item text from roadmap}"
**Date**: {DATE}
**Status**: CLAIMED | IN-PROGRESS | COMPLETE | BLOCKED

## Goal

{1-3 sentences: what this module does, why the roadmap needs it}

## READ FIRST

{List interface/header files that inform your API design}
- `{SOURCE_DIR}/{area}/{existing_module}.{HEADER_EXT}`
- `docs/STATE.md` (check for overlap)

## Files to Create

- `{SOURCE_DIR}/{area}/{name}.{HEADER_EXT}`
- `{SOURCE_DIR}/{area}/{name}.{SOURCE_EXT}`
- `{TEST_DIR}/{area}/test_{name}.{SOURCE_EXT}`

## API

{Types and function signatures. Pure functions. Return values, not pointers.}

```
{LANGUAGE}
// Example:
// {return_type} {module}_{function}({params});
```

## DONE WHEN

- [ ] All functions declared in interface and implemented in source
- [ ] >= {MIN_TESTS_PER_MODULE} tests covering: {list specific scenarios}
- [ ] All tests pass with zero warnings
- [ ] Quality checks pass ({QUALITY_CHECK_RULES})
- [ ] Compiles: `{COMPILE_COMMAND}`
- [ ] No duplication with existing modules (checked against docs/STATE.md)

## Constraints

{Project-specific constraints, e.g.:}
- {LANGUAGE_STANDARD}
- {COMPILE_FLAGS}
- {QUALITY_CHECK_RULES}

## Dependencies

{List modules this task depends on, or "None"}

## Notes

{Any additional context, edge cases, or research needed}
