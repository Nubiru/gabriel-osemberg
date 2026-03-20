//! Server-side modules for database access and API integration.

#[cfg(feature = "ssr")]
pub mod error;

#[cfg(feature = "ssr")]
pub mod health;

#[cfg(feature = "ssr")]
pub mod middleware;
