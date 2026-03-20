# 002 — World Survey: Best-in-Class Project Showcases

**Stream**: SHOWCASE
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

Analyzed 8+ portfolio sites and industry best practices to identify patterns for world-class project showcase pages. The strongest developer portfolios share: **case study format over project lists**, **metrics with context**, **visual storytelling**, and **scannable structure for busy hiring managers**.

---

## 1. Brittany Chiang (brittanychiang.com)

**Why notable**: Gold standard for software engineer portfolios. Referenced in nearly every "best developer portfolio" list.

**Project showcase pattern**:
- Single-column vertical card layout (not grid)
- Each card: title (linked), description, screenshot, tech tags as pills
- Clean, minimal aesthetic with generous whitespace
- Pixel-perfect, accessibility-focused design
- Featured projects get hero-sized cards; other projects get compact list treatment

**Takeaway for SHOWCASE**: The two-tier approach (featured + archive) is powerful — give the 3-4 strongest projects full case study treatment, list the rest compactly.

---

## 2. Lee Robinson (leerob.com)

**Why notable**: VP of Developer Experience at Cursor (previously Vercel). Known for excellent developer content.

**Project showcase pattern**:
- No traditional "projects" grid — showcases work through **thematic categories** and writing
- Professional context first (role, company)
- Content-first approach: articles, talks, and repos as proof of expertise
- Minimalist dark/light design, underlined hyperlinks, semantic HTML
- Responsive typography scaling

**Takeaway for SHOWCASE**: For senior engineers, "thought leadership as showcase" works. Gabriel has a unique angle: the AI collaboration framework itself IS content worth showcasing. The methodology section should read like Lee Robinson's featured articles — explaining WHY, not just WHAT.

---

## 3. Case Study Format (Industry Consensus)

**Structure that hiring managers expect** (from InfluenceFlow, UXfol.io, Toptal research):

```
1. Hero: Project name + tagline + key visual (screenshot/video)
2. Context: What is this? Who was it for? Why does it matter?
3. Challenge: The specific problem or opportunity
4. Approach: How you solved it (process, decisions, tradeoffs)
5. Tech Stack: Visual representation (tags, not skill bars)
6. Impact: Metrics, results, before/after (quantitative when possible)
7. AI Role: (unique to Gabriel) Human vs AI collaboration breakdown
8. Links: Live site, repo, demo video
```

**Key stats**:
- 72% of hiring managers spend more time on portfolios with detailed case studies (HubSpot 2025)
- 800-1,500 words per case study is the sweet spot
- 4-6 curated projects beats 20 listed ones
- Before/after metrics are "the single most powerful tool" in a case study

**Takeaway for SHOWCASE**: Every project page must follow this structure. The `time` project with 90K LOC and 95.9% coverage is the hero; gabriel-osemberg (this site) is the meta/recursive closer.

---

## 4. Metrics Visualization Approaches

### Option A: Pure Rust SVG (Recommended)

**lodviz-rs / Plotters / Charming**:
- Pure Rust libraries that compile to WASM
- SVG output, no JavaScript dependency
- Plotters supports SVG, bitmap, and real-time rendering with WASM target
- Charming has WASM renderer (feature-gated)
- Approach: generate SVG chart components directly in Leptos `view!` macros

**Pros**: Zero JS, aligns with "built in Rust" philosophy, single binary
**Cons**: Less ecosystem, fewer chart types, more manual work

### Option B: Hand-rolled SVG in Leptos

Generate `<svg>` elements directly in Leptos components:
- Progress rings for test coverage (SVG circle with stroke-dasharray)
- Bar charts for LOC comparison (SVG rect elements)
- Radar/spider chart for tech stack (SVG polygon with calculated points)

**Pros**: Full control, minimal dependencies, impressive to interviewers ("I built charts from scratch in Rust")
**Cons**: More code, need to handle responsive sizing manually

### Option C: JS Interop (NOT recommended)

Use Chart.js or D3.js via wasm-bindgen. Defeats the purpose of the Rust portfolio.

**Recommendation**: **Option B (hand-rolled SVG)** for L0/L1 metrics. Simple, impressive, zero dependencies. Graduate to Option A (Plotters/Charming) only if complex chart types are needed at L2+.

