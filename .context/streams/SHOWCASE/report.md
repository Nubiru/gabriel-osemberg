# SHOWCASE — Build Report: L0 Foundation

**Date**: 2026-03-20
**Tasks**: L0-001, L0-002, L0-003, L0-004 (+ L0-005 through L0-008 resolved)
**Status**: COMPLETE

---

## Delivered

### New Files

| File | Component | Lines | Tests |
|------|-----------|-------|-------|
| `src/components/project_card.rs` | `ProjectCard` — card for projects grid | 125 | 4 unit tests |
| `src/components/projects_page.rs` | `ProjectsPage` — `/projects` route | 71 | — |
| `src/components/project_detail.rs` | `ProjectDetailPage` — `/projects/:slug` route | 179 | — |

### Modified Files

| File | Change |
|------|--------|
| `src/components/mod.rs` | Added 3 new module exports |
| `src/app.rs` | Added 2 routes: `/projects`, `/projects/:slug` |

### Resolved Without Code

| Task | Resolution |
|------|-----------|
| L0-005 Tech stack tags | Existing `Badge` component + split logic in ProjectCard |
| L0-006 Metrics badge row | Deferred to L1 (needs DATA `ProjectMetric` model) |
| L0-007 Loading fallback | Already delivered by DESIGN (`Skeleton`, `SkeletonCard`, `Spinner`) |
| L0-008 Error display | Already delivered by DESIGN (`ErrorDisplay`) |

---

## Architecture Decisions

1. **ProjectCard takes `Project` by value** — destructured into owned fields to avoid borrow checker issues with Leptos's `view!` macro. Justified: cards are rendered from a list iterator, each project is consumed once.

2. **Tech stack parsed from comma-separated string** — `tech_stack.split(", ")` produces `Badge` pills. Simple and correct for L0. L1 will normalize to `TechTag` entities via DATA stream.

3. **Case study layout is flat** — all 7 sections (back link, hero, tech badges, description paragraphs, external links) rendered in a single component. L1/L2 will extract sub-components as they grow.

4. **Description rendered as paragraphs** — split on `"\n\n"` for multi-paragraph content from the database.

---

## Quality Gates

| Gate | Result |
|------|--------|
| `cargo build` | PASS |
| `cargo clippy -- -D warnings` | PASS (zero warnings) |
| `cargo test --features ssr` | PASS (15/15) |
| `cargo fmt --check` | PASS |
| No `.unwrap()` in app code | PASS |
| No unnecessary `.clone()` | PASS |
| Design tokens used | PASS (all colors, fonts, spacing from tokens) |
| Accessibility | Back link, aria-hidden on icons, semantic headings, focus-visible outlines |

---

## What's Next

L1 Integration: SVG charts (progress ring, bar chart), project thumbnails, case study hero enhancement, responsive grid refinement, and page meta tags.
