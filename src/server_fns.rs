//! Server functions for data access — visible to both client and server.
//!
//! The `#[server]` macro generates client-side stubs (HTTP calls) and
//! server-side implementations. These must NOT be behind `#[cfg(feature = "ssr")]`.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{CvSection, Experience, Project, ProjectMetric, Skill, TechTag};

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

/// Fetch all metrics for a given project.
#[server]
pub async fn get_project_metrics(project_id: i64) -> Result<Vec<ProjectMetric>, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let metrics = sqlx::query_as::<_, ProjectMetric>(
        "SELECT id, project_id, metric_name, metric_value, metric_unit, source, measured_at \
         FROM project_metrics WHERE project_id = ?1 ORDER BY metric_name ASC",
    )
    .bind(project_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch project metrics: {e}")))?;

    Ok(metrics)
}

/// Fetch all tech tags for a given project.
#[server]
pub async fn get_project_tags(project_id: i64) -> Result<Vec<TechTag>, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let tags = sqlx::query_as::<_, TechTag>(
        "SELECT t.id, t.name, t.category \
         FROM tech_tags t \
         INNER JOIN project_tags pt ON pt.tag_id = t.id \
         WHERE pt.project_id = ?1 \
         ORDER BY t.category ASC, t.name ASC",
    )
    .bind(project_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch project tags: {e}")))?;

    Ok(tags)
}

/// Fetch all projects that have a given tech tag.
#[server]
pub async fn get_projects_by_tag(tag_name: String) -> Result<Vec<Project>, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let projects = sqlx::query_as::<_, Project>(
        "SELECT p.id, p.name, p.slug, p.tagline, p.description, p.tech_stack, \
         p.repo_url, p.live_url, p.image_path, p.sort_order, p.visible, \
         p.created_at, p.updated_at \
         FROM projects p \
         INNER JOIN project_tags pt ON pt.project_id = p.id \
         INNER JOIN tech_tags t ON t.id = pt.tag_id \
         WHERE t.name = ?1 AND p.visible = 1 \
         ORDER BY p.sort_order ASC",
    )
    .bind(&tag_name)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch projects by tag: {e}")))?;

    Ok(projects)
}

/// Fetch aggregated stats across all visible projects.
#[server]
pub async fn get_aggregated_stats() -> Result<AggregatedStats, ServerFnError> {
    let pool = use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let total_projects =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects WHERE visible = 1")
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to count projects: {e}")))?;

    let total_loc = sqlx::query_scalar::<_, f64>(
        "SELECT COALESCE(SUM(metric_value), 0) FROM project_metrics WHERE metric_name = 'loc'",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(0.0);

    let total_tests = sqlx::query_scalar::<_, f64>(
        "SELECT COALESCE(SUM(metric_value), 0) FROM project_metrics WHERE metric_name = 'test_count'",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(0.0);

    let languages_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(DISTINCT name) FROM tech_tags WHERE category = 'language'",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    Ok(AggregatedStats {
        total_projects,
        total_loc,
        total_tests,
        languages_count,
    })
}

/// Aggregated statistics across all showcase projects.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregatedStats {
    pub total_projects: i64,
    pub total_loc: f64,
    pub total_tests: f64,
    pub languages_count: i64,
}
