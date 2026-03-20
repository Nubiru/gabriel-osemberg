# DESIGN — Build Report: L0.5 + L0.1 + L0.2 + L0.3 + L0.4

**Date**: 2026-03-20
**Task**: Foundation — Module structure, Design tokens, Font system, Theme system, Layout shell
**Result**: PASS

---

## What Was Built

### L0.5: Module Structure
- Created `src/components/mod.rs` with module declarations
- Updated `src/lib.rs` to export `components` module
- Added `web-sys` dependency (hydrate feature) for theme toggle DOM access

### L0.1: Design Token System (271 lines in `style/tailwind.css`)
- **Colors**: Full OKLCH palette — 4 surfaces, 4 text levels, 3 accent variants, 3 border variants, 4 state colors
- **Typography**: 3 font families (display, body, mono), 10 size steps, 5 weights, 4 line-heights, 4 letter-spacings
- **Spacing**: Base unit `0.25rem` (Tailwind-standard)
- **Shadows**: 4 levels (sm/md/lg/xl), dark-optimized with light-mode overrides
- **Border Radius**: 6 levels (sm through full)
- **Breakpoints**: 4 levels (sm/md/lg/xl)
- **Easing**: 3 curves (default, snappy, fluid)
- **Animations**: 3 keyframe sets (fade-in, fade-up, pulse-subtle)
- **Dark/light mode**: Dark default via `@theme`, light overrides via `:root:not(.dark)` in `@layer base`

### L0.2: Font System
- Downloaded 3 variable woff2 files (Latin subset):
  - Space Grotesk (22KB) — headings
  - Inter (48KB) — body text
  - JetBrains Mono (31KB) — code
- Total font weight: ~102KB (well under budget)
- `@font-face` declarations with `font-display: swap`
- Registered in `@theme` as `--font-display`, `--font-body`, `--font-mono`
- Font preload `<link>` tags in `shell()`

### L0.3: Dark/Light Theme System
- `@custom-variant dark` for class-based toggle
- Anti-FOUC inline script in `<head>` (reads localStorage before paint)
- `ThemeToggle` component (3-way: Light/Dark/System cycle)
- localStorage persistence
- System preference detection via `matchMedia`
- `theme-transitioning` CSS class for smooth color transitions

### L0.4: Layout Shell
- `Layout` component — flex column, min-h-screen, wraps all pages
- `Header` — sticky, backdrop-blur, logo/name, nav links, theme toggle
- `Footer` — "Built with Rust + Leptos + WebAssembly", GitHub + LinkedIn links
- `NavLink` — reusable nav link with hover styling
- Semantic HTML: `<header>`, `<nav>`, `<main>`, `<footer>`
- Max-width container (5xl / 1024px)
- Responsive nav: hidden on mobile (md:flex), placeholder for mobile menu (L1.1)

### Updated app.rs
- Integrated Layout component wrapping all routes
- Styled 404 fallback page
- HomePage uses all design tokens as visual proof
- Updated document title to "Gabriel Osemberg — AI-Augmented Engineer"

### Bug Fix (cross-stream)
- Fixed `server/error.rs`: removed conflicting `From<DataError> for ServerFnError` impl (blanket impl already exists in Leptos)

---

## Quality Gates

| Gate | Result |
|------|--------|
| `cargo build --features ssr` | PASS |
| `cargo build --target wasm32 --features hydrate` | PASS |
| `cargo clippy -- -D warnings` (SSR) | PASS |
| `cargo clippy -- -D warnings` (WASM) | PASS |
| `cargo fmt --check` | PASS |

---

## Files Created/Modified

**Created**:
- `src/components/mod.rs` (2 lines)
- `src/components/layout.rs` (86 lines)
- `src/components/theme_toggle.rs` (117 lines)
- `public/fonts/inter-latin-variable.woff2` (48KB)
- `public/fonts/space-grotesk-latin-variable.woff2` (22KB)
- `public/fonts/jetbrains-mono-latin-variable.woff2` (31KB)

**Modified**:
- `style/tailwind.css` (2 → 271 lines)
- `src/app.rs` (68 → 157 lines)
- `src/lib.rs` (added `pub mod components`)
- `Cargo.toml` (added `web-sys` dependency)
- `src/server/error.rs` (removed conflicting From impl)

---

## What This Unblocks

SHOWCASE and IDENTITY can now:
- Use `bg-surface-*`, `text-text-*`, `color-accent-*` utility classes
- Wrap pages in `<Layout>` for consistent header/footer
- Use `font-display`, `font-body`, `font-mono` classes
- Use `dark:` variant for dark mode styling
- Build components within the established design system
