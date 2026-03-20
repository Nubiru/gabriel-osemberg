//! Health check endpoint for deployment monitoring.
//!
//! Returns 200 OK with `{"status":"ok"}` when the server and database
//! are operational. Used by Fly.io machine health checks.

use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;
use sqlx::SqlitePool;

/// Health check response body.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// Core health check logic — verifies database connectivity.
///
/// Separated from the Axum handler for testability.
pub async fn check_health(pool: &SqlitePool) -> Result<HealthResponse, sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(HealthResponse { status: "ok" })
}

/// Axum handler for `GET /health`.
pub async fn health_handler(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<HealthResponse>, StatusCode> {
    check_health(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
}
