# 001 — Inventory: What Exists in the Codebase for DESIGN

**Stream**: DESIGN
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

The project is at **scaffold stage** — a default Leptos starter with minimal Tailwind integration. There is no design system, no layout structure, no theme, no animations, and no reusable visual components. Everything in the DESIGN domain needs to be built from scratch.

---

## Codebase Scan

### Source Files (design-relevant)

| File | What It Contains | Design Relevance |
|------|-----------------|-----------------|
| `src/app.rs` | Shell HTML + `App` component + `HomePage` (counter demo) | **Primary target** — all visual output lives here currently |
| `src/lib.rs` | Module exports + WASM hydrate entry | None — plumbing only |
| `src/main.rs` | Axum server setup | None — server plumbing only |

### Style Files

| File | What It Contains | Design Relevance |
|------|-----------------|-----------------|
| `style/tailwind.css` | 2 lines: `@import "tailwindcss"` + `@source "../src/**/*.rs"` | **Foundation** — Tailwind v4 CSS-first config entry point. No custom tokens, themes, or layers defined. |

### Configuration

| File | What It Contains | Design Relevance |
|------|-----------------|-----------------|
| `Cargo.toml` | Leptos 0.8, leptos_meta, leptos_router | Meta tags + routing available for SEO and page structure |
| `package.json` | `tailwindcss ^4.2.2` (devDependency) | Tailwind v4 ready — CSS-first config (no JS config file needed) |
| `rustfmt.toml` | Formatting rules (100 char width, 4 spaces) | Code style only |

### Assets

| Path | What It Contains |
|------|-----------------|
| `public/favicon.ico` | Default favicon |

No fonts, images, videos, SVGs, or other visual assets exist.

### Existing Design Patterns

**Inline Tailwind classes in `app.rs`:**
- `text-4xl font-bold text-center mt-8` — heading
- `flex justify-center mt-4` — button container
- `px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700` — button

These are default Tailwind utility classes with no design system coherence — just placeholder styling from the scaffold.

---

## Module Inventory by Domain

### Layout & Navigation
- **Header component**: Does not exist
- **Footer component**: Does not exist
- **Layout shell**: Does not exist — content renders directly in `<main>`
- **Navigation**: Does not exist — single page, no nav
- **Mobile menu/drawer**: Does not exist

### Design Tokens
- **Color palette**: Does not exist — using default Tailwind `blue-600/700`
- **Typography scale**: Does not exist — using default Tailwind sizes
- **Spacing system**: Does not exist — using arbitrary Tailwind values
- **Shadow system**: Does not exist
- **Border radius system**: Does not exist — using default `rounded`
- **Animation tokens**: Does not exist

### Theme System
- **Dark/light mode**: Does not exist
- **Theme toggle**: Does not exist
- **CSS custom properties**: Does not exist
- **Theme persistence**: Does not exist

### Animations & Interactions
- **Page transitions**: Does not exist
- **Scroll animations**: Does not exist
- **Microinteractions**: Does not exist (only a basic `hover:bg-blue-700`)
- **Loading states**: Does not exist
- **Error boundary styling**: Does not exist

### Responsive Design
- **Breakpoints**: Using Tailwind defaults (no custom breakpoints defined)
- **Mobile-first approach**: Not implemented (no responsive variants used)
- **Container queries**: Not implemented

### Visual Components
- **Hero section**: Does not exist
- **Card component**: Does not exist
- **Timeline component**: Does not exist
- **Skills visualization**: Does not exist
- **Metrics charts**: Does not exist

---

## Technology Assessment

### What's Ready

1. **Leptos 0.8** — latest stable, fine-grained reactivity, SSR + hydration
2. **Tailwind CSS v4** — CSS-first config, `@theme` directive available for design tokens
3. **leptos_meta** — document head management (title, meta tags, stylesheets)
4. **leptos_router** — client-side routing with `<Routes>` + `<Route>`
5. **cargo-leptos** — build tool with hot-reload and Tailwind integration
6. **WASM target** — `wasm32-unknown-unknown` configured

### What's Missing (needs installation/configuration)

1. **Fonts** — no web fonts loaded, no font files in `public/`
2. **Icon library** — no icon system (consider SVG sprites or Leptos icon crate)
3. **Animation library** — no animation infrastructure beyond CSS transitions
4. **Image optimization** — no tooling for responsive images or WebP conversion
5. **Component library** — no reusable component system

---

## Leptos Component Architecture (for DESIGN to build on)

Leptos components are Rust functions annotated with `#[component]`. The current code shows:

```rust
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1 class="...">"Text"</h1>
    }
}
```

**Key Leptos design patterns DESIGN will use:**
- `#[component]` for reusable UI pieces
- `view!` macro for HTML-like templates with Tailwind classes
- Reactive signals (`RwSignal`, `ReadSignal`, `WriteSignal`) for interactive state
- `#[prop(optional)]` and `#[prop(into)]` for component APIs
- `children: Children` for composition/slots
- `leptos_meta` for injecting `<link>` tags (fonts, styles)

---

## Gap Summary

| Domain | Exists | Needed | Gap |
|--------|--------|--------|-----|
| Layout shell | 0 | 4 components (Header, Footer, Layout, Nav) | 100% |
| Design tokens | 0 | Full system (colors, type, spacing, shadows) | 100% |
| Theme system | 0 | Dark/light with toggle + persistence | 100% |
| Animations | 0 | Page transitions, scroll reveals, microinteractions | 100% |
| Visual components | 0 | Hero, cards, timeline, charts | 100% |
| Assets | 1 (favicon) | Fonts, icons, images, OG images | ~95% |
| Responsive design | 0 | Mobile-first responsive system | 100% |

**Everything in the DESIGN domain is greenfield.** The foundation (Leptos + Tailwind) is solid but completely unconfigured for this project's visual identity.
