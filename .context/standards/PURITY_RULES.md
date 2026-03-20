# Purity Rules

Rules that define "clean architecture" for this codebase. Verified by automated grep/lint checks.

---

## Concept

The codebase is divided into **Pure Zones** (no side effects allowed) and **Stateful Zones** (controlled mutation). Purity rules are grep-checkable constraints that enforce this boundary. Violations are caught in CI or by agent audit commands.

---

## Pure Zones (MUST NOT violate)

<!-- CUSTOMIZE: List your pure zone directories. These are where domain logic, algorithms, and business rules live. -->

Directories where code must be stateless and side-effect-free:

```
{PURE_ZONE_DIRS}
```

<!-- CUSTOMIZE EXAMPLES:
Python:  src/domain/, src/services/
React:   src/domain/, src/utils/, src/hooks/pure/
Rust:    src/domain/, src/logic/
Go:      internal/domain/, internal/rules/
C:       src/math/, src/systems/
-->

### Rule P1: No I/O in pure zones  [CRITICAL]

```
FORBIDDEN in pure zones: {FORBIDDEN_IO_PATTERNS}
```

Pure zones must not perform any input/output operations -- no file reads, no network calls, no database queries, no console output (except under debug flags).

| Stack | Forbidden Patterns |
|-------|-------------------|
| Python | `open(`, `requests.`, `urllib.`, `sqlite3.`, `print(` (unless `DEBUG`), `logging.` |
| TypeScript/React | `fetch(`, `axios.`, `console.log` (unless `DEBUG`), `fs.`, `process.env` |
| Rust | `std::fs::`, `std::net::`, `reqwest::`, `println!` (unless `#[cfg(debug)]`), `tokio::` |
| Go | `os.Open`, `http.Get`, `sql.Open`, `fmt.Print` (unless debug build tag) |

### Rule P2: No mutable global state in pure zones  [CRITICAL]

```
FORBIDDEN in pure zones: {FORBIDDEN_GLOBAL_PATTERNS}
```

Pure functions take inputs and return outputs. No reading or writing global/module-level mutable state.

| Stack | Forbidden Patterns |
|-------|-------------------|
| Python | Module-level mutable variables, `global` keyword, class-level mutable attributes |
| TypeScript | `let` at module scope (mutable), direct `window.` or `document.` access |
| Rust | `static mut`, `lazy_static!` with interior mutability in domain modules |
| Go | Package-level `var` (mutable), `sync.Mutex` in domain packages |

### Rule P3: No framework/infrastructure imports in pure zones  [CRITICAL]

```
FORBIDDEN in pure zones: {FORBIDDEN_FRAMEWORK_PATTERNS}
```

Domain logic must not depend on frameworks, ORMs, or infrastructure libraries.

| Stack | Forbidden Imports |
|-------|------------------|
| Python | `django.`, `flask.`, `sqlalchemy.`, `celery.`, `boto3.` |
| TypeScript/React | `react` (in domain/), `express`, `prisma`, `@aws-sdk/` |
| Rust | `actix`, `diesel`, `sqlx`, `tokio` (in domain modules) |
| Go | `net/http`, `database/sql`, `github.com/gin-gonic/` (in domain packages) |

### Rule P4: No heap allocation in hot paths  [WARNING]

```
FORBIDDEN in performance-critical pure zones: {FORBIDDEN_ALLOCATION_PATTERNS}
```

<!-- CUSTOMIZE: This rule is most relevant for systems programming (C, C++, Rust, Go). For higher-level languages, replace with "no unnecessary object creation in hot loops." -->

In performance-critical code, avoid dynamic allocation. Prefer stack allocation, pre-allocated buffers, or arena patterns.

### Rule P5: No application state references in pure zones  [CRITICAL]

```
FORBIDDEN in pure zones: {FORBIDDEN_STATE_PATTERNS}
```

Pure zones must never reference the application's global state object, store, context, or singleton.

| Stack | Forbidden References |
|-------|---------------------|
| Python | `app.state`, `g.`, `current_app`, `request` (Flask/Django globals) |
| TypeScript/React | `useContext(AppState)`, `store.getState()`, `window.__STATE__` |
| Rust | `APP_STATE`, any `static` singleton reference |
| Go | `appState`, any package-level `*App` reference |

---

## Stateful Zones (controlled mutation)

<!-- CUSTOMIZE: List your stateful zone directories. These are where I/O, framework integration, and state management live. -->

Directories where state mutation is allowed but controlled:

