# 005 — Roadmap: Layered Build Plan (L0 → L3)

**Stream**: DESIGN
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

Four layers from foundation to perfection. L0 unblocks other streams. L1 makes pages functional. L2 makes the site impressive. L3 makes it bulletproof. Each layer builds on the previous. TDD applies to every component.

---

## L0 — Foundation (Unblocks SHOWCASE + IDENTITY)

**Goal**: Design tokens, fonts, dark/light theme, layout shell. After L0, other streams can build visual components.

### L0.1: Design Token System
- [ ] Define OKLCH color palette (surfaces, text, accents, borders, states)
- [ ] Define typography scale (sizes, weights, line-heights, letter-spacing)
- [ ] Define spacing system (base unit + derived values)
- [ ] Define shadow system (sm, md, lg, xl)
- [ ] Define border radius system (sm, md, lg, full)
- [ ] Define breakpoints (sm, md, lg, xl)
- [ ] Define easing curves (ease-default, ease-snappy, ease-fluid)
- [ ] Write all tokens in `style/tailwind.css` using `@theme`
- [ ] Test: tokens compile, utilities generate, CSS variables accessible

**Files**: `style/tailwind.css`

### L0.2: Font System
- [ ] Download Space Grotesk, Inter, JetBrains Mono (woff2, variable)
- [ ] Place in `public/fonts/`
- [ ] Add `@font-face` declarations in `style/tailwind.css`
- [ ] Register in `@theme` as `--font-display`, `--font-body`, `--font-mono`
- [ ] Add font preload `<link>` tags in `shell()`
- [ ] Test: fonts load, correct weights render, fallbacks work

**Files**: `style/tailwind.css`, `src/app.rs`, `public/fonts/`

### L0.3: Dark/Light Theme System
- [ ] Add `@custom-variant dark` to `style/tailwind.css`
- [ ] Define dark mode semantic token overrides in `@layer base`
- [ ] Create anti-FOUC script in shell `<head>`
- [ ] Create `ThemeToggle` component (3-way: light/dark/system)
- [ ] Implement `localStorage` persistence
- [ ] Add smooth color transition on toggle (`transition-colors`)
- [ ] Test: respects OS preference, toggle works, persists across reload, no FOUC

**Files**: `style/tailwind.css`, `src/app.rs`, `src/components/theme_toggle.rs`

### L0.4: Layout Shell
- [ ] Create `Layout` wrapper component (provides header + main + footer structure)
- [ ] Create `Header` component (logo/name, navigation placeholder, theme toggle)
- [ ] Create `Footer` component (links, copyright, "Built with Rust + Leptos")
- [ ] Add semantic HTML (`<header>`, `<main>`, `<footer>`, `<nav>`)
- [ ] Ensure proper document flow and flex/grid layout
- [ ] Test: shell renders, semantic structure correct, responsive at all breakpoints

**Files**: `src/components/layout.rs`, `src/components/header.rs`, `src/components/footer.rs`, `src/app.rs`

### L0.5: Module Structure
- [ ] Create `src/components/` module with `mod.rs`
- [ ] Establish component file naming convention
- [ ] Create shared component types/traits if needed
- [ ] Update `src/lib.rs` to export components module

**Files**: `src/components/mod.rs`, `src/lib.rs`

**L0 Delivery Criteria**:
- `cargo leptos watch` compiles and serves with design tokens active
- Dark/light toggle works with persistence
- Layout shell visible on all pages
- Fonts load correctly
- Other streams can use `bg-surface-base`, `text-text-primary`, `font-display`, etc.

---

## L1 — Structure (Makes multi-page app functional)

**Goal**: Navigation, responsive system, loading/error states. After L1, the app is navigable and handles all UI states.

### L1.1: Responsive Navigation
- [ ] Desktop navigation bar (horizontal links)
- [ ] Mobile hamburger menu with slide-out drawer
- [ ] Active route highlighting
- [ ] Keyboard navigation (Tab, Enter, Escape to close)
- [ ] Skip-to-content link (accessibility)
- [ ] ARIA attributes on nav elements
- [ ] Test: navigation works at all breakpoints, keyboard accessible

