# 005 — Roadmap: Infrastructure

**Stream**: INFRA
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Layered Roadmap

### L0 — Foundation (Must Have Before First Public Visit)

| # | Task | Description | Dependencies | Deliverable |
|---|------|-------------|-------------|-------------|
| L0.1 | **Dockerfile** | Multi-stage Debian build: cargo-leptos + wasm32 target -> slim runtime | None | `Dockerfile` |
| L0.2 | **fly.toml** | Fly.io app config: region, port 8080, statics, auto-HTTPS, min 1 machine | L0.1 | `fly.toml` |
| L0.3 | **Health endpoint** | `/health` route returning 200 OK (Fly.io health checks) | None | `src/server/health.rs` |
| L0.4 | **CD pipeline** | GH Action: on push to main -> build Docker image -> deploy to Fly.io | L0.1, L0.2 | `.github/workflows/deploy.yml` |
| L0.5 | **First deploy** | Verify site is live on `*.fly.dev` | L0.1-L0.4 | Running app |
| L0.6 | **Fly Volume** | Persistent volume for SQLite database | L0.5 | `fly volumes create` |
| L0.7 | **robots.txt** | Static file: allow all, point to sitemap | None | `public/robots.txt` or Axum route |
| L0.8 | **sitemap.xml** | Static sitemap with main pages | None | `public/sitemap.xml` or Axum route |

**Exit criteria**: Site accessible on `*.fly.dev`, health check passing, auto-deploying on push, robots.txt + sitemap serving.

### L1 — Integration (Professional Presentation)

| # | Task | Description | Dependencies | Deliverable |
|---|------|-------------|-------------|-------------|
| L1.1 | **Custom domain** | Register + configure domain, Fly.io cert provisioning | L0.5 | Domain DNS + fly certs |
| L1.2 | **SEO meta tags** | `<Title>`, `<Meta>` (OG, Twitter), `<meta description>` via leptos_meta | DESIGN stream pages | Updated components |
| L1.3 | **JSON-LD structured data** | Person schema for rich search results | L1.2 | JSON-LD in `<head>` |
| L1.4 | **Cache headers** | Axum middleware: immutable for hashed assets, no-cache for HTML | L0.5 | Cache middleware |
| L1.5 | **Error pages** | Styled 404 and 500 pages | DESIGN stream | Error components |
| L1.6 | **WASM compression** | Verify Fly.io serves gzip/brotli, configure if needed | L0.5 | Compression verified |

**Exit criteria**: Custom domain with HTTPS, SEO-ready meta tags, proper caching, styled error pages.

### L2 — Enhancement (Quality Assurance)

| # | Task | Description | Dependencies | Deliverable |
|---|------|-------------|-------------|-------------|
| L2.1 | **Analytics** | GoatCounter integration (script tag + noscript fallback) | L0.5 | Analytics script in layout |
| L2.2 | **Lighthouse CI** | GH Action running Lighthouse, failing on <90 | L0.4, build artifacts | `.github/workflows/lighthouse.yml` |
| L2.3 | **wasm-opt** | Add binaryen wasm-opt to Dockerfile for additional size reduction | L0.1 | Updated Dockerfile |
| L2.4 | **Social preview images** | Default OG image (branded), per-page if possible | DESIGN stream | OG images in public/ |
| L2.5 | **Environment config** | Separate dev/staging/production configs | L0.5 | `.env.example`, docs |
| L2.6 | **E2E smoke tests** | Playwright tests for home, projects, about pages | DESIGN + DATA | `end2end/` tests |

**Exit criteria**: Analytics tracking, Lighthouse scores >= 90 in CI, optimized WASM, E2E coverage for critical paths.

### L3 — Perfection (Production Excellence)

| # | Task | Description | Dependencies | Deliverable |
|---|------|-------------|-------------|-------------|
| L3.1 | **CDN optimization** | Fine-tune Fly.io statics, evaluate dedicated CDN for heavy assets | L1.4 | Optimized config |
| L3.2 | **Uptime monitoring** | External health check (UptimeRobot, Fly.io Machines API) | L0.3, L1.1 | Monitoring setup |
| L3.3 | **Error monitoring** | Structured logging + alerting (Fly.io logs or lightweight service) | L0.5 | Logging config |
| L3.4 | **Production smoke tests** | Post-deploy automated verification suite | L2.6 | Post-deploy script |
| L3.5 | **Performance dashboard** | Track FCP, LCP, WASM size trends over time | L2.2 | Metrics tracking |
| L3.6 | **Security headers** | CSP, X-Frame-Options, X-Content-Type-Options, Referrer-Policy | L0.5 | Axum middleware |

**Exit criteria**: Full production monitoring, security hardened, performance tracked, automated post-deploy verification.

---

## Task Estimates (Complexity, Not Time)

| Layer | Tasks | Can Do Independently | Needs Other Streams |
|-------|-------|---------------------|-------------------|
| L0 | 8 | 8 | 0 |
| L1 | 6 | 3 (domain, cache, compression) | 3 (SEO, errors, JSON-LD) |
| L2 | 6 | 3 (analytics, Lighthouse, wasm-opt) | 3 (OG images, env, E2E) |
| L3 | 6 | 6 | 0 |

**Key insight**: L0 is entirely self-contained. INFRA can complete L0 without waiting on any other stream. L1+ has cross-stream dependencies with DESIGN.
