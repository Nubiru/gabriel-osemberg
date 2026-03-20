//! Skill model — represents a technical skill with category and proficiency.

use serde::{Deserialize, Serialize};

/// A technical skill displayed on the CV page.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub proficiency: i64,
    pub visible: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_serialization_roundtrip() {
        let skill = Skill {
            id: 1,
            name: "Rust".to_string(),
            category: "language".to_string(),
            proficiency: 2,
            visible: true,
        };
        let json = serde_json::to_string(&skill).expect("serialize");
        let deserialized: Skill = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(skill, deserialized);
    }

    #[test]
    fn skill_proficiency_range() {
        let skill = Skill {
            id: 1,
            name: "C11".to_string(),
            category: "language".to_string(),
            proficiency: 5,
            visible: true,
        };
        assert!(
            (1..=5).contains(&skill.proficiency),
            "Proficiency should be 1-5"
        );
    }
}
