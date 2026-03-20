# 003 — Gap Analysis: What's Missing Between Codebase and Vision

**Stream**: DESIGN
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

The codebase is a **bare scaffold** — a default Leptos starter with Tailwind v4 imported but unconfigured. The vision (from ROADMAP.md Phase 1 + Phase 5) requires a complete design system, layout shell, theme system, animation framework, and responsive architecture. Every gap is severity P0 or P1 because nothing exists yet.

---

## Gap Inventory

### P0 — Foundation (Must exist before ANY other stream can build)

Other streams (SHOWCASE, IDENTITY) are blocked on these items. CROSS.md shows SHOWCASE and IDENTITY both depend on DESIGN's layout shell and design tokens.

| # | Gap | Current State | Required State | Severity | Blocks |
|---|-----|--------------|----------------|----------|--------|
| 1 | **Design token system** | Zero tokens. Default Tailwind only. | Full `@theme` block: colors (OKLCH), typography, spacing, shadows, radii, breakpoints | P0 | Everything |
| 2 | **Color palette** | `bg-blue-600` (scaffold default) | OKLCH-based palette: surfaces, text, accents, states. Light + dark variants. | P0 | All visual work |
| 3 | **Typography system** | No web fonts loaded | Font loading (Space Grotesk + Inter + JetBrains Mono), size scale, weight scale, line-height scale | P0 | All text rendering |
| 4 | **Layout shell** | Content renders directly in `<main>` | Header + Footer + main content area component. Consistent page wrapper. | P0 | SHOWCASE, IDENTITY |
| 5 | **Dark/light theme** | No theme system | `prefers-color-scheme` detection, class-based toggle, `localStorage` persistence, smooth transition | P0 | All streams |

### P1 — Structure (Must exist before pages can be built)

| # | Gap | Current State | Required State | Severity | Blocks |
|---|-----|--------------|----------------|----------|--------|
| 6 | **Responsive navigation** | No navigation (single page) | Desktop nav + mobile hamburger/drawer. Route links. Accessible. | P1 | Multi-page app |
| 7 | **Responsive breakpoints** | Tailwind defaults (untested) | Verified breakpoints in @theme. Mobile-first approach. Container queries where needed. | P1 | Mobile experience |
| 8 | **Loading states** | No loading UI | Skeleton screens or spinner for async data (server functions) | P1 | DATA integration |
| 9 | **Error boundary styling** | `"Page not found.".into_view()` (text only) | Styled 404 page + error boundary component | P1 | Polish |
| 10 | **Component API patterns** | Only `HomePage` exists | Reusable component conventions: props, children slots, composition | P1 | All component work |

### P2 — Enhancement (Required for "impressive" — Phase 5 scope)

| # | Gap | Current State | Required State | Severity | Blocks |
|---|-----|--------------|----------------|----------|--------|
| 11 | **Page transitions** | None | View Transitions API integration with Leptos router | P2 | Polish |
| 12 | **Scroll-triggered reveals** | None | Intersection Observer system for section reveals | P2 | Polish |
| 13 | **Microinteractions** | `hover:bg-blue-700` only | Hover, focus, click feedback on all interactive elements | P2 | Polish |
| 14 | **Hero section** | "Gabriel Osemberg" h1 + counter button | Impactful first impression with identity + value proposition + CTA | P2 | First impression |
| 15 | **Custom scrollbar** | Browser default | Styled scrollbar matching theme | P2 | Visual cohesion |

### P3 — Perfection (Required for "world-class")

| # | Gap | Current State | Required State | Severity | Blocks |
|---|-----|--------------|----------------|----------|--------|
| 16 | **Visual regression testing** | None | Playwright screenshot comparison | P3 | Quality assurance |
| 17 | **Cross-browser polish** | Untested | Chrome + Firefox + Safari verified | P3 | Production |
| 18 | **Print stylesheet** | None | Clean print layout for CV page | P3 | PDF/print |
| 19 | **Lighthouse ≥ 95 accessibility** | Untested | Full WCAG 2.1 AA compliance | P3 | Accessibility |
| 20 | **Cursor/pointer effects** | None | Subtle, non-distracting cursor feedback | P3 | Delight |

---

## Cross-Stream Impact

### What DESIGN blocks

| Stream | What They Need | When |
|--------|---------------|------|
| **SHOWCASE** | Card component, layout shell, design tokens | L0 (their first buildable task) |
| **IDENTITY** | Timeline component styling, layout shell, design tokens | L0 (their first buildable task) |
| **INFRA** | Working build (DESIGN changes affect compilation) | L0 (deployment needs compilable code) |

### What DESIGN needs from others

| Stream | What We Need | When |
|--------|-------------|------|
| **DATA** | Server function signatures (to know loading states shape) | P1 (for loading/error states) |
| **SHOWCASE** | Project card content requirements (to design card component) | P1 (for visual design) |
| **IDENTITY** | Timeline data shape (to design timeline component) | P1 (for visual design) |

---

## Priority Matrix

```
URGENCY →
        Low              Medium           High
High    P2: transitions  P1: navigation   P0: tokens, colors,
        P2: scroll       P1: breakpoints  typography, layout,
        P2: microinteract P1: loading     theme
IMPACT  P2: hero section P1: error states
↑
Medium  P3: print CSS    P2: scrollbar    P1: component API
        P3: cursor fx    P3: visual regr.

Low     P3: cross-browser
        polish
```

**Critical path**: Design tokens → Color palette → Typography → Dark/light theme → Layout shell → Navigation → Components for other streams.

---

## Effort Estimates (relative sizing)

| Gap | Size | Notes |
|-----|------|-------|
| Design tokens (@theme) | M | Research complete, writing CSS |
| Color palette (OKLCH) | M | Need to select hues, generate scales |
| Typography (font loading) | S | 3 fonts, load + configure |
| Layout shell (Header/Footer) | M | Components + responsive |
| Dark/light theme | M | Toggle + persistence + transitions |
| Navigation | M | Desktop + mobile + accessibility |
| Page transitions | L | View Transitions API + web-sys |
| Scroll reveals | M | Intersection Observer setup |
| Hero section | L | Design + animation + content |
| Loading/error states | S | Component patterns |

S = small (< 1 session), M = medium (1 session), L = large (2+ sessions)

---

## Conclusion

The gap is 100% — everything must be built. But the foundation (Leptos + Tailwind v4) is solid and ready. The research from 002-world-survey.md gives clear direction for every gap. The critical path is clear: **P0 design tokens → P0 layout shell → P0 theme → P1 navigation → hand off to SHOWCASE + IDENTITY**.

DESIGN is the bottleneck. Other streams cannot build visual components until DESIGN delivers the foundation.
