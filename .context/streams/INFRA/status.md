# Stream: INFRA
**Phase**: 1 — ROADMAP
**Owner**: INFRA stream
**Last Updated**: 2026-03-20
**Readiness**: GREEN

## Current Work

Research phase COMPLETE (7/7 sections). Phase advanced to ROADMAP. Layered roadmap (L0-L3) designed in 005-roadmap.md. Ready to refine and advance to BUILD.

## Key Decisions from Research

- **Deployment**: Fly.io (Shuttle.rs eliminated, Railway optional backup)
- **Analytics**: GoatCounter (free, privacy-first, zero infrastructure)
- **WASM optimization**: Add wasm-opt to Dockerfile (10-20% additional savings)
- **Caching**: Hash-based immutable for WASM/JS/CSS, no-cache for HTML
- **Lighthouse CI**: treosh/lighthouse-ci-action, fail on <90

## Research Progress

| # | Section | Status |
|---|---------|--------|
| 1 | 001-inventory.md | COMPLETE |
| 2 | 002-world-survey.md | COMPLETE |
| 3 | 003-gap-analysis.md | COMPLETE |
| 4 | 004-questions.md | COMPLETE |
| 5 | 005-roadmap.md | COMPLETE |
| 6 | 006-dependencies.md | COMPLETE |
| 7 | 007-resources.md | COMPLETE |

## Build Progress

### L0 Foundation
- [ ] Dockerfile (multi-stage Debian build)
- [ ] fly.toml (Fly.io app config)
- [ ] Health check endpoint (`/health`)
- [ ] CD pipeline (GH Action: push -> build -> deploy)
- [ ] First deploy (live on *.fly.dev)
- [ ] Fly Volume (SQLite persistence)
- [ ] robots.txt
- [ ] sitemap.xml

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
- To DATA: Seed data idempotency (INSERT OR IGNORE)

## Blockers

None. L0 is entirely self-contained.

## Human Actions Needed (before BUILD phase)

- [ ] Create Fly.io account
- [ ] Decide on domain name
- [ ] Install flyctl CLI

## Metrics

- Research sections: 7/7
- Source files: 0 (research phase, no code yet)
- Commits: 0
