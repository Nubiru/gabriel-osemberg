# 001 — Inventory: IDENTITY Domain

**Stream**: IDENTITY
**Date**: 2026-03-20
**Scope**: About page, CV content, experience timeline, skills visualization, personal narrative, PDF generation, contact, education

---

## What Exists

### Data Models (REAL — fully implemented + tested)

| File | Struct | Fields | Tests |
|------|--------|--------|-------|
| `src/models/experience.rs` | `Experience` | id, role, company, company_url, start_date, end_date (Option), description, highlights (JSON), sort_order, visible | 3 unit tests |
| `src/models/skill.rs` | `Skill` | id, name, category, proficiency (1-5), visible | 2 unit tests |
| `src/models/cv_section.rs` | `CvSection` | id, section_type, title, content (markdown), sort_order, visible | 1 unit test |
| `src/models/project.rs` | `Project` | 12 fields (SHOWCASE domain, but PDF may reference) | Fully tested |

All models use `serde` serialization + `#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]`.

### Database Schema (REAL)

| File | Tables | Notes |
|------|--------|-------|
| `migrations/001_create_tables.sql` | experiences, skills, cv_sections, projects | All with id, visible, sort_order |
| `migrations/002_seed_showcase_data.sql` | 27 skills, 3 cv_sections seeded | **No experience entries seeded** — pending Gabriel's content |

Seeded cv_sections: about, methodology, philosophy. No education section yet.

### Server Functions (REAL)

| Function | Location | Returns |
|----------|----------|---------|
| `get_experiences()` | `src/server_fns.rs` | `Vec<Experience>` — visible, ordered by sort_order |
| `get_skills()` | `src/server_fns.rs` | `Vec<Skill>` — visible, ordered by category then proficiency desc |
| `get_cv_sections()` | `src/server_fns.rs` | `Vec<CvSection>` — visible, ordered by sort_order |

All use `#[server]` macro, proper error handling (`Result<T, ServerFnError>`), no `.unwrap()`.

### Components (DELIVERED from DESIGN stream)

| File | Component | Status |
|------|-----------|--------|
| `src/components/layout.rs` | Layout, Header, Footer, NavLink | REAL — wraps all pages, semantic HTML, ARIA |
| `src/components/theme_toggle.rs` | ThemeToggle | REAL — light/system/dark, localStorage |
| `src/components/mobile_menu.rs` | HamburgerButton, MobileMenu, MobileNavLink | REAL — **not yet integrated into Header** |

### Design Tokens (DELIVERED from DESIGN stream)

`style/tailwind.css` — Tailwind v4 @theme syntax with:
- Colors: OKLCH surfaces, text, accent (cyan-teal), borders, states
- Typography: Space Grotesk (headings), Inter (body), JetBrains Mono (code)
- Spacing, radius, shadows, easing, animations (fade-in, fade-up, pulse-subtle)
- Dark mode default, light mode overrides

### Routes (SCAFFOLD)

| Route | Status |
|-------|--------|
| `/` | Temporary design token showcase — NOT real home page |
| `/about` | Route defined in router, **no component** |
| `/projects` | Route defined in router, **no component** (SHOWCASE domain) |

### Tests (REAL)

`tests/data_queries.rs` — 8 integration tests:
- Skills seeded correctly (20+)
- CV sections seeded (3)
- Experiences table exists but empty (pending content)
- Projects tested (SHOWCASE domain)

### Server Infrastructure (REAL)

`src/main.rs` — Axum + SQLite pool (5 max connections) + auto-migrations on startup.

---

## What Does NOT Exist

| Item | Category | Blocking? |
|------|----------|-----------|
| About page component | UI | No — can build now with cv_sections data |
| Experience timeline component | UI | Partially — model ready, but no seed data |
| Skills visualization component | UI | No — 27 skills seeded, server fn ready |
| Contact section/page | UI | No — can build now |
| Education section content | Data | Yes — no seed data, no section_type="education" |
| Experience seed data | Data | Yes — table empty, needs Gabriel's real CV |
| PDF generation (`src/pdf/`) | Feature | Not blocking L0-L1 |
| Download CV endpoint | Feature | Not blocking L0-L1 |
| Home page real content | UI | Shared with other streams |
| Mobile menu integration | UI | Minor — components exist, need wiring |
| Methodology showcase page | UI | No — cv_section seeded |

---

## Readiness Assessment

**Strong foundation.** DATA and DESIGN streams delivered L0 infrastructure. Models, schema, server functions, layout, and design tokens are all real and tested.

**Primary gap**: No UI components for IDENTITY's pages. The route `/about` exists but renders nothing.

**Content gap**: No experience entries seeded. Gabriel must provide real CV content (companies, roles, dates, descriptions). Skills and cv_sections have real seed data.

**Unblocked work**: About page, skills section, methodology section, and contact section can all be built today using existing data and design tokens.
