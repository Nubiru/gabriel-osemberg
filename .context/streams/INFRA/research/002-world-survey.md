# 002 — World Survey: Infrastructure Best Practices

**Stream**: INFRA
**Date**: 2026-03-20
**Status**: COMPLETE

---

## 1. Deployment Platforms for Leptos SSR

### Fly.io — RECOMMENDED

**Why**: Best Rust story, edge deployment, the Leptos official website (leptos-website.fly.dev) runs on Fly.io.

**Key facts**:
- `fly launch` generates fly.toml + Dockerfile from project
- Set `min_machines_running = 1` to avoid cold starts (Fly auto-stops idle machines)
- `[[statics]]` section serves static assets directly from CDN edge (bypassing app)
- Port: Leptos must bind `0.0.0.0:8080` (not localhost)
- Free tier: 3 shared VMs, 160GB outbound, 3GB persistent storage
- Global edge network: deploy close to users with `fly regions add`

**fly.toml skeleton**:
```toml
app = "gabriel-osemberg"
primary_region = "iad"  # US East (or relevant region)

[build]

[env]
  LEPTOS_SITE_ADDR = "0.0.0.0:8080"
  LEPTOS_SITE_ROOT = "site"
  RUST_LOG = "info"

[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = "stop"
  auto_start_machines = true
  min_machines_running = 1

[[statics]]
  guest_path = "/app/site"
  url_prefix = "/pkg"
```

### Shuttle.rs — NOT VIABLE

Shuttle.rs does NOT officially support Leptos SSR. Known issues with SSR feature flag compatibility. Community repo exists but is unmaintained. **Skip**.

### Railway — VIABLE BACKUP

- Free tier (no credit card), Docker-based deployment
- Less Rust-specific tooling than Fly.io
- Community template available with Renovate + GitHub Actions
- Good for staging environments

**Recommendation**: Fly.io for production, Railway for staging (optional).

---

## 2. Dockerfile — Multi-Stage Build

The Leptos book provides the canonical Dockerfile pattern. Two variants:

### Debian-based (recommended for stability)

```dockerfile
# Stage 1: Build
FROM rustlang/rust:nightly-bookworm AS builder
RUN cargo install cargo-binstall
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
COPY . .
RUN cargo leptos build --release -vv

# Stage 2: Runtime
FROM debian:trixie-slim
COPY --from=builder /app/target/release/gabriel-osemberg /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/migrations /app/migrations
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD ["/app/gabriel-osemberg"]
```

### Alpine-based (smaller image, ~50% size reduction)

Uses `rustlang/rust:nightly-alpine` + `apk add bash curl npm libc-dev binaryen`. Smaller runtime but may have musl compatibility issues.

**Key considerations**:
- Always build in `--release` mode (huge performance/size difference)
- Copy `migrations/` directory for SQLx runtime migrations
- Set `DATABASE_URL` at runtime (not build time) for SQLite
- WASM assets are embedded in `target/site/pkg/`

---

## 3. WASM Binary Size Optimization

### Already Implemented (in Cargo.toml)

| Setting | Value | Effect |
|---------|-------|--------|
| `opt-level` | `'z'` | Optimize for minimum size |
| `lto` | `true` | Link-time optimization |
| `codegen-units` | `1` | Better optimization (slower compile) |
| `panic` | `"abort"` | Smaller panic handling |

### Additional Techniques (not yet implemented)

| Technique | Savings | Complexity |
|-----------|---------|------------|
| **wasm-opt** (binaryen) | 10-20% additional | Low — add to CI/Dockerfile |
| **Rebuild std with panic_immediate_abort** (nightly) | 15-30% | Medium — requires nightly Rust |
| **Lighter serde** (`miniserde` or `serde-lite`) | Varies | Medium — framework integration |
| **Avoid heavy crates** (e.g., `regex` adds ~500KB) | Depends | Ongoing vigilance |
| **Code splitting** (`#[lazy]` + `cargo leptos --split`) | Significant for large apps | Medium — requires Leptos 0.8+ |
| **Compression** (gzip/brotli) | 50%+ reduction | Low — reverse proxy config |

**Key insight**: WASM compresses exceptionally well. A 400KB raw WASM file typically compresses to <200KB gzipped. Always measure gzipped size for budget compliance.

---

## 4. SEO Infrastructure

### robots.txt

```
User-agent: *
Allow: /

Sitemap: https://gabriel-osemberg.dev/sitemap.xml
```

Simple — allow everything, point to sitemap. No pages need blocking on a portfolio.

### sitemap.xml

For a portfolio site with few pages, a static sitemap suffices:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://gabriel-osemberg.dev/</loc><priority>1.0</priority></url>
  <url><loc>https://gabriel-osemberg.dev/projects</loc><priority>0.8</priority></url>
  <url><loc>https://gabriel-osemberg.dev/about</loc><priority>0.7</priority></url>
  <url><loc>https://gabriel-osemberg.dev/cv</loc><priority>0.7</priority></url>
