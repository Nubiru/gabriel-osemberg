# 007 — Resources: IDENTITY Domain

**Stream**: IDENTITY
**Date**: 2026-03-20

---

## Crates Needed

| Crate | Purpose | Layer | Status |
|-------|---------|-------|--------|
| `pulldown-cmark` | Markdown → HTML for cv_sections content | L0 | Available, well-maintained (3K+ stars) |
| `typst-as-lib` | PDF generation from Typst templates | L2 | Available (98 stars, 266 commits, API unstable) |
| `typst` | Direct Typst compiler embedding (alternative to typst-as-lib) | L2 | Available (40K+ stars) |
| `leptos_i18n` | Internationalization (IF implemented) | L4+ | Available but NOT recommended — see 004 |

Note: Leptos, SQLx, Axum, serde, tokio already in Cargo.toml.

---

## Reference Portfolios (for design inspiration during build)

| Site | What to Reference |
|------|-------------------|
| brittanychiang.com | Tabbed experience section, cursor spotlight effect, archive table |
| joshwcomeau.com | Interactive demos as proof, personality in writing, "site IS the portfolio" |
| leerob.com | Impact-first framing, radical content pruning, minimalism |
| cassie.codes | Custom hero element, personality-first positioning statement |

---

## Reference Documents (for content during build)

| Document | Path | Use For |
|----------|------|---------|
| Portfolio inventory | Memory: `portfolio_inventory.md` | Drafting experience entries, project metrics |
| Project data seed | `migrations/002_seed_showcase_data.sql` | Existing project descriptions and metrics |
| SOUL.md | `SOUL.md` | Voice, vision, positioning language |
| Anthropic careers | anthropic.com/careers | Aligning positioning with what they value |
| Anthropic AI work research | anthropic.com/research/how-ai-is-transforming-work-at-anthropic | MEGA framework parallels |

---

## Books (from tracks.md — in Gabriel's library)

| Book | Author | Relevant To |
|------|--------|-------------|
| *Show Your Work!* | Austin Kleon | Sharing creative process, building narrative |
| *Success in Programming* | Frederic Harper | Developer personal branding |
| *Vibe Coding* | Addy Osmani | AI-assisted engineering positioning, avoiding "vibe coder" label |

**Action**: These could inform the "How I Work" section and technical writing content. If Gabriel has notes or highlights, they could feed the methodology narrative.

---

## Technical References (for build phase)

| Topic | Resource | Notes |
|-------|----------|-------|
| Leptos components | leptos.dev/book | Component patterns, signals, resources, server functions |
| Leptos 0.8 migration | leptos.dev/migration | Breaking changes from 0.7 → 0.8 |
| Intersection Observer in WASM | web-sys IntersectionObserver | For scroll-triggered animations in Leptos |
| Typst documentation | typst.app/docs | Template language for PDF generation |
| Typst embedding | typst.app/blog/2025/automated-generation/ | How to embed Typst as Rust library |
| WCAG 2.1 AA | w3.org/WAI/WCAG21/quickref/ | Accessibility requirements |
| Tailwind v4 | tailwindcss.com/docs | @theme syntax, utility classes |
| ATS resume format | CVCraft + PitchMeAI guides | Requirements for machine-parseable PDFs |

---

## Existing Codebase Patterns to Follow

| Pattern | Where | Apply To |
|---------|-------|----------|
| Leptos component with server data | `src/app.rs::HomePage` (temporary) | About page fetching cv_sections |
| Server function with SQLx | `src/server_fns.rs` | Any new server functions |
| Model with serde + sqlx | `src/models/experience.rs` | Any new models |
| Integration test with in-memory SQLite | `tests/data_queries.rs` | Testing new queries |
| Design token usage | `style/tailwind.css` | All new components must use tokens, not raw values |
| Semantic HTML + ARIA | `src/components/layout.rs` | All new components |

---

## Knowledge Gaps (need research during build)

| Gap | When Needed | Research Method |
|-----|-------------|----------------|
| Leptos `create_resource` / `Suspense` pattern for fetching data on page load | L0 build | Read Leptos docs, check existing app.rs pattern |
| Intersection Observer API in web-sys for scroll animations | L1 build | web-sys docs, WASM bindgen examples |
| Typst template language syntax | L2 build | Typst docs |
| `pulldown-cmark` API for markdown rendering in Leptos | L0 build | Crate docs + examples |
| SVG generation in Leptos for radar chart | L2 build | Leptos SVG support docs |

---

## Content Resources Needed from Gabriel

| Content | Priority | Alternative if Unavailable |
|---------|----------|---------------------------|
| Work history (companies, roles, dates) | CRITICAL | Draft from project inventory, Gabriel reviews |
| Education (degrees, institutions) | HIGH | Omit section until provided |
| Contact info (email, LinkedIn) | CRITICAL | GitHub profile (Nubiru) as minimum |
| Professional photo | MEDIUM | Skip photo, use typography-driven design |
| About narrative voice approval | HIGH | Use seed data, iterate after review |

---

## Summary

**L0 build requires**: `pulldown-cmark` crate addition, existing codebase patterns, design tokens, seeded data. No external resources blocking.

**L1 build requires**: Intersection Observer research, experience seed data (IDENTITY drafts).

**L2 build requires**: Typst crate evaluation, template design, technical writing content.

All critical resources are either available or can be produced by IDENTITY drafting content for Gabriel's review.
