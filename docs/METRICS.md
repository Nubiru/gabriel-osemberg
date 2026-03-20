# gabriel-osemberg — Project Metrics

**Last refreshed**: 2026-03-20

---

## Codebase

| Metric | Count |
|--------|-------|
| Source files (.rs) | 18 |
| Lines of code (src/) | 1,148 |
| Migration files (.sql) | 2 |
| Style files (.css) | 1 (271 lines) |
| Font files (.woff2) | 3 |
| Contributors | 2 (Gabriel + Claude) |

## Testing

| Metric | Count |
|--------|-------|
| Test functions | 8 |
| Test failures | 0 |
| Test run time | 0.12s |

### Test Pyramid

| Label | Count | Notes |
|-------|-------|-------|
| unit | 3 | Model serialization roundtrips |
| integration | 5 | Database query tests (SQLite in-memory) |
| end-to-end | 0 | Playwright — not yet set up |

## Build

| Target | Status |
|--------|--------|
| SSR (cargo build --features ssr) | PASS |
| Hydrate/WASM (cargo build --features hydrate --target wasm32) | PASS |
| cargo-leptos build | PASS |
| Tailwind CSS v4 | PASS |

## Quality

| Metric | Value |
|--------|-------|
| Clippy warnings (SSR) | 0 |
| Clippy warnings (hydrate) | 0 |
| rustfmt violations | 0 |
| `.unwrap()` in app code | 0 |
| Hardcoded CSS values | 0 |
| TODOs in code | 0 |

## Stream Progress

| Stream | Research | Build Phase | L0 | L1 | L2 | L3 |
|--------|----------|-------------|----|----|----|----|
| DESIGN | 7/7 | BUILD | DONE | IN PROGRESS | - | - |
| DATA | 7/7 | BUILD | DONE | - | - | - |
| SHOWCASE | 7/7 | ROADMAP | - | - | - | - |
| IDENTITY | 2/7 | RESEARCH | - | - | - | - |
| INFRA | 7/7 | ROADMAP | - | - | - | - |

## Architecture Decision Records

| ADR | Title | Status |
|-----|-------|--------|
| 000 | Use ADRs | Accepted |
| 001 | Leptos (Rust + WASM) | Accepted |
| 002 | Project Showcase Selection | Accepted |
| 003 | SQLite as Primary Database | Accepted |
| 004 | Code as Portfolio (REST API + /architecture) | Accepted |
| 005 | Fly.io as Deployment Platform | Accepted |

## Health

| Check | Status |
|-------|--------|
| Git integrity (fsck) | PASS |
| CI pipeline (GitHub Actions) | ACTIVE |
| All tests pass | PASS |
| Clippy clean | PASS |
| Git commits | 9 |

---

*Updated by `/refresh` command or MEGA cycle.*
