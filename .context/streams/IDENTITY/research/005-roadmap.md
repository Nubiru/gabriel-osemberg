# 005 — Roadmap: IDENTITY Domain

**Stream**: IDENTITY
**Date**: 2026-03-20
**Axis**: PRESENCE — "Does this present Gabriel as someone worth hiring?"

---

## Layered Plan

### L0 — Foundation (unblocked, can start now)

The minimum viable identity. A visitor to `/about` sees real content, not a blank page.

| Task | Component | Dependencies | Done When |
|------|-----------|-------------|-----------|
| About page shell | `src/components/about/page.rs` | Layout (delivered), cv_sections data (seeded) | `/about` renders real "about" content from database |
| Skills section | `src/components/about/skills.rs` | Skills data (27 seeded) | Categorized grid of skills rendered from database |
| Contact section | `src/components/about/contact.rs` | Gabriel's contact info | Email, GitHub, LinkedIn displayed |
| Markdown renderer | Utility for cv_sections content | `pulldown-cmark` crate or Leptos markdown | cv_section markdown renders as HTML |
| Draft experience content | Seed data proposal | Gabriel's project inventory | experience entries proposed for Gabriel's review |
| Draft education content | Seed data proposal | Gabriel's info | education cv_section proposed |
| Route wiring | `src/app.rs` | About page component | `/about` route renders AboutPage component |

**Exit criteria**: `/about` page loads with real content. Skills are categorized and displayed. Contact info visible.

### L1 — Integration (needs experience data)

Interactive, polished, all sections populated with real content.

| Task | Component | Dependencies | Done When |
|------|-----------|-------------|-----------|
| Experience timeline | `src/components/about/timeline.rs` | Experience seed data (L0 draft → Gabriel approval) | Vertical timeline renders all experiences |
| Scroll-reveal animations | Animation utility | Leptos Intersection Observer pattern | Elements animate in on scroll |
| "How I Work" section | `src/components/about/methodology.rs` | Methodology cv_section (seeded) | MEGA framework presented with visual architecture diagram |
| Skills proficiency display | Enhancement to skills.rs | None | Proficiency indicators (not percentage bars — contextual) |
| Technical Writing section | Infrastructure for writing/posts | Database model or cv_section type="writing" | Section exists, ready for content |
| Navigation integration | Header links | About page exists | `/about` link works in nav, active state shows |

**Exit criteria**: Timeline shows real experience entries. "How I Work" section presents MEGA framework. Scroll animations enhance without blocking content. Writing section infrastructure ready.

### L2 — Enhancement (PDF, depth)

PDF generation and deeper content. Portfolio becomes downloadable.

| Task | Component | Dependencies | Done When |
|------|-----------|-------------|-----------|
| PDF CV generation | `src/server/pdf.rs` | Typst crate, template design | Server function generates CV PDF from database |
| ATS-friendly PDF variant | Second Typst template | PDF infrastructure | Plain layout PDF passes ATS parsing |
| Download button + feedback | UI component | PDF endpoint | Button shows generation state, triggers download |
| First technical writing pieces | Content | Gabriel writes 2-3 articles | Real articles displayed in writing section |
| Expandable timeline nodes | Timeline enhancement | Timeline component (L1) | Click/tap expands into detailed case study |
| Skills radar chart | `src/components/about/radar.rs` | Data (seeded) | One overview radar chart of engineering shape |

**Exit criteria**: CV downloadable as PDF (both visual and ATS versions). At least 2 technical writing pieces published. Timeline nodes expandable.

### L3 — Perfection (polish, edge cases)

Production-grade polish.

| Task | Component | Dependencies | Done When |
|------|-----------|-------------|-----------|
| PDF visual polish | Typst template refinement | L2 PDF | Typography, whitespace, layout perfected |
| PDF versioning + caching | Server-side cache | L2 PDF | PDFs cached, regenerated only on data change |
| Print stylesheet | CSS @media print | L2 | Browser print matches PDF design |
| Hero animation | WASM-powered interactive element | Architectural decision | About page hero has subtle, meaningful animation |
| Accessibility audit | Manual + automated | All L0-L2 components | WCAG 2.1 AA verified on every component |
| Performance optimization | Bundle analysis | All components | WASM budget maintained, FCP < 3s |
| Open Graph / social meta | Meta tags | Content finalized | LinkedIn/Twitter previews look professional |

**Exit criteria**: PDF is production-quality. All components pass accessibility audit. Performance budgets met. Social sharing looks good.

---

## Key Decisions for Build Phase

1. **About page structure**: Single scrollable page with sections (about, experience, skills, methodology, contact) — NOT separate routes for each. Reason: hiring managers scan one page, don't click through multiple routes.

2. **Skills display**: Categorized grid (Languages, Frameworks, Tools, Concepts) with contextual proficiency indicators. NO percentage bars. Proficiency shown as text: "5 years daily use" or "learning" — honest and meaningful.

3. **Experience data strategy**: IDENTITY drafts proposed entries from known project inventory. Gabriel reviews and approves. This unblocks the timeline without waiting indefinitely.

4. **PDF approach**: Typst (Rust-native). Dual output: visual + ATS. Same database, different templates.

5. **Writing section**: Labeled "Technical Writing" not "Blog." Infrastructure in L1, content in L2. Anthropic explicitly asks for this.

6. **No i18n**: English only. Correct for target audience.

---

## Content Inventory Needed from Gabriel

| Content | Priority | Proposed Action |
|---------|----------|----------------|
| Work experience entries | CRITICAL | IDENTITY drafts from known projects, Gabriel reviews |
| Education details | HIGH | Gabriel provides degrees, institutions, dates |
| Contact preference | CRITICAL | Email, LinkedIn URL, preferred contact method |
| About narrative voice check | HIGH | Review seed data for accuracy and personal voice |
| "How I Work" approval | HIGH | Review MEGA framework presentation approach |
