//! Project-tag junction model — many-to-many relationship between projects and tech tags.

use serde::{Deserialize, Serialize};

/// Links a project to a tech tag (many-to-many).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProjectTag {
    pub project_id: i64,
    pub tag_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_tag_serialization_roundtrip() {
        let pt = ProjectTag {
            project_id: 1,
            tag_id: 3,
        };
        let json = serde_json::to_string(&pt).expect("serialize");
        let deserialized: ProjectTag = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(pt, deserialized);
    }
}
