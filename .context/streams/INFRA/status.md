# Stream: INFRA
**Phase**: 2 — BUILD
**Owner**: INFRA stream
**Last Updated**: 2026-03-20
**Readiness**: YELLOW — all achievable code complete, blocked on Fly.io deploy + domain

## Current Work

INFRA has completed all tasks that can be done without a live deployment. Remaining work requires human actions (Fly.io account, domain) or other stream deliveries (OG image, E2E content).

## Build Progress

### L0 Foundation — 6/8
- [x] Dockerfile (multi-stage Debian build + wasm-opt)
- [x] fly.toml (health checks, statics, SQLite volume)
- [x] Health check endpoint `/health` + 2 tests
- [x] CD pipeline (deploy.yml — triggers after CI passes)
- [ ] First deploy — **BLOCKED: needs `fly apps create` + `fly deploy`**
- [ ] Fly Volume — **BLOCKED: needs `fly volumes create sqlite_data`**
- [x] robots.txt
- [x] sitemap.xml

### L1 Integration — 4/6
- [ ] Custom domain + HTTPS — **BLOCKED: needs domain purchase**
- [x] SEO meta tags (OG/Twitter defaults in App, leptos_meta per-page)
- [x] JSON-LD structured data (Person schema in shell head)
- [x] Cache headers (Axum middleware — immutable for /pkg/, no-cache for HTML)
- [x] Error pages (DESIGN delivered NotFoundPage + ServerErrorPage)
- [ ] WASM compression verification — **needs live site**

### L2 Enhancement — 4/6
- [x] GoatCounter analytics (script in body, needs account creation)
- [x] Lighthouse CI (workflow + budget.json)
- [x] wasm-opt in Dockerfile (binaryen installed, runs after cargo leptos build)
- [ ] Social preview images — **needs DESIGN OG image**
- [x] Environment config (.env.example)
- [ ] E2E smoke tests — **needs pages rendering real content**

### L3 Perfection — 1/6
- [ ] CDN optimization — needs live site
- [ ] Uptime monitoring — needs live site
- [ ] Error monitoring / structured logging — needs live site
- [ ] Production smoke tests — needs live site
- [ ] Performance dashboard — needs live site
- [x] Security headers (X-Content-Type-Options, X-Frame-Options, Referrer-Policy, Permissions-Policy)

## Summary: 15/26 tasks complete

All remaining tasks are blocked on:
1. **Fly.io deployment** (L0.5, L0.6, L1.6, all L3) — human action
2. **Domain purchase** (L1.1) — human action
3. **DESIGN deliveries** (L2.4 OG image) — cross-stream
4. **Content rendering** (L2.6 E2E) — cross-stream

## Files Delivered

| File | Purpose |
|------|---------|
| `src/server/health.rs` | Health check endpoint with DB connectivity test |
| `src/server/middleware.rs` | Cache-Control + security headers middleware (5 unit tests) |
| `src/server/mod.rs` | Module exports for health + middleware |
| `src/main.rs` | Router with /health route, middleware layers, Extension(pool) |
| `src/app.rs` | OG/Twitter meta, JSON-LD Person, GoatCounter analytics |
| `Dockerfile` | Multi-stage build + wasm-opt |
| `.dockerignore` | Efficient Docker builds |
| `fly.toml` | Fly.io config with health checks + SQLite volume |
| `.github/workflows/deploy.yml` | CD pipeline (deploy after CI) |
| `.github/workflows/lighthouse.yml` | Lighthouse CI audit |
| `budget.json` | Performance budget for Lighthouse |
| `public/robots.txt` | SEO crawler instructions |
| `public/sitemap.xml` | Page listing for search engines |
| `.env.example` | Environment configuration template |
| `tests/health_endpoint.rs` | Health endpoint integration tests |

## Human Actions Needed to Unblock

1. `fly auth login`
2. `fly apps create gabriel-osemberg`
3. `fly volumes create sqlite_data --region iad --size 1`
4. `fly deploy` (or push to main → CD auto-deploys)
5. Add `FLY_API_TOKEN` to GitHub repo secrets (for CD pipeline)
6. Create GoatCounter account at goatcounter.com (site: gabriel-osemberg)
7. Register custom domain when ready (L1.1)

## Blockers

- Fly.io deployment (human action)
- Domain registration (human action)
- DESIGN: OG image (1200x630) for social sharing
- Migration conflict: two 005 files exist on disk (IDENTITY issue — renamed one to 006 locally)

## Metrics

- Research sections: 7/7
- L0: 6/8 | L1: 4/6 | L2: 4/6 | L3: 1/6
- Total: 15/26 tasks (58%)
- Files delivered: 15
- Tests added: 7 (2 integration + 5 unit)
