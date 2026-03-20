//! Experience model — represents a work experience entry on the CV.

use serde::{Deserialize, Serialize};

/// A work experience entry displayed on the CV page.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Experience {
    pub id: i64,
    pub role: String,
    pub company: String,
    pub company_url: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub description: String,
    pub highlights: String,
    pub sort_order: i64,
    pub visible: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_experience() -> Experience {
        Experience {
            id: 1,
            role: "Software Engineer".to_string(),
            company: "Acme Corp".to_string(),
            company_url: Some("https://acme.example.com".to_string()),
            start_date: "2024-01".to_string(),
            end_date: None,
            description: "Building things.".to_string(),
            highlights: r#"["Shipped feature X","Improved perf by 40%"]"#.to_string(),
            sort_order: 1,
            visible: true,
        }
    }

    #[test]
    fn experience_serialization_roundtrip() {
        let exp = sample_experience();
        let json = serde_json::to_string(&exp).expect("serialize");
        let deserialized: Experience = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(exp, deserialized);
    }

    #[test]
    fn experience_end_date_none_means_present() {
        let exp = sample_experience();
        assert!(exp.end_date.is_none(), "None end_date represents 'Present'");
    }

    #[test]
    fn experience_highlights_is_valid_json_array() {
        let exp = sample_experience();
        let parsed: Vec<String> =
            serde_json::from_str(&exp.highlights).expect("highlights should be valid JSON array");
        assert_eq!(parsed.len(), 2);
    }
}
