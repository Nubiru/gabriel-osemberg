# 007 — Resources: Reference Materials & Knowledge Sources

**Stream**: DESIGN
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Primary References

### Tailwind CSS v4

| Resource | URL | Use For |
|----------|-----|---------|
| Theme variables docs | https://tailwindcss.com/docs/theme | @theme directive syntax, namespaces, examples |
| Dark mode docs | https://tailwindcss.com/docs/dark-mode | @custom-variant, class-based toggle |
| Adding custom styles | https://tailwindcss.com/docs/adding-custom-styles | @layer, custom utilities |
| Tailwind v4 blog post | https://tailwindcss.com/blog/tailwindcss-v4 | Migration notes, new features overview |

### OKLCH Color

| Resource | URL | Use For |
|----------|-----|---------|
| OKLCH Color Picker | https://oklch.com | Interactive palette exploration |
| OKLCH Palette Generator | https://oklch.fyi | Brand-based palette generation with dark mode |
| Evil Martians OKLCH Guide | https://evilmartians.com/chronicles/exploring-the-oklch-ecosystem-and-its-tools | Ecosystem tools, best practices |
| OKLCH for Designers (UX Collective) | https://uxdesign.cc/oklch-explained-for-designers-dc6af4433611 | Conceptual understanding |
| LogRocket OKLCH Guide | https://blog.logrocket.com/oklch-css-consistent-accessible-color-palettes | CSS implementation patterns |

### Leptos

| Resource | URL | Use For |
|----------|-----|---------|
| Leptos Book: web-sys integration | https://book.leptos.dev/web_sys.html | NodeRef, web-sys bindings for Intersection Observer, View Transitions |
| Leptos API docs | https://docs.rs/leptos/latest/leptos/ | Component API, signals, view! macro |
| leptos_animate crate | https://crates.io/crates/leptos_animate | FLIP animations, in/out transitions |
| scroll-rs crate | https://crates.io/crates/scroll-rs | Scroll-to-target component |

### View Transitions API

| Resource | URL | Use For |
|----------|-----|---------|
| MDN View Transitions | https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API | API reference, usage guide |
| Chrome View Transitions 2025 | https://developer.chrome.com/blog/view-transitions-in-2025 | Latest features, cross-document support |
| Can I Use: View Transitions | https://caniuse.com/view-transitions | Browser support table |

### Typography

| Resource | URL | Use For |
|----------|-----|---------|
| Space Grotesk (Google Fonts) | https://fonts.google.com/specimen/Space+Grotesk | Font files, weight options |
| Inter (Google Fonts) | https://fonts.google.com/specimen/Inter | Font files, variable font |
| JetBrains Mono | https://www.jetbrains.com/lp/mono/ | Font files, ligature info |
| Typewolf: Space Grotesk pairings | https://www.typewolf.com/space-grotesk | Pairing inspiration |

### Portfolio Inspiration

| Resource | URL | Use For |
|----------|-----|---------|
| Brittany Chiang | https://brittanychiang.com | Layout, content structure, restraint |
| Awwwards Portfolios | https://www.awwwards.com/websites/portfolio/ | Design trends, award-winning examples |
| Muz.li Top 100 2025 | https://muz.li/blog/top-100-most-creative-and-unique-portfolio-websites-of-2025/ | Trend overview |
| Leptos Portfolio Admin (GitHub) | https://github.com/DevsHero/leptos_portfolio_admin | Leptos-specific patterns |
| portfolio-rs (GitHub) | https://github.com/Kanerix/portfolio-rs | Minimal Leptos portfolio + Fly.io deploy |

### Accessibility

| Resource | URL | Use For |
|----------|-----|---------|
| WCAG 2.1 Guidelines | https://www.w3.org/WAI/WCAG21/quickref/ | Compliance checklist |
| apcach (APCA contrast) | npm: apcach | Contrast validation for OKLCH |
| Lighthouse CI | https://github.com/GoogleChrome/lighthouse-ci | Automated accessibility scoring |

---

## Font Files to Download (for self-hosting)

All fonts are available as variable fonts in woff2 format. Download from Google Fonts API or directly:

| Font | Weights Needed | Approx Size (woff2) |
|------|---------------|---------------------|
| Space Grotesk Variable | 400-700 | ~35KB |
| Inter Variable | 300-700 | ~95KB |
| JetBrains Mono Variable | 400-700 | ~55KB |
| **Total** | | **~185KB** |

With subsetting (Latin only), total can be reduced to ~120KB.

---

## Crates to Evaluate

| Crate | Purpose | Priority |
|-------|---------|----------|
| `web-sys` (features: IntersectionObserver, Document) | Scroll reveals, View Transitions, theme toggle | High — L0/L2 |
| `leptos_animate` | FLIP animations for lists | Low — only if needed |
| `scroll-rs` | Scroll-to-target | Low — can use native scroll-behavior |
| `icondata` | Typed icon references | Low — start with inline SVGs |

---

## Knowledge Gaps

| Gap | Impact | How to Fill |
|-----|--------|-------------|
| Leptos 0.8 component patterns | High — affects all component architecture | Read Leptos book + examine leptos-rs/leptos examples |
| web-sys IntersectionObserver in Leptos | Medium — needed for scroll reveals | Prototype in Build phase, translate JS patterns to Rust |
| View Transitions API via web-sys | Medium — needed for page transitions | Check if web-sys has binding, else use wasm-bindgen raw JS |
| Tailwind v4 + cargo-leptos interaction | Medium — build tool integration | Test during L0.1 implementation |
| OKLCH browser rendering | Low — 93%+ support | Test in target browsers during L3.2 |

---

## Research Complete — Summary of Actionable Decisions

1. **Color**: OKLCH, dark-first, blue-violet base + cyan-teal accent
2. **Typography**: Space Grotesk + Inter + JetBrains Mono, self-hosted woff2 variable
3. **Theme**: Class-based dark mode via `@custom-variant dark`, localStorage persistence, anti-FOUC script
4. **Tokens**: All in `style/tailwind.css` via `@theme`, semantic naming
5. **Animation**: CSS-first, Intersection Observer for scroll reveals, View Transitions for pages
6. **Icons**: Inline SVGs initially, `icondata` crate if count grows
7. **Layout**: Semantic HTML, flex/grid, Header + Footer + main with responsive navigation
8. **Deploy relevance**: Fly.io confirmed by portfolio-rs example
