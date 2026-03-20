//! Integration tests for the health check endpoint.
//!
//! Run with: cargo test --no-default-features --features=ssr

#![cfg(feature = "ssr")]

use sqlx::SqlitePool;

use gabriel_osemberg::server::health::check_health;

/// Create an in-memory database with migrations applied.
async fn test_pool() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

#[tokio::test]
async fn health_check_succeeds_with_valid_pool() {
    let pool = test_pool().await;

    let result = check_health(&pool).await;
    assert!(
        result.is_ok(),
        "Health check should succeed with valid pool"
    );

    let response = result.expect("checked above");
    assert_eq!(response.status, "ok");
}

#[tokio::test]
async fn health_check_fails_with_closed_pool() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create pool");
    pool.close().await;

    let result = check_health(&pool).await;
    assert!(result.is_err(), "Health check should fail with closed pool");
}
