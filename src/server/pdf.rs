//! PDF CV generation using Typst.
//!
//! Compiles an ATS-friendly CV from database content using the Typst
//! compiler embedded as a library. The template is baked in at compile time.

use derive_typst_intoval::{IntoDict, IntoValue};
use typst::foundations::{Dict, IntoValue};
use typst_as_lib::TypstEngine;

use crate::models::{CvSection, Experience, Project, Skill};

/// The Typst template, embedded at compile time.
static CV_TEMPLATE: &str = include_str!("cv_template.typ");

/// CV data structured for Typst template injection.
#[derive(Debug, Clone, IntoValue, IntoDict)]
pub struct CvPdfData {
    pub name: String,
    pub location: String,
    pub phone: String,
    pub email: String,
    pub linkedin: String,
    pub github: String,
    pub summary: String,
    pub experiences: Vec<CvExperience>,
    pub skill_categories: Vec<SkillCategory>,
    pub projects: Vec<CvProject>,
    pub methodology: String,
    pub military: Option<String>,
    pub education: String,
    pub languages: String,
}

impl From<CvPdfData> for Dict {
    fn from(value: CvPdfData) -> Self {
        value.into_dict()
    }
}

/// Experience entry for the PDF.
#[derive(Debug, Clone, IntoValue, IntoDict)]
pub struct CvExperience {
    pub role: String,
    pub company: String,
    pub date_range: String,
    pub description: String,
    pub highlights: Vec<String>,
}

/// Skill category for the PDF (e.g., "Languages": ["Rust", "C", ...]).
#[derive(Debug, Clone, IntoValue, IntoDict)]
pub struct SkillCategory {
    pub label: String,
    pub skills: Vec<String>,
}

/// Project entry for the PDF.
#[derive(Debug, Clone, IntoValue, IntoDict)]
pub struct CvProject {
    pub name: String,
    pub tagline: String,
    pub tech_stack: String,
    pub highlights: Vec<String>,
}

/// Generate a PDF CV from database content.
///
/// Returns the raw PDF bytes or an error message.
pub fn generate_cv_pdf(
    experiences: Vec<Experience>,
    skills: Vec<Skill>,
    cv_sections: Vec<CvSection>,
    projects: Vec<Project>,
) -> Result<Vec<u8>, String> {
    let data = build_cv_data(experiences, skills, cv_sections, projects);

    let engine = TypstEngine::builder()
        .main_file(CV_TEMPLATE)
        .build();

    let doc = engine
        .compile_with_input(data)
        .output
        .map_err(|errs| format!("Typst compilation failed: {errs:?}"))?;

    let options = typst_pdf::PdfOptions::default();
    typst_pdf::pdf(&doc, &options)
        .map_err(|errs| format!("PDF generation failed: {errs:?}"))
}

/// Build CvPdfData from database models.
fn build_cv_data(
    experiences: Vec<Experience>,
    skills: Vec<Skill>,
    cv_sections: Vec<CvSection>,
    projects: Vec<Project>,
) -> CvPdfData {
    // Extract cv_sections by type
    let summary = cv_sections
        .iter()
        .find(|s| s.section_type == "about")
        .map(|s| s.content.clone())
        .unwrap_or_default();

    let methodology = cv_sections
        .iter()
        .find(|s| s.section_type == "methodology")
        .map(|s| strip_markdown_bold(&s.content))
        .unwrap_or_default();

    let military = cv_sections
        .iter()
        .find(|s| s.section_type == "military")
        .map(|s| strip_markdown_bold(&s.content));

    let education = cv_sections
        .iter()
        .find(|s| s.section_type == "education")
        .map(|s| strip_markdown_bold(&s.content))
        .unwrap_or_default();

    let languages = cv_sections
        .iter()
        .find(|s| s.section_type == "languages")
        .map(|s| strip_markdown_bold(&s.content))
        .unwrap_or_default();

    // Build experiences
    let cv_experiences: Vec<CvExperience> = experiences
        .into_iter()
        .map(|exp| {
            let date_range = match &exp.end_date {
                Some(end) => format!("{} — {}", exp.start_date, end),
                None => format!("{} — Present", exp.start_date),
            };
            let highlights: Vec<String> =
                serde_json::from_str(&exp.highlights).unwrap_or_default();
            CvExperience {
                role: exp.role,
                company: exp.company,
                date_range,
                description: exp.description,
                highlights,
            }
        })
        .collect();

    // Group skills by category
    let skill_categories = build_skill_categories(skills);

    // Build projects
    let cv_projects: Vec<CvProject> = projects
        .into_iter()
        .map(|p| CvProject {
            name: p.name,
            tagline: p.tagline,
            tech_stack: p.tech_stack,
            highlights: vec![],
        })
        .collect();

    CvPdfData {
        name: "Gabriel Osemberg".to_string(),
        location: "Capilla del Monte, Córdoba, Argentina".to_string(),
        phone: "(+54) 911 2548-0642".to_string(),
        email: "osemberg.gabi@gmail.com".to_string(),
        linkedin: "linkedin.com/in/gabrielosem".to_string(),
        github: "github.com/Nubiru".to_string(),
        summary,
        experiences: cv_experiences,
        skill_categories,
        projects: cv_projects,
        methodology,
        military,
        education,
        languages,
    }
}

