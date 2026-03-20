//! Tech tag model — normalized technology names reusable across projects and skills.

use serde::{Deserialize, Serialize};

/// A normalized technology name (e.g., "Rust", "WebGL2") linked to projects via ProjectTag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct TechTag {
    pub id: i64,
    pub name: String,
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tech_tag_serialization_roundtrip() {
        let tag = TechTag {
            id: 1,
            name: "Rust".to_string(),
            category: "language".to_string(),
        };
        let json = serde_json::to_string(&tag).expect("serialize");
        let deserialized: TechTag = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(tag, deserialized);
    }
}
