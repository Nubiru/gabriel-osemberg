# 003 — Gap Analysis: IDENTITY Domain

**Stream**: IDENTITY
**Date**: 2026-03-20
**Method**: Inventory (001) vs Vision (SOUL.md + tracks.md + world survey 002)

---

## Gap Categories

### CRITICAL — Must exist for L0 launch

| Gap | What Exists | What's Needed | Blocked By |
|-----|-------------|---------------|------------|
| About page component | Route `/about` defined, no component | Leptos component fetching cv_sections, rendering markdown | Nothing — unblocked |
| Experience entries in database | Table exists, server fn exists, 0 rows | Gabriel's real CV content (companies, roles, dates, descriptions, highlights) | **Gabriel's input** |
| Skills page/section component | 27 skills seeded, server fn ready | Leptos component with categorized grid display | Nothing — unblocked |
| Contact section | Nothing exists | Contact info component (email, GitHub, LinkedIn) | Nothing — unblocked |
| About page narrative content | cv_sections seeded (about, methodology, philosophy) but content is seed data | Review/refine seed content for accuracy and voice | **Gabriel's review** |

### HIGH — Must exist for L1 (interactive, polished)

| Gap | What Exists | What's Needed | Blocked By |
|-----|-------------|---------------|------------|
| Experience timeline component | Data model + server fn ready | Interactive vertical timeline with scroll reveals | Experience seed data (CRITICAL gap) |
| Skills visualization | Data model + server fn ready | Radar chart or categorized grid with proficiency indicators | Nothing — unblocked |
| "How I Work" / methodology page | cv_section type="methodology" seeded | Dedicated page/section showcasing AI-Augmented Engineering approach | Nothing — unblocked |
| Education section | cv_sections table can hold type="education" | Seed education data, render in component | Gabriel's education info |
| Mobile menu integration | HamburgerButton + MobileMenu components exist | Wire into Header component in layout.rs | Nothing — unblocked (DESIGN domain, but shared) |

### MEDIUM — L2 features

| Gap | What Exists | What's Needed | Blocked By |
|-----|-------------|---------------|------------|
| PDF CV generation | Nothing — no src/pdf/ | Typst integration, template, server endpoint, download button | Typst crate research, template design |
| ATS-friendly PDF variant | Nothing | Second Typst template, plain layout, standard fonts | PDF infrastructure (above) |
| Scroll animations | fade-in/fade-up keyframes in tailwind.css | Intersection Observer equivalent in Leptos, scroll-triggered reveals | Nothing — design tokens ready |
| Expandable timeline nodes | Nothing | Click/tap to expand into case study per experience entry | Timeline component (HIGH gap) |

### LOW — L3 perfection

| Gap | What Exists | What's Needed | Blocked By |
|-----|-------------|---------------|------------|
| PDF visual polish | Nothing | Typography, layout, whitespace refinement | L2 PDF infrastructure |
| PDF versioning + caching | Nothing | Cache strategy, version tracking | L2 PDF infrastructure |
| Print stylesheet | Nothing | CSS @media print matching PDF design | L2 PDF infrastructure |
| Multi-language (EN + ES) | Nothing | i18n infrastructure, translated content | Architectural decision from MEGA |
| Hero animation | Nothing | WASM-powered interactive element (mini WebGL2? particle system?) | Architectural decision |

---

## Content Gaps (require human input)

These gaps cannot be resolved by code alone — they need Gabriel's real information:

| Content | Status | Action |
|---------|--------|--------|
| Work experience entries | Empty table | Gabriel provides: companies, roles, dates, descriptions, highlights with impact metrics |
| Education details | No seed data | Gabriel provides: degrees, institutions, dates, relevant coursework |
| Contact information | Nothing | Gabriel provides: email, LinkedIn URL, GitHub (already known: Nubiru), preferred contact method |
| About narrative voice | Seed exists, needs review | Gabriel reviews seed content for accuracy and personal voice |
| Project highlights for CV PDF | DATA stream has project data | Coordinate with SHOWCASE for curated 3-5 project summaries |

---

## Infrastructure Gaps (code needed)

| Component | Severity | Effort Estimate | Dependencies |
|-----------|----------|-----------------|--------------|
| `/about` page component | CRITICAL | Small — fetch cv_sections + render | DESIGN tokens (delivered) |
| `/about` experience timeline | HIGH | Medium — new component with scroll animation | Experience seed data |
| `/about` skills section | HIGH | Medium — categorized grid + optional chart | Skills data (seeded) |
| Contact component | CRITICAL | Small — static info display | Contact info from Gabriel |
| PDF generation server fn | MEDIUM | Large — Typst integration, template authoring | Typst crate evaluation |
| Intersection Observer in Leptos | MEDIUM | Small — utility for scroll reveals | Research Leptos scroll APIs |
| Markdown renderer for cv_sections | HIGH | Small — `pulldown_cmark` or similar in Leptos | None |

---

## Cross-Stream Gaps

| This Stream Needs | From Stream | Status | Notes |
|-------------------|-------------|--------|-------|
| Layout shell + design tokens | DESIGN | **DELIVERED** | L0 complete |
| CV data server functions | DATA | **DELIVERED** | get_experiences, get_skills, get_cv_sections all exist |
| Timeline component design | DESIGN | OPEN (L2.5) | IDENTITY can build basic timeline; DESIGN polishes later |
| Project summaries for PDF | SHOWCASE | OPEN | L2 dependency |
| CV content text for seed data | DATA needs FROM IDENTITY | **ROUTED** | IDENTITY should draft, Gabriel approves |

---

## Priority Order for Build Phase

Based on gap severity, dependencies, and what's unblocked:

1. **About page component** — fetch cv_sections, render markdown (CRITICAL, unblocked)
2. **Skills section** — categorized grid from seeded data (HIGH, unblocked)
3. **Contact section** — static display, needs Gabriel's info (CRITICAL, needs input)
4. **Draft experience content** — propose entries for Gabriel's review (CRITICAL blocker removal)
5. **Draft education content** — propose entries for Gabriel's review (HIGH blocker removal)
6. **Experience timeline** — vertical timeline component (HIGH, needs step 4)
7. **Methodology/"How I Work" section** — expand methodology cv_section (HIGH, unblocked)
8. **Scroll animations** — intersection observer for reveals (MEDIUM, unblocked)
9. **PDF generation** — Typst integration (MEDIUM, large effort)
10. **PDF ATS variant** — second template (MEDIUM, needs step 9)

---

## Summary

**Foundation is strong.** DATA and DESIGN streams delivered everything IDENTITY needs for L0-L1 infrastructure. The gaps are:
- **UI components** — no pages exist yet, all must be built
- **Human content** — experience entries and education data need Gabriel's real information
- **PDF system** — entirely future work (L2)

**Biggest risk**: Experience data depends on Gabriel's input. IDENTITY should draft proposed content based on known projects (from portfolio inventory) and present for review, rather than blocking on this.