**Files**: `src/components/nav.rs`, `src/components/mobile_menu.rs`

### L1.2: Responsive Design System
- [ ] Verify breakpoints work correctly with Leptos
- [ ] Create responsive container component (max-width + padding)
- [ ] Implement mobile-first approach across layout
- [ ] Test touch targets >= 44px on mobile
- [ ] Test: layout shifts gracefully at every breakpoint

**Files**: `src/components/container.rs`

### L1.3: Loading & Error States
- [ ] Skeleton screen component (pulsing placeholder)
- [ ] Spinner/loading indicator component
- [ ] Error boundary component with styled error messages
- [ ] 404 page design
- [ ] Test: loading states display correctly, error boundaries catch errors

**Files**: `src/components/loading.rs`, `src/components/error.rs`, `src/pages/not_found.rs`

### L1.4: Base Component Library
- [ ] Button component (variants: primary, secondary, ghost, icon)
- [ ] Link component (internal vs external, with arrow/icon)
- [ ] Badge/tag component (for tech stacks)
- [ ] Section heading component (consistent spacing + typography)
- [ ] Test: all components render correctly with design tokens

**Files**: `src/components/button.rs`, `src/components/link.rs`, `src/components/badge.rs`, `src/components/heading.rs`

**L1 Delivery Criteria**:
- Full navigation works across all routes
- Mobile experience is polished
- Loading and error states are handled
- Base components available for SHOWCASE + IDENTITY to use

---

## L2 — Enhancement (Makes the site impressive)

**Goal**: Animations, hero section, visual effects. After L2, the site has "wow factor."

### L2.1: Scroll-Triggered Reveals
- [ ] Create `ScrollReveal` wrapper component using Intersection Observer (web-sys)
- [ ] Support reveal variants: fade-up, fade-in, slide-left, slide-right
- [ ] Configurable threshold and delay
- [ ] Respect `prefers-reduced-motion`
- [ ] Test: elements animate on scroll, respect motion preference

**Files**: `src/components/scroll_reveal.rs`

### L2.2: Page Transitions
- [ ] Implement View Transitions API wrapper (web-sys)
- [ ] Integrate with Leptos router navigation
- [ ] CSS transition definitions (fade, slide, morph)
- [ ] Fallback for unsupported browsers (instant transition)
- [ ] Test: page transitions animate, fallback works

**Files**: `src/components/page_transition.rs`, `style/tailwind.css`

### L2.3: Microinteractions
- [ ] Hover effects on all interactive elements (buttons, cards, links)
- [ ] Focus ring styling (visible, accessible, themed)
- [ ] Click/tap feedback (scale transform)
- [ ] Tooltip component
- [ ] Test: interactions feel responsive, accessible

**Files**: `style/tailwind.css` (interaction tokens), `src/components/tooltip.rs`

### L2.4: Hero Section
- [ ] Design hero layout (name, title, value proposition, CTA)
- [ ] Animated text reveal (staggered entrance)
- [ ] Subtle background effect (gradient mesh, particles, or geometric pattern)
- [ ] CTA buttons (View Projects, Download CV)
- [ ] Responsive hero for mobile
- [ ] Test: hero renders, animations play, CTAs work

**Files**: `src/components/hero.rs`

### L2.5: Visual Components for Other Streams
- [ ] Card component (for project cards — shared with SHOWCASE)
- [ ] Timeline component (for experience — shared with IDENTITY)
- [ ] Metrics visualization base (bar charts, progress rings — shared with SHOWCASE)
- [ ] Test: components render with mock data, responsive

**Files**: `src/components/card.rs`, `src/components/timeline.rs`, `src/components/metrics.rs`

**L2 Delivery Criteria**:
- Site has motion and feels alive
- Hero section makes strong first impression
- All interactive elements have feedback
- Visual components ready for SHOWCASE + IDENTITY data integration

