# 004 — Questions: Infrastructure

**Stream**: INFRA
**Date**: 2026-03-20
**Status**: COMPLETE

---

## Questions from tracks.md (all answered)

### Q1: Fly.io vs Shuttle.rs vs Railway for Leptos deployment?

**Answer: Fly.io**

| Platform | Leptos SSR Support | Pros | Cons |
|----------|-------------------|------|------|
| **Fly.io** | Full (official Leptos site runs here) | Best Rust story, edge CDN, `[[statics]]` for assets, auto-HTTPS, generous free tier | Requires Dockerfile, machines auto-stop (solvable with `min_machines_running = 1`) |
| **Shuttle.rs** | Broken | Rust-native, no Docker needed | SSR feature flag incompatibility, unmaintained community repo, blocked |
| **Railway** | Works via Docker | Free tier, no credit card, simple setup | Less Rust tooling, no edge CDN, community template only |

**Decision**: Fly.io for production. Shuttle.rs eliminated. Railway optional for staging.

**Confidence**: HIGH — the Leptos official website runs on Fly.io, which is the strongest possible endorsement.

---

### Q2: How to optimize WASM bundle size? wasm-opt, tree-shaking, code splitting?

**Answer: Layered approach, most already implemented**

| Layer | Status | Expected Savings |
|-------|--------|-----------------|
| `opt-level = 'z'` | DONE (Cargo.toml) | Baseline size optimization |
| `lto = true` | DONE | 10-25% reduction |
| `codegen-units = 1` | DONE | Better optimization |
| `panic = "abort"` | DONE | Smaller panic handling |
| **wasm-opt** (binaryen) | NOT DONE — add to Dockerfile | 10-20% additional |
| **Compression** (gzip/brotli) | NOT DONE — Fly.io handles this | 50%+ reduction |
| **Code splitting** (`#[lazy]`, `#[lazy_route]`) | NOT NEEDED YET | Significant for large apps |
| **Rebuild std with panic_immediate_abort** | NOT RECOMMENDED | Requires nightly, fragile |
| **Lighter serde** (miniserde/serde-lite) | EVALUATE LATER | Varies by usage |

**Action items**:
1. Add `wasm-opt -Oz` to Dockerfile after cargo-leptos build
2. Ensure Fly.io serves compressed responses (default behavior)
3. Monitor bundle size in CI (already done)
4. Consider code splitting only if bundle exceeds 200KB gzipped

**Confidence**: HIGH — current optimizations are the canonical set. wasm-opt is the main missing piece.

---

### Q3: Which analytics tool? Plausible vs Umami vs GoatCounter?

**Answer: GoatCounter for launch, evaluate Plausible later**

| Criterion | Plausible | Umami | GoatCounter |
|-----------|-----------|-------|-------------|
| **Cost** | $9/mo cloud | Free self-hosted | Free (non-commercial) |
| **Infrastructure** | Postgres + ClickHouse (heavy) | Postgres (medium) | Single binary (minimal) |
| **Script size** | ~1.5 KB | ~2 KB | ~3 KB |
| **Setup effort** | Medium (Docker Compose) | Medium (Docker) | Minimal (script tag) |
| **Dashboard** | Beautiful, professional | Customizable | Simple, functional |

**Rationale**: GoatCounter is the right choice for a portfolio site because:
1. **Zero infrastructure**: No additional servers, databases, or Docker containers
2. **Free**: Non-commercial personal portfolio qualifies
3. **Privacy-first**: No cookies, GDPR-compliant, no consent banner needed
4. **Minimal impact**: ~3 KB script, no performance penalty
5. **Sufficient**: Page views, referrers, browser stats — all a portfolio needs

Upgrade path: If Gabriel gets hired and wants to track traffic more seriously, switch to Plausible Cloud ($9/mo) — it's a script tag swap.

**Confidence**: HIGH — GoatCounter is the pragmatic choice for the constraints.

---

### Q4: How to set up Lighthouse CI in GitHub Actions for a Leptos project?

**Answer: treosh/lighthouse-ci-action with local server**