---

## 5. Project Card Design Patterns

**Grid vs List**: Grid (2-3 columns on desktop) for index page; full-width for detail pages.

**Card anatomy** (synthesis of best practices):
```
┌─────────────────────────────────┐
│  [Screenshot/Thumbnail]         │
│                                 │
│  PROJECT NAME                   │
│  One-line tagline               │
│                                 │
│  [Rust] [WebGL2] [WASM] [C11]  │  ← tech tags as pills
│                                 │
│  90K LOC  │  95.9%  │  14K     │  ← key metrics row
│  lines    │  tests  │  tests   │
│                                 │
│  [View Case Study →]            │  ← CTA link to detail page
└─────────────────────────────────┘
```

**Hover effects**: Subtle elevation (shadow increase), slight scale, border accent color transition. NOT flashy — professional.

**Bento grid**: Trending in 2025-2026. Modular grid layout where cards have different sizes based on project importance. `time` gets 2x size.

---

## 6. Leptos Portfolio Precedents

Two known Leptos portfolio examples found:

1. **Vasantha Kumar** (Medium article): Broke portfolio into reusable Leptos components (About, Experience, Skills). Data loaded from JSON file. Basic but functional.

2. **leptos_portfolio_admin** (GitHub): Full-stack SSR with Actix Web + SurrealDB + Tailwind CSS. Dynamic admin panel, WYSIWYG editor, real-time content updates. Over-engineered for a portfolio but demonstrates Leptos capability.

**Takeaway**: Gabriel's portfolio will be the most technically sophisticated Leptos portfolio site publicly known — database-driven content, hand-rolled SVG charts, TDD methodology, AI collaboration showcase. This is a differentiator.

---

## 7. What Makes Gabriel's Showcase Unique

No surveyed portfolio does ALL of these:

| Feature | Brittany Chiang | Lee Robinson | Standard Portfolios | Gabriel |
|---------|----------------|--------------|--------------------:|---------|
| Case study format | Minimal | Content-first | Sometimes | Full |
| Quantitative metrics | No | No | Rarely | Yes — LOC, coverage, test counts |
| Hand-built charts | No | No | No | Yes — SVG in Rust/WASM |
| AI collaboration section | No | No | No | Yes — CLAUDE.md, MEGA framework |
| Built in a language being learned | No | No | No | Yes — Rust as proof of learning |
| Database-driven content | No | Likely | Sometimes | Yes — SQLx + SQLite |
| Self-referential project | No | No | No | Yes — this site IS a portfolio item |

---

## Key Decisions for SHOWCASE

1. **Case study format**: Problem → Approach → AI Role → Metrics → Impact
2. **Charts**: Hand-rolled SVG in Leptos components (L0/L1), consider Plotters for L2+
3. **Card layout**: Bento-style grid on index, full-width case study on detail
4. **Metrics to highlight**: LOC, test coverage %, test function count, src file count, quality scores
5. **AI section**: Per-project breakdown of human vs AI contributions (unique differentiator)
6. **Featured vs archive**: Top 3 projects get full treatment, remaining get compact cards

---

## Sources

- [Brittany Chiang](https://brittanychiang.com/) — engineer portfolio gold standard
- [Lee Robinson](https://leerob.com/) — content-first developer presence
- [InfluenceFlow Case Study Guide](https://influenceflow.io/resources/guide-to-portfolio-case-studies-showcase-your-work-land-more-opportunities-in-2026/)
- [UXfol.io Case Study Template](https://blog.uxfol.io/ux-case-study-template/)
- [Plotters — Rust charting library](https://github.com/plotters-rs/plotters)
- [Charming — Rust visualization](https://github.com/yuankunzhang/charming)
- [Leptos Portfolio Admin](https://github.com/DevsHero/leptos_portfolio_admin)
- [Vasantha Kumar — Leptos portfolio blog](https://medium.com/google-cloud/building-my-portfolio-website-with-rust-and-leptos-a-journey-into-webassembly-12709ee4ab10)
- [Engineer Portfolios — 20+ Examples](https://www.sitebuilderreport.com/inspiration/engineer-portfolios)
- [Dribbble — Project Card Designs](https://dribbble.com/tags/project-cards)
