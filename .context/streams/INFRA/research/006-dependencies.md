# 006 — Dependencies: Infrastructure

**Stream**: INFRA
**Date**: 2026-03-20
**Status**: COMPLETE

---

## What INFRA Needs From Other Streams

| # | Need | From Stream | When | Blocking? |
|---|------|------------|------|-----------|
| D1 | **Page components with leptos_meta** | DESIGN | L1 (SEO meta tags) | No — INFRA can scaffold SEO infrastructure, DESIGN adds component-level meta |
| D2 | **Styled error page components** | DESIGN | L1 (Error pages) | No — INFRA can create fallback, DESIGN styles later |
| D3 | **OG image assets** | DESIGN | L2 (Social preview) | No — can use placeholder |
| D4 | **Functioning pages to test** | DESIGN + DATA | L2 (E2E tests, Lighthouse) | Partially — need at least home page rendering real content |
| D5 | **Content in database** | DATA | L2 (Lighthouse scores) | No — seed data exists |

**Summary**: INFRA has **zero blocking dependencies** for L0. L1+ dependencies are soft — INFRA can scaffold infrastructure and other streams fill in content.

---

## What INFRA Provides To Other Streams

| # | Provide | To Stream | When | Description |
|---|---------|----------|------|-------------|
| P1 | **Live deployment URL** | ALL | After L0.5 | Public URL for testing, sharing, Lighthouse |
| P2 | **Health endpoint** | ALL | After L0.3 | `/health` route for monitoring and CI verification |
| P3 | **Cache headers middleware** | DESIGN | After L1.4 | Ensures assets served with correct caching |
| P4 | **SEO infrastructure** | DESIGN | After L1.2 | robots.txt, sitemap, meta tag framework |
| P5 | **CD pipeline** | ALL | After L0.4 | Every push auto-deploys — no manual deployment needed |
| P6 | **Lighthouse CI** | ALL | After L2.2 | Automated quality gate for performance and accessibility |
| P7 | **Analytics** | ALL | After L2.1 | Traffic data for measuring impact |

---

## Cross-Stream Communication Plan

### Messages to Send

| To | Message | Priority |
|----|---------|----------|
| DESIGN | "INFRA will scaffold SEO meta tag infrastructure (leptos_meta setup). DESIGN should use `<Title>` and `<Meta>` components in page-level components. INFRA will add default fallbacks." | MEDIUM |
| DESIGN | "INFRA needs a default OG image (1200x630px) for social sharing. Can be a simple branded image with Gabriel's name + tagline. Needed at L2." | LOW |
| DATA | "INFRA will deploy SQLite with Fly Volume for persistence. DATA stream should ensure seed migrations are idempotent (INSERT OR IGNORE, not just INSERT)." | MEDIUM |

### Messages to Receive (anticipated)

| From | Expected Message |
|------|-----------------|
| DESIGN | Confirmation of leptos_meta usage pattern in components |
| DESIGN | OG image asset delivery |
| DATA | Confirmation seed data works with fresh database creation |

---

## Dependency Graph

```
INFRA L0 (self-contained)
  |
  v
INFRA L1 ----needs----> DESIGN (meta tags, error pages)
  |
  v
INFRA L2 ----needs----> DESIGN (OG images) + DATA (content for E2E/Lighthouse)
  |
  v
INFRA L3 (self-contained)
```

No circular dependencies. INFRA is a **provider** stream — it unblocks others more than it depends on them.
