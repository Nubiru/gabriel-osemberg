# 001 — Inventory: What Exists in the Codebase for the DATA Domain

**Stream**: DATA
**Date**: 2026-03-20
**Status**: Complete

---

## Summary

The DATA domain (database schema, models, server functions, migrations, seed data, external API integrations) has **zero implementation**. The codebase is a freshly scaffolded Leptos project with only the default template files.

---

## Current Codebase State

### Source Files in DATA Domain

| Directory | Expected Purpose | Files Found | Status |
|-----------|-----------------|-------------|--------|
| `src/models/` | Database models (SQLx structs) | 0 | **Does not exist** |
| `src/server/` | Server functions (data access, GitHub API, PDF) | 0 | **Does not exist** |
| `src/pdf/` | PDF generation logic | 0 | **Does not exist** |
| `migrations/` | SQLx database migrations | 0 | **Does not exist** |
| `tests/unit/` | Unit tests | 0 | **Does not exist** |
| `tests/integration/` | Integration tests (DB, server fns) | 0 | **Does not exist** |
| `data/` | Static data files | 1 | Template only |

### Existing Files (3 total Rust source files)

1. **`src/main.rs`** — Axum server entry point. Uses `tokio::main`, creates Leptos routes, binds to `127.0.0.1:3000`. Contains `.unwrap()` calls that violate project conventions (needs future fix).

2. **`src/app.rs`** — Root Leptos component. Contains `App`, `shell`, and a `HomePage` with a reactive counter. No data fetching, no server functions, no database integration. All content is hardcoded strings.

3. **`src/lib.rs`** — Exports `app` module and provides WASM `hydrate()` entry point. Minimal.

### Dependencies Relevant to DATA Domain

**Present in Cargo.toml:**
- `leptos` 0.8 — framework (has server function support)
- `leptos_axum` 0.8 — server integration
- `axum` 0.8 — HTTP framework
- `tokio` 1 — async runtime

**NOT present (needed for DATA domain):**
- `sqlx` — compile-time checked SQL queries
- `serde` / `serde_json` — serialization (needed for models)
- `thiserror` — domain error types
- `reqwest` — HTTP client for GitHub API
- `chrono` — date/time handling
- `dotenvy` — environment variable loading (.env for DATABASE_URL)

### Data Files

- **`data/contributors.json`** — Template with placeholder values. Not populated with real data.

### Database

- No database file (`.db`) exists
- No `DATABASE_URL` configuration found
- No SQLx configuration (`.sqlx/` directory for offline mode)
- No migration files

---

## Seed Data Sources

Per ADR-002, 4 showcase projects need seed data. All projects exist at `/home/gabiota/personal/projects/code/`:

| Project | Key Metrics for Seeding |
|---------|------------------------|
| **time** | 90K+ LOC, 3,773 src files, 14,789 test functions, 95.9% coverage, C11 + WebGL2 + WASM |
| **blocksight-main** | 2,876 src files, 1,620 test files, Node + React + Next + TimescaleDB |
| **anan-yarok** | 1,048 src files, 246 test files, Next.js 16 + Prisma + PostgreSQL |
| **chamana** | 203 src files, 21 test files, Next.js 16 + Payload CMS |
| **gabriel-osemberg** (this) | Rust + Leptos + WASM (self-referential — metrics update as project grows) |

---

## Architecture Decisions Affecting DATA

- **ADR-001**: Leptos + Rust + WASM. Database: SQLx with SQLite or PostgreSQL (decision pending).
- **ADR-002**: 4+1 showcase projects selected. Seed data must come from actual project metrics.

### Open Question from ADR-001

SQLite vs PostgreSQL is **not yet decided**. This affects:
- Development workflow (SQLite = zero-config embedded; PostgreSQL = separate service)
- Deployment (SQLite = single-file; PostgreSQL = managed DB service)
- SQLx compile-time checks (both supported, workflow differs)
- Query syntax differences (minor but real)

---

## Conventions Affecting DATA

From CONVENTIONS.md:
- Database tables: `snake_case` (plural) — e.g., `projects`, `experiences`
- Database columns: `snake_case` — e.g., `tech_stack`, `test_coverage`
- Server functions: `#[server(GetProjects)]` pattern with `Result<T, ServerFnError>`
- Error handling: `thiserror` for domain errors, `?` operator, no `.unwrap()`
- Import ordering: std -> third-party -> project-local
- One concept per file, files under 400 lines

---

## Gap Assessment

| Category | Exists | Needed | Gap |
|----------|--------|--------|-----|
| Database schema | 0 tables | 4+ tables (projects, experiences, skills, cv_sections) | **Total** |
| Models | 0 structs | 4+ model structs with derive macros | **Total** |
| Migrations | 0 files | Schema creation + seed data | **Total** |
| Server functions | 0 fns | CRUD for all models + aggregation | **Total** |
| GitHub API integration | 0 | Repo stats, languages, commits | **Total** |
| Seed data | 0 records | 4 projects + experience + skills | **Total** |
| Tests | 0 | Unit + integration for all data operations | **Total** |
| Dependencies | 4 crates | 10+ crates | **Partial** (need sqlx, serde, thiserror, etc.) |

**Conclusion**: The DATA domain starts from zero. Every component — schema, models, migrations, server functions, seed data, tests, and external API integration — must be designed and built from scratch.