/// Group skills into categories for the PDF.
fn build_skill_categories(skills: Vec<Skill>) -> Vec<SkillCategory> {
    let categories = [
        ("language", "Languages"),
        ("framework", "Frameworks"),
        ("tool", "Tools & Platforms"),
        ("concept", "Specialties"),
    ];

    categories
        .iter()
        .filter_map(|(cat, label)| {
            let names: Vec<String> = skills
                .iter()
                .filter(|s| s.category == *cat)
                .map(|s| s.name.clone())
                .collect();
            if names.is_empty() {
                None
            } else {
                Some(SkillCategory {
                    label: label.to_string(),
                    skills: names,
                })
            }
        })
        .collect()
}

/// Strip **bold** markers from markdown text for plain-text PDF.
fn strip_markdown_bold(text: &str) -> String {
    text.replace("**", "")
}

/// Axum handler for PDF CV download.
///
/// GET /api/cv.pdf — returns a generated PDF with proper headers.
pub async fn cv_pdf_handler(
    axum::extract::Extension(pool): axum::extract::Extension<sqlx::SqlitePool>,
) -> impl axum::response::IntoResponse {
    use axum::http::{header, StatusCode};
    use axum::response::Response;

    // Fetch all data from database
    let experiences = sqlx::query_as::<_, Experience>(
        "SELECT id, role, company, company_url, start_date, end_date, \
         description, highlights, sort_order, visible \
         FROM experiences WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await;

    let skills = sqlx::query_as::<_, Skill>(
        "SELECT id, name, category, proficiency, visible \
         FROM skills WHERE visible = 1 AND category != 'spoken_language' \
         ORDER BY category ASC, proficiency DESC",
    )
    .fetch_all(&pool)
    .await;

    let cv_sections = sqlx::query_as::<_, CvSection>(
        "SELECT id, section_type, title, content, sort_order, visible \
         FROM cv_sections WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await;

    let projects = sqlx::query_as::<_, Project>(
        "SELECT id, name, slug, tagline, description, tech_stack, \
         repo_url, live_url, image_path, sort_order, visible, \
         created_at, updated_at \
         FROM projects WHERE visible = 1 ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await;

    // Check for database errors
    let (experiences, skills, cv_sections, projects) = match (experiences, skills, cv_sections, projects) {
        (Ok(e), Ok(s), Ok(c), Ok(p)) => (e, s, c, p),
        _ => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Failed to fetch CV data"))
                .expect("response build");
        }
    };

    // Generate PDF
    match generate_cv_pdf(experiences, skills, cv_sections, projects) {
        Ok(pdf_bytes) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/pdf")
            .header(
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"Gabriel_Osemberg_CV.pdf\"",
            )
            .body(axum::body::Body::from(pdf_bytes))
            .expect("response build"),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from(format!("PDF generation failed: {err}")))
            .expect("response build"),
    }
}
