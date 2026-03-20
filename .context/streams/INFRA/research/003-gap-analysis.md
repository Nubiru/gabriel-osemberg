# 003 — Gap Analysis: Infrastructure

**Stream**: INFRA
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Methodology

Compared current inventory (001) against the vision in CLAUDE.md, SOUL.md, status.md build checklist, and world survey (002). Gaps categorized by severity:

- **CRITICAL**: Blocks launch or user experience
- **HIGH**: Significantly impacts professional presentation
- **MEDIUM**: Improves quality but not blocking
- **LOW**: Nice-to-have, polish-level

---

## Gap Matrix

### CRITICAL — Must Have Before Launch

| # | Gap | Current State | Target State | Axis |
|---|-----|--------------|-------------|------|
| C1 | **Deployment config** | No Dockerfile, no fly.toml | Multi-stage Dockerfile + fly.toml deploying to Fly.io | PRESENCE |
| C2 | **CD pipeline** | CI builds but never deploys | Auto-deploy on push to main | PRESENCE |
| C3 | **Custom domain + HTTPS** | No domain | `gabriel-osemberg.dev` (or similar) with forced HTTPS | PRESENCE |
| C4 | **SEO meta tags** | No Open Graph, no description | Full OG/Twitter cards, JSON-LD Person schema, page descriptions | PRESENCE |

**Rationale**: Without deployment, the site doesn't exist publicly. Without SEO, sharing a link produces a blank preview card. These are table stakes for a professional portfolio.

### HIGH — Should Have at Launch

| # | Gap | Current State | Target State | Axis |
|---|-----|--------------|-------------|------|
| H1 | **robots.txt** | Missing | Static file allowing all crawlers, pointing to sitemap | PRESENCE |
| H2 | **sitemap.xml** | Missing | Dynamic or static sitemap listing all public pages | PRESENCE |
| H3 | **Cache headers** | None configured | Immutable for hashed WASM/JS/CSS, short for HTML | CRAFT |
| H4 | **WASM compression** | No compression layer | Brotli/gzip served by Fly.io edge or Axum middleware | CRAFT |
| H5 | **Error pages** | No custom error pages | Styled 404 and 500 pages | CRAFT |
| H6 | **Health check endpoint** | None | `/health` returning 200 for Fly.io machine health | PRESENCE |

### MEDIUM — Improve Quality

| # | Gap | Current State | Target State | Axis |
|---|-----|--------------|-------------|------|
| M1 | **Analytics** | None | GoatCounter (or Plausible) tracking page views | PRESENCE |
| M2 | **Lighthouse CI** | Not in pipeline | GH Action running Lighthouse, failing on <90 scores | CRAFT |
| M3 | **E2E tests** | Playwright configured, no tests | At least smoke tests for key pages | CRAFT |
| M4 | **Social preview images** | None | OG image per page (at minimum a default branded image) | PRESENCE |
| M5 | **Environment separation** | Single .env | Dev/staging/production configs | CRAFT |
| M6 | **wasm-opt** | Not used | wasm-opt in Dockerfile or CI for additional 10-20% size reduction | CRAFT |

### LOW — Polish

| # | Gap | Current State | Target State | Axis |
|---|-----|--------------|-------------|------|
| L1 | **CDN for static assets** | Fly.io statics (basic) | Explicit CDN config with edge caching | CRAFT |
| L2 | **Uptime monitoring** | None | External uptime check (UptimeRobot, Fly.io built-in) | PRESENCE |
| L3 | **Error monitoring** | None | Lightweight error tracking (could be custom logging) | CRAFT |
| L4 | **Production smoke tests** | None | Automated post-deploy health verification | CRAFT |
| L5 | **Performance monitoring** | WASM size in CI only | Runtime performance metrics (FCP, LCP) | CRAFT |

---

## Severity Distribution

| Severity | Count | Summary |
|----------|-------|---------|
| CRITICAL | 4 | Deployment, CD, domain, SEO meta |
| HIGH | 6 | robots.txt, sitemap, caching, compression, errors, health |
| MEDIUM | 6 | Analytics, Lighthouse CI, E2E, OG images, env separation, wasm-opt |
| LOW | 5 | CDN, uptime, error monitoring, smoke tests, perf monitoring |

---

## What Already Works Well

| Item | Quality | Notes |
|------|---------|-------|
| CI pipeline | SOLID | 3-job workflow: check, test, build with zero-warning enforcement |
| Build optimization | SOLID | WASM profile with opt-level z, LTO, codegen-units 1, panic abort |
| WASM size reporting | GOOD | CI reports raw + gzipped size to GitHub summary |
| Database setup | SOLID | SQLx with compile-time checking, migrations, seeded data |
| Code quality gates | SOLID | clippy -D warnings, rustfmt, integration tests |
| Git hygiene | GOOD | Comprehensive .gitignore, tracked paths defined |

---

## Priority Order for Build Phase

Based on gap severity and dependencies:

1. **Dockerfile** (C1) — everything depends on this
2. **fly.toml + Fly.io setup** (C1 continued) — deployment target
3. **CD pipeline** (C2) — auto-deploy on push
4. **Health check endpoint** (H6) — Fly.io needs this
5. **SEO meta tags** (C4) — leptos_meta integration
6. **robots.txt + sitemap.xml** (H1, H2) — serve as static files or routes
7. **Custom domain + HTTPS** (C3) — Fly.io handles certs
8. **Cache headers** (H3) — Axum middleware
9. **Error pages** (H5) — Leptos components
10. **Analytics** (M1) — single script tag
11. **Lighthouse CI** (M2) — GH Action addition
12. **wasm-opt** (M6) — Dockerfile addition

---

## Cross-Stream Dependencies

| Gap | Depends On | Stream |
|-----|-----------|--------|
| C4 (SEO meta) | Page components must use `<Title>` and `<Meta>` | DESIGN |
| H5 (Error pages) | Error page design/styling | DESIGN |
| M4 (OG images) | Branded image assets | DESIGN |
| M3 (E2E tests) | Pages must exist to test against | DESIGN, DATA |
| M2 (Lighthouse CI) | Deployed site serving real content | DATA, DESIGN |

INFRA can proceed with C1, C2, C3, H1, H2, H3, H6 independently. SEO meta and error pages need coordination with DESIGN.
