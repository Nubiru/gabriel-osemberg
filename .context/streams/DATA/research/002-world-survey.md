# 002 — World Survey: Best-in-Class Data Layer Examples

**Stream**: DATA
**Date**: 2026-03-20
**Status**: Complete

---

## Summary

Surveyed 6 real-world projects and official documentation to understand how Rust/Leptos applications (and comparable full-stack frameworks) structure their data layers. Key patterns emerged for database pool management, model design, and portfolio data architecture.

---

## 1. Leptos Official Todo App (SQLite) — leptos-rs/leptos

**Source**: [github.com/leptos-rs/leptos/examples/todo_app_sqlite](https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite/src/todo.rs)

**Architecture**:
- Single `SqliteConnection` per request (not pooled — simple example)
- Models use conditional compilation: `#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]`
- Server functions use `#[server]` macro, access DB via helper `db()` function
- Queries use `sqlx::query_as::<_, Todo>("SELECT * FROM todos")` pattern
- Parameterized queries with `.bind()` for safety

**Key Pattern — Conditional SSR Derives**:
```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    id: u16,
    title: String,
    completed: bool,
}
```

This is critical: `sqlx::FromRow` only compiles on the server. The model struct itself is shared between client and server (for serialization), but DB-specific derives are gated behind `feature = "ssr"`.

**Takeaway**: Simple, functional, but not production-grade. No connection pooling, no migrations, no error types. Good starting point for understanding the pattern.

---

## 2. Leptos Heavy Metal Stack — benwis

