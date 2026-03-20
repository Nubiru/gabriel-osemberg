# 006 — Dependencies: Cross-Stream Needs and Provides

**Stream**: DESIGN
**Date**: 2026-03-20
**Status**: COMPLETE

---

## What DESIGN Provides to Other Streams

### To SHOWCASE (P0 — they are blocked)

| What | Deliverable | Layer | Status |
|------|------------|-------|--------|
| Design tokens | `@theme` block with colors, typography, spacing, shadows | L0.1 | NOT STARTED |
| Layout shell | `Layout`, `Header`, `Footer` components | L0.4 | NOT STARTED |
| Card component | Reusable project card with hover effects | L2.5 | NOT STARTED |
| Badge/tag component | Tech stack badges | L1.4 | NOT STARTED |
| Dark/light theme | Working theme system SHOWCASE can use via `dark:` | L0.3 | NOT STARTED |
| Metrics visualization | Chart/progress components for project metrics | L2.5 | NOT STARTED |

**SHOWCASE can start building once L0 (tokens + layout + theme) is delivered.** They don't need to wait for L1 or L2.

### To IDENTITY (P0 — they are blocked)

| What | Deliverable | Layer | Status |
|------|------------|-------|--------|
| Design tokens | Same as above | L0.1 | NOT STARTED |
| Layout shell | Same as above | L0.4 | NOT STARTED |
| Timeline component | Experience timeline visual component | L2.5 | NOT STARTED |
| Section heading | Consistent heading component for CV sections | L1.4 | NOT STARTED |
| Dark/light theme | Same as above | L0.3 | NOT STARTED |
| Print stylesheet | Clean print layout for CV/PDF page | L3.5 | NOT STARTED |

**IDENTITY can start building once L0 is delivered.** Timeline component (L2.5) is needed before their L1 work.

### To INFRA

| What | Deliverable | Layer | Status |
|------|------------|-------|--------|
| Compilable project | DESIGN changes must not break `cargo leptos build` | L0 | NOT STARTED |
| Performance budget compliance | WASM < 200KB, FCP < 3s | L3.4 | NOT STARTED |
| Lighthouse scores | Accessibility >= 95, Performance >= 90 | L3.3 | NOT STARTED |

### To DATA

| What | Deliverable | Layer | Status |
|------|------------|-------|--------|
| Loading state patterns | How async data should look while loading | L1.3 | NOT STARTED |

---

## What DESIGN Needs from Other Streams

### From DATA

| What | Why | When | Blocking? |
|------|-----|------|-----------|
| Server function signatures | To know what loading states look like, what data shapes to expect for component props | L1 (loading states) | No — can mock data shapes |
| Actual data available | To test components with real content | L2 (visual components) | No — use mock data until then |

**DESIGN is NOT blocked by DATA.** We can build everything with mock data and hardcoded placeholder content. DATA integration happens when both streams are ready.

### From SHOWCASE

| What | Why | When | Blocking? |
|------|-----|------|-----------|
| Project card content requirements | What fields appear on a project card (title, description, tech stack, metrics, image?) | L2.5 (card component design) | Partially — but can infer from ADR-002 and ROADMAP.md |
| Metrics to visualize | Which metrics matter most (LOC, coverage, file count, commit count?) | L2.5 (metrics components) | Partially — but can infer from portfolio_inventory.md |

**DESIGN is NOT blocked by SHOWCASE.** ADR-002 and the portfolio inventory give enough information to design card and metrics components. We'll iterate when SHOWCASE provides specific requirements.

### From IDENTITY

| What | Why | When | Blocking? |
|------|-----|------|-----------|
| Timeline data shape | How many entries, what fields (company, role, dates, highlights?) | L2.5 (timeline component) | Partially — but can infer from ROADMAP Phase 4 |
| CV section structure | Which sections, what order | L3.5 (print stylesheet) | No — can design general layout |

**DESIGN is NOT blocked by IDENTITY.**

### From INFRA

| What | Why | When | Blocking? |
|------|-----|------|-----------|
| Nothing currently | DESIGN works locally | — | — |

---

## Dependency Summary

```
DESIGN provides → SHOWCASE, IDENTITY, INFRA (they wait for us)
DESIGN needs ← DATA, SHOWCASE, IDENTITY (nice-to-have, not blocking)

DESIGN IS THE BOTTLENECK for L0.
DESIGN IS NOT BLOCKED by anyone.
```

**Priority**: Deliver L0 as fast as possible. Every day L0 is not done, SHOWCASE and IDENTITY cannot build visual components.

---

## Messages to Send

### To SHOWCASE inbox
```
FROM: DESIGN
DATE: 2026-03-20
RE: L0 delivery timeline

DESIGN is starting Phase 0 research today. L0 (design tokens, layout shell,
dark/light theme) is the first build target. When L0 is delivered, you will
have:
- bg-surface-*, text-text-*, color-accent-* utility classes
- Layout/Header/Footer components to wrap your pages
- Dark/light theme that works via dark: variant

You can start building project cards and pages immediately after L0.

QUESTION: Can you confirm what fields appear on a project card? I'm inferring
from ADR-002: title, description, tech_stack[], metrics{}, image, links[].
Correct?
```

### To IDENTITY inbox
```
FROM: DESIGN
DATE: 2026-03-20
RE: L0 delivery timeline

Same as above — L0 will give you design tokens + layout shell + theme.

QUESTION: For the timeline component, how many experience entries should it
handle? And what fields: company, role, date_range, description, highlights[]?
This informs the timeline component design in L2.5.
```

---

## CROSS.md Updates

No new dependencies discovered beyond what's already in CROSS.md. The existing entries correctly capture:
- SHOWCASE → DESIGN: Card component, layout shell, design tokens (L0, OPEN)
- IDENTITY → DESIGN: Timeline component, layout shell, design tokens (L0, OPEN)
