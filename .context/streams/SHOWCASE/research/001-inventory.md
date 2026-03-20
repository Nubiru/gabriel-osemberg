# 001 — Inventory: What Exists for SHOWCASE

**Stream**: SHOWCASE
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

The SHOWCASE domain (project cards, case study pages, metrics visualization, tech stack charts, AI collaboration display) has **zero stream-specific components built** — but its **entire foundation layer is ready**: data models, server functions, seed data, design tokens, and layout shell are all delivered and working.

---

## Existing Modules

### Data Layer (owned by DATA, consumed by SHOWCASE)

| File | What | Status | Lines |
|------|------|--------|-------|
| `src/models/project.rs` | `Project` struct — 13 fields including slug, tagline, tech_stack, image_path | Working | 61 |
| `src/models/experience.rs` | `Experience` struct — 10 fields | Working | 61 |
| `src/models/skill.rs` | `Skill` struct — 5 fields, seeded with 24 skills | Working | 48 |
| `src/models/cv_section.rs` | `CvSection` struct — 6 fields, 3 sections seeded | Working | 35 |
| `src/models/mod.rs` | Module hub with re-exports | Working | 11 |

### Server Functions (owned by DATA, consumed by SHOWCASE)

| File | Functions | Status |
|------|-----------|--------|
| `src/server_fns.rs` | `get_projects()`, `get_project_by_slug(slug)`, `get_experiences()`, `get_skills()`, `get_cv_sections()` | Working, 100 lines |

All return `Result<T, ServerFnError>`. Compile-time checked SQLx queries.

### Database (owned by DATA)

| File | What | Status |
|------|------|--------|
| `migrations/001_create_tables.sql` | DDL for projects, experiences, skills, cv_sections | Working |
| `migrations/002_seed_showcase_data.sql` | 5 projects, 24 skills, 3 CV sections seeded with real metrics | Working |

### Design System (owned by DESIGN, consumed by SHOWCASE)

| File | What | Status |
|------|------|--------|
| `style/tailwind.css` | OKLCH tokens: surfaces (6 levels), text (4 levels), accents, borders, shadows, animations, typography scale | Working, 271+ lines |
| `src/components/layout.rs` | `Layout`, `Header`, `Footer`, `NavLink` — responsive shell | Working, 86 lines |
| `src/components/theme_toggle.rs` | Light/System/Dark toggle with localStorage | Working, 117 lines |
| `public/fonts/` | Space Grotesk, Inter, JetBrains Mono (woff2, ~102KB) | Working |

### Routing (shared)

| File | What | Status |
|------|------|--------|
| `src/app.rs` | Routes: `/` (home), `/projects` (stub), `/about` (stub), 404 fallback. Leptos Router + SSR + meta + font preload + anti-FOUC | Working, 157 lines |

### Tests

| File | What | Status |
|------|------|--------|
| `tests/data_queries.rs` | 8 tests covering all server functions and data integrity | Working, 198 lines |

---

## What Does NOT Exist (SHOWCASE must build)

| Component | Description | Layer |
|-----------|-------------|-------|
| Project card component | Grid card with thumbnail, title, tagline, tech badges | L0 |
| Projects index page | `/projects` route — grid of cards with layout shell | L0 |
| Project detail page | `/projects/:slug` route — case study format | L0 |
| Basic metrics display | Numbers + badges for LOC, tests, coverage | L0 |
| Metrics visualization | Charts (bar, progress ring) for quantitative data | L1 |
| Tech stack chart | Radar/spider or tag cloud per project | L1 |
| Photo/video gallery | Lazy-loaded media per project | L1 |
| GitHub live stats | API integration for stars, commits, languages | L1 |
| AI collaboration section | Per-project human vs AI contribution narrative | L2 |
| Project filtering/sorting | By tech, date, type | L2 |
| Interactive code snippets | Embedded code examples per project | L2 |
| Animated chart transitions | Smooth data visualization animations | L3 |
| Skeleton loading states | Suspense fallbacks with personality | L3 |
| SEO per project page | leptos_meta + Open Graph images | L3 |

---

## Seeded Project Data (from ADR-002)

| # | Slug | Name | Stack | Key Metric |
|---|------|------|-------|------------|
| 1 | time | 3D temporal artwork | C11 + WebGL2 + WASM | 90K LOC, 95.9% coverage |
| 2 | blocksight | Bitcoin blockchain analytics | Node + React + TimescaleDB | 2,876 src files |
| 3 | anan-yarok | B2B wholesale e-commerce | Next.js + Prisma + PostgreSQL | Real business |
| 4 | chamana | Artisanal clothing catalog | Next.js + Payload CMS | Complete |
| 5 | gabriel-osemberg | This website (meta) | Rust + Leptos + WASM | Self-referential |

---

## Dependencies Status

| Need | From | Status |
|------|------|--------|
| Design tokens | DESIGN L0 | **DELIVERED** |
| Layout shell | DESIGN L0 | **DELIVERED** |
| Project data + server fns | DATA L0 | **DELIVERED** (models + server_fns.rs working) |
| Card component template | DESIGN L2.5 | PENDING (non-blocking — can build with tokens) |
| ProjectMetric model | DATA L1 | PENDING (L1 work) |
| TechTag normalization | DATA L1 | PENDING (can use raw tech_stack string for L0) |

**Conclusion**: SHOWCASE has no blockers for L0 build. All foundation dependencies are satisfied.
