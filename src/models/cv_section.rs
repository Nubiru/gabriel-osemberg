//! CV section model — represents an ordered content block on the about/CV page.

use serde::{Deserialize, Serialize};

/// An ordered content section on the CV/about page.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct CvSection {
    pub id: i64,
    pub section_type: String,
    pub title: String,
    pub content: String,
    pub sort_order: i64,
    pub visible: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cv_section_serialization_roundtrip() {
        let section = CvSection {
            id: 1,
            section_type: "about".to_string(),
            title: "About Me".to_string(),
            content: "I build things.".to_string(),
            sort_order: 1,
            visible: true,
        };
        let json = serde_json::to_string(&section).expect("serialize");
        let deserialized: CvSection = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(section, deserialized);
    }
}
