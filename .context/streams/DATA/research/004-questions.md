# 004 — Open Questions: Researched and Answered

**Stream**: DATA
**Date**: 2026-03-20
**Status**: Complete

---

## Questions from tracks.md

### Q1: SQLite vs PostgreSQL?

**Question**: "SQLite (simple, embedded, zero-config) vs PostgreSQL (full-featured, better for production)? SQLite is simpler for a portfolio site."

**Answer**: **SQLite**. Confirmed after research.

**Evidence**:
- Portfolio site workload is read-heavy, single-writer — SQLite's sweet spot
- Zero-config: no separate database service to manage during development or deployment
- Fly.io supports SQLite via persistent volumes (or LiteFS for replication)
- Shuttle.rs also supports SQLite natively
- SQLx provides full compile-time checked queries for SQLite, same as PostgreSQL
- No concurrent write pressure — a portfolio serves reads; admin writes are rare
- Backup is trivial: copy the `.db` file
- The Leptos Heavy Metal Stack and official Leptos examples both use SQLite

**Fly.io caveat**: SQLite file must be on a persistent volume, not baked into the Docker image. Otherwise data resets on each deploy. This is a deployment concern, not a schema concern.

**Recommendation**: Use SQLite for both development and production. If scaling needs change (they won't for a portfolio), migration to PostgreSQL via SQLx is straightforward — same query syntax, same `sqlx::query!` macros, change the connection string and feature flag.

**Status**: Escalated to MEGA for formal ADR. Confidence: HIGH.

---

### Q2: How to structure project metrics data?

**Question**: "How to structure project metrics data? Snapshot at build time vs live GitHub API?"

**Answer**: **Hybrid — snapshot with optional live refresh**.

**Design**:
```
project_metrics table:
  id              INTEGER PRIMARY KEY
  project_id      INTEGER REFERENCES projects(id)
  metric_name     TEXT NOT NULL        -- "loc", "test_count", "coverage", "src_files"
  metric_value    REAL NOT NULL        -- 90000, 14789, 95.9, 3773
  metric_unit     TEXT                 -- "lines", "functions", "%", "files"
  source          TEXT NOT NULL        -- "manual", "github_api", "cloc"
  measured_at     TEXT NOT NULL        -- ISO 8601 timestamp
```

**Rationale**:
1. **Base metrics** (LOC, tests, coverage) are seeded manually from the actual project analysis we already have. These are snapshots — the `time` project has 90K LOC and 95.9% coverage as measured values. They don't need live updating.
2. **GitHub metrics** (stars, forks, last commit, languages) are fetched from GitHub API and cached in a `github_cache` table with TTL-based staleness.
3. **Self-referential metrics** (this project's own stats) update as the project grows. These could be refreshed at build time.

**Why not all-live?**: Most metrics don't change frequently. `time` has 90K LOC whether we check today or next month. Fetching live adds latency and rate-limit risk for data that's essentially static. The hybrid approach gives us fresh GitHub stats where they matter and stable metrics where stability is the truth.

---

### Q3: SQLx compile-time checked queries with Leptos server functions?

**Question**: "What's the best approach for SQLx compile-time checked queries with Leptos server functions?"

**Answer**: Use **`sqlx::query_as!` macros** with **offline mode** for CI/CD, and **context-based pool sharing**.

**Pattern**:

1. **Models with conditional derives**:
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub slug: String,
    // ...
}
```

2. **Pool via Leptos context** (from Leptos book):
```rust
// main.rs
let pool = SqlitePool::connect(&database_url).await?;
sqlx::migrate!().run(&pool).await?;

Router::new()
    .leptos_routes_with_context(
        &leptos_options,
        routes,
        move || provide_context(pool.clone()),
        move || shell(leptos_options.clone()),
    )
```

3. **Server functions access pool via context**:
```rust
#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let pool = use_context::<SqlitePool>()
        .ok_or(ServerFnError::new("Database pool not available"))?;

    let projects = sqlx::query_as!(Project, "SELECT * FROM projects WHERE visible = true ORDER BY sort_order")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    Ok(projects)
}
```

4. **Offline mode for CI** (no database needed to compile):
```bash
# Developer runs after changing queries:
cargo sqlx prepare --workspace