---

## L3 — Perfection (Makes the site bulletproof)

**Goal**: Testing, cross-browser, accessibility audit, print, performance. After L3, the site is production-ready.

### L3.1: Visual Regression Testing
- [ ] Set up Playwright screenshot comparison
- [ ] Capture baseline screenshots (dark + light, mobile + desktop)
- [ ] Add to CI pipeline
- [ ] Test: visual regressions detected

### L3.2: Cross-Browser Polish
- [ ] Test in Chrome, Firefox, Safari (desktop)
- [ ] Test in Chrome, Safari (mobile/iOS)
- [ ] Fix any rendering inconsistencies
- [ ] Test WASM loading across browsers

### L3.3: Accessibility Audit
- [ ] Run Lighthouse accessibility audit (target >= 95)
- [ ] Test with screen reader (VoiceOver or NVDA)
- [ ] Verify keyboard navigation on every interactive element
- [ ] Verify color contrast ratios (APCA via OKLCH)
- [ ] Add missing ARIA attributes
- [ ] Test: Lighthouse >= 95, no critical a11y issues

### L3.4: Performance Optimization
- [ ] Audit WASM bundle size (target < 200KB gzipped)
- [ ] Optimize font loading (subset if needed)
- [ ] Audit CSS output size
- [ ] Lighthouse performance score >= 90
- [ ] First Contentful Paint < 3s

### L3.5: Print Stylesheet
- [ ] Create print-specific styles for CV page
- [ ] Hide navigation, footer, interactive elements in print
- [ ] Ensure clean, ATS-friendly layout when printed
- [ ] Test: print preview looks professional

### L3.6: Custom Scrollbar
- [ ] Style scrollbar to match theme (webkit + Firefox)
- [ ] Ensure scrollbar is visible and usable
- [ ] Respect OS scrollbar preference

**L3 Delivery Criteria**:
- All Lighthouse scores >= 90 (accessibility >= 95)
- Visual regression tests in CI
- Print-ready CV page
- Cross-browser verified
- WASM < 200KB gzipped

---

## Layer Dependencies

```
L0 (Foundation)
 ├── L0.1 Design Tokens  ←── everything depends on this
 ├── L0.2 Fonts  ←── L0.1
 ├── L0.3 Theme  ←── L0.1
 ├── L0.4 Layout Shell  ←── L0.1, L0.2, L0.3
 └── L0.5 Module Structure  ←── first

L1 (Structure)  ←── all of L0
 ├── L1.1 Navigation  ←── L0.4
 ├── L1.2 Responsive  ←── L0.1, L0.4
 ├── L1.3 Loading/Error  ←── L0.1
 └── L1.4 Base Components  ←── L0.1, L0.2

L2 (Enhancement)  ←── all of L1
 ├── L2.1 Scroll Reveals  ←── L1 (needs pages to scroll)
 ├── L2.2 Page Transitions  ←── L1.1 (needs routes)
 ├── L2.3 Microinteractions  ←── L1.4 (needs base components)
 ├── L2.4 Hero  ←── L0.4, L1.4
 └── L2.5 Shared Visual Components  ←── L0.1, L1.4

L3 (Perfection)  ←── all of L2
 └── all items can run in parallel
```

---

## Estimated Build Order (within sessions)

| Session | Tasks | Unblocks |
|---------|-------|----------|
| Build 1 | L0.5 + L0.1 + L0.2 | Token system, fonts |
| Build 2 | L0.3 + L0.4 | Theme + layout → SHOWCASE, IDENTITY unblocked |
| Build 3 | L1.1 + L1.2 | Navigation, responsive |
| Build 4 | L1.3 + L1.4 | Loading states, base components |
| Build 5 | L2.1 + L2.3 | Scroll reveals, microinteractions |
| Build 6 | L2.2 + L2.4 | Page transitions, hero section |
| Build 7 | L2.5 | Shared visual components for other streams |
| Build 8+ | L3.* | Production readiness |
