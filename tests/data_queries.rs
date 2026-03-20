//! Integration tests for DATA layer queries.
//!
//! Each test creates an in-memory SQLite database, runs migrations,
//! and verifies queries return correct results from seeded data.

use sqlx::SqlitePool;

use gabriel_osemberg::models::{CvSection, Experience, Project, ProjectMetric, Skill, TechTag};

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

    assert!(
        sections.len() >= 3,
        "Should have at least 3 CV sections, got {}",
        sections.len()
    );
    // First section should be "about"
    assert_eq!(sections[0].section_type, "about");
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
async fn get_experiences_returns_seeded_cv_data() {
    let pool = test_pool().await;

    let experiences = sqlx::query_as::<_, Experience>(
        "SELECT id, role, company, company_url, start_date, end_date, \
         description, highlights, sort_order, visible \
         FROM experiences WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert!(
        experiences.len() >= 6,
        "Should have at least 6 experience entries, got {}",
        experiences.len()
    );

    // First entry is the most recent (current role)
    assert!(experiences[0]
        .role
        .contains("Independent Software Engineer"));
    assert!(
        experiences[0].end_date.is_none(),
        "Current role has no end date"
    );

    // Last entry is military service
    let last = experiences.last().expect("should have entries");
    assert!(last.company.contains("Israel Navy"));

    // All highlights should be valid JSON arrays
    for exp in &experiences {
        let parsed: Vec<String> = serde_json::from_str(&exp.highlights)
            .unwrap_or_else(|_| panic!("Invalid highlights JSON for {}", exp.role));
        assert!(
            !parsed.is_empty(),
            "Each experience should have at least one highlight"
        );
    }
}

#[tokio::test]
async fn cv_sections_include_education_and_military() {
    let pool = test_pool().await;

    let sections = sqlx::query_as::<_, CvSection>(
        "SELECT id, section_type, title, content, sort_order, visible \
         FROM cv_sections WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert!(
        sections.len() >= 5,
        "Should have at least 5 CV sections (about, methodology, philosophy, education, military)"
    );

    let types: Vec<&str> = sections.iter().map(|s| s.section_type.as_str()).collect();
    assert!(
        types.contains(&"education"),
        "Should have education section"
    );
    assert!(types.contains(&"military"), "Should have military section");
    assert!(
        types.contains(&"languages"),
        "Should have languages section"
    );
}

#[tokio::test]
async fn spoken_languages_seeded_as_skills() {
    let pool = test_pool().await;

    let spoken = sqlx::query_as::<_, Skill>(
        "SELECT id, name, category, proficiency, visible \
         FROM skills WHERE category = 'spoken_language' AND visible = 1",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert_eq!(spoken.len(), 4, "Should have 4 spoken languages");
    let names: Vec<&str> = spoken.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Hebrew"));
    assert!(names.contains(&"English"));
    assert!(names.contains(&"Spanish"));
    assert!(names.contains(&"Portuguese"));
}

// ---- L1 Integration Tests ----

#[tokio::test]
async fn get_project_metrics_returns_time_metrics() {
    let pool = test_pool().await;

    // Get project ID for 'time'
    let project = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE slug = 'time'",
    )
    .fetch_one(&pool)
    .await
    .expect("time project should exist");

    let metrics = sqlx::query_as::<_, ProjectMetric>(
        "SELECT id, project_id, metric_name, metric_value, metric_unit, source, measured_at \
         FROM project_metrics WHERE project_id = ?1 ORDER BY metric_name ASC",
    )
    .bind(project.id)
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert_eq!(
        metrics.len(),
        4,
        "time should have 4 metrics (loc, test_count, test_coverage, src_file_count)"
    );

    let loc = metrics
        .iter()
        .find(|m| m.metric_name == "loc")
        .expect("loc metric");
    assert_eq!(loc.metric_value, 90000.0);

    let coverage = metrics
        .iter()
        .find(|m| m.metric_name == "test_coverage")
        .expect("test_coverage metric");
    assert_eq!(coverage.metric_value, 95.9);

    let tests = metrics
        .iter()
        .find(|m| m.metric_name == "test_count")
        .expect("test_count metric");
    assert_eq!(tests.metric_value, 14789.0);
}

