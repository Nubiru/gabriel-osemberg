# Architecture

**Version**: 1.0

---

## System Overview

<!-- CUSTOMIZE: Replace this diagram with your actual architecture. The principle remains: pure core, stateful shell. -->

```
                    {DELIVERY_LAYER}
                         |
                    [{RUNTIME}]
                         |
              +----------+----------+
              |          |          |
         {LAYER_1}   {LAYER_2}   {LAYER_3}
              |          |          |
              +-----+----+         |
                    |              |
              Pure Core       Platform
              (no side effects) Layer
```

---

## Core Principle: Functional Core / Imperative Shell

The architecture separates code into two zones:

**Pure Core** -- Functions that take inputs and return outputs. No side effects, no global state, no I/O. Easy to test, easy to reason about, easy to reuse.

**Imperative Shell** -- Code that talks to the outside world: databases, APIs, file systems, UI frameworks, hardware. This layer orchestrates pure functions and manages state transitions.

```
+--------------------------------------------------+
|              Imperative Shell                     |
|  (I/O, state management, framework glue)         |
|                                                  |
|    +--------------------------------------+      |
|    |          Pure Core                   |      |
|    |  (domain logic, algorithms, rules)   |      |
|    |  No side effects. No dependencies.   |      |
|    +--------------------------------------+      |
|                                                  |
+--------------------------------------------------+
```

**The dependency arrow always points INWARD**: Shell depends on Core. Core depends on nothing.

---

## Layer Architecture

### Pure Core (no side effects)

<!-- CUSTOMIZE: Replace with your actual pure zone directories. -->

**Location**: `{PURE_ZONES}`
**Purpose**: Domain logic, algorithms, business rules, data transformations
**Rules**:
- Pure functions only -- same input always produces same output
- No side effects (no I/O, no network, no database, no file system)
- No global/mutable state
- No framework imports
- No dependencies on the shell layer

### Imperative Shell (controlled state)

<!-- CUSTOMIZE: Replace with your actual stateful zone directories. -->

**Location**: `{STATEFUL_ZONES}`
**Purpose**: I/O operations, state management, framework integration, external service calls
**Rules**:
- Orchestrates pure core functions
- Manages state transitions
- Handles all external interactions
- Keeps mutation controlled and traceable

### Example Layouts by Stack

**Python (Django/FastAPI)**:
```
src/
  domain/          # PURE: business rules, value objects, entities
  services/        # PURE: application logic, use cases
  infrastructure/  # STATEFUL: database, APIs, file I/O
  api/             # STATEFUL: HTTP handlers, serialization
  config/          # STATEFUL: settings, env vars
```

**TypeScript (React + Node)**:
```
src/
  domain/          # PURE: business logic, types, validators
  utils/           # PURE: helper functions, transformations
  hooks/           # MIXED: React hooks (some pure, some stateful)
  components/      # STATEFUL: React components (UI side effects)
  services/        # STATEFUL: API calls, database access
  pages/           # STATEFUL: route handlers, data fetching
```

**Rust**:
```
src/
  domain/          # PURE: structs, enums, impl blocks (no I/O)
  logic/           # PURE: algorithms, computations
  db/              # STATEFUL: database queries, connections
  api/             # STATEFUL: HTTP handlers, middleware
  main.rs          # STATEFUL: entry point, wiring
```

---

## Data Flow

<!-- CUSTOMIZE: Replace with your project's actual data flow. -->

```
External Input ({INPUT_SOURCE})
     |
     v
{CANONICAL_REPRESENTATION} (internal data format)
     |
     +---> Domain A: {DOMAIN_A_DESCRIPTION}
     +---> Domain B: {DOMAIN_B_DESCRIPTION}
     +---> Domain C: {DOMAIN_C_DESCRIPTION}
     |
     v
{OUTPUT_LAYER}: composes domain outputs
     |
     v
{DELIVERY_TARGET} ({DELIVERY_DESCRIPTION})
```

---

## Dependency Rules

| Layer | Can depend on | Cannot depend on |
|-------|--------------|-----------------|
| Pure Core | Nothing (self-contained) | Shell, Platform, Framework |
| Domain Services | Pure Core | Shell, Platform, Framework |
| Shell / Infrastructure | Pure Core, Domain Services | (it's the outer layer) |
| Platform / Framework | Pure Core (optionally) | Domain-specific logic |
| Entry Point / Orchestrator | Everything | (it wires the layers) |

**The rule is simple**: Dependencies point inward. Inner layers never import from outer layers.

---

## Module Boundaries

- Each domain/module is an independent unit
- Modules communicate through well-defined interfaces (function signatures, types, protocols)
- No module reaches into another module's internals
- Shared code lives in a dedicated `shared/` or `common/` directory, NOT duplicated

---

## Shared State Policy

<!-- CUSTOMIZE: Adapt to your project's state management needs. -->

- **Prefer no shared mutable state**. Pass data through function arguments and return values.
- If shared state is unavoidable, centralize it:
  - Exactly **one** state container (store, context, global) managed by the shell layer
  - Pure core functions never read or write shared state directly
  - State is passed INTO pure functions as arguments, and new state is returned as output
- **Never scatter global variables** across modules

---

## Key Design Principles

1. **Canonical internal format**: All domains receive the same internal representation, convert as needed
2. **No global state in pure core**: Pure functions enable testing and reuse
3. **Each domain is independent**: Can be developed and tested in isolation
4. **Stack allocation / value semantics preferred**: Minimize heap/dynamic allocation in hot paths
5. **Platform isolation**: Platform-specific code lives behind clean interfaces
