# 004 — Questions: Open Questions Researched & Answered

**Stream**: SHOWCASE
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

All 5 open questions from `tracks.md` researched and answered with evidence-backed recommendations.

---

## Q1: How to render charts/graphs in Leptos?

**Question**: "How to render charts/graphs in Leptos? SVG components in Rust, or a JS charting library via WASM interop?"

### Answer: **Hand-rolled inline SVG via Leptos `view!` macro** (L0/L1), no JS.

**Evidence**:
- Leptos has a dedicated `leptos::svg` module with typed SVG elements
- The `view!` macro renders SVG elements (path, rect, circle, polygon) identically to HTML elements
- Every SVG element can be produced by Rust code compiled to WASM and rendered as pure inline SVG
- The `#[component]` macro works for creating reusable SVG chart components

**Approach**:
```
L0: Static metric badges (numbers in styled divs — no SVG needed)
L1: Hand-rolled SVG components:
    - Progress ring (circle + stroke-dasharray) for test coverage %
    - Horizontal bar chart (rect elements) for LOC comparison
    - Tag cloud (styled divs) for tech stacks
L2+: Consider Plotters crate if complex chart types (radar, spider) are needed
```

**Why NOT JS charting libraries**: Defeats the "built entirely in Rust" philosophy. Gabriel's portfolio message is "I learn new languages for the challenge." Using Chart.js via interop undermines that message.

**Why NOT Plotters/Charming at L0**: Adds dependency weight, learning curve, and bundle size for charts that are simple enough to hand-roll. Plotters is excellent but overkill for progress rings and bar charts.

---

## Q2: What case study format best demonstrates value?

**Question**: "What case study format best demonstrates value? Problem/Approach/AI-Role/Metrics/Outcome?"

### Answer: **Yes, with refinement.** Use 7 sections:

```
1. HERO          — Project name, tagline, key visual, tech tags
2. CONTEXT       — What is this? Who was it for? Why does it exist?
3. CHALLENGE     — The specific problem or opportunity addressed
4. APPROACH      — Process, architecture decisions, key tradeoffs
5. AI ROLE       — Human vs AI collaboration breakdown (UNIQUE)
6. METRICS       — Quantitative results: LOC, coverage, tests, performance
7. IMPACT        — What it achieved, lessons learned, links
```

**Evidence**:
- 72% of hiring managers spend more time on portfolios with detailed case studies (HubSpot 2025)
- The industry-standard structure is Problem → Solution → Impact
- Gabriel's unique addition: the **AI ROLE** section, which no surveyed portfolio includes
- 800-1,500 words per case study is the sweet spot for engagement
- Before/after metrics are "the single most powerful tool" in a case study

**Per-project emphasis**:
| Project | Lead With | AI Role Angle |
|---------|----------|---------------|
| time | 90K LOC, 95.9% coverage, 20+ calendar systems | C project predates AI collaboration — shows range |
| blocksight | Real-time blockchain data, complex architecture | Technical architecture decisions |
| anan-yarok | Real business, real users, real revenue | Practical engineering, not just code |
| chamana | Complete delivery, clean design | End-to-end delivery capability |
| gabriel-osemberg | Meta/recursive — this site IS the showcase | Full AI-augmented engineering: CLAUDE.md, MEGA, TDD |

---

## Q3: How to present the AI collaboration methodology visually?

**Question**: "How to present the AI collaboration methodology visually? Show CLAUDE.md? Show the MEGA framework diagram?"

### Answer: **Show the framework diagram + methodology narrative, NOT raw CLAUDE.md.**

**Approach**:
1. **Methodology narrative** (500-800 words): Explain the philosophy — evidence-first protocol, scientific method, TDD integration, multi-session orchestration. Written for hiring managers, not for AI instances.

2. **Framework diagram** (SVG in Rust): Visual showing the MEGA architecture:
   ```
   Gabriel (Owner)
       │
       MEGA (Vision + Coordination)
       ├── Stream: SHOWCASE
       ├── Stream: DATA
       ├── Stream: DESIGN
       ├── Stream: IDENTITY
       └── Stream: INFRA
           └── Writer → Checker → Maintainer
   ```

3. **Per-project AI contribution**: A simple breakdown showing what was human-driven vs AI-assisted:
   - Architecture decisions: Human
   - Implementation: AI-assisted (with human review)
   - Testing strategy: Human
   - Code generation: AI
   - Quality gates: Automated + AI