# CI builds with:
SQLX_OFFLINE=true cargo build
```

**Key insight**: `sqlx` dependencies must be `optional = true` in Cargo.toml, gated behind `feature = "ssr"`. The WASM build (client) never touches the database. Only the server binary links sqlx.

---

### Q4: How to seed data from sibling projects?

**Question**: "How to seed data from the actual sibling projects at /home/gabiota/personal/projects/code/?"

**Answer**: Use a **seed migration** with hardcoded values derived from project analysis.

**Approach**:
1. We already have the metrics from the portfolio inventory (measured during a prior session)
2. Create a migration file `002_seed_showcase_projects.sql` that inserts the data directly
3. Metrics are point-in-time measurements — they don't auto-update from sibling project directories

**Why not dynamic scanning?**: The sibling projects exist on Gabriel's development machine but won't exist on the production server. The seed data must be self-contained in the migration. The portfolio inventory memory file has all the numbers we need.

**Seed data sources**:
| Project | LOC | Src Files | Tests | Coverage | Stack |
|---------|-----|-----------|-------|----------|-------|
| time | 90,000+ | 3,773 | 14,789 fns | 95.9% | C11, WebGL2, WASM |
| blocksight | — | 2,876 | 1,620 files | — | Node, React, Next, TimescaleDB |
| anan-yarok | — | 1,048 | 246 files | — | Next.js 16, Prisma, PostgreSQL |
| chamana | — | 203 | 21 files | — | Next.js 16, Payload CMS |
| gabriel-osemberg | growing | growing | growing | growing | Rust, Leptos, WASM |

Missing data (LOC for non-C projects, coverage percentages) should be measured before seed creation. This is a build-phase task.

---

### Q5: Caching strategy for GitHub API responses?

**Question**: "Caching strategy for GitHub API responses (rate limit: 60 req/hr unauthenticated, 5000 authenticated)?"

**Answer**: **In-database caching with TTL-based staleness**.

**Design**:
```sql
CREATE TABLE github_cache (
    id INTEGER PRIMARY KEY,
    cache_key TEXT NOT NULL UNIQUE,    -- "repo:gabriel-osemberg/time:stats"
    response_json TEXT NOT NULL,        -- full API response
    fetched_at TEXT NOT NULL,           -- ISO 8601
    expires_at TEXT NOT NULL            -- fetched_at + TTL
);
```

**Access pattern**:
```rust
#[server]
pub async fn get_github_stats(repo: String) -> Result<RepoStats, ServerFnError> {
    let pool = use_context::<SqlitePool>()?;

    // Check cache first
    if let Some(cached) = get_cached(&pool, &cache_key).await? {
        if !cached.is_stale() {
            return Ok(serde_json::from_str(&cached.response_json)?);
        }
    }

    // Fetch fresh from GitHub API
    let stats = fetch_from_github(&repo).await?;

    // Update cache
    upsert_cache(&pool, &cache_key, &stats).await?;

    Ok(stats)
}
```

**TTL strategy**:
- Repository stats (stars, forks): 1 hour
- Language breakdown: 24 hours (rarely changes)
- Commit count: 6 hours
- Stale-while-revalidate: serve stale data while fetching fresh in background

**Authentication**: Use a GitHub personal access token (stored in `.env`, never committed) for 5000 req/hr limit. For a portfolio with 4 repos, even unauthenticated (60/hr) is sufficient — but authenticated is safer.

**No Redis needed**: SQLite handles this workload trivially. Redis adds operational complexity for zero benefit at this scale.

---

## Additional Questions Discovered During Research

### Q6: How to handle `sqlx::FromRow` with Leptos 0.8's feature gating?

**Answer**: Use `#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]` on model structs. The model is shared between client and server for serde serialization, but sqlx derives only compile on the server. This is the established Leptos pattern from the official examples.

### Q7: Migration runner — CLI vs embedded?

**Answer**: **Embedded** (`sqlx::migrate!()` macro) in `main.rs`. Runs migrations automatically on server start. No separate CLI step needed for development. For production, this ensures the database is always up-to-date on deploy.

The `sqlx::migrate!()` macro embeds migration files at compile time from the `migrations/` directory. It's idempotent — already-applied migrations are skipped.

### Q8: Should `contributors.json` move into the database?

**Answer**: **No**. Contributors data is attribution metadata, not CV content. It's a static JSON file that's checked into version control for transparency. It doesn't need querying, sorting, or filtering — it's read once for a credits section. Keep it as JSON.

---

## Sources

- [SQLx CLI README](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [SQLx Offline Mode](https://deepwiki.com/launchbadge/sqlx/8.3-offline-mode-(prepare-command))
- [SQLx query! macro docs](https://docs.rs/sqlx/latest/sqlx/macro.query.html)
- [Leptos Book — Extractors](https://book.leptos.dev/server/26_extractors.html)
- [Leptos Book — Server Functions](https://book.leptos.dev/server/25_server_functions.html)
- [Fly.io — SQLite advice](https://community.fly.io/t/advice-sqlite-or-postgres/21353)
- [GitHub API — Repository Statistics](https://docs.github.com/en/rest/metrics/statistics)
