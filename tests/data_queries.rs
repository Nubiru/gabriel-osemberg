//! Integration tests for DATA layer queries.
//!
//! Each test creates an in-memory SQLite database, runs migrations,
//! and verifies queries return correct results from seeded data.

use sqlx::SqlitePool;

use gabriel_osemberg::models::{CvSection, Experience, Project, Skill};

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
async fn get_all_projects_returns_seeded_data() {
    let pool = test_pool().await;

    let projects = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert_eq!(projects.len(), 5, "Should have 5 showcase projects");
    assert_eq!(projects[0].slug, "time");
    assert_eq!(projects[1].slug, "blocksight");
    assert_eq!(projects[2].slug, "anan-yarok");
    assert_eq!(projects[3].slug, "chamana");
    assert_eq!(projects[4].slug, "gabriel-osemberg");
}

#[tokio::test]
async fn get_project_by_slug_returns_correct_project() {
    let pool = test_pool().await;
    let slug = "time";

    let project = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE slug = ?1 AND visible = 1",
    )
    .bind(slug)
    .fetch_optional(&pool)
    .await
    .expect("query should succeed");

    let project = project.expect("project 'time' should exist");
    assert_eq!(project.name, "time");
    assert!(project.tech_stack.contains("C11"));
    assert!(project.tech_stack.contains("WebGL2"));
}

#[tokio::test]
async fn get_project_by_slug_returns_none_for_missing() {
    let pool = test_pool().await;

    let project = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE slug = ?1 AND visible = 1",
    )
    .bind("nonexistent")
    .fetch_optional(&pool)
    .await
    .expect("query should succeed");

    assert!(project.is_none());
}

#[tokio::test]
async fn get_all_skills_returns_seeded_data() {
    let pool = test_pool().await;

    let skills = sqlx::query_as::<_, Skill>(
        "SELECT id, name, category, proficiency, visible \
         FROM skills WHERE visible = 1 ORDER BY category ASC, proficiency DESC",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert!(skills.len() >= 20, "Should have 20+ seeded skills");

    // Verify categories exist
    let categories: Vec<&str> = skills.iter().map(|s| s.category.as_str()).collect();
    assert!(categories.contains(&"language"));
    assert!(categories.contains(&"framework"));
    assert!(categories.contains(&"tool"));
    assert!(categories.contains(&"concept"));

    // Verify ordering: within each category, proficiency descends
    let languages: Vec<i64> = skills
        .iter()
        .filter(|s| s.category == "language")
        .map(|s| s.proficiency)
        .collect();
    for window in languages.windows(2) {
        assert!(
            window[0] >= window[1],
            "Languages should be ordered by proficiency descending"
        );
    }
}

#[tokio::test]
async fn get_all_cv_sections_returns_seeded_data() {
    let pool = test_pool().await;

    let sections = sqlx::query_as::<_, CvSection>(
        "SELECT id, section_type, title, content, sort_order, visible \
         FROM cv_sections WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert_eq!(sections.len(), 3, "Should have 3 CV sections");
    assert_eq!(sections[0].section_type, "about");
    assert_eq!(sections[1].section_type, "methodology");
    assert_eq!(sections[2].section_type, "philosophy");
}

#[tokio::test]
async fn projects_have_unique_slugs() {
    let pool = test_pool().await;

    let projects = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    let mut slugs: Vec<&str> = projects.iter().map(|p| p.slug.as_str()).collect();
    let original_len = slugs.len();
    slugs.sort();
    slugs.dedup();
    assert_eq!(slugs.len(), original_len, "All slugs should be unique");
}

#[tokio::test]
async fn skills_have_unique_names() {
    let pool = test_pool().await;

    let skills =
        sqlx::query_as::<_, Skill>("SELECT id, name, category, proficiency, visible FROM skills")
            .fetch_all(&pool)
            .await
            .expect("query should succeed");

    let mut names: Vec<&str> = skills.iter().map(|s| s.name.as_str()).collect();
    let original_len = names.len();
    names.sort();
    names.dedup();
    assert_eq!(
        names.len(),
        original_len,
        "All skill names should be unique"
    );
}

#[tokio::test]
async fn experiences_table_exists_and_is_empty() {
    let pool = test_pool().await;

    // No experience entries are seeded (pending real CV content from Gabriel)
    let experiences = sqlx::query_as::<_, Experience>(
        "SELECT id, role, company, company_url, start_date, end_date, \
         description, highlights, sort_order, visible \
         FROM experiences WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    // Table exists and is queryable (may be empty until CV content is provided)
    assert!(
        experiences.is_empty(),
        "No experience entries seeded yet — pending real CV content"
    );
}