**Source**: [github.com/benwis/leptos-heavy-metal-stack](https://github.com/benwis/leptos-heavy-metal-stack)

**Architecture**:
- Full-stack: Leptos + Axum + SQLite + Tailwind + cargo-leptos
- Uses `SqlitePool` (connection pool, not per-request connections)
- Includes user authentication (login/signup)
- Migrations in `migrations/` directory
- Todo CRUD as demonstration

**Key Pattern — Pool via Axum State**:
The pool is shared through Axum's state mechanism and accessed in server functions via extractors. This is the production-grade approach vs. the todo example's per-request connection.

**Takeaway**: Closest to our target architecture. Same stack (Leptos + Axum + SQLite + Tailwind). Should be the primary reference for pool management and migration patterns.

---

## 3. Leptos Portfolio Admin — DevsHero

**Source**: [github.com/DevsHero/leptos_portfolio_admin](https://github.com/DevsHero/leptos_portfolio_admin)

**Architecture**:
- Leptos 0.6 + Actix Web + SurrealDB + Tailwind + Redis caching
- Dynamic admin panel for updating portfolio content in real-time
- PDF generation via server-side Chromium rendering
- Redis caches profile data and generated PDFs

**Data Model (6 entities)**:
| Entity | Fields |
|--------|--------|
| Profile | name, age, nationality, job role, rich HTML about |
| Skills | name, proficiency level |
| Experiences | company, position, dates, descriptions, logos |
| Projects | descriptions, tech stacks, image galleries |
| Education | institution, degree, GPA |
| Languages | language, proficiency |

**Key Patterns**:
- Contact entries are dynamically configurable with associated icons
- Rich HTML content supported (TinyMCE for about sections)
- PDF uses "minimal, cool resume template" rendered via Chromium
- Redis caching layer reduces DB queries

**Takeaway**: Most feature-complete portfolio data model found. Their 6-entity design covers everything we need. However, they use SurrealDB + Actix Web, not SQLx + Axum. The data model is transferable; the access patterns are not.

---

## 4. Portfolio-RS — Kanerix

**Source**: [github.com/Kanerix/portfolio-rs](https://github.com/Kanerix/portfolio-rs)

**Architecture**:
- Leptos + Tailwind + Docker + Fly.io deployment
- No database — static content
- Archived (Dec 2025)

**Takeaway**: Proves Leptos + Fly.io deployment works. But no data layer — opposite of our approach. Demonstrates what we're building beyond: dynamic, database-driven content vs static HTML.

---

## 5. RealWorld Leptos — Bechma

**Source**: [github.com/Bechma/realworld-leptos](https://github.com/Bechma/realworld-leptos)

**Architecture**:
- Full RealWorld spec implementation in Leptos
- Uses `cargo sqlx database setup` for initialization
- Complete CRUD with authentication
- Full-stack SSR

**Takeaway**: Production-quality example of SQLx + Leptos integration. RealWorld spec is well-documented and tested — good reference for server function patterns and query organization.

---

## 6. Leptos Book — Official Documentation

**Source**: [book.leptos.dev/server](https://book.leptos.dev/server/25_server_functions.html)

**Two Patterns for Database Pool Access**:

### Pattern A: Context-based (Recommended)
```rust
// In main.rs — provide pool via context
let pool = SqlitePool::connect("sqlite:app.db").await?;
let app = Router::new()
    .leptos_routes_with_context(
        &leptos_options,
        routes,
        move || provide_context(pool.clone()),
        move || shell(leptos_options.clone()),
    );

// In server function — access via use_context
#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let pool = use_context::<SqlitePool>()
        .ok_or(ServerFnError::new("No database pool"))?;
    // query...
}
```

### Pattern B: Axum State + FromRef
```rust
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub leptos_options: LeptosOptions,
}

// In server function — use extract_with_state
```

**Takeaway**: Pattern A (context-based) is simpler, officially recommended, and sufficient for our needs. Pattern B is for when you need to share multiple pieces of state.

---

## Cross-Cutting Insights

### SQLite vs PostgreSQL for This Project

| Factor | SQLite | PostgreSQL |
|--------|--------|------------|
| Setup | Zero-config, embedded | Requires service |
| Deployment (Fly.io) | Needs persistent volume or LiteFS | Managed Postgres available |
| Development | Single file, portable | Separate server |
| SQLx support | Full, compile-time checked | Full, compile-time checked |
| Concurrent writes | WAL mode handles moderate load | Native MVCC |
| Backup | Copy file | pg_dump |
| Portfolio site load | More than sufficient | Overkill |

**Recommendation**: SQLite for development AND production. A portfolio site has read-heavy, single-writer workloads — SQLite's sweet spot. Fly.io supports SQLite via persistent volumes or LiteFS for replication. This avoids managing a separate database service for a personal portfolio.

### Caching Strategy for GitHub API

- GitHub API: 60 req/hr unauthenticated, 5000 req/hr authenticated
- Stats endpoints return 202 on first call (cache miss) — must retry
- `http-cache-reqwest` provides HTTP-spec-compliant caching middleware
- Alternative: cache in SQLite (store last-fetched stats, refresh on schedule)
- Best approach: **store GitHub stats in SQLite as cached data, refresh via server-side cron or on-demand with staleness check**

### PDF Generation Options

| Approach | Pros | Cons |
|----------|------|------|
| Chromium (headless) | Pixel-perfect, CSS-based templates | Heavy dependency, slow |
| `printpdf` crate | Pure Rust, lightweight | Manual layout, no CSS |
| `typst` | Modern typesetting, programmable | Separate language to learn |
| `weasyprint` (Python) | CSS-based, lighter than Chromium | Python dependency |

**Recommendation**: `typst` or `printpdf` for pure Rust stack. Chromium is the fallback if CSS-based layout is essential. Decision deferred to build phase (escalate to MEGA).

---

## Synthesis: Data Model for gabriel-osemberg

Based on the survey, the ideal schema combines Leptos Portfolio Admin's entity design with our project-specific needs:

### Core Tables

```
projects         — 4+1 showcase projects with metrics
experiences      — work history entries
skills           — technical skills with categories and proficiency
cv_sections      — ordered content blocks for about/methodology/philosophy
project_metrics  — per-project quantitative data (LOC, tests, coverage)
tech_tags        — technology names (reusable across projects and skills)
project_tags     — many-to-many: project <-> tech_tag
github_cache     — cached GitHub API responses with timestamps
```

### Key Design Decisions

1. **Metrics are first-class**: Unlike typical portfolios, our projects have real quantitative data (95.9% coverage, 90K LOC). This deserves its own table, not a JSON blob.
2. **Tech tags are normalized**: Reusable across projects and skills. "Rust" appears once, linked to multiple entities.
3. **GitHub cache is in-database**: No Redis needed. SQLite can cache API responses with a `fetched_at` timestamp for staleness checks.
4. **CV sections are ordered**: The about page content is database-driven with sort ordering, allowing dynamic composition.

---

## Sources

- [Leptos Official Todo App (SQLite)](https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite/src/todo.rs)
- [Leptos Heavy Metal Stack](https://github.com/benwis/leptos-heavy-metal-stack)
- [Leptos Portfolio Admin](https://github.com/DevsHero/leptos_portfolio_admin)
- [Portfolio-RS](https://github.com/Kanerix/portfolio-rs)
- [RealWorld Leptos](https://github.com/Bechma/realworld-leptos)
- [Leptos Book — Server Functions](https://book.leptos.dev/server/25_server_functions.html)
- [Leptos Book — Extractors](https://book.leptos.dev/server/26_extractors.html)
- [Fly.io — SQLite vs Postgres](https://community.fly.io/t/advice-sqlite-or-postgres/21353)
- [http-cache-reqwest](https://crates.io/crates/http-cache-reqwest)
- [GitHub API — Repository Statistics](https://docs.github.com/en/rest/metrics/statistics)
- [Shuttle — Building Rust Web Apps](https://www.shuttle.dev/blog/2025/11/12/build-rust-web-apps)
