# Stream: DESIGN
**Phase**: 1 — ROADMAP (advancing to BUILD)
**Owner**: DESIGN stream orchestrator
**Last Updated**: 2026-03-20
**Readiness**: GREEN

## Current Work

Research phase COMPLETE (7/7 sections). Roadmap defined in `research/005-roadmap.md` with L0-L3 layered plan. Cross-stream messages sent to SHOWCASE and IDENTITY inboxes. DATA has already responded with model definitions — no blocking questions remain.

Ready to advance to Phase 2 (BUILD) on next session launch.

## Research Progress

| # | Section | Status |
|---|---------|--------|
| 1 | 001-inventory.md | COMPLETE |
| 2 | 002-world-survey.md | COMPLETE |
| 3 | 003-gap-analysis.md | COMPLETE |
| 4 | 004-questions.md | COMPLETE |
| 5 | 005-roadmap.md | COMPLETE |
| 6 | 006-dependencies.md | COMPLETE |
| 7 | 007-resources.md | COMPLETE |

## Key Design Decisions (from research)

- **Color**: OKLCH, dark-first, blue-violet base (H≈265) + cyan-teal accent (H≈195)
- **Typography**: Space Grotesk (headings) + Inter (body) + JetBrains Mono (code), self-hosted woff2 variable
- **Theme**: Class-based dark mode via `@custom-variant dark`, localStorage persistence, anti-FOUC script
- **Tokens**: All in `style/tailwind.css` via Tailwind v4 `@theme`, semantic naming
- **Animation**: CSS-first, Intersection Observer for scroll reveals, View Transitions API for pages
- **Icons**: Inline SVGs initially
- **Layout**: Semantic HTML, Header + Footer + main, responsive navigation

## Build Progress

### L0 Foundation (NEXT — unblocks SHOWCASE + IDENTITY)
- [ ] L0.5: Module structure (`src/components/`)
- [ ] L0.1: Design tokens (@theme in tailwind.css)
- [ ] L0.2: Font system (self-hosted woff2)
- [ ] L0.3: Dark/light theme (toggle + persistence)
- [ ] L0.4: Layout shell (Header, Footer, Layout components)

### L1 Structure
- [ ] L1.1: Responsive navigation (desktop + mobile)
- [ ] L1.2: Responsive design system (containers, breakpoints)
- [ ] L1.3: Loading & error states
- [ ] L1.4: Base component library (Button, Link, Badge, Heading)

### L2 Enhancement
- [ ] L2.1: Scroll-triggered reveals (Intersection Observer)
- [ ] L2.2: Page transitions (View Transitions API)
- [ ] L2.3: Microinteractions (hover, focus, click)
- [ ] L2.4: Hero section
- [ ] L2.5: Shared visual components (Card, Timeline, Metrics)

### L3 Perfection
- [ ] L3.1: Visual regression testing
- [ ] L3.2: Cross-browser polish
- [ ] L3.3: Accessibility audit (Lighthouse >= 95)
- [ ] L3.4: Performance optimization (WASM < 200KB)
- [ ] L3.5: Print stylesheet for CV page
- [ ] L3.6: Custom scrollbar

## Cross-Stream Communication

**Sent**:
- → SHOWCASE: L0 delivery notice + card field question (answered by DATA)
- → IDENTITY: L0 delivery notice + timeline field question (answered by DATA)

**Received**:
- ← DATA (via IDENTITY inbox): Experience model confirmed — role, company, dates, description, highlights[]
- ← DATA (via SHOWCASE inbox): Project model confirmed — name, tagline, tech_stack, image_path, repo_url, live_url

## Blockers

None. DESIGN is not blocked by any stream. DESIGN IS the bottleneck — L0 must be delivered for SHOWCASE and IDENTITY to build.

## Metrics

- Research sections: 7/7
- Tests: 0
- Source files: 0
- Commits: 0
