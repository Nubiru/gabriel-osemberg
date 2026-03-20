//! Project model — represents a showcase project in the portfolio.

use serde::{Deserialize, Serialize};

/// A project displayed in the portfolio showcase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub tagline: String,
    pub description: String,
    pub tech_stack: String,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub image_path: Option<String>,
    pub sort_order: i64,
    pub visible: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project() -> Project {
        Project {
            id: 1,
            name: "time".to_string(),
            slug: "time".to_string(),
            tagline: "3D interactive temporal artwork".to_string(),
            description: "A WebGL2-powered visualization engine.".to_string(),
            tech_stack: "C11, WebGL2, WebAssembly".to_string(),
            repo_url: Some("https://github.com/example/time".to_string()),
            live_url: None,
            image_path: None,
            sort_order: 1,
            visible: true,
            created_at: "2026-03-20T00:00:00".to_string(),
            updated_at: "2026-03-20T00:00:00".to_string(),
        }
    }

    #[test]
    fn project_serialization_roundtrip() {
        let project = sample_project();
        let json = serde_json::to_string(&project).expect("serialize");
        let deserialized: Project = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(project, deserialized);
    }

    #[test]
    fn project_optional_fields_serialize_as_null() {
        let project = sample_project();
        let json = serde_json::to_string(&project).expect("serialize");
        assert!(json.contains("\"live_url\":null"));
        assert!(json.contains("\"image_path\":null"));
    }
}
