//! Server functions for data access — visible to both client and server.
//!
//! The `#[server]` macro generates client-side stubs (HTTP calls) and
//! server-side implementations. These must NOT be behind `#[cfg(feature = "ssr")]`.

use leptos::prelude::*;

use crate::models::{CvSection, Experience, Project, Skill};

/// Fetch all visible projects, ordered by sort_order.
#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let projects = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch projects: {e}")))?;

    Ok(projects)
}

/// Fetch a single project by its URL slug.
#[server]
pub async fn get_project_by_slug(slug: String) -> Result<Project, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let project = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE slug = ?1 AND visible = 1",
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch project: {e}")))?
    .ok_or_else(|| ServerFnError::new(format!("Project not found: {slug}")))?;

    Ok(project)
}

/// Fetch all visible experiences, ordered by sort_order.
#[server]
pub async fn get_experiences() -> Result<Vec<Experience>, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let experiences = sqlx::query_as::<_, Experience>(
        "SELECT id, role, company, company_url, start_date, end_date, \
         description, highlights, sort_order, visible \
         FROM experiences WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch experiences: {e}")))?;

    Ok(experiences)
}

/// Fetch all visible skills, ordered by category then proficiency (descending).
#[server]
pub async fn get_skills() -> Result<Vec<Skill>, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let skills = sqlx::query_as::<_, Skill>(
        "SELECT id, name, category, proficiency, visible \
         FROM skills WHERE visible = 1 ORDER BY category ASC, proficiency DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch skills: {e}")))?;

    Ok(skills)
}

/// Fetch all visible CV sections, ordered by sort_order.
#[server]
pub async fn get_cv_sections() -> Result<Vec<CvSection>, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let sections = sqlx::query_as::<_, CvSection>(
        "SELECT id, section_type, title, content, sort_order, visible \
         FROM cv_sections WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch CV sections: {e}")))?;

    Ok(sections)
}