</urlset>
```

Could be generated dynamically from the database (project slugs as URLs), but static is fine for launch.

### Open Graph / Meta Tags

Leptos provides `leptos_meta` for SSR-rendered meta tags. Critical tags:

| Tag | Purpose |
|-----|---------|
| `<Title>` | Page title (Leptos component) |
| `og:title`, `og:description`, `og:image` | Social sharing cards |
| `twitter:card`, `twitter:title`, `twitter:image` | Twitter/X cards |
| `<meta name="description">` | Search result snippet |
| JSON-LD structured data | Rich search results (Person schema) |

**Key insight**: SSR is critical for SEO. Leptos SSR renders full HTML on the server, so search engine crawlers see real content (unlike CSR-only WASM apps).

---

## 5. Privacy-First Analytics

### Comparison

| Feature | Plausible | Umami | GoatCounter |
|---------|-----------|-------|-------------|
| **Script size** | ~1.5 KB | ~2 KB | ~3 KB |
| **Cookies** | None | None | None |
| **Self-hosted** | Docker Compose (PostgreSQL + ClickHouse, 2GB+ RAM) | Docker (PostgreSQL, lighter) | Single Go binary (minimal) |
| **Cloud option** | Yes ($9/mo) | Yes (free tier) | Yes (free for non-commercial) |
| **Dashboard** | Single-page, elegant | Single-page, customizable | Simple, functional |
| **GDPR** | Compliant by design | Compliant by design | Compliant by design |
| **Best for** | Professional look, ease of use | Customization, self-hosting | Minimalism, zero maintenance |

### Recommendation

**GoatCounter** for MVP (zero infrastructure, free for non-commercial, single `<script>` tag). Upgrade to **Plausible Cloud** ($9/mo) or **Umami self-hosted** when traffic justifies it.

For a portfolio site expecting modest traffic, GoatCounter is the pragmatic choice — it adds almost no weight and requires no additional infrastructure.

---

## 6. Lighthouse CI

### GitHub Actions Integration

```yaml
- name: Lighthouse CI
  uses: treosh/lighthouse-ci-action@v12
  with:
    urls: |
      http://localhost:8080/
      http://localhost:8080/projects
    budgetPath: ./budget.json
    uploadArtifacts: true
```

### Performance Budget (`budget.json`)

```json
[{
  "resourceSizes": [
    { "resourceType": "script", "budget": 200 },
    { "resourceType": "document", "budget": 50 },
    { "resourceType": "total", "budget": 500 }
  ],
  "resourceCounts": [
    { "resourceType": "third-party", "budget": 3 }
  ]
}]
```

### Integration Pattern

1. Build release in CI
2. Start server in background
3. Run Lighthouse against localhost
4. Fail pipeline if scores drop below thresholds (90+ all categories per CLAUDE.md)
5. Upload report as artifact for review

---

## 7. Caching Strategy for SSR + WASM Hybrid

### Asset Types and Cache Policies

| Asset Type | Cache Strategy | Header |
|------------|---------------|--------|
| WASM bundle (`/pkg/*.wasm`) | Immutable (hash in filename) | `Cache-Control: public, max-age=31536000, immutable` |
| JS bundle (`/pkg/*.js`) | Immutable (hash in filename) | `Cache-Control: public, max-age=31536000, immutable` |
| CSS (`/pkg/*.css`) | Immutable (hash in filename) | `Cache-Control: public, max-age=31536000, immutable` |
| Fonts (`/fonts/*.woff2`) | Long-lived | `Cache-Control: public, max-age=31536000` |
| Images (`/assets/*`) | Long-lived | `Cache-Control: public, max-age=604800` |
| HTML pages (SSR) | Short/no cache | `Cache-Control: no-cache` or `max-age=60` |
| favicon.ico | Medium | `Cache-Control: public, max-age=86400` |

**Key insight**: cargo-leptos generates hashed filenames for WASM/JS/CSS, enabling aggressive immutable caching. HTML must not be cached aggressively since it references these hashed assets.

### Implementation

Cache headers can be set in:
1. **Fly.io's `[[statics]]`** — for assets served from edge CDN
2. **Axum middleware** — for SSR responses and dynamic assets
3. **Reverse proxy** (if using one) — Caddy/Nginx layer

---

## 8. Best-in-Class Portfolio Infrastructure Examples

### 1. Leptos Official Site (leptos-website.fly.dev)

- Deployed on Fly.io
- Leptos SSR with hydration
- The canonical example of Leptos deployment

### 2. Deno Fresh Portfolios

- Edge-deployed, instant cold starts
- SSR with island architecture (similar to Leptos islands)
- Good reference for SSR + selective hydration patterns

### 3. Astro-based Developer Portfolios

- Static generation with selective hydration
- Excellent Lighthouse scores (90+)
- Strong SEO with built-in sitemap generation
- Reference for what scores to target

### 4. Vercel/Next.js Portfolios (anti-pattern for this project)

- Excellent DX but wrong platform per CLAUDE.md constraints
- Reference for UX patterns only, not infrastructure

### 5. Rust Community Sites (e.g., areweweb yet)

- Deployed on various platforms
- Reference for Rust-specific CI/CD patterns

---

## Summary

The infrastructure landscape for Leptos is maturing but still requires manual configuration for production readiness. **Fly.io** is the clear deployment choice. The project already has strong build optimization but lacks deployment, SEO, analytics, caching, and monitoring layers. All of these are well-understood problems with established solutions.

**Sources**:
- [Leptos SSR Deployment Book](https://book.leptos.dev/deployment/ssr.html)
- [Leptos Binary Size Optimization](https://book.leptos.dev/deployment/binary_size.html)
- [Fly.io Rust Documentation](https://fly.io/docs/rust/)
- [Lighthouse CI GitHub](https://github.com/GoogleChrome/lighthouse-ci)
- [treosh/lighthouse-ci-action](https://github.com/treosh/lighthouse-ci-action)
- [Privacy Analytics Comparison](https://aaronjbecker.com/posts/umami-vs-plausible-vs-matomo-self-hosted-analytics/)
- [Fly.io App Configuration](https://fly.io/docs/reference/configuration/)
