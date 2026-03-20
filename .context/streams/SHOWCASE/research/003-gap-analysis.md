# 003 — Gap Analysis: Codebase vs Vision

**Stream**: SHOWCASE
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

The SHOWCASE domain has a **solid foundation** (data models, server functions, design tokens, layout shell) but **zero stream-specific code**. Every component the user will interact with on the `/projects` pages does not yet exist. The gap is entirely in the presentation layer — the data and design systems are ready.

---

## Gap Matrix

### CRITICAL — Must exist for L0 (Minimum Viable Showcase)

| Gap | What's Missing | What Exists | Severity | Blocked? |
|-----|---------------|-------------|----------|----------|
| Projects index page | No `/projects` route component — route defined but renders nothing | Route exists in `app.rs`, Layout shell available | **CRITICAL** | No |
| Project card component | No card component for listing projects | Project model, design tokens, 5 seeded projects | **CRITICAL** | No |
| Project detail page | No `/projects/:slug` route component | `get_project_by_slug()` server function ready | **CRITICAL** | No |
| Case study layout | No case study template (hero, context, approach, metrics, impact) | Project model has all needed fields | **CRITICAL** | No |
| Basic metrics display | No numbers/badges for LOC, tests, coverage per project | Data exists in seed migration descriptions (not yet in ProjectMetric model — L1) | **HIGH** | Partial — L0 can hardcode key metrics in project description, L1 adds ProjectMetric |

### HIGH — Required for compelling showcase (L1)

| Gap | What's Missing | What Exists | Severity | Blocked? |
|-----|---------------|-------------|----------|----------|
| Metrics visualization | No charts (bar, progress ring, radar) | Nothing | **HIGH** | No — hand-roll SVG |
| Tech stack display | No visual tech stack representation | `tech_stack` field as comma-separated string | **HIGH** | No |
| Project thumbnails | No screenshots/images for projects | `image_path` field exists, `public/` dir exists, no actual images | **HIGH** | Need screenshots from each project |
| Loading states | No Suspense/fallback components for async data | Leptos has built-in `Suspense` + `Transition` | **HIGH** | No |
| Error handling | No error boundary for failed data fetches | ServerFnError returns exist | **HIGH** | No |

### MEDIUM — Enhances quality (L2)

| Gap | What's Missing | What Exists | Severity | Blocked? |
|-----|---------------|-------------|----------|----------|
| AI collaboration section | No per-project human/AI breakdown display | No data model for this either | **MEDIUM** | Needs data model + seed data |
| Project filtering | No filter/sort UI for project cards | 5 projects (may not need filtering at this scale) | **MEDIUM** | No |
| GitHub live stats | No API integration | `repo_url` field on Project model | **MEDIUM** | Needs GitHub API key + caching |
| Interactive code snippets | No embedded code viewer | Nothing | **MEDIUM** | No |

### LOW — Polish layer (L3)

| Gap | What's Missing | What Exists | Severity | Blocked? |
|-----|---------------|-------------|----------|----------|
| Animated chart transitions | No animation system for data viz | CSS animation tokens in tailwind.css | **LOW** | No |
| Skeleton loading states | No custom skeleton components | Leptos Suspense fallback available | **LOW** | No |
| SEO per project | No leptos_meta usage per project page | leptos_meta dependency exists | **LOW** | No |
| Open Graph images | No per-project social preview images | Nothing | **LOW** | Needs image generation strategy |

---

## Gap Severity Distribution

```
CRITICAL  ████████████  4 gaps  (L0 — blocking launch)
HIGH      ██████████    5 gaps  (L1 — required for quality)
MEDIUM    ████████      4 gaps  (L2 — differentiators)
LOW       ████          4 gaps  (L3 — polish)
                       17 gaps total
```

---

## What's NOT a Gap (Already Solved)

| Concern | Status | Evidence |
|---------|--------|----------|
| Data model | DONE | `src/models/project.rs` — 13 fields, working |
| Server functions | DONE | `src/server_fns.rs` — 5 functions, working |
| Seed data | DONE | `migrations/002_seed_showcase_data.sql` — 5 projects with real metrics |
| Design tokens | DONE | `style/tailwind.css` — OKLCH palette, typography, spacing, shadows |
| Layout shell | DONE | `src/components/layout.rs` — Header, Footer, NavLink |
| Theme support | DONE | `src/components/theme_toggle.rs` — Light/System/Dark |
| Routing infrastructure | DONE | `src/app.rs` — Router with SSR + hydration |
| Data layer tests | DONE | `tests/data_queries.rs` — 8 tests, full coverage |

---

## Content Gaps (Non-Code)

| Content | Status | Action Needed |
|---------|--------|---------------|
| Project screenshots | MISSING | Capture screenshots from time, blocksight, anan-yarok, chamana |
| Project demo videos | MISSING | Record short demo clips (optional for L0) |
| Case study copy | PARTIAL | Seed data has taglines + descriptions. Full case study narrative needed per project |
| AI collaboration data | MISSING | Define human vs AI contribution percentages/narrative per project |

---

## Priority Order for Build Phase

Based on gap severity and dependencies:

1. **Project card component** → enables index page
2. **Projects index page** (`/projects`) → first visible output
3. **Project detail page** (`/projects/:slug`) → case study pages
4. **Case study template** → structure for detail pages
5. **Tech stack tags display** → visual tech representation on cards + detail
6. **Basic metrics row** → numbers on cards (LOC, tests, coverage)
7. **Metrics charts** (SVG) → visualization on detail pages
8. **Loading/error states** → production readiness
9. **Project thumbnails** → visual assets
10. **AI collaboration section** → unique differentiator

---

## Cross-Stream Gaps

| Gap | Depends On | Stream | Status |
|-----|-----------|--------|--------|
| ProjectMetric model (per-project metrics as structured data) | DATA L1 | DATA | Not yet built |
| TechTag normalization (tags as entities vs comma-separated string) | DATA L1 | DATA | Not yet built |
| Card component template | DESIGN L2.5 | DESIGN | Not yet built (non-blocking — can build with tokens) |
| Project screenshots | SHOWCASE itself | N/A | Need to capture from actual projects |
| AI collaboration data model | DATA + SHOWCASE | Both | Not yet designed |

---

## Conclusion

**The gap is entirely in the presentation layer.** All data infrastructure, design infrastructure, and routing infrastructure is ready. SHOWCASE's L0 build can start immediately with no blockers. The 4 CRITICAL gaps (index page, card component, detail page, case study layout) are pure Leptos component work using existing server functions and design tokens.
