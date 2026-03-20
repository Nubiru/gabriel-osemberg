# 005 — Roadmap: Layered Build Plan

**Stream**: SHOWCASE
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

4-layer plan from minimum viable showcase to polished production. Each layer builds on the previous. All L0 tasks have no external blockers.

---

## L0 — Foundation (Minimum Viable Showcase)

**Goal**: A user can visit `/projects`, see project cards, click one, and read a case study page with basic metrics.

**Axis served**: PROOF — "Does this demonstrate Gabriel's skill?"

| # | Task | Component/File | Dependencies | Test First |
|---|------|---------------|--------------|------------|
| L0-001 | Project card component | `src/components/project_card.rs` | Project model, design tokens | Test: renders name, tagline, tech tags, link |
| L0-002 | Projects index page | `src/components/projects_page.rs` | L0-001, get_projects() | Test: fetches projects, renders cards in grid |
| L0-003 | Project detail page (case study) | `src/components/project_detail.rs` | get_project_by_slug() | Test: fetches by slug, renders all 7 sections |
| L0-004 | Route wiring | `src/app.rs` | L0-002, L0-003 | Test: /projects and /projects/:slug resolve |
| L0-005 | Tech stack tags component | `src/components/tech_tag.rs` | Design tokens | Test: parses comma-separated string, renders pills |
| L0-006 | Metrics badge row | `src/components/metrics_badge.rs` | Design tokens | Test: renders number + label pairs |
| L0-007 | Loading fallback component | `src/components/loading.rs` | Leptos Suspense | Test: renders during async fetch |
| L0-008 | Error display component | `src/components/error_display.rs` | ServerFnError | Test: renders user-friendly error message |

**Deliverable**: 8 new components, 2 routed pages, 8+ tests. A user can browse all 5 projects.

**Estimated new files**: ~8 component files + test file additions

---

## L1 — Integration (Compelling Visualization)

**Goal**: Projects have visual metrics (SVG charts), proper thumbnails, and the showcase feels polished enough to share with a recruiter.

**Axis served**: PROOF + CRAFT

| # | Task | Component/File | Dependencies | Test First |
|---|------|---------------|--------------|------------|
| L1-001 | Progress ring (SVG) | `src/components/charts/progress_ring.rs` | leptos::svg | Test: coverage % → correct stroke-dasharray |
| L1-002 | Horizontal bar chart (SVG) | `src/components/charts/bar_chart.rs` | leptos::svg | Test: values → correct rect widths |
| L1-003 | Metrics dashboard per project | `src/components/metrics_dashboard.rs` | L1-001, L1-002 | Test: renders ring for coverage, bars for LOC |
| L1-004 | Project thumbnail component | `src/components/project_thumbnail.rs` | Asset files in public/ | Test: renders img with lazy loading, fallback |
| L1-005 | Case study hero section | `src/components/case_study_hero.rs` | L1-004, design tokens | Test: renders hero image, title overlay, tech tags |
| L1-006 | Responsive card grid layout | Update project_card.rs | Tailwind responsive classes | Test: 1-col mobile, 2-col tablet, 3-col desktop |
| L1-007 | Page meta tags per project | Update project_detail.rs | leptos_meta | Test: title and description set from project data |

**Deliverable**: SVG charts, thumbnails, responsive grid, SEO meta. Recruiter-shareable.

---

## L2 — Enhancement (Differentiators)

**Goal**: The AI collaboration methodology is visually presented. Projects can be compared. The showcase tells a story, not just displays data.

**Axis served**: PROOF + PRESENCE

| # | Task | Component/File | Dependencies | Test First |
|---|------|---------------|--------------|------------|
| L2-001 | AI collaboration section | `src/components/ai_collaboration.rs` | Case study data model | Test: renders methodology narrative + contribution breakdown |
| L2-002 | MEGA framework diagram (SVG) | `src/components/charts/framework_diagram.rs` | leptos::svg | Test: renders correct node count and connections |
| L2-003 | Project comparison highlight | `src/components/project_highlights.rs` | All project data | Test: renders top metrics across all projects |
| L2-004 | Tech stack tag normalization | Consume DATA L1 TechTag model | DATA stream L1 | Test: tags render as linked, grouped entities |
| L2-005 | Project filtering (if 5+ projects) | Update projects_page.rs | L2-004 | Test: filter by tech tag updates visible cards |
| L2-006 | Scroll-triggered reveal animations | Update all showcase components | CSS/Intersection Observer | Test: elements have animation classes |

**Deliverable**: AI methodology showcase, framework diagram, enhanced interactivity.

---

## L3 — Perfection (Polish)

**Goal**: Every detail is refined. Animations are smooth. Loading states have personality. The showcase is indistinguishable from a professional product.

**Axis served**: CRAFT + PRESENCE

| # | Task | Component/File | Dependencies | Test First |
|---|------|---------------|--------------|------------|
| L3-001 | Animated chart transitions | Update chart components | CSS transitions | Test: animation classes applied on data change |
| L3-002 | Skeleton loading states | `src/components/skeleton.rs` | Design tokens | Test: renders skeleton matching card/detail layout |
| L3-003 | Open Graph images per project | Server-side image generation | Image generation strategy | Test: OG meta tags present per page |
| L3-004 | Project demo video embeds | `src/components/video_player.rs` | Video assets | Test: renders video with controls, lazy loads |
| L3-005 | GitHub live stats integration | `src/components/github_stats.rs` | GitHub API, caching | Test: fetches and displays repo stats |
| L3-006 | Lighthouse optimization pass | All showcase components | Complete L0-L2 | Test: Lighthouse >= 90 all categories |
| L3-007 | E2E tests (Playwright) | `tests/e2e/showcase.spec.ts` | All showcase pages | Full user journey tests |

**Deliverable**: Production-grade polish. Every detail intentional.

---

## Layer Dependencies

```
L0 Foundation ──→ L1 Integration ──→ L2 Enhancement ──→ L3 Perfection
     │                  │                   │
     │                  │                   └── Needs DATA L1 (TechTag)
     │                  └── Needs project screenshots (manual)
     └── No blockers (all dependencies satisfied)
```

---

## Task Count Summary

| Layer | Tasks | New Files | Tests |
|-------|-------|-----------|-------|
| L0 | 8 | ~8 | 8+ |
| L1 | 7 | ~5 | 7+ |
| L2 | 6 | ~4 | 6+ |
| L3 | 7 | ~3 | 7+ |
| **Total** | **28** | **~20** | **28+** |

---

## Build Order (Critical Path)

```
1. L0-001 ProjectCard  ──→ L0-002 ProjectsPage  ──→ L0-004 Routes
2. L0-003 ProjectDetail (parallel with #1)
3. L0-005 TechTags + L0-006 MetricsBadge (parallel, feed into cards + detail)
4. L0-007 Loading + L0-008 Error (parallel, production readiness)
5. L1-001 ProgressRing + L1-002 BarChart (parallel, chart primitives)
6. L1-003 MetricsDashboard (depends on charts)
7. L1-004 Thumbnail + L1-005 CaseStudyHero (parallel, visual upgrade)
8. L1-006 ResponsiveGrid + L1-007 PageMeta (parallel, polish)
```

**First visible milestone**: After L0-004 (route wiring), a user can browse projects. Target: first 4 tasks.
