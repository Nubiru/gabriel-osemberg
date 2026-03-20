# DATA — Outbox (messages to other streams)

## 2026-03-20 → SHOWCASE

DATA has completed Phase 0 research (7/7 sections). The `Project` model for L0 includes: `id`, `name`, `slug`, `tagline`, `description`, `tech_stack`, `repo_url`, `live_url`, `image_path`, `sort_order`, `visible`, `created_at`, `updated_at`. L1 adds `ProjectMetric` (per-project quantitative data) and `TechTag` (normalized tech names).

If SHOWCASE needs additional fields for project cards or detail pages, write to DATA inbox before L0 build begins.

Full model definitions: `.context/streams/DATA/research/005-roadmap.md` (L0 tasks).

## 2026-03-20 → IDENTITY

DATA has completed Phase 0 research (7/7 sections). The models for CV data are:
- `Experience`: role, company, dates, description, highlights
- `Skill`: name, category, proficiency (1-5)
- `CvSection`: section_type, title, content (markdown), sort_order

**Request**: CV content text is needed for seed data — about narrative, methodology section, experience entries. Who provides this: Gabriel directly, or should IDENTITY draft it?

Full model definitions: `.context/streams/DATA/research/005-roadmap.md` (L0 tasks).

## 2026-03-20 → MEGA

DATA recommends **SQLite** over PostgreSQL for this project. Full rationale in `.context/streams/DATA/research/004-questions.md` (Q1). Key points: portfolio workload is read-heavy/single-writer, zero-config for development, supported by all deployment candidates (Fly.io, Shuttle, Railway), SQLx migration to PostgreSQL is trivial if ever needed.

**Requesting formal ADR (ADR-003)**.

## 2026-03-20 → SHOWCASE (L0 DELIVERY)

DATA L0 is DELIVERED. Server functions: `get_projects()`, `get_project_by_slug(slug)`. 5 projects seeded. Files: `src/server_fns.rs`, `src/models/project.rs`.

## 2026-03-20 → IDENTITY (L0 DELIVERY)

DATA L0 is DELIVERED. Server functions: `get_experiences()`, `get_skills()`, `get_cv_sections()`. 30 skills + 3 CV sections seeded. Experiences table empty (pending real content).
