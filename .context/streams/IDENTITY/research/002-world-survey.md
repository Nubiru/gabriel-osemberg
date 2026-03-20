# 002 — World Survey: IDENTITY Domain

**Stream**: IDENTITY
**Date**: 2026-03-20
**Scope**: Best-in-class developer portfolios, CV presentation, experience timelines, skills viz, PDF generation, personal narrative, hiring signals

---

## Best-in-Class Examples (7)

### 1. Brittany Chiang — brittanychiang.com

The gold standard. Dark theme, spotlight cursor effect, tabbed experience section (company tabs left, details right), "Archive" page with sortable project table.

**Steal**: Tabbed experience section (compact, scannable). Cursor spotlight effect (implementable in Leptos/WASM). Archive table for all past projects.
**Avoid**: Single-page limits depth per project. No methodology section.
**Relevance**: Cursor effect is a WASM showcase opportunity. Tabbed experience is the right pattern for Gabriel's multiple roles.

### 2. Josh W. Comeau — joshwcomeau.com

Portfolio as educational platform. Interactive code playgrounds, sound design on interactions, personality-driven writing that remains technically rigorous. The site IS the portfolio piece.

**Steal**: "The site IS the proof" philosophy. Interactive demos embedded in content. Warm but rigorous tone.
**Avoid**: Heavy JS payload. Whimsy can distract serious hiring managers.
**Relevance**: Core philosophy directly matches Gabriel's positioning. A Rust+WASM portfolio with live WebGL2 previews IS the Comeau approach for systems engineering.

### 3. Lee Robinson — leerob.com

Radical minimalism. Impact-first positioning ("helped grow Next.js to over 1.3M developers"). Content pruning — only what matters survives.

**Steal**: Impact-first framing ("90K LOC" > "I know C"). "Stack" page with brief justifications.
**Avoid**: Too minimal without name recognition.
**Relevance**: Gabriel should lead with impact metrics, not tech lists. But needs more visual wow-factor than Robinson.

### 4. Cassie Evans — cassie.codes

Custom SVG illustrated desk scene as hero. Personality-first headline ("I like making fun, interactive things with code"). The illustration IS the brand.

**Steal**: Custom visual as hero element. Personality-first positioning statement.
**Avoid**: Illustrative approach requires design skill. Less serious for engineering roles.
**Relevance**: Hero could be a stylized visualization of the human-AI collaboration framework, or a live WebGL2 preview of `time`.

### 5. Tamal Sen (Creative Developer)

Bold opening statement. 3D project previews. Multimedia case studies with video. Subtle scroll-triggered reveals.

**Steal**: Bold opening statement. Scroll-reveal animations. Multimedia case studies.
**Avoid**: 3D elements are heavy on mobile.
**Relevance**: 3D preview concept — Gabriel's `time` project as embedded live WASM preview = massive differentiator.

### 6. Sean Halpin (Minimalist Personal Brand)

"Calm, personable" premium feel through restraint. Generous whitespace, typography as primary design element, warm conversational tone.

**Steal**: Whitespace as design tool. Typography-driven design.
**Relevance**: Aligns with accessibility-first approach. A portfolio that treats every visitor with respect IS a statement about engineering values.

### 7. Anthology Pattern (Creative Developer — dev.to)

Portfolio as narrative anthology — each project is a "chapter." Scroll-driven storytelling. The development journey itself becomes content.

**Steal**: Narrative structure. Scrollytelling. Career arc as content.
**Relevance**: Most aligned pattern. Gabriel's story (JS/TS → C/WebGL2 → Rust/WASM) IS the differentiator. The portfolio should TELL this story.

---

## Experience Timeline Patterns

| Pattern | Technique | Accessibility | Recommended |
|---------|-----------|---------------|-------------|
| Vertical scroll-triggered | Intersection Observer, alternating sides | Good (server-renderable) | **YES — primary** |
| Tabbed company switcher | Click company → show details | Good (keyboard-friendly) | YES — alternative |
| S-curve animated path | SVG path illumination on scroll | Poor (decoration only) | No — too complex |
| GSAP pinned sections | Career phases pin while content animates | Medium | Maybe L3 |

**Recommendation**: Vertical timeline with scroll-triggered reveals. Each node: role, company, date range, 1-2 impact metrics, tech tags. Click to expand into case study. Server-render HTML for SEO/a11y; enhance with WASM animations client-side.

---

## Skills Visualization Patterns

| Approach | Effectiveness | Honesty | Recommended |
|----------|--------------|---------|-------------|
| Categorized grid + proficiency context | HIGH — most scannable for recruiters | HIGH — "daily use 5 years" > "75%" | **YES — primary** |
| Radar/spider chart | MEDIUM — shows relative strengths | MEDIUM | YES — one overview chart |
| Tech stack per project | HIGH — ties skills to evidence | HIGH | YES — contextual |
| Percentage progress bars | LOW — meaningless numbers | LOW | **NO — never** |
| Self-assessed expert/intermediate | LOW — no calibration | LOW | NO |

**Recommendation**: Categorized grid (Languages, Frameworks, Tools, Concepts) with one radar chart showing engineering shape (Systems / Frontend / AI Collaboration / Creative Coding). Each project page shows its tech stack with brief justifications.

---

## PDF CV Generation

### Technical Options (Rust)

