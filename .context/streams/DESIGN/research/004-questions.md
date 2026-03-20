# 004 — Questions: Open Questions Researched and Answered

**Stream**: DESIGN
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Questions from tracks.md

### Q1: "What color palette best represents an AI-augmented engineer? Dark-first or light-first?"

**Answer: Dark-first, with light mode available.**

**Research basis**:
- 6/8 surveyed portfolios use dark-first designs (Brittany Chiang, Devon Stank, Bruno Simon, leptos_portfolio_admin)
- Technical portfolios overwhelmingly favor dark backgrounds — they evoke IDEs, terminals, and technical depth
- Dark backgrounds make colored accent elements (charts, code highlights, data visualizations) pop more
- Light mode is still necessary for accessibility, outdoor reading, and user preference

**Recommended palette direction**:
- **Base hue**: Blue-violet (H ≈ 260-270) — conveys technology, precision, intelligence
- **Accent hue**: Cyan-teal (H ≈ 180-195) — conveys innovation, freshness, WASM/systems programming
- **Warning/highlight**: Amber (H ≈ 80) — for attention, CTAs
- **Surfaces**: Near-neutral with very low chroma (C ≈ 0.01-0.02) and blue-violet hue — not pure gray, but "cool gray"

**OKLCH implementation**:
```css
/* Dark mode surfaces (default) */
--color-surface-base:    oklch(0.12 0.015 265);  /* near-black with blue tint */
--color-surface-raised:  oklch(0.16 0.015 265);  /* cards, panels */
--color-surface-overlay: oklch(0.20 0.015 265);  /* modals, dropdowns */

/* Light mode surfaces (via .light or prefers-color-scheme: light) */
--color-surface-base:    oklch(0.97 0.005 265);  /* near-white with blue tint */
--color-surface-raised:  oklch(0.94 0.008 265);
--color-surface-overlay: oklch(0.99 0.003 265);

/* Accent — same in both modes, adjusted for contrast */
--color-accent:          oklch(0.65 0.18 195);   /* teal-cyan */
--color-accent-hover:    oklch(0.72 0.18 195);
```

**Decision**: Dark-first with `prefers-color-scheme` as default, class-based toggle override. This is a DESIGN recommendation — escalate to MEGA as ADR if Gabriel disagrees.

---

### Q2: "Which animation approach works best with Leptos/WASM? CSS-only vs Rust-driven?"

**Answer: CSS-first, with surgical web-sys calls for advanced patterns.**

**Research basis**:
- **CSS transitions/animations**: Zero WASM overhead, hardware-accelerated (transform, opacity), work with Tailwind utilities. Cover 80% of animation needs (hover, focus, reveal, page transitions).
- **View Transitions API**: Browser-native page transitions. Requires `document.startViewTransition()` via web-sys, but the actual animation is CSS-controlled.
- **Intersection Observer via web-sys**: Required for scroll-triggered reveals. The observer callback fires in JS, but the animation itself is CSS class toggling.
- **leptos_animate crate**: FLIP animations for list reordering. Useful but niche — only needed for dynamic list transitions.
- **Pure Rust animation (requestAnimationFrame)**: Possible via web-sys but overkill. Every frame crosses the WASM-JS boundary. Only justified for canvas/WebGL, which Gabriel's `time` project already covers.

**Recommended approach**:

| Animation Type | Implementation | Why |
|---------------|---------------|-----|
| Hover/focus/click | CSS transitions via Tailwind | Zero overhead, declarative |
| Element appear/reveal | CSS animations triggered by Intersection Observer (web-sys) | Hardware-accelerated |
| Page transitions | View Transitions API via web-sys | Browser-native, smooth |
| Theme toggle | CSS transitions on `color`, `background-color` | Smooth, no flash |
| List reordering | leptos_animate (FLIP) | Only if project cards are reorderable |
| Loading skeletons | CSS @keyframes animation | Pure CSS shimmer |

