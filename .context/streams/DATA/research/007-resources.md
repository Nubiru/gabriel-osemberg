# 007 — Resources: Reference Materials and Knowledge Gaps

**Stream**: DATA
**Date**: 2026-03-20
**Status**: Complete

---

## Primary References

### Official Documentation

| Resource | URL | Use |
|----------|-----|-----|
| Leptos Book — Server Functions | https://book.leptos.dev/server/25_server_functions.html | `#[server]` macro patterns |
| Leptos Book — Extractors | https://book.leptos.dev/server/26_extractors.html | Database pool access via context |
| SQLx Documentation | https://docs.rs/sqlx/latest/sqlx/ | Query macros, pool config, migrations |
| SQLx CLI README | https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md | `cargo sqlx prepare`, migrations |
| SQLx query! macro | https://docs.rs/sqlx/latest/sqlx/macro.query.html | Compile-time checked queries |
| GitHub REST API — Repo Stats | https://docs.github.com/en/rest/metrics/statistics | Stars, forks, languages, commits |
| Serde Documentation | https://serde.rs/ | Serialization/deserialization |
| thiserror Documentation | https://docs.rs/thiserror/latest/thiserror/ | Error type derives |

### Reference Projects

| Project | URL | Relevance |
|---------|-----|-----------|
| Leptos Todo SQLite | https://github.com/leptos-rs/leptos/tree/main/examples/todo_app_sqlite | Model pattern, server fn pattern |
| Heavy Metal Stack | https://github.com/benwis/leptos-heavy-metal-stack | Full stack reference (same tech) |
| Leptos Portfolio Admin | https://github.com/DevsHero/leptos_portfolio_admin | Portfolio data model (6 entities) |
| RealWorld Leptos | https://github.com/Bechma/realworld-leptos | Production SQLx + Leptos patterns |

### Crate Documentation

| Crate | Version | Purpose |
|-------|---------|---------|
| `sqlx` | 0.8 | Async SQL with compile-time checks |
| `serde` | 1.x | Serialize/Deserialize derives |
| `serde_json` | 1.x | JSON parsing for GitHub API responses |
| `thiserror` | 2.x | Domain error types |
| `reqwest` | 0.12 | HTTP client for GitHub API |
| `chrono` | 0.4 | Date/time handling |
| `dotenvy` | 0.15 | .env file loading |

---

## Knowledge Gaps

### Resolved This Session

| Gap | Resolution |
|-----|-----------|
| SQLite vs PostgreSQL | SQLite — see Q1 in 004-questions.md |
| How to share DB pool with Leptos server fns | `provide_context` + `use_context` — see Q3 |
| SQLx offline mode for CI | `cargo sqlx prepare` → `.sqlx/` directory — see Q3 |
| Conditional SSR derives for models | `#[cfg_attr(feature = "ssr", derive(...))]` pattern |
| How to seed data | Migration SQL files with INSERT statements — see Q4 |
| GitHub API caching | In-database with TTL — see Q5 |

### Remaining Gaps (to resolve during build)

| Gap | Priority | When to Resolve |
|-----|----------|----------------|
| Exact SQLx + Leptos 0.8 pool setup code | HIGH | L0 build — test with actual compilation |
| SQLx migration file naming convention | MEDIUM | L0 build — check `sqlx-cli` docs |
| `query_as!` vs `query_as` (macro vs function) | MEDIUM | L0 build — test both approaches |
| Reqwest + GitHub API error handling patterns | MEDIUM | L1 build |
| SQLite FTS5 setup for full-text search | LOW | L2 build |
| Connection pool tuning parameters | LOW | L3 build |

### Skills Gabriel Will Learn (Mentor Opportunities)

| Concept | Rust-Specific Aspect | Teaching Moment |
|---------|---------------------|-----------------|
| Compile-time SQL checking | `sqlx::query!` macros verify SQL at compile time | Unique to Rust — TypeScript/JS can't do this |
| Feature gating | `#[cfg_attr(feature = "ssr", ...)]` for SSR-only code | Conditional compilation, a Rust superpower |
| Error propagation | `thiserror` + `?` operator chains | Rust's `Result` type vs try/catch |
| Connection pooling | `SqlitePool` with `Arc` sharing | Ownership patterns for shared resources |
| Async/await in Rust | `async fn` + `.await` with Tokio runtime | Futures, Send + Sync bounds |
| Serde derives | `#[derive(Serialize, Deserialize)]` | Zero-cost abstraction for serialization |

---

## Tool Requirements

| Tool | Purpose | Install |
|------|---------|---------|
| `cargo-leptos` | Dev server, build | `cargo install cargo-leptos` (already installed) |
| `sqlx-cli` | Database management, migrations, prepare | `cargo install sqlx-cli --features sqlite` |
| SQLite3 CLI | Manual DB inspection | `sudo apt install sqlite3` (likely installed) |

---

## Data Sources for Seed Content

| Data | Source | Status |
|------|--------|--------|
| Project metrics (LOC, tests, files) | Portfolio inventory memory file | Available |
| Project descriptions | To be written (Gabriel or IDENTITY stream) | Pending |
| Experience entries | To be written (Gabriel) | Pending |
| Skills list | Derivable from project tech stacks | Available |
| About/methodology text | To be written (Gabriel) | Pending |
| GitHub repo URLs | Known from project directories | Available |
| Project screenshots | To be captured/created | Pending (DESIGN/SHOWCASE) |

---

## Session Reading List (for build phase)

Before starting L0 build, read in this order:
1. Leptos Book ch. 25 (Server Functions) — 10 min
2. Leptos Book ch. 26 (Extractors) — 5 min
3. SQLx docs — query macros section — 10 min
4. Heavy Metal Stack `main.rs` — pool setup pattern — 5 min
5. Heavy Metal Stack `migrations/` — naming convention — 2 min