| Library | Approach | Pros | Cons |
|---------|----------|------|------|
| **Typst** (Rust-native) | Template + JSON → PDF | Fast, type-safe, Rust-native, beautiful output | New ecosystem, learning curve |
| headless_chrome | HTML → PDF via Chromium | Full CSS support | Requires Chromium binary, heavy |
| html2pdf | HTML → PDF via WeasyPrint | Well-tested | Non-Rust dependency |
| printpdf | Direct PDF construction | Pure Rust | Low-level, tedious layout |

**Recommendation**: Typst. Rust-native, embeddable as library (`typst` crate, `papermake` crate). Template + data pipeline.

### Dual PDF Strategy

Generate TWO versions from same SQLite data:
1. **Visual PDF** — branded, uses site's design system. For human readers, direct sharing.
2. **ATS PDF** — single column, standard fonts (Arial/Calibri), no graphics. For machine parsing.

ATS requirements (97.8% of Fortune 500 use ATS):
- Selectable/copyable text (not image-based)
- Standard section headings ("Experience," "Education," "Skills")
- No tables, text boxes, icons, or images
- Tagged PDF from structured markup
- 10-12pt body, 12-14pt headers

---

## What AI Companies Actually Look For

### Anthropic (direct from hiring page)
- "If you have done interesting independent research, written an insightful blog post, or made substantial contributions to open-source software, put that at the TOP of your resume"
- Mission alignment screened from FIRST conversation
- First-principles thinking: novel problems, watch you reason live
- Intellectual humility: admitting "I don't know" is a strength
- ~50% technical staff had no prior ML experience

### Key Signals Across AI Companies
- Impact metrics on everything (Google formula: "Accomplished [X] as measured by [Y], by doing [Z]")
- Production systems > toy projects
- Reasoning process > outcomes
- "Proof of work" portfolios replacing traditional resumes (87% of hiring managers prefer candidates with portfolios)
- "Tired of seeing 'I built a chatbot' portfolios" — want systems that demonstrate thinking beyond happy path

### What Gabriel Should Emphasize
1. **The collaboration framework itself** — multi-session AI orchestration = "interesting independent research"
2. **Deliberate Rust choice** — learning hard things on purpose = engineering judgment
3. **Metrics everywhere** — 90K LOC, 95.9% coverage, WASM bundle sizes, Lighthouse scores
4. **The site itself** — built in Rust+WASM, a11y-first, database-driven = the proof

---

## 2025-2026 Trends

1. **"Proof of work" > credentials** — the portfolio IS the resume
2. **The site IS the portfolio piece** — implementation demonstrates competence
3. **Scroll-driven storytelling** — mainstream via Intersection Observer / scroll-linked animations
4. **Dark themes + accent color** — still dominant
5. **Mobile-first mandatory** — 60%+ of portfolio views are mobile
6. **Impact metrics > tech lists** — "reduced load time 40%" > "I know React"
7. **Speed IS the portfolio** — slow site = developer who doesn't optimize
8. **AI integration as differentiator** — portfolio proves AI skills, influence, readiness

---

## Common Mistakes to Avoid

1. Including every project ever (curate: 3-5 exceptional > 15 mediocre)
2. Excessive animations before content loads
3. No mobile optimization (60% of views)
4. Outdated projects signaling inactivity
5. Generic positioning ("Full-Stack Developer")
6. No call-to-action
7. Code without context (problem → decision → outcome)
8. Arbitrary skill percentage bars
9. Fighting ATS (beautiful PDF for humans, plain for machines)
10. Spending so long deciding that you never ship

---

## Synthesis: IDENTITY Domain Architecture

**Core Positioning**: "I engineer the collaboration between human creativity and AI capability."

### Page Structure
1. **Hero**: Bold positioning statement + subtle WASM animation (mini WebGL2 preview of `time`?)
2. **About narrative**: 3-4 paragraphs. Career arc (JS/TS → C/WebGL2 → Rust/WASM) as deliberate choices
3. **Experience timeline**: Vertical scroll-triggered, impact metrics per node, expandable case studies
4. **Skills**: Categorized grid + one radar chart + contextual tech per project
5. **"How I Work"**: Three pillars — AI-Augmented Engineering, Learning as Signal, Scientific Method
6. **PDF CV**: Dual-output (visual + ATS) from SQLite via Typst
7. **Footer proof**: Performance metrics (WASM size, Lighthouse, build time) as engineering flex

### Key Differentiators
- Portfolio built in Rust+WASM = proof of Rust learning claim
- Human-AI collaboration framework = what Anthropic/OpenAI care about
- 90K LOC C project = scale and persistence most portfolios can't match
- Dual PDF generation from database = showcase feature itself

---

## Sources

- brittanychiang.com, joshwcomeau.com, leerob.com, cassie.codes
- Colorlib "21 Best Developer Portfolios 2026"
- DEV Community "The Anthology of a Creative Developer"
- Sundeep Teki "How to Get Hired at OpenAI, Anthropic, DeepMind 2026"
- DataExec "Breaking Into AI in 2026"
- Anthropic Careers page
- CVCraft "ATS Resume Format 2026", PitchMeAI "ATS-Compatible Resume Formats"
- Typst blog "Automated PDF Generation", crates.io (papermake, typst-pdf, html2pdf)
- GSAP ScrollTrigger, uiCookies "CSS Timeline Designs 2026"
- NovoResume "Resume Trends 2026", Converge "Portfolio Resume: Proof of Work 2026"
