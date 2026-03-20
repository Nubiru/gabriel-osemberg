# 006 — Dependencies: Cross-Stream Needs and Provides

**Stream**: DATA
**Date**: 2026-03-20
**Status**: Complete

---

## What DATA Provides

### To SHOWCASE

| What | When Available | Interface |
|------|---------------|-----------|
| `get_projects()` → `Vec<Project>` | L0 | Server function |
| `get_project_by_slug(slug)` → `Project` | L0 | Server function |
| `get_project_metrics(project_id)` → `Vec<ProjectMetric>` | L1 | Server function |
| `get_project_tags(project_id)` → `Vec<TechTag>` | L1 | Server function |
| `get_github_stats(repo)` → `RepoStats` | L1 | Server function (cached) |
| `get_projects_filtered(filters)` → `Vec<Project>` | L2 | Server function |
| `search_projects(query)` → `Vec<Project>` | L2 | Server function |
| `get_aggregated_stats()` → `AggregatedStats` | L2 | Server function |

### To IDENTITY

| What | When Available | Interface |
|------|---------------|-----------|
| `get_experiences()` → `Vec<Experience>` | L0 | Server function |
| `get_skills()` → `Vec<Skill>` | L0 | Server function |
| `get_cv_sections()` → `Vec<CvSection>` | L0 | Server function |
| `get_skills_by_category(cat)` → `Vec<Skill>` | L2 | Server function |

### To DESIGN

| What | When Available | Interface |
|------|---------------|-----------|
| Loading state patterns for server functions | L0 | Documented pattern |
| Data shapes for component props | L0 | Model struct definitions |

### To INFRA

| What | When Available | Interface |
|------|---------------|-----------|
| SQLite database file location | L0 | Configuration |
| Migration strategy (embedded) | L0 | `sqlx::migrate!()` in main.rs |
| Backup approach | L3 | Documentation |
| `.sqlx/` offline mode files | L0 | Checked into version control |

---

## What DATA Needs

### From MEGA

| What | Priority | Status |
|------|----------|--------|
| ADR for SQLite vs PostgreSQL | MEDIUM | Recommendation: SQLite (see 004-questions Q1). Awaiting formal ADR. |
| ADR for PDF generation approach | LOW | Deferred to build phase. |

### From DESIGN

| What | Priority | Status |
|------|----------|--------|
| Nothing for L0-L1 | — | DATA is foundational, no design dependency |
| Error/loading state component patterns | LOW | Needed when integrating server fns with UI |

### From SHOWCASE

| What | Priority | Status |
|------|----------|--------|
| Data shape requirements for project cards | MEDIUM | DATA proposes models; SHOWCASE may request fields |
| Image asset paths per project | LOW | Needed for seed data `image_path` field |

### From IDENTITY

| What | Priority | Status |
|------|----------|--------|
| CV content text for seed data | MEDIUM | About narrative, methodology section, education details |
| Experience entries text | MEDIUM | Role descriptions, company info, highlights |

### From INFRA

| What | Priority | Status |
|------|----------|--------|
| Deployment platform confirmation | LOW | SQLite works on all candidates (Fly.io, Shuttle, Railway) |
| Persistent volume configuration | LOW | Needed for SQLite in production |

---

## Cross-Stream Communication Plan

### Messages to Send (outbox items)

1. **To SHOWCASE**: "DATA L0 defines `Project` model with these fields: [list]. If you need additional fields for project cards, write to DATA inbox before L0 build begins."

2. **To IDENTITY**: "DATA L0 defines `Experience`, `Skill`, `CvSection` models. CV content text is needed for seed data — who provides the actual text? Gabriel or IDENTITY?"

3. **To MEGA**: "DATA recommends SQLite over PostgreSQL for this project. Rationale in 004-questions.md Q1. Requesting formal ADR."

---

## Dependency Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| SHOWCASE needs fields not in L0 model | Medium | Low | Models are easily extensible via migrations |
| SQLite inadequate for production | Very Low | High | SQLx makes PostgreSQL migration trivial |
| GitHub API rate limits hit | Low | Medium | Cache with generous TTL, authenticated token |
| CV content text not available for seeding | Medium | Medium | Use placeholder text, replace when available |
| Leptos 0.8 API changes for server functions | Low | High | Pin dependency version, test frequently |