```
{STATEFUL_ZONE_DIRS}
```

<!-- CUSTOMIZE EXAMPLES:
Python:  src/infrastructure/, src/api/, src/config/
React:   src/components/, src/pages/, src/services/, src/api/
Rust:    src/api/, src/db/, src/main.rs
Go:      cmd/, internal/api/, internal/db/
C:       src/core/, src/render/, src/platform/, src/ui/
-->

### Rule S1: Centralized state  [CRITICAL]

```
ALLOWED: exactly {MAX_STATE_OBJECTS} mutable state container(s)
CHECK: grep for state object references across codebase -- must be centralized
```

All mutable application state lives in a single, well-defined location. No scattered globals.

### Rule S2: Platform code behind abstractions  [WARNING]

```
REQUIRED: platform-specific APIs wrapped in clean interfaces
CHECK: direct platform API calls only in platform/ or infrastructure/ directories
```

Platform-specific code (OS APIs, browser APIs, cloud SDKs) must be isolated behind interfaces so the rest of the codebase does not depend on a specific platform.

---

## Test Coverage Rules

### Rule T1: Every pure module has a test file  [CRITICAL]

```
For each {PURE_ZONE}/{module} -> {TEST_DIR}/{module}/test_{module} must exist
```

Every pure zone module must have corresponding tests. No exceptions.

### Rule T2: Zero test failures  [CRITICAL]

```
{TEST_COMMAND} must exit with code 0
All test suites must report 0 failures
```

The test suite must pass completely before any code is merged or deployed.

---

## Code Hygiene

### Rule H1: No naked TODOs  [WARNING]

```
WARN: TODO, FIXME, HACK, XXX without a ticket/issue reference
VALID: TODO(ISSUE-42), FIXME(#123), HACK(JIRA-456)
```

Every deferred work item must be tracked. Naked TODOs rot.

### Rule H2: No unused imports/includes  [WARNING]

```
WARN: imports/includes that are not referenced in the file
CHECK: language-specific linter or manual review
```

---

## Severity Levels

| Level | Meaning | Action |
|-------|---------|--------|
| CRITICAL | Architecture boundary violated | Must fix before merge. Blocks CI. |
| WARNING | Code quality concern | Note in review. Fix in next cleanup pass. |

---

## How to Check (Automation)

<!-- CUSTOMIZE: Replace with your actual check commands. -->

```bash
# P1: No I/O in pure zones
grep -rn "{FORBIDDEN_IO_PATTERN_1}\|{FORBIDDEN_IO_PATTERN_2}" {PURE_ZONE_DIR}/
# Expected: empty (0 matches)

# P2: No mutable globals in pure zones
grep -rn "{FORBIDDEN_GLOBAL_PATTERN}" {PURE_ZONE_DIR}/
# Expected: empty (0 matches)

# P3: No framework imports in pure zones
grep -rn "{FORBIDDEN_FRAMEWORK_PATTERN}" {PURE_ZONE_DIR}/
# Expected: empty (0 matches)

# P5: No app state in pure zones
grep -rn "{FORBIDDEN_STATE_PATTERN}" {PURE_ZONE_DIR}/
# Expected: empty (0 matches)

# H1: No naked TODOs
grep -rn "TODO\|FIXME\|HACK\|XXX" {SOURCE_DIR}/ | grep -v "TODO("
# Expected: empty (all TODOs have ticket references)

# T2: Tests pass
{TEST_COMMAND}
# Expected: exit code 0
```

---

## Examples by Stack

### Python: No DB imports in domain logic

```
PURE ZONE: src/domain/
FORBIDDEN: import sqlalchemy, import django.db, from myapp.models import
RATIONALE: Domain logic must not know about persistence. Use repository interfaces.
```

### React: No API calls in components

```
PURE ZONE: src/components/ (presentational)
FORBIDDEN: fetch(, axios., useSWR(, useQuery(
RATIONALE: Components render UI. Data fetching belongs in pages/, hooks/, or services/.
```

### Rust: No unsafe in domain modules

```
PURE ZONE: src/domain/
FORBIDDEN: unsafe {, unsafe fn, unsafe impl
RATIONALE: Domain logic must be memory-safe by construction. Unsafe code belongs in platform/ wrappers.
```

### Go: No HTTP in domain packages

```
PURE ZONE: internal/domain/
FORBIDDEN: net/http, gin.Context, echo.Context
RATIONALE: Domain logic must not know about HTTP. Use handler functions in internal/api/.
```
