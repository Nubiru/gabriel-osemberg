# DATA — Report: L0 Foundation Complete

**Date**: 2026-03-20
**Task**: L0 Foundation — Dependencies + Models + Migrations + Server Functions + Tests
**Status**: COMPLETE

## What Was Built

### Files Created by DATA Stream (this session)
- `src/server_fns.rs` — 5 server functions (get_projects, get_project_by_slug, get_experiences, get_skills, get_cv_sections)
- `migrations/002_seed_showcase_data.sql` — Seed data: 5 projects, 30 skills, 3 CV sections
- `tests/data_queries.rs` — 8 integration tests (in-memory SQLite)
- Unit tests added to all 4 model files (8 tests total)

### Files Created by Prior Sessions (MEGA/DESIGN)
- `src/models/mod.rs`, `project.rs`, `experience.rs`, `skill.rs`, `cv_section.rs` — 4 model structs
- `src/server/mod.rs`, `error.rs` — DataError with thiserror
- `migrations/001_create_tables.sql` — Schema for 4 tables
- `src/main.rs` — Pool setup with `provide_context`, embedded migrations
- `Cargo.toml` — sqlx, serde, thiserror, dotenvy dependencies

### Files Modified by DATA Stream (this session)
- `src/lib.rs` — Added `pub mod server_fns;`
- `Cargo.toml` — Added `[dev-dependencies]` (serde_json, sqlx, tokio for tests)
- `.gitignore` — Added `*.db`, `*.db-shm`, `*.db-wal`
- Model files — Added unit tests to all 4

## Seed Data Summary

| Table | Records | Content |
|-------|---------|---------|
| projects | 5 | time, blocksight, anan-yarok, chamana, gabriel-osemberg |
| skills | 30 | 8 languages, 7 frameworks, 8 tools, 7 concepts |
| cv_sections | 3 | about, methodology, philosophy |
| experiences | 0 | Pending real CV content from Gabriel |

## Quality Gates

| Gate | Result |
|------|--------|
| `cargo build --features ssr` | PASS |
| `cargo build --features hydrate` | PASS |
| `cargo clippy --features ssr -- -D warnings` | PASS (zero warnings) |
| `cargo fmt --check` | PASS |
| `cargo test --features ssr` | 16/16 PASS |

## Architecture Notes

- Server functions are in `src/server_fns.rs` (ungated module) because `#[server]` macro needs visibility from both client and server features
- `src/server/` remains SSR-gated for implementation details (error types, future DB helpers)
- Uses `sqlx::query_as::<_, T>()` (runtime) not `sqlx::query_as!()` (compile-time macro) — simpler, no `cargo sqlx prepare` needed. Can upgrade to macros in L3.
- Tests use in-memory SQLite (`sqlite::memory:`) with embedded migrations — fast, isolated, no file cleanup

## What Other Streams Can Now Do

- **SHOWCASE**: Call `get_projects()` and `get_project_by_slug(slug)` to load project data for cards and detail pages
- **IDENTITY**: Call `get_experiences()`, `get_skills()`, `get_cv_sections()` to render CV pages
- Both can use `leptos::prelude::Resource` to load data reactively in components
