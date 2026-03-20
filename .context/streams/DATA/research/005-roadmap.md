# 005 — Roadmap: Layered Build Plan (L0–L3)

**Stream**: DATA
**Date**: 2026-03-20
**Status**: Complete

---

## Summary

Four layers from foundation to perfection. Each layer is independently useful — L0 alone unblocks other streams. Each subsequent layer adds depth and polish.

---

## L0: Foundation — Database + Models + Basic CRUD

**Goal**: Working database with models, migrations, seed data, and server functions. Other streams can start building pages.

**Unblocks**: SHOWCASE (project data), IDENTITY (CV data), DESIGN (dynamic content patterns)

### Tasks

- [ ] **L0-01**: Add SQLx + serde + thiserror + chrono + dotenvy to Cargo.toml (SSR-gated)
- [ ] **L0-02**: Create database error type (`src/server/error.rs`) with `thiserror`
- [ ] **L0-03**: Create `Project` model (`src/models/project.rs`)
  ```rust
  #[derive(Clone, Debug, Serialize, Deserialize)]
  #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
  pub struct Project {
      pub id: i64,
      pub name: String,
      pub slug: String,
      pub tagline: String,
      pub description: String,
      pub tech_stack: String,         // comma-separated initially, normalized in L1
      pub repo_url: Option<String>,
      pub live_url: Option<String>,
      pub image_path: Option<String>,
      pub sort_order: i64,
      pub visible: bool,
      pub created_at: String,
      pub updated_at: String,
  }
  ```
- [ ] **L0-04**: Create `Experience` model (`src/models/experience.rs`)
  ```rust
  pub struct Experience {
      pub id: i64,
      pub role: String,
      pub company: String,
      pub company_url: Option<String>,
      pub start_date: String,
      pub end_date: Option<String>,    // None = "Present"
      pub description: String,
      pub highlights: String,          // JSON array stored as text
      pub sort_order: i64,
      pub visible: bool,
  }
  ```
- [ ] **L0-05**: Create `Skill` model (`src/models/skill.rs`)
  ```rust
  pub struct Skill {
      pub id: i64,
      pub name: String,
      pub category: String,           // "language", "framework", "tool", "concept"
      pub proficiency: i64,           // 1-5 scale
      pub visible: bool,
  }
  ```
- [ ] **L0-06**: Create `CvSection` model (`src/models/cv_section.rs`)
  ```rust
  pub struct CvSection {
      pub id: i64,
      pub section_type: String,       // "about", "methodology", "philosophy", "education"
      pub title: String,
      pub content: String,            // markdown or HTML
      pub sort_order: i64,
      pub visible: bool,
  }
  ```
- [ ] **L0-07**: Create `src/models/mod.rs` exporting all models
- [ ] **L0-08**: Create migration `001_create_tables.sql` — all 4 tables
- [ ] **L0-09**: Create migration `002_seed_showcase_data.sql` — 4 showcase projects + CV content
- [ ] **L0-10**: Set up database pool in `main.rs` with `provide_context`
- [ ] **L0-11**: Create server functions module (`src/server/mod.rs`)
- [ ] **L0-12**: Create `get_projects()` server function
- [ ] **L0-13**: Create `get_project_by_slug(slug)` server function
- [ ] **L0-14**: Create `get_experiences()` server function
- [ ] **L0-15**: Create `get_skills()` server function
- [ ] **L0-16**: Create `get_cv_sections()` server function
- [ ] **L0-17**: Write unit tests for all models (serialization roundtrip)
- [ ] **L0-18**: Write integration tests for all server functions (real DB)
- [ ] **L0-19**: Run `cargo sqlx prepare` for offline mode
- [ ] **L0-20**: Fix `.unwrap()` calls in `main.rs` (use `Result` + `?`)

**Exit criteria**: `cargo test` passes, `cargo clippy -- -D warnings` clean, all 4 showcase projects queryable via server functions.

---

## L1: Integration — Metrics, Tags, GitHub API

**Goal**: Rich project data with quantitative metrics, normalized tech tags, and live GitHub stats.

### Tasks

