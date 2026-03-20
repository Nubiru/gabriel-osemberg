# INFRA — Active Task

**Task**: L0.1 + L0.2 + L0.4 — Dockerfile, fly.toml, CD pipeline
**Phase**: BUILD (L0 Foundation)
**Started**: 2026-03-20
**Status**: IN-PROGRESS

## Previous Task (COMPLETE)
L0.3 + L0.7 + L0.8 — Health endpoint, robots.txt, sitemap.xml
- All tests pass (22 total, 2 new)
- Clippy clean (SSR + hydrate)
- Formatting clean

## Current Scope

1. **L0.1 — Dockerfile**: Multi-stage Debian build with cargo-leptos, wasm32 target, slim runtime
2. **L0.2 — fly.toml**: Fly.io app config with statics, health check, min 1 machine
3. **L0.4 — CD pipeline**: GitHub Action deploying to Fly.io on push to main

## Deliverables

- `Dockerfile`
- `fly.toml`
- `.github/workflows/deploy.yml`
- `.dockerignore`