#[tokio::test]
async fn get_project_tags_returns_time_tags() {
    let pool = test_pool().await;

    let project = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE slug = 'time'",
    )
    .fetch_one(&pool)
    .await
    .expect("time project should exist");

    let tags = sqlx::query_as::<_, TechTag>(
        "SELECT t.id, t.name, t.category \
         FROM tech_tags t \
         INNER JOIN project_tags pt ON pt.tag_id = t.id \
         WHERE pt.project_id = ?1 \
         ORDER BY t.name ASC",
    )
    .bind(project.id)
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    let tag_names: Vec<&str> = tags.iter().map(|t| t.name.as_str()).collect();
    assert!(tag_names.contains(&"C11"), "time should have C11 tag");
    assert!(tag_names.contains(&"WebGL2"), "time should have WebGL2 tag");
    assert!(
        tag_names.contains(&"WebAssembly"),
        "time should have WebAssembly tag"
    );
    assert!(tag_names.contains(&"GLSL"), "time should have GLSL tag");
}

#[tokio::test]
async fn get_projects_by_tag_returns_correct_projects() {
    let pool = test_pool().await;

    // TypeScript is used by blocksight, anan-yarok, chamana
    let projects = sqlx::query_as::<_, Project>(
        "SELECT p.id, p.name, p.slug, p.tagline, p.description, p.tech_stack, \
         p.repo_url, p.live_url, p.image_path, p.sort_order, p.visible, \
         p.created_at, p.updated_at \
         FROM projects p \
         INNER JOIN project_tags pt ON pt.project_id = p.id \
         INNER JOIN tech_tags t ON t.id = pt.tag_id \
         WHERE t.name = 'TypeScript' AND p.visible = 1 \
         ORDER BY p.sort_order ASC",
    )
    .fetch_all(&pool)
    .await
    .expect("query should succeed");

    assert_eq!(
        projects.len(),
        3,
        "TypeScript should be tagged on 3 projects"
    );
    let slugs: Vec<&str> = projects.iter().map(|p| p.slug.as_str()).collect();
    assert!(slugs.contains(&"blocksight"));
    assert!(slugs.contains(&"anan-yarok"));
    assert!(slugs.contains(&"chamana"));
}

#[tokio::test]
async fn tech_tags_have_unique_names() {
    let pool = test_pool().await;

    let tags =
        sqlx::query_as::<_, TechTag>("SELECT id, name, category FROM tech_tags ORDER BY name ASC")
            .fetch_all(&pool)
            .await
            .expect("query should succeed");

    assert!(tags.len() >= 20, "Should have 20+ tech tags");

    let mut names: Vec<&str> = tags.iter().map(|t| t.name.as_str()).collect();
    let original_len = names.len();
    names.dedup();
    assert_eq!(
        names.len(),
        original_len,
        "All tech tag names should be unique"
    );
}

#[tokio::test]
async fn aggregated_stats_are_correct() {
    let pool = test_pool().await;

    let total_projects =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects WHERE visible = 1")
            .fetch_one(&pool)
            .await
            .expect("count query");

    assert_eq!(total_projects, 5);

    let total_loc = sqlx::query_scalar::<_, f64>(
        "SELECT COALESCE(SUM(metric_value), 0) FROM project_metrics WHERE metric_name = 'loc'",
    )
    .fetch_one(&pool)
    .await
    .expect("sum query");

    assert_eq!(total_loc, 90000.0, "Only time has LOC measured");

    let languages_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(DISTINCT name) FROM tech_tags WHERE category = 'language'",
    )
    .fetch_one(&pool)
    .await
    .expect("count query");

    assert!(languages_count >= 6, "Should have 6+ language tags");
}
