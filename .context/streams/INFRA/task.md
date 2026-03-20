# INFRA — Active Task

**Task**: L1.4 + L3.6 — Cache headers + Security headers (Axum middleware)
**Phase**: BUILD (L1 Integration + L3 Perfection)
**Started**: 2026-03-20
**Status**: IN-PROGRESS

## Scope

1. **L1.4 — Cache headers**: Axum middleware setting cache-control headers:
   - Immutable (1 year) for hashed assets in /pkg/ (WASM, JS, CSS)
   - Long-lived (1 year) for fonts
   - Short (1 day) for favicon
   - No-cache for HTML (SSR pages)

2. **L3.6 — Security headers**: Axum middleware adding:
   - X-Content-Type-Options: nosniff
   - X-Frame-Options: DENY
   - Referrer-Policy: strict-origin-when-cross-origin
   - Permissions-Policy: camera=(), microphone=(), geolocation=()

## Approach

Single middleware module `src/server/middleware.rs` with both concerns.
Add as tower layer in main.rs.