**WASM bundle impact**: CSS-first approach adds zero bytes to WASM. web-sys Intersection Observer adds ~2-5KB. View Transitions API adds ~1KB. Total animation footprint: < 10KB of WASM, rest is CSS.

---

### Q3: "How to implement View Transitions API with Leptos router?"

**Answer: Wrap router navigation callbacks with `document.startViewTransition()`.**

**Research basis**: View Transitions API is now Baseline across all major browsers (Chrome 111+, Edge 111+, Safari 18+, Firefox 144+) for same-document transitions. Leptos is a SPA, so same-document transitions apply.

**Implementation pattern** (conceptual — exact Leptos 0.8 API may vary):

```rust
use leptos::prelude::*;
use web_sys::js_sys;

// In the App component or a navigation wrapper
fn navigate_with_transition(path: &str) {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");

    // Check if View Transitions are supported
    if js_sys::Reflect::has(&document, &"startViewTransition".into()).unwrap_or(false) {
        // Use View Transitions API
        // The callback performs the actual DOM update (Leptos route change)
        let _ = document.start_view_transition_with_update_callback(
            // closure that triggers the route change
        );
    } else {
        // Fallback: navigate without transition
    }
}
```

**CSS for transition customization**:
```css
::view-transition-old(root) {
  animation: 300ms ease-out fade-out;
}
::view-transition-new(root) {
  animation: 300ms ease-in fade-in;
}

@keyframes fade-out {
  from { opacity: 1; }
  to { opacity: 0; }
}
@keyframes fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}
```

**Note**: The exact web-sys binding for `startViewTransition` may need to be checked against the web-sys version in Cargo.toml. If the binding doesn't exist yet, use `js_sys::Function` + `wasm_bindgen` to call it manually.

**Decision**: Implement as P2 enhancement. The CSS side is trivial. The Rust/web-sys integration needs prototyping during Build phase. If the binding is missing, fall back to CSS-only fade transitions.

---

### Q4: "What font pairing communicates technical sophistication + approachability?"

**Answer: Space Grotesk (headings) + Inter (body) + JetBrains Mono (code).**

**Research basis**:

| Font | Role | Why |
|------|------|-----|
| **Space Grotesk** | Headings, display text | Derived from Space Mono (monospace DNA = technical credibility). Proportional so it reads cleanly at heading sizes. Distinctive geometric forms signal sophistication. Variable font available. |
| **Inter** | Body text, UI labels | The definitive screen-optimized sans-serif. Used by Vercel, GitHub, Brittany Chiang. Excellent legibility at all sizes. Variable font = single file, all weights. Neutral enough to not distract from content. |
| **JetBrains Mono** | Code blocks, technical details, metrics | Designed for developers. Ligatures for code readability. Monospace. The "developer credibility" typeface. Variable font available. |

**Performance strategy**:
- All three are variable fonts → 3 files cover all weights
- Use `font-display: swap` for fast initial render
- Preload the primary fonts (Inter regular, Space Grotesk bold) in `<head>`
- JetBrains Mono can lazy-load (only used in code blocks)
- Total font weight: ~150-200KB (variable formats, woff2)

**Alternative considered**: Montserrat + Inter + Fira Code. Rejected — Montserrat is ubiquitous (low distinctiveness). Fira Code is great but JetBrains Mono is more modern and has better ligature coverage.

**Alternative considered**: Space Grotesk for everything. Rejected — Space Grotesk at body sizes (14-16px) feels slightly too distinctive for long reading. Inter is optimized for body text legibility.

**Loading approach in Leptos**:
```rust
// In shell() function, add to <head>:
view! {
    <link rel="preconnect" href="https://fonts.googleapis.com"/>
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
    <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500;700&display=swap"
    />
}
```

**Self-hosting alternative**: Download woff2 files to `public/fonts/`, reference in `style/tailwind.css` via `@font-face`. Better for performance (no third-party DNS), worse for cache hit rate. **Recommendation: self-host** — we control the performance and avoid GDPR issues with Google Fonts.

