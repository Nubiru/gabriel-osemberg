# Quality Gates

**Version**: 1.0

Machine-checkable quality gates for the development pipeline. Run these after every implementation cycle before merging or deploying.

---

## Gate Definitions

<!-- CUSTOMIZE: Replace placeholder values with your project's actual commands, thresholds, and tools. -->

### Universal Gates (apply to every project)

| # | Gate | Check | Threshold | Tool | Severity |
|---|------|-------|-----------|------|----------|
| G1 | Compilation / Build | `{BUILD_COMMAND}` | 0 errors, 0 warnings | Bash | CRITICAL |
| G2 | Tests | `{TEST_COMMAND}` | 0 failures | Bash | CRITICAL |
| G3 | Regression | `{CLEAN_BUILD_AND_TEST_COMMAND}` | 0 failures | Bash | CRITICAL |
| G4 | Linting | `{LINT_COMMAND}` | 0 errors | Bash | CRITICAL |
| G5 | Type Checking | `{TYPE_CHECK_COMMAND}` | 0 errors | Bash | CRITICAL |
| G6 | Formatting | `{FORMAT_CHECK_COMMAND}` | 0 diffs | Bash | WARNING |

<!-- CUSTOMIZE EXAMPLES:
G1: `npm run build`, `cargo build`, `go build ./...`, `gcc -Wall -Werror`
G2: `npm test`, `cargo test`, `go test ./...`, `pytest`
G3: `make clean && make test`, `cargo clean && cargo test`
G4: `eslint .`, `clippy`, `golangci-lint run`, `flake8`
G5: `tsc --noEmit`, `mypy .`, (Rust/Go: handled by compiler)
G6: `prettier --check .`, `rustfmt --check`, `gofmt -l .`
-->

### Architecture Gates

| # | Gate | Check | Threshold | Tool | Severity |
|---|------|-------|-----------|------|----------|
| G7 | Purity P1 | No I/O in pure zones | 0 matches | Grep | CRITICAL |
| G8 | Purity P2 | No framework imports in pure zones | 0 matches | Grep | CRITICAL |
| G9 | Purity P3 | No global state in pure zones | 0 matches | Grep | CRITICAL |
| G10 | Naming | `{NAMING_CONVENTION}` followed consistently | Manual/Lint | Grep | WARNING |
| G11 | Dead Code | No unused exports/declarations | 0 orphans | Grep | WARNING |
| G12 | Naked TODOs | No TODO/FIXME without ticket reference | 0 naked | Grep | WARNING |
| G13 | Duplication | No duplicate functionality | 0 duplicates | Grep | WARNING |

### Project-Specific Gates

<!-- CUSTOMIZE: Add gates specific to your project. Delete examples that don't apply. -->

| # | Gate | Check | Threshold | Tool | Severity |
|---|------|-------|-----------|------|----------|
| G14 | {GATE_NAME} | `{GATE_COMMAND}` | {GATE_THRESHOLD} | {GATE_TOOL} | {GATE_SEVERITY} |
| G15 | {GATE_NAME} | `{GATE_COMMAND}` | {GATE_THRESHOLD} | {GATE_TOOL} | {GATE_SEVERITY} |
| G16 | {GATE_NAME} | `{GATE_COMMAND}` | {GATE_THRESHOLD} | {GATE_TOOL} | {GATE_SEVERITY} |

**Common project-specific gates to consider**:

| Gate | When to use | Example |
|------|------------|---------|
| Bundle Size | Frontend apps | `< 500KB gzipped` |
| Binary Size | Systems programming | `< 1MB release build` |
| Performance Budget | Interactive apps | `60fps, < 100ms response` |
| Memory Limit | Constrained environments | `< 256MB peak RSS` |
| Security Scan | Apps with user input | `npm audit`, `cargo audit` |
| Accessibility | User-facing apps | `axe-core` score > 90 |
| API Contract | Microservices | Schema validation passes |
| Migration Safety | Database changes | `up` and `down` both work |
| Docker Build | Containerized apps | `docker build` succeeds |
| Attribution | Projects using external work | All external sources credited |

---

## Grep Patterns

<!-- CUSTOMIZE: Replace with your actual grep patterns for architecture gates. -->

### G7: Purity P1 -- No I/O in pure zones

```bash
grep -rn "{IO_PATTERN_1}\|{IO_PATTERN_2}\|{IO_PATTERN_3}" {PURE_ZONE_DIRS}
```
Expected: empty (0 matches)

### G8: Purity P2 -- No framework imports in pure zones

```bash
grep -rn "{FRAMEWORK_PATTERN_1}\|{FRAMEWORK_PATTERN_2}" {PURE_ZONE_DIRS}
```
Expected: empty (0 matches)

### G9: Purity P3 -- No global state in pure zones

```bash
grep -rn "{STATE_PATTERN_1}\|{STATE_PATTERN_2}" {PURE_ZONE_DIRS}
```
Expected: empty (0 matches)

### G12: Naked TODOs

```bash
grep -rn "TODO\|FIXME\|HACK\|XXX" {SOURCE_DIR}/ --include="*.{EXT}" | grep -v "TODO("
```
Expected: empty (all TODOs should have a reference like `TODO(ISSUE-42)`)

---

## Severity Levels

| Level | Meaning | Pipeline Action |
|-------|---------|----------------|
| CRITICAL | Blocks delivery | Must fix before merge. CI fails. Re-run checks after fix. |
| WARNING | Non-blocking | Noted in review. Pipeline continues. Fix in next cleanup. |

After 3 CRITICAL cycles on the same issue: escalate to tech lead or project owner.

---

## Coverage Targets

<!-- CUSTOMIZE: Set realistic targets for each layer. -->

| Layer | Target | Rationale |
|-------|--------|-----------|
| Pure Core (domain logic) | 90%+ | Critical business rules, easy to test |
| Application Services | 80%+ | Orchestration logic, moderate complexity |
| Infrastructure (I/O) | 50%+ | Hard to unit test, integration test preferred |
| UI / Presentation | 40%+ | Visual output often verified manually |
| Entry Point / Glue | 60%+ | Integration-level testing |

---

## Deployment Gates (Manual)

<!-- CUSTOMIZE: Define manual checks before deployment. -->

| Gate | Check | Method |
|------|-------|--------|
| Smoke Test | Application starts and serves basic requests | Manual or automated |
| Performance | Meets performance targets under load | Profiling / Load test |
| Cross-Platform | Works on target platforms/browsers/devices | Manual test |
| Error-Free | No errors in logs/console during smoke test | Log review |
| Build Size | Build artifact within size budget | `ls -la {BUILD_ARTIFACT}` |
