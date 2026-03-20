# 006 — Dependencies: IDENTITY Domain

**Stream**: IDENTITY
**Date**: 2026-03-20

---

## What IDENTITY Needs FROM Other Streams

| From | What | Layer | Status | Notes |
|------|------|-------|--------|-------|
| DESIGN | Layout shell, Header, Footer, NavLink | L0 | **DELIVERED** | `src/components/layout.rs` |
| DESIGN | Design tokens (surfaces, text, accent, borders) | L0 | **DELIVERED** | `style/tailwind.css` |
| DESIGN | Font system (Space Grotesk, Inter, JetBrains Mono) | L0 | **DELIVERED** | Self-hosted woff2 in `public/fonts/` |
| DESIGN | Theme toggle (dark/light/system) | L0 | **DELIVERED** | `src/components/theme_toggle.rs` |
| DESIGN | Mobile menu integration into Header | L1 | **OPEN** | Components exist (`mobile_menu.rs`) but not wired into Header |
| DESIGN | Timeline component visual design guidance | L2 | **OPEN** | DESIGN lists as L2.5 |
| DESIGN | Scroll animation patterns (timing, easing) | L1 | **PARTIALLY DELIVERED** | Keyframes in CSS, but no Leptos-side scroll trigger utility |
| DATA | Experience model + server function | L0 | **DELIVERED** | `src/models/experience.rs`, `src/server_fns.rs::get_experiences()` |
| DATA | Skill model + server function | L0 | **DELIVERED** | `src/models/skill.rs`, `src/server_fns.rs::get_skills()` |
| DATA | CvSection model + server function | L0 | **DELIVERED** | `src/models/cv_section.rs`, `src/server_fns.rs::get_cv_sections()` |
| DATA | Database schema + migrations | L0 | **DELIVERED** | `migrations/001_create_tables.sql`, `migrations/002_seed_showcase_data.sql` |
| SHOWCASE | Project summaries for CV PDF | L2 | **OPEN** | PDF includes project highlights section |
| INFRA | Deployment infrastructure | L1 | **OPEN** | Need live URL for portfolio to be usable |

---

## What IDENTITY Provides TO Other Streams

| To | What | Layer | Status | Notes |
|----|------|-------|--------|-------|
| DATA | CV content text for seed data (experience entries, education, about narrative refinements) | L0 | **ACTION NEEDED** | DATA requested this in inbox. IDENTITY should draft proposed entries. |
| DESIGN | Timeline entry count + field requirements | L0 | **ANSWERABLE** | DATA confirmed model. IDENTITY answers: N entries, fields = company, role, dates, description, highlights[] |
| SHOWCASE | "How I Work" content (MEGA framework explanation) | L1 | **OPEN** | May be referenced from project case studies |
| ALL | Gabriel's positioning statement | L0 | **DECIDED** | "AI-Augmented Engineer" — research in 004-questions.md |

---

## Cross-Stream Messages to Send

### → DATA inbox

**RE: CV content text for seed data**

IDENTITY will draft proposed experience entries and education content based on the project inventory (portfolio_inventory memory + known projects). Draft will include:
- 4-6 experience entries with company, role, date ranges, descriptions, impact highlights
- 1 education cv_section entry
- Review of existing about/methodology/philosophy seed text

DATA should expect these as a migration file or markdown for Gabriel's review. Content will be proposed, not final — Gabriel must approve voice and accuracy.

### → DESIGN inbox

**RE: Timeline entry count and fields**

- Entry count: Data-driven, expect 4-6 entries. Timeline component should handle N entries gracefully.
- Fields per entry: company, role, start_date, end_date (None = "Present"), description (1-2 paragraphs), highlights (array of 2-4 impact bullet points), company_url.
- Request: When mobile menu is wired into Header, notify IDENTITY so we can verify navigation works on About page.

---

## Dependency Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Experience data blocked on Gabriel's input | HIGH | IDENTITY drafts proposed content; doesn't wait |
| PDF generation requires Typst crate evaluation | MEDIUM | Defer to L2; website content is higher priority |
| Mobile menu not integrated | LOW | About page works without it; DESIGN domain |
| Deployment infrastructure not ready | LOW | Local dev is sufficient for L0-L1 |
| SHOWCASE project summaries for PDF | LOW | L2 dependency; PDF is L2 itself |

---

## Unblocked Work (no dependencies)

Everything in L0 except experience entries and education data. IDENTITY can build:
1. About page component (cv_sections data exists)
2. Skills section (27 skills seeded)
3. Contact section (needs Gabriel's info, but component can be built with placeholder)
4. Markdown rendering utility
5. Route wiring for `/about`
