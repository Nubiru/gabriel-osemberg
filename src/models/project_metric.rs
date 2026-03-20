//! Project metric model — quantitative data per showcase project.

use serde::{Deserialize, Serialize};

/// A quantitative metric for a project (LOC, test count, coverage, etc.).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProjectMetric {
    pub id: i64,
    pub project_id: i64,
    pub metric_name: String,
    pub metric_value: f64,
    pub metric_unit: Option<String>,
    pub source: String,
    pub measured_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_metric_serialization_roundtrip() {
        let metric = ProjectMetric {
            id: 1,
            project_id: 1,
            metric_name: "loc".to_string(),
            metric_value: 90000.0,
            metric_unit: Some("lines".to_string()),
            source: "cloc".to_string(),
            measured_at: "2026-03-20T00:00:00".to_string(),
        };
        let json = serde_json::to_string(&metric).expect("serialize");
        let deserialized: ProjectMetric = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(metric, deserialized);
    }

    #[test]
    fn project_metric_coverage_is_percentage() {
        let metric = ProjectMetric {
            id: 2,
            project_id: 1,
            metric_name: "test_coverage".to_string(),
            metric_value: 95.9,
            metric_unit: Some("%".to_string()),
            source: "manual".to_string(),
            measured_at: "2026-03-20T00:00:00".to_string(),
        };
        assert!(metric.metric_value >= 0.0 && metric.metric_value <= 100.0);
    }
}
