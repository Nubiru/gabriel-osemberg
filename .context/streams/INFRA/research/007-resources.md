# 007 — Resources: Infrastructure

**Stream**: INFRA
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Reference Materials

### Official Documentation

| Resource | URL | Used For |
|----------|-----|----------|
| Leptos Deployment Book | https://book.leptos.dev/deployment/ssr.html | Dockerfile, deployment patterns |
| Leptos Binary Size Optimization | https://book.leptos.dev/deployment/binary_size.html | WASM size reduction techniques |
| Fly.io Rust Docs | https://fly.io/docs/rust/ | Fly.io deployment, fly.toml config |
| Fly.io App Configuration | https://fly.io/docs/reference/configuration/ | fly.toml reference |
| Fly.io Volumes | https://fly.io/docs/volumes/ | SQLite persistence |
| cargo-leptos GitHub | https://github.com/leptos-rs/cargo-leptos | Build tool reference |
| Lighthouse CI | https://github.com/GoogleChrome/lighthouse-ci | CI performance auditing |
| treosh/lighthouse-ci-action | https://github.com/treosh/lighthouse-ci-action | GitHub Actions integration |
| GoatCounter | https://www.goatcounter.com/ | Privacy-first analytics |

### Community Examples

| Resource | URL | Used For |
|----------|-----|----------|
| Leptos Official Site (Fly.io) | https://leptos-website.fly.dev/ | Reference deployment |
| leptos-community/deployment-ssr-shuttle | https://github.com/leptos-community/deployment-ssr-shuttle | (Eliminated — Shuttle not viable) |
| fly-apps/hello-rust | https://github.com/fly-apps/hello-rust | Fly.io Rust starter |

### Articles & Guides

| Resource | Used For |
|----------|----------|
| Privacy Analytics Comparison (aaronjbecker.com) | Plausible vs Umami vs GoatCounter |
| Leptos Complete Guide 2026 (reintech.io) | General Leptos patterns |
| Building Portfolio with Rust and Leptos (Medium) | Real-world Leptos portfolio experience |

---

## Tools Required

| Tool | Version | Purpose | Status |
|------|---------|---------|--------|
| `flyctl` | Latest | Fly.io CLI for deployment | NOT INSTALLED |
| `cargo-leptos` | Latest | Build tool (already in use) | INSTALLED |
| `binaryen` (wasm-opt) | Latest | WASM binary optimization | NOT INSTALLED (add to Dockerfile) |
| `docker` / `podman` | Latest | Container builds | INSTALLED (system) |
| Playwright | Latest | E2E testing | CONFIGURED (not yet used) |

---

## Accounts & Services Required

| Service | Purpose | Status | Cost |
|---------|---------|--------|------|
| **Fly.io** | Production hosting | NEEDS ACCOUNT | Free tier (3 shared VMs) |
| **GoatCounter** | Analytics | NEEDS ACCOUNT | Free (non-commercial) |
| **Domain registrar** | Custom domain | NEEDS PURCHASE | ~$10-15/year |
| **GitHub** | CI/CD (already in use) | ACTIVE (Nubiru) | Free |

---

## Knowledge Gaps

| Gap | Impact | How to Fill |
|-----|--------|-------------|
| Fly.io Volumes + SQLite best practices | MEDIUM — affects data persistence | Read Fly.io volumes docs, test locally with Docker |
| Axum cache header middleware | LOW — straightforward tower middleware | Read tower-http docs, look at examples |
| leptos_meta SSR rendering behavior | LOW — need to verify meta tags render in SSR HTML | Test with `curl` against running server |
| Nightly vs stable for Dockerfile | MEDIUM — affects reliability | Test `cargo leptos build --release` on stable |

---

## Resource Readiness

All critical resources are identified and accessible. No paywalled or unavailable resources block L0 progress. The main action items are:

1. **Install flyctl** on development machine
2. **Create Fly.io account** (requires Gabriel's action)
3. **Test Docker build locally** before pushing to Fly.io
4. **Register domain** (requires Gabriel's decision on domain name)

Items 2 and 4 require human action. Items 1 and 3 can be done by the stream.