- [ ] **L1-01**: Create `ProjectMetric` model (`src/models/project_metric.rs`)
  ```rust
  pub struct ProjectMetric {
      pub id: i64,
      pub project_id: i64,
      pub metric_name: String,
      pub metric_value: f64,
      pub metric_unit: Option<String>,
      pub source: String,
      pub measured_at: String,
  }
  ```
- [ ] **L1-02**: Create `TechTag` model (`src/models/tech_tag.rs`)
- [ ] **L1-03**: Create `ProjectTag` junction model (many-to-many)
- [ ] **L1-04**: Migration `003_add_metrics_and_tags.sql`
- [ ] **L1-05**: Migration `004_seed_project_metrics.sql` — real metrics from project inventory
- [ ] **L1-06**: Migration `005_seed_tech_tags.sql` — tag normalization
- [ ] **L1-07**: Create `get_project_metrics(project_id)` server function
- [ ] **L1-08**: Create `get_project_tags(project_id)` server function
- [ ] **L1-09**: Create `get_projects_by_tag(tag)` server function
- [ ] **L1-10**: Add `reqwest` dependency (SSR-gated)
- [ ] **L1-11**: Create `GitHubCache` model + migration `006_github_cache.sql`
- [ ] **L1-12**: Create GitHub API client (`src/server/github.rs`)
- [ ] **L1-13**: Create `get_github_stats(repo)` server function with caching
- [ ] **L1-14**: Create aggregation server functions (total LOC, total tests, etc.)
- [ ] **L1-15**: Tests for all new models and server functions

**Exit criteria**: Each project has quantitative metrics, tech tags are queryable, GitHub stats cached and served.

---

## L2: Enhancement — Search, Filtering, Aggregation

**Goal**: Interactive data access — full-text search, multi-axis filtering, dashboard-level aggregations.

### Tasks

- [ ] **L2-01**: Create `search_projects(query)` server function (LIKE-based, SQLite FTS5 if needed)
- [ ] **L2-02**: Create `get_projects_filtered(filters)` — by tag, by status, by tech
- [ ] **L2-03**: Create `get_skills_by_category(category)` server function
- [ ] **L2-04**: Create `get_aggregated_stats()` — cross-project summary
  ```rust
  pub struct AggregatedStats {
      pub total_projects: i64,
      pub total_loc: f64,
      pub total_tests: f64,
      pub languages_count: i64,
      pub years_experience: i64,
  }
  ```
- [ ] **L2-05**: Add data validation module (`src/server/validation.rs`)
- [ ] **L2-06**: Input validation on all write server functions
- [ ] **L2-07**: Tests for search, filtering, aggregation
- [ ] **L2-08**: Query performance review — add indexes where needed

**Exit criteria**: Projects searchable and filterable, aggregated stats serve hero section, all inputs validated.

---

## L3: Perfection — Performance, Monitoring, Resilience

**Goal**: Production-hardened data layer. Optimized queries, monitoring, backup strategy, graceful degradation.

### Tasks

- [ ] **L3-01**: Query performance audit — EXPLAIN QUERY PLAN on all queries
- [ ] **L3-02**: Add database indexes based on query patterns
- [ ] **L3-03**: Database backup strategy documentation
- [ ] **L3-04**: GitHub API graceful degradation (serve stale cache on API failure)
- [ ] **L3-05**: Data freshness monitoring (log stale cache entries)
- [ ] **L3-06**: Rate limiting on server functions (prevent abuse)
- [ ] **L3-07**: Connection pool tuning (max connections, timeout)
- [ ] **L3-08**: Load testing with realistic read patterns
- [ ] **L3-09**: Final test coverage review — no untested paths
- [ ] **L3-10**: Documentation: data architecture in `docs/ARCHITECTURE.md`

**Exit criteria**: All queries under 50ms, graceful degradation on external failures, documentation complete, test coverage stable.

---

## Layer Dependencies

```
L0 Foundation ─────> L1 Integration ─────> L2 Enhancement ─────> L3 Perfection
(unblocks all)       (rich data)           (interactivity)        (production)
     │
     └── Other streams can start building pages after L0
```

**Critical path**: L0 is the bottleneck. Once L0 is complete, SHOWCASE and IDENTITY streams can begin building pages. L1-L3 can be developed in parallel with those streams.