**What NOT to do**:
- Don't dump CLAUDE.md raw — it's an internal document, not a presentation
- Don't use percentages like "60% AI, 40% human" — too reductive
- Don't hide the AI involvement — own it as a methodology, not a shortcut

**Why this matters**: No other developer systematically presents AI collaboration as an engineering methodology. This is Gabriel's strongest differentiator for AI companies (Anthropic, Google, OpenAI).

---

## Q4: How to handle project screenshots and demo videos?

**Question**: "How to handle project screenshots and demo videos? Local assets vs CDN?"

### Answer: **Self-hosted in `public/assets/projects/` for L0-L2. CDN only if needed at L3.**

**Reasoning**:
- Portfolio is a personal site, not a high-traffic SaaS — CDN overhead is unjustified
- Self-hosting gives full control over optimization, formats, and caching
- Leptos serves static files from `public/` directory efficiently
- Single deployment target (Fly.io/Shuttle/Railway) keeps assets co-located with the binary
- Cost: $0 for self-hosted vs unpredictable CDN costs

**Asset strategy**:
```
public/
  assets/
    projects/
      time/
        thumbnail.webp          (400x300, < 50KB)
        hero.webp               (1200x800, < 150KB)
        screenshots/
          calendar-view.webp
          shader-demo.webp
      blocksight/
        thumbnail.webp
        hero.webp
        ...
```

**Format**: WebP for images (best compression/quality ratio, universal browser support in 2026). MP4 for video clips if added later.

**Optimization**:
- Thumbnails: 400x300px, < 50KB each
- Hero images: 1200x800px, < 150KB each
- Lazy loading via `loading="lazy"` attribute
- `srcset` for responsive image sizes (if needed at L2)

**Screenshot sources**: Capture from the actual running projects at `/home/gabiota/personal/projects/code/`. This is a manual step Gabriel must do — SHOWCASE can build the components, but the screenshots come from real projects.

---

## Q5: What metrics are most impressive to show?

**Question**: "What metrics are most impressive to show? Test coverage, LOC, commit frequency, quality scores?"

### Answer: **Primary: Coverage + LOC + Test Count. Secondary: File Count + Tech Diversity.**

**Tier 1 — Always show (on cards + detail pages)**:
| Metric | Why Impressive | Example |
|--------|---------------|---------|
| Test coverage % | Proves engineering discipline | 95.9% (time) |
| Lines of code | Proves project scale | 90,000+ (time) |
| Test function count | Proves thoroughness | 14,789 (time) |

**Tier 2 — Show on detail pages**:
| Metric | Why Impressive | Example |
|--------|---------------|---------|
| Source file count | Proves architecture complexity | 3,773 (time) |
| Tech stack breadth | Proves polyglot capability | C, Rust, JS, TS, GDScript |
| Real-world usage | Proves business value | "Serves a real distribution business" (anan-yarok) |

**Tier 3 — Show if available, don't force**:
| Metric | When To Show |
|--------|-------------|
| Commit frequency | Only if it shows sustained effort (not AI-generated bulk) |
| Build time | Only if impressively fast |
| Bundle size | Only for web projects where it's a differentiator |
| Lighthouse score | Only for this site (meta/recursive showcase) |

**What NOT to show**:
- Commit count (easily inflated, means nothing alone)
- GitHub stars (only relevant if significant)
- "Years of experience" per tech (use proficiency visualization instead)
- Skill bars (universally mocked — meaningless without context)

**Visualization recommendation**:
- Coverage: Progress ring (SVG circle, 95.9% fills almost completely — visually striking)
- LOC: Large styled number with unit label ("90K+ lines")
- Test count: Large styled number ("14,789 tests")
- Tech stack: Pill/tag badges grouped by category

---

## All Questions Answered — No Blocking Unknowns Remain

| # | Question | Answer | Confidence |
|---|----------|--------|-----------|
| Q1 | Chart rendering | Hand-rolled SVG in Leptos view! macro | HIGH |
| Q2 | Case study format | 7-section: Hero → Context → Challenge → Approach → AI Role → Metrics → Impact | HIGH |
| Q3 | AI methodology visual | Framework diagram (SVG) + narrative text, NOT raw CLAUDE.md | HIGH |
| Q4 | Asset hosting | Self-hosted in public/assets/projects/, WebP format, lazy loading | HIGH |
| Q5 | Which metrics | Coverage + LOC + Test Count (primary), File Count + Tech Diversity (secondary) | HIGH |
