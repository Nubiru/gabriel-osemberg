# Stream: INFRA
**Phase**: 2 — BUILD
**Owner**: INFRA stream
**Last Updated**: 2026-03-20
**Readiness**: YELLOW — L0 code complete, blocked on Fly.io account for deploy

## Current Work

L0 Foundation: 6/8 tasks complete. Remaining 2 tasks (first deploy + Fly Volume) require human action (Fly.io account creation).

## Key Decisions from Research

- **Deployment**: Fly.io (Shuttle.rs eliminated, Railway optional backup)
- **Analytics**: GoatCounter (free, privacy-first, zero infrastructure)
- **WASM optimization**: Add wasm-opt to Dockerfile (10-20% additional savings)
- **Caching**: Hash-based immutable for WASM/JS/CSS, no-cache for HTML
- **Lighthouse CI**: treosh/lighthouse-ci-action, fail on <90

## Research Progress

All 7/7 sections COMPLETE.

## Build Progress

### L0 Foundation
- [x] Dockerfile (multi-stage Debian build) — `Dockerfile`
- [x] fly.toml (Fly.io app config) — `fly.toml`
- [x] Health check endpoint (`/health`) — `src/server/health.rs`
- [x] CD pipeline (GH Action: CI pass -> deploy) — `.github/workflows/deploy.yml`
- [ ] First deploy (live on *.fly.dev) — **BLOCKED: needs Fly.io account**
- [ ] Fly Volume (SQLite persistence) — **BLOCKED: needs Fly.io account**
- [x] robots.txt — `public/robots.txt`
- [x] sitemap.xml — `public/sitemap.xml`

### L1 Integration
- [ ] Custom domain + HTTPS
- [ ] SEO meta tags (leptos_meta) — needs DESIGN
- [ ] JSON-LD structured data (Person schema)
- [ ] Cache headers (Axum middleware)
- [ ] Error pages (404, 500) — needs DESIGN
- [ ] WASM compression verification

### L2 Enhancement
- [ ] GoatCounter analytics
- [ ] Lighthouse CI (GH Action)
- [ ] wasm-opt in Dockerfile
- [ ] Social preview images — needs DESIGN
- [ ] Environment config separation
- [ ] E2E smoke tests — needs DESIGN + DATA

### L3 Perfection
- [ ] CDN optimization
- [ ] Uptime monitoring
- [ ] Error monitoring / structured logging
- [ ] Production smoke tests (post-deploy)
- [ ] Performance dashboard (FCP/LCP trends)
- [ ] Security headers (CSP, X-Frame-Options, etc.)

## Cross-Stream Messages Sent

- To DESIGN: SEO meta tag coordination, OG image request, error page request
- To DATA: Seed data idempotency (INSERT OR IGNORE) — DATA confirmed processed

## Blockers

- **L0.5 + L0.6**: Need Fly.io account + `FLY_API_TOKEN` GitHub secret

## Human Actions Needed

1. Create Fly.io account
2. Install flyctl CLI
3. `fly apps create gabriel-osemberg`
4. `fly volumes create sqlite_data --region iad --size 1`
5. Add `FLY_API_TOKEN` to GitHub repository secrets
6. First deploy: `fly deploy` or push to main

## Quality Gates (Last Run)

- `cargo test --features=ssr`: PASS (15 tests)
- `cargo clippy --features=ssr -- -D warnings`: PASS
- `cargo clippy --features=hydrate --target=wasm32-unknown-unknown -- -D warnings`: PASS
- `cargo fmt --check`: PASS

## Metrics

- Research sections: 7/7
- Source files created: 4 (health.rs, health_endpoint.rs, robots.txt, sitemap.xml)
- Config files created: 4 (Dockerfile, .dockerignore, fly.toml, deploy.yml)
- Tests added: 2
- L0 tasks: 6/8 complete
