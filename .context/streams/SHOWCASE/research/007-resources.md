# 007 — Resources: Reference Materials & Knowledge Sources

**Stream**: SHOWCASE
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Summary

Maps all reference materials SHOWCASE needs, categorized by availability. Most Leptos resources are available online. The key missing resources are project screenshots (manual capture needed).

---

## Leptos Framework Resources

| Resource | URL/Path | Purpose | Status |
|----------|----------|---------|--------|
| Leptos Book | https://book.leptos.dev/ | Component patterns, reactivity, SSR | Available |
| Leptos API docs | https://docs.rs/leptos/latest/leptos/ | `view!` macro, SVG module, component macro | Available |
| Leptos SVG module | https://docs.rs/leptos/latest/leptos/svg/ | Typed SVG elements for charts | Available |
| Leptos Suspense | https://book.leptos.dev/ (async chapter) | Loading states, resource pattern | Available |
| Leptos Router | https://docs.rs/leptos_router/ | Nested routes, params, navigation | Available |
| leptos_meta | https://docs.rs/leptos_meta/ | Page titles, meta tags, OG tags | Available |

## Rust + WASM Visualization

| Resource | URL/Path | Purpose | Status |
|----------|----------|---------|--------|
| Plotters crate | https://docs.rs/plotters/ | Backup chart library if hand-rolled SVG insufficient | Available (L2+ fallback) |
| Charming crate | https://github.com/yuankunzhang/charming | ECharts-like visualization in Rust | Available (L2+ fallback) |
| SVG spec (MDN) | https://developer.mozilla.org/en-US/docs/Web/SVG | SVG element reference for hand-rolled charts | Available |
| stroke-dasharray | MDN SVG docs | Key technique for progress ring charts | Available |

## Design Reference

| Resource | URL/Path | Purpose | Status |
|----------|----------|---------|--------|
| Brittany Chiang | https://brittanychiang.com/ | Card layout, featured/archive pattern | Available |
| Lee Robinson | https://leerob.com/ | Content-first showcase approach | Available |
| Dribbble project cards | https://dribbble.com/tags/project-cards | Visual inspiration for card design | Available |
| DESIGN stream tokens | style/tailwind.css | Color, typography, spacing, shadow system | Local — delivered |

## Case Study Format Reference

| Resource | URL/Path | Purpose | Status |
|----------|----------|---------|--------|
| InfluenceFlow guide | https://influenceflow.io/resources/guide-to-portfolio-case-studies-... | Case study structure best practices | Available |
| UXfol.io template | https://blog.uxfol.io/ux-case-study-template/ | 8-section case study template | Available |
| Toptal case study analysis | https://www.toptal.com/designers/ui/case-study-portfolio | Process-driven portfolio design | Available |

## Project Data Sources (for screenshots + content)

| Project | Local Path | What's Needed | Status |
|---------|-----------|---------------|--------|
| time | /home/gabiota/personal/projects/code/time | Screenshots of WebGL2 calendar renders, shader demos | **NEEDS CAPTURE** |
| blocksight | /home/gabiota/personal/projects/code/blocksight-main | Screenshots of dashboard, blockchain visualizations | **NEEDS CAPTURE** |
| anan-yarok | /home/gabiota/personal/projects/code/anan-yarok | Screenshots of storefront, admin panel | **NEEDS CAPTURE** |
| chamana | /home/gabiota/personal/projects/code/chamana | Screenshots of catalog pages | **NEEDS CAPTURE** |
| gabriel-osemberg | This project | Screenshots after showcase pages are built | **LATER** |

## Codebase Files (Local)

| File | Purpose for SHOWCASE |
|------|---------------------|
| `src/models/project.rs` | Project struct definition — component props source |
| `src/server_fns.rs` | Server function signatures — data fetching patterns |
| `src/components/layout.rs` | Layout shell — wrap all showcase pages |
| `src/components/theme_toggle.rs` | Theme reference — dark/light styling approach |
| `style/tailwind.css` | Design tokens — all styling values |
| `src/app.rs` | Router — add showcase routes |
| `tests/data_queries.rs` | Test patterns — follow same structure for component tests |
| `migrations/002_seed_showcase_data.sql` | Seed data — actual project content to display |

## Architecture Documents

| File | Purpose for SHOWCASE |
|------|---------------------|
| `docs/DECISIONS.md` — ADR-002 | Project selection rationale — informs case study narrative |
| `docs/DECISIONS.md` — ADR-003 | SQLite decision — data access patterns |
| `.context/streams/DATA/research/005-roadmap.md` | DATA L1 model definitions (ProjectMetric, TechTag) |
| `.context/streams/DESIGN/research/005-roadmap.md` | DESIGN component roadmap (what's coming) |

---

## Resource Gaps

| Gap | Impact | Resolution |
|-----|--------|------------|
| Project screenshots | HIGH — visual showcase needs images | Gabriel must capture from running projects |
| Demo videos | MEDIUM — nice-to-have for L3 | Gabriel records screen captures later |
| AI collaboration narrative text | MEDIUM — L2 feature | Write during L2, or IDENTITY provides |
| Case study prose per project | HIGH — detail pages need written content | Seed migration has descriptions; expand during L1 build |

---

## Key Takeaway

All **technical resources** are available. The primary gap is **visual content** (screenshots) which requires manual capture by Gabriel from running projects. SHOWCASE can build all components and pages without waiting for screenshots — use placeholder images for L0, replace with real screenshots before L1 delivery.
