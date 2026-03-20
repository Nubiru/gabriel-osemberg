//! HTTP middleware for cache control and security headers.
//!
//! - Cache headers: path-based cache-control (immutable for hashed assets, no-cache for HTML)
//! - Security headers: standard hardening headers on all responses

use axum::{
    extract::Request,
    http::{header, HeaderValue},
    middleware::Next,
    response::Response,
};

/// Determine the appropriate Cache-Control value for a request path.
///
/// - `/pkg/*` — hashed assets (WASM, JS, CSS): immutable, 1 year
/// - `/fonts/*` — font files: 1 year (rarely change)
/// - `/favicon.ico` — 1 day
/// - Everything else (HTML, API) — no-cache
pub fn cache_control_for_path(path: &str) -> &'static str {
    if path.starts_with("/pkg/") {
        "public, max-age=31536000, immutable"
    } else if path.starts_with("/fonts/") {
        "public, max-age=31536000"
    } else if path == "/favicon.ico" {
        "public, max-age=86400"
    } else {
        "no-cache"
    }
}

/// Axum middleware that sets Cache-Control headers based on request path.
pub async fn cache_headers(request: Request, next: Next) -> Response {
    let cache_value = cache_control_for_path(request.uri().path());
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static(cache_value));
    response
}

/// Axum middleware that adds security hardening headers to all responses.
///
/// Headers added:
/// - `X-Content-Type-Options: nosniff` — prevent MIME sniffing
/// - `X-Frame-Options: DENY` — prevent clickjacking
/// - `Referrer-Policy: strict-origin-when-cross-origin` — limit referrer leakage
/// - `Permissions-Policy` — disable unnecessary browser APIs
pub async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    headers.insert(
        "referrer-policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        "permissions-policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_control_hashed_assets() {
        assert_eq!(
            cache_control_for_path("/pkg/gabriel-osemberg-abc123.wasm"),
            "public, max-age=31536000, immutable"
        );
        assert_eq!(
            cache_control_for_path("/pkg/gabriel-osemberg.js"),
            "public, max-age=31536000, immutable"
        );
        assert_eq!(
            cache_control_for_path("/pkg/gabriel-osemberg.css"),
            "public, max-age=31536000, immutable"
        );
    }

    #[test]
    fn cache_control_fonts() {
        assert_eq!(
            cache_control_for_path("/fonts/inter-latin-variable.woff2"),
            "public, max-age=31536000"
        );
    }

    #[test]
    fn cache_control_favicon() {
        assert_eq!(
            cache_control_for_path("/favicon.ico"),
            "public, max-age=86400"
        );
    }

    #[test]
    fn cache_control_html_pages() {
        assert_eq!(cache_control_for_path("/"), "no-cache");
        assert_eq!(cache_control_for_path("/projects"), "no-cache");
        assert_eq!(cache_control_for_path("/about"), "no-cache");
        assert_eq!(cache_control_for_path("/projects/time"), "no-cache");
    }

    #[test]
    fn cache_control_health_endpoint() {
        assert_eq!(cache_control_for_path("/health"), "no-cache");
    }
}
