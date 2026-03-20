# {PROJECT_NAME} — State Inventory

**Last Updated**: {DATE}

This document tracks what is **pure** (stateless) and what is **stateful** in the codebase. The goal: maximize pure code, minimize and isolate mutable state.

---

## Architecture Pattern: Functional Core, Imperative Shell

```
 PURE CORE (stateless)          IMPERATIVE SHELL (stateful)
 ---------------------          ---------------------------
 Input -> Output                Mutates state, side effects
 No side effects                I/O, database, network, UI
 Fully testable                 Tested via integration
 ---------------------          ---------------------------
 {PURE_DIRECTORIES}             {STATEFUL_DIRECTORIES}
```

---

## State Container

**{STATE_DESCRIPTION}**: How is mutable state organized? (e.g., single state struct, Redux store, database, etc.)

---

## Pure Modules ({PURE_COUNT} total)

### {DOMAIN_1} ({MODULE_COUNT} modules)
{MODULE_LIST}

### {DOMAIN_2} ({MODULE_COUNT} modules)
{MODULE_LIST}

<!--
List all pure modules grouped by domain/directory.
A module is pure if it:
- Takes input, returns output
- Has no side effects
- Does not mutate global state
- Does not perform I/O
-->

---

## Stateful Modules ({STATEFUL_COUNT} total)

| Module | Side effects | Guarded by |
|--------|-------------|-----------|
| {MODULE} | {WHAT_IT_MUTATES} | {GUARD_MECHANISM} |

<!--
List all stateful modules. For each one, document:
- What side effects it has
- How those side effects are guarded/isolated
-->

---

## Dependency Rules (enforced)

```
PURE code must NEVER:
  - {PURITY_RULE_1}
  - {PURITY_RULE_2}
  - {PURITY_RULE_3}

STATEFUL code:
  - {STATEFUL_RULE_1}
  - {STATEFUL_RULE_2}
```

---

## How to Verify Purity

```bash
# Example verification commands:
# {VERIFICATION_COMMAND_1}
# {VERIFICATION_COMMAND_2}
```

---

*Updated by `/purity` command. Keep this document in sync with the actual codebase.*
