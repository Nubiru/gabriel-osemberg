# 003 — Gap Analysis: What's Missing Between Codebase and Vision

**Stream**: DATA
**Date**: 2026-03-20
**Status**: Complete

---

## Summary

The DATA domain has a **total gap** — zero implementation exists. Every component must be designed and built from scratch. This analysis categorizes the gaps by severity and maps them to the vision from ROADMAP.md Phase 2, ADR-001, ADR-002, and the world survey findings.

---

## Gap Categories

### CRITICAL — Blocks other streams from starting

| # | Gap | Current State | Target State | Blocks |
|---|-----|---------------|--------------|--------|
| C1 | **No database** | No SQLx dependency, no DB file, no pool | SQLite with SQLx, compile-time checked queries, pool shared via context | SHOWCASE, IDENTITY, all page data |
| C2 | **No data models** | Zero structs | 6+ model structs with serde + sqlx derives | SHOWCASE (project cards), IDENTITY (CV page) |
| C3 | **No migrations** | No `migrations/` directory | Schema creation + seed data migrations | Everything DB-dependent |
| C4 | **No server functions** | Zero `#[server]` functions | CRUD + aggregation for all models | SHOWCASE, IDENTITY, DESIGN (dynamic content) |
| C5 | **No seed data** | No project/experience/skill records | 4 showcase projects + full CV content from real data | All content pages |

### HIGH — Required for core functionality

| # | Gap | Current State | Target State | Impact |
|---|-----|---------------|--------------|--------|
| H1 | **No project metrics model** | Metrics not modeled | Dedicated table for LOC, tests, coverage, file counts per project | Project showcase quality |
| H2 | **No GitHub API integration** | No `reqwest` dependency | Server-side GitHub stats fetching with caching | Live repo data |
| H3 | **No error handling** | `.unwrap()` in main.rs | Domain error types with `thiserror`, `ServerFnError` propagation | Reliability |
| H4 | **No serde dependency** | Not in Cargo.toml | `serde` + `serde_json` for model serialization | Client-server data transfer |
| H5 | **No test infrastructure** | Zero tests | Unit tests for models, integration tests for queries + server fns | Quality gate compliance |

### MEDIUM — Required for full vision

| # | Gap | Current State | Target State | Impact |
|---|-----|---------------|--------------|--------|
| M1 | **No tech tag normalization** | Not designed | Many-to-many relationship: projects <-> tech_tags | Tech stack visualization |
| M2 | **No CV sections model** | Not designed | Ordered content blocks for about/methodology | About page content |
| M3 | **No caching layer** | Not designed | In-database caching for GitHub API responses | Performance, rate limits |
| M4 | **No search/filter server functions** | Not designed | Full-text search, sorting, filtering | Projects page interactivity |
| M5 | **No metrics aggregation** | Not designed | Total LOC, total tests, total coverage across projects | Dashboard/hero stats |

### LOW — Polish and enhancement

| # | Gap | Current State | Target State | Impact |
|---|-----|---------------|--------------|--------|
| L1 | **No data validation layer** | Not designed | Input validation before DB writes | Data integrity |
| L2 | **No database backup strategy** | Not designed | SQLite file backup approach | Production safety |
| L3 | **No query performance optimization** | Not applicable yet | Indexed queries, query plans reviewed | Scale readiness |
| L4 | **No data freshness monitoring** | Not designed | Staleness checks for cached data | Data accuracy |

---

## Dependency Analysis

### What DATA provides to other streams

| Consuming Stream | What They Need | Gap Severity |
|-----------------|----------------|--------------|
| **SHOWCASE** | `get_projects()`, `get_project_by_slug()`, project metrics, tech stacks | CRITICAL |
| **IDENTITY** | `get_experiences()`, `get_skills()`, `get_cv_sections()`, about content | CRITICAL |
| **DESIGN** | Dynamic content loading patterns, loading states for server fns | HIGH |
| **INFRA** | Database deployment config, migration strategy, backup approach | MEDIUM |

### What DATA needs from other streams

| Providing Stream | What We Need | Gap Severity |
|-----------------|--------------|--------------|
| **DESIGN** | None for schema/models (DATA is foundational) | — |
| **INFRA** | Deployment target decision (affects SQLite vs PostgreSQL) | LOW (SQLite works everywhere) |
| **MEGA** | SQLite vs PostgreSQL ADR (recommendation: SQLite) | MEDIUM |
| **MEGA** | PDF generation approach ADR | LOW (deferred to build) |

---

## Gap Severity Map

```
CRITICAL (blocks other streams):
  C1 ── C3 ── C4 ── C5
  |                  |
  C2 ────────────────┘

HIGH (core functionality):
  H1   H2   H3   H4   H5
  |    |         |
  └────┘         └── H5 (tests validate all above)

MEDIUM (full vision):
  M1   M2   M3   M4   M5

LOW (polish):
  L1   L2   L3   L4
```

### Recommended Build Order

1. **C1 + C3 + H3 + H4** — Database setup, migrations, error types, serde (foundation)
2. **C2** — Model structs (depends on schema)
3. **C4** — Server functions (depends on models + pool)
4. **C5 + H1** — Seed data with real metrics (depends on server functions)
5. **H5** — Tests for everything above (continuous, but formal pass here)
6. **H2 + M3** — GitHub API integration with caching
7. **M1 + M2** — Tag normalization, CV sections
8. **M4 + M5** — Search, filtering, aggregation
9. **L1-L4** — Validation, backup, optimization, monitoring

---

## Cargo.toml Gap

Dependencies needed but not present:

```toml
# Required (CRITICAL)
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
serde = { version = "1", features = ["derive"] }

# Required (HIGH)
thiserror = "2"
reqwest = { version = "0.12", features = ["json"], optional = true }
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15"

# Optional (MEDIUM)
serde_json = "1"
```

All DB-related dependencies should be gated behind `feature = "ssr"` (optional, only for server).

---

## Conclusion

The DATA stream has a total gap at the CRITICAL level — nothing exists. However, the architecture is well-defined (ADR-001, world survey patterns, conventions), and the data model is clear from the project inventory and showcase selection (ADR-002). The primary risk is the SQLite vs PostgreSQL decision, which I recommend resolving as SQLite (see 002-world-survey.md rationale). The build order is linear and unblocked — no external dependencies prevent DATA from starting immediately.
