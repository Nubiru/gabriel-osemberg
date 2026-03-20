# 001 — Inventory: Infrastructure Domain

**Stream**: INFRA
**Date**: 2026-03-20
**Status**: COMPLETE

---

## What Exists

### CI/CD — `.github/workflows/ci.yml`

GitHub Actions pipeline with 3 jobs:

| Job | What It Does |
|-----|-------------|
| **check** | `cargo fmt --check` + `cargo clippy -- -D warnings` on SSR and hydrate features |
| **test** | `cargo test` with SSR feature |
| **build** | `cargo leptos build --release`, installs Tailwind CSS globally, reports WASM bundle size (raw + gzipped) to GitHub summary |

- **Caching**: cargo registry, git objects, build targets keyed by `Cargo.lock` hash
- **Toolchain**: stable Rust, `wasm32-unknown-unknown` target, clippy + rustfmt components
- **Environment**: `CARGO_TERM_COLOR=always`, `RUSTFLAGS="-D warnings"`
- **E2E**: configured in Cargo.toml (`end2end-cmd = "npx playwright test"`) but no Playwright tests exist yet

### Build System — `Cargo.toml`

- **Crate types**: `cdylib` (WASM) + `rlib` (Rust library)
- **Feature gates**: `ssr` (server + DB) and `hydrate` (WASM client)
- **Dependencies**: leptos 0.8, leptos_router 0.8, leptos_meta 0.8, axum 0.8, sqlx 0.8 (SQLite), tokio 1
- **WASM profile** (`wasm-release`): `opt-level = 'z'`, `lto = true`, `codegen-units = 1`, `panic = "abort"`
- **Leptos config**: site at `target/site`, assets from `public/`, Tailwind input `style/tailwind.css`, server at `127.0.0.1:3000`

### Server — `src/main.rs` (62 lines)

- SSR-only entry point (Tokio + Axum)
- Loads `DATABASE_URL` from .env (fallback: `sqlite:gabriel_osemberg.db?mode=rwc`)
- SQLitePool with 5 max connections
- Runs `sqlx::migrate!()` at startup
- Leptos router integration via `leptos_axum`

### Database — `migrations/`

| File | Content |
|------|---------|
| `001_create_tables.sql` | 4 tables: projects, experiences, skills, cv_sections |
| `002_seed_showcase_data.sql` | 5 projects, 26 skills, 3 CV sections |

- SQLx compile-time verification configured
- In-memory SQLite used in integration tests

### Environment — `.env`

```
DATABASE_URL=sqlite:gabriel_osemberg.db?mode=rwc
```

- `.env` in `.gitignore` (not tracked)

### Styling Build — `style/tailwind.css` + `package.json`

- Tailwind CSS v4 (CSS-first, no JS config)
- `package.json` has `tailwindcss ^4.2.2` as devDependency
- CI installs `@tailwindcss/cli` globally

### Public Assets — `public/`

| File | Size |
|------|------|
| `favicon.ico` | 15.4 KB |
| `fonts/inter-latin-variable.woff2` | 47.1 KB |
| `fonts/jetbrains-mono-latin-variable.woff2` | 30.7 KB |
| `fonts/space-grotesk-latin-variable.woff2` | 21.8 KB |

Total font weight: ~99 KB (pre-gzip).

### Code Quality — `rustfmt.toml`

```toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
```

### Git — `.gitignore`

Excludes: target/, build*/, node_modules/, .env, .context/active/, .context/archive/, editor files, OS files.

---

## What Does NOT Exist

| Category | Status | Notes |
|----------|--------|-------|
| **Deployment config** | MISSING | No Dockerfile, fly.toml, shuttle.toml, or railway.json |
| **CD pipeline** | MISSING | No auto-deploy on push (CI builds but doesn't deploy) |
| **Staging environment** | MISSING | No staging config or environment separation |
| **Custom domain / HTTPS** | MISSING | No domain config |
| **SEO** | MISSING | No robots.txt, sitemap.xml, Open Graph tags, structured data |
| **Analytics** | MISSING | No Plausible, Umami, GoatCounter, or any tracking |
| **Error monitoring** | MISSING | No Sentry, DataDog, or error tracking |
| **Performance monitoring** | MISSING | CI reports WASM size but no runtime perf tracking |
| **Lighthouse CI** | MISSING | No automated Lighthouse score enforcement |
| **E2E tests** | MISSING | Playwright configured but no test files |
| **CDN** | MISSING | No CDN for static assets |
| **Cache headers** | MISSING | No cache strategy configured |
| **Uptime monitoring** | MISSING | No health checks or uptime services |
| **Social preview images** | MISSING | No OG images or Twitter card images |

---

## Summary

The INFRA domain has a **solid CI pipeline** and **optimized build configuration** but is otherwise undeveloped. The deployment pipeline, SEO, analytics, monitoring, and production readiness layers are entirely absent. The existing CI covers compilation correctness and WASM size reporting, which is a good foundation to build on.

**Axis served**: PRESENCE (infrastructure enables professional presentation) + CRAFT (build quality enforces engineering standards).