---

### Q5: "How to build a design token system in Tailwind v4 CSS-first config?"

**Answer: Use `@theme` directive in `style/tailwind.css` with semantic naming.**

**Research basis**: Tailwind v4 documentation + Evil Martians OKLCH guide + Mavik Labs design tokens article.

**Architecture**:

```
style/tailwind.css          ← Entry point, imports + @theme
  @import "tailwindcss"     ← Tailwind base
  @source "../src/**/*.rs"  ← Class scanning
  @theme { ... }            ← Design tokens (generates utilities + CSS vars)
  @custom-variant dark ...  ← Dark mode strategy
  @layer base { ... }       ← Dark mode overrides, font-face, resets
  @layer components { ... } ← Reusable component classes (if any)
```

**Token naming convention** (3 layers):

1. **Primitive tokens** — raw OKLCH values (not exposed as utilities)
   ```css
   :root {
     --raw-blue-50: oklch(0.97 0.01 260);
     --raw-blue-500: oklch(0.55 0.15 260);
     --raw-cyan-500: oklch(0.65 0.18 195);
   }
   ```

2. **Semantic tokens** — purpose-driven (exposed via @theme)
   ```css
   @theme {
     --color-surface-base: var(--raw-blue-50);
     --color-surface-raised: ...;
     --color-text-primary: ...;
     --color-text-secondary: ...;
     --color-accent: var(--raw-cyan-500);
     --color-accent-hover: ...;
     --color-border-default: ...;
     --color-border-focus: ...;
   }
   ```

3. **Component tokens** (optional, via @layer components) — only if patterns repeat
   ```css
   @layer components {
     .card { background: var(--color-surface-raised); }
   }
   ```

**Dark mode override**:
```css
@custom-variant dark (&:where(.dark, .dark *));

@layer base {
  .dark {
    --color-surface-base: oklch(0.12 0.015 265);
    --color-text-primary: oklch(0.93 0.01 265);
    /* ... override all semantic tokens */
  }
}
```

**Note on `@theme inline`**: When referencing other CSS variables inside `@theme`, use `@theme inline { ... }` so Tailwind resolves the reference correctly.

**Decision**: This is the definitive approach. Single file (`style/tailwind.css`), no JS config, full Tailwind v4 integration. Implement during L0 Build phase.

---

## Additional Questions (emerged during research)

### Q6: How to prevent Flash of Unstyled Content (FOUC) with dark mode?

**Answer**: Inject a blocking `<script>` in `<head>` (before body render) that reads `localStorage.theme` and applies the `.dark` class immediately. In Leptos, this goes in the `shell()` function:

```rust
view! {
    <script>
        "document.documentElement.classList.toggle('dark',
            localStorage.theme === 'dark' ||
            (!('theme' in localStorage) &&
             window.matchMedia('(prefers-color-scheme: dark)').matches)
        );"
    </script>
}
```

This is a render-blocking script by design — it must execute before any CSS paints.

### Q7: Should DESIGN use component-level CSS classes or utility-only?

**Answer**: **Utility-first with rare component classes.** Tailwind's utility-first approach in Rust/Leptos view! macros works well because:
- Component reuse is handled by Leptos components (Rust functions), not CSS classes
- `class="bg-surface-base text-text-primary"` is readable and design-token-aware
- `@layer components` only for truly repeated patterns (e.g., `.prose` for markdown content)

This avoids the CSS abstraction layer that fights Tailwind's philosophy.

### Q8: What icon system should DESIGN use?

**Answer**: Research found two options:
1. **icondata crate** — used by leptos_portfolio_admin. Provides typed icon references.
2. **SVG sprites** — inline SVGs in `view!` macro. Zero dependencies.

**Recommendation**: Start with inline SVGs for the few icons needed (theme toggle, nav hamburger, external links, GitHub). Add `icondata` crate only if icon count exceeds ~15. This keeps WASM bundle minimal.
