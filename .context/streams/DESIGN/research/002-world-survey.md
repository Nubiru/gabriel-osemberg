# 002 — World Survey: Best-in-Class Design Examples

**Stream**: DESIGN
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

Surveyed 8 portfolio websites, 3 Leptos-specific projects, and researched key design technologies (Tailwind v4 @theme, OKLCH color, View Transitions API, font pairing, dark mode systems). The findings establish clear patterns for what "world-class" looks like in a developer portfolio and what technical approaches are available in the Leptos + Tailwind v4 ecosystem.

---

## Portfolio Examples Analyzed

### 1. Brittany Chiang (brittanychiang.com) — Gold Standard

**What**: Single-page developer portfolio. Currently at Klaviyo, previously Apple.
**Tech**: Next.js + Tailwind CSS + Vercel
**Design approach**:
- Dark blue-toned color scheme, clean and professional
- Inter typeface throughout (readable, neutral, modern)
- Anchor-based navigation (#about, #experience, #projects)
- Persistent social links (GitHub, LinkedIn, CodePen)
- Projects shown with visual previews + tech stack tags + quantified impact ("100K+ Installs")
- No dark/light toggle — committed to one dark palette
- Minimal animations, lets content speak

**Why it works**: Extremely focused. No visual noise. Projects have measurable outcomes. The site itself demonstrates competence without screaming it. "View Full Résumé" and "View Full Project Archive" for depth.

**Lesson for gabriel-osemberg**: Brittany's approach proves that restraint beats spectacle. But Gabriel needs MORE visual impact because his projects (90K LOC C/WASM, blockchain analytics) are more technically impressive and deserve richer presentation. The balance is: Brittany's clarity + more visual data.

---

### 2. Bruno Simon (bruno-simon.com) — Maximum Spectacle

**What**: 3D interactive portfolio. Drive a virtual jeep to explore projects.
**Tech**: Three.js + WebGL
**Design approach**: Full 3D world, game-like navigation, physics-based interactions

**Why it works**: Instantly memorable. Proves creative + technical capability simultaneously.

**Lesson for gabriel-osemberg**: This level of 3D is excessive for a "hire me" portfolio, but the principle — portfolio-as-proof — is exactly right. Gabriel already built 90K LOC of WebGL2. The CV website should hint at that capability without making navigation a game.

---

### 3. Jesse Zhou — Conceptual Innovation

**What**: 3D ramen hut with vending machine of projects and TV for video demos.
**Tech**: Three.js + custom 3D assets
**Design approach**: Themed 3D environment where exploration IS the portfolio experience

**Lesson for gabriel-osemberg**: Creative theming works when it serves the narrative. Gabriel's narrative is "AI-augmented engineer who builds real systems." The theme should communicate precision, depth, and sophistication — not whimsy.

---

### 4. Devon Stank — Dark Mode Excellence

**What**: Developer portfolio with hero video background.
**Design approach**: Dark-first design, video backgrounds, pleasant reading experience

**Lesson for gabriel-osemberg**: Dark mode is the right default for a technical portfolio. Video backgrounds work when subtle. The reading experience must be paramount.

---

### 5. Leptos Portfolio Admin (DevsHero/leptos_portfolio_admin) — Leptos Reference

**What**: Full-stack SSR portfolio with dynamic admin panel, built in Leptos.
**Tech**: Leptos 0.6 + Actix Web + SurrealDB + Tailwind CSS + Redis
**Features**:
- Dark mode toggle with intro animation
- WYSIWYG editor for content management
- PDF generation via Chromium
- LLM chat interface for visitor interaction
- Icon system using `icondata` crate
- Responsive design with Tailwind

**Lesson for gabriel-osemberg**: Proves the full feature set is achievable in Leptos. Their dark mode + animation approach validates our direction. Their admin panel is overkill for our needs (we use database seeding), but their PDF generation pattern via Chromium is worth noting. Uses Leptos 0.6 — we're on 0.8, which is more mature.

---

### 6. portfolio-rs (Kanerix/portfolio-rs) — Minimal Leptos Portfolio

**What**: Learning project portfolio in Leptos, deployed to Fly.io.
**Tech**: Leptos + Tailwind CSS + Docker + GitHub Actions + Fly.io
**Features**: Clean, minimal portfolio with light mode default

**Lesson for gabriel-osemberg**: Proves Fly.io deployment works. Confirms our toolchain choice. But the design is basic — we need to go significantly further in visual quality.

---

### 7. Vasantha Kumar — Medium Article on Leptos Portfolio

**What**: Blog post documenting building a portfolio with Rust + Leptos.
**Key insight**: Components broken into About, Experience, Skills modules — modular Leptos architecture works for portfolio domains.

**Lesson for gabriel-osemberg**: Modular component architecture is the right approach. Each section = Leptos component. Data-driven rendering.

---

### 8. Muz.li Top 100 Creative Portfolios 2025

**Trends across 100 portfolios**:
- Motion and storytelling are dominant (not static pages)
- Interactivity as engagement tool
- Bold aesthetics balanced with readability
- Bento grid layouts gaining traction
- Scroll-driven narratives replacing traditional sections

**Lesson for gabriel-osemberg**: Movement matters. Static = forgettable. But movement must serve content, not replace it.

---

## Key Design Technologies

### Tailwind CSS v4 @theme Directive

The foundation for our design token system. Key capabilities:

```css
@import "tailwindcss";

@theme {
  /* Colors auto-generate utility classes: bg-*, text-*, border-* */
  --color-surface-primary: oklch(0.15 0.01 260);
  --color-accent: oklch(0.65 0.18 260);

  /* Typography */
  --font-sans: "Inter", system-ui, sans-serif;
  --font-mono: "JetBrains Mono", monospace;

  /* Spacing base unit */
  --spacing: 0.25rem;

  /* Shadows, radii, breakpoints — all auto-generate utilities */
}
```

**Critical finding**: `@theme` generates both CSS variables AND utility classes. This means our design tokens are usable via Tailwind classes (`bg-surface-primary`) AND in custom CSS (`var(--color-surface-primary)`). No JS config file needed in v4.

**Dark mode**: Use Tailwind's `dark:` variant with `prefers-color-scheme` or class-based toggle. CSS `light-dark()` function is another option but less integrated with Tailwind.

---

### OKLCH Color Space

**Why OKLCH over HSL/RGB**:
- Perceptually uniform — equal lightness values LOOK equally bright
- P3 gamut support — 30% more colors than sRGB
- Dark mode via lightness inversion — swap L values, keep C and H
- Predictable contrast — same L difference = same perceived contrast
- Browser support: 93.1%+ globally (2026)

**Palette strategy**:
- Define hue angles for brand colors
- Create scales by varying L (lightness) at fixed C (chroma) and H (hue)
- Dark mode: invert L scale (light bg L=0.97 → dark bg L=0.12)
- Accent colors: high chroma at medium lightness (L=0.60-0.70, C=0.15-0.25)
- Surface colors: near-zero chroma for neutrals (C=0.01-0.02)

**Tools**: oklch.com, oklch.fyi, Harmonizer (Figma + standalone), apcach (JS contrast lib)

---

### View Transitions API

**Status (March 2026)**: Same-document transitions are Baseline across all major browsers (Chrome 111+, Edge 111+, Safari 18+, Firefox 144+). Cross-document (MPA) transitions are Chrome/Edge only.

**For Leptos SPA**: Use `document.startViewTransition()` from Rust via `web-sys` or `wasm-bindgen`. Wrap Leptos router navigation in view transitions for smooth page changes.

**CSS control**:
```css
::view-transition-old(root) {
  animation: fade-out 0.3s ease;
}
::view-transition-new(root) {
  animation: fade-in 0.3s ease;
}
```

**Decision**: Use View Transitions API for page-level transitions. Use CSS animations/transitions for component-level animations. This is the modern approach — no JS animation library needed for page transitions.

---

### Font Pairing

**Best candidates for technical portfolio**:

| Pairing | Headings | Body | Code | Vibe |
|---------|----------|------|------|------|
| **Option A** | Space Grotesk | Inter | JetBrains Mono | Technical + clean |
| **Option B** | Montserrat | Inter | Fira Code | Geometric + readable |
| **Option C** | Plus Jakarta Sans | Inter | JetBrains Mono | Friendly + professional |
| **Option D** | Space Grotesk | Space Grotesk | Space Mono | Cohesive monospace-derived family |

**Recommendation**: Option A (Space Grotesk + Inter + JetBrains Mono). Space Grotesk has monospace DNA (derived from Space Mono) giving technical credibility while being proportional and readable. Inter is the workhorse body font (used by Brittany Chiang, Vercel, GitHub). JetBrains Mono for code blocks — ligatures, designed for developers.

---

### Animation Approach for Leptos

**Available tools**:
1. **CSS transitions/animations** — works everywhere, no crate needed
2. **leptos_animate** — FLIP animations, in/out transitions for Leptos components
3. **scroll-rs** — headless scroll-to-target for WASM frameworks
4. **View Transitions API** — page-level transitions via web-sys
5. **Intersection Observer** (via web-sys) — scroll-triggered reveals

**Recommended approach**:
- **CSS-first**: Use Tailwind's transition/animation utilities for micro-interactions
- **View Transitions API**: For page navigation animations
- **Intersection Observer via web-sys**: For scroll-triggered reveals
- **leptos_animate**: For list/card reordering animations (if needed)
- **Avoid**: Heavy JS animation libraries. Keep WASM bundle small.

---

## Design Principles Distilled

From all 8 portfolios and technology research:

1. **Dark-first, with light option**: Technical portfolios look better dark. Honor `prefers-color-scheme` as default, allow manual toggle.

2. **Content density > visual spectacle**: Show real metrics, real code, real impact. The data IS the visual.

3. **Motion as communication**: Animations should reveal information (scroll reveals) or confirm interaction (hover feedback), never just decorate.

4. **Typography hierarchy is king**: Clear heading/body/code distinction. 3 weights max per font.

5. **Constraint breeds distinction**: A limited, intentional color palette (2-3 hues + neutrals) is more memorable than a rainbow.

6. **Performance as design**: < 200KB WASM, < 3s FCP. A slow portfolio is a failed portfolio.

7. **Accessibility as baseline**: WCAG 2.1 AA minimum. Semantic HTML. Keyboard navigation. Contrast ratios via OKLCH.

8. **The portfolio IS the proof**: Every technical choice in the site should be defensible in an interview. "I used OKLCH because perceptual uniformity makes dark/light mode trivial" > "I used whatever Tailwind defaulted to."