**Strategy**: Build the release binary in CI, start it in background, run Lighthouse against localhost, fail on score thresholds.

**Implementation sketch**:

```yaml
lighthouse:
  runs-on: ubuntu-latest
  needs: build  # Reuse build artifacts
  steps:
    - uses: actions/checkout@v4
    - name: Download build artifacts
      uses: actions/download-artifact@v4
      with:
        name: release-binary
    - name: Start server
      run: |
        chmod +x ./gabriel-osemberg
        DATABASE_URL="sqlite:gabriel_osemberg.db?mode=rwc" \
        LEPTOS_SITE_ADDR="0.0.0.0:8080" \
        LEPTOS_SITE_ROOT="site" \
        ./gabriel-osemberg &
        sleep 3  # Wait for server startup
    - name: Lighthouse CI
      uses: treosh/lighthouse-ci-action@v12
      with:
        urls: |
          http://localhost:8080/
        budgetPath: ./budget.json
        uploadArtifacts: true
```

**Challenges**:
- Need to upload the compiled binary + site directory as CI artifacts from build job
- SQLite database must exist with migrations run
- Server needs a few seconds to start (startup migrations)
- Lighthouse needs a real HTTP server, not just static files

**Thresholds** (from CLAUDE.md): >= 90 all categories (Performance, Accessibility, Best Practices, SEO).

**Confidence**: MEDIUM — the pattern is established but Leptos-specific integration needs testing. The main risk is CI artifact management between jobs.

---

### Q5: What's the right caching strategy for a WASM + SSR hybrid?

**Answer: Hash-based immutable caching for assets, no-cache for HTML**

**Principle**: cargo-leptos generates hashed filenames for WASM/JS/CSS (e.g., `gabriel-osemberg-abc123.wasm`). When the code changes, the hash changes, so the filename changes. This means:

| Asset Type | Filename Changes? | Cache Strategy |
|------------|------------------|----------------|
| `/pkg/*.wasm` | Yes (hashed) | `max-age=31536000, immutable` (1 year) |
| `/pkg/*.js` | Yes (hashed) | `max-age=31536000, immutable` |
| `/pkg/*.css` | Yes (hashed) | `max-age=31536000, immutable` |
| `/fonts/*.woff2` | No | `max-age=31536000` (fonts rarely change) |
| `/favicon.ico` | No | `max-age=86400` (1 day) |
| HTML (SSR pages) | N/A (dynamic) | `no-cache` or `max-age=60` |
| API responses | N/A (dynamic) | `no-store` |

**Implementation layers**:
1. **Fly.io `[[statics]]`**: Serves `/pkg/` directly from edge CDN with automatic caching
2. **Axum middleware**: Set cache headers for SSR responses and non-static assets
3. **Leptos/cargo-leptos**: Handles hash generation automatically

**Key insight**: The SSR HTML must NOT be aggressively cached because it contains `<script>` and `<link>` tags referencing the hashed asset filenames. If the HTML is cached but assets change, users get broken references.

**Confidence**: HIGH — this is a well-understood pattern used by every SSR framework.

---

## Additional Questions Discovered During Research

### Q6: Should we use Rust stable or nightly for the Dockerfile?

**Answer**: The Leptos book examples use nightly, but the project uses stable (per CLAUDE.md). Check if `cargo-leptos build --release` works on stable. If it does, use stable in the Dockerfile for reliability. If nightly is required (some Leptos features need it), document the reason.

**Action**: Test `cargo leptos build --release` on stable before writing the Dockerfile.

**Confidence**: MEDIUM — needs verification.

### Q7: How should the SQLite database be handled in production?

**Answer**: SQLite with `mode=rwc` creates the database file if it doesn't exist. Migrations run at startup via `sqlx::migrate!()`. For Fly.io:
- Use a **Fly Volume** to persist the SQLite database across deploys
- Mount the volume at `/data/` and set `DATABASE_URL=sqlite:/data/gabriel_osemberg.db?mode=rwc`
- Without a volume, the database is recreated on every deploy (acceptable for a portfolio with seeded data, but not ideal)

**Confidence**: HIGH — standard Fly.io + SQLite pattern.
