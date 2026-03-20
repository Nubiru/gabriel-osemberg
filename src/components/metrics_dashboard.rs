//! Metrics dashboard — combines chart components to display a project's metrics.

use leptos::prelude::*;

use crate::components::charts::metric_stat::MetricStat;
use crate::components::charts::progress_ring::ProgressRing;
use crate::components::loading::Skeleton;
use crate::models::ProjectMetric;
use crate::server_fns::get_project_metrics;

/// Format a metric value for dashboard display based on its name.
///
/// Different metric types get different formatting:
/// - `loc`: thousands abbreviated (e.g., 90000 -> "90K+")
/// - `test_count`, `src_file_count`, `test_file_count`: comma-separated integers
/// - `test_coverage`: one decimal with percent sign
/// - Fallback: plain integer
fn format_metric_value(value: f64, name: &str) -> String {
    match name {
        "loc" => {
            if value >= 100_000.0 {
                format!("{}K+", (value / 1000.0) as i64)
            } else if value >= 1000.0 {
                format!("{:.0}K+", value / 1000.0)
            } else {
                format!("{:.0}", value)
            }
        }
        "test_count" | "src_file_count" | "test_file_count" => format_with_commas(value as i64),
        "test_coverage" => format!("{:.1}%", value),
        _ => format!("{:.0}", value),
    }
}

/// Format an integer with comma separators (e.g., 14789 -> "14,789").
fn format_with_commas(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Convert a snake_case metric name to a human-readable label.
fn metric_label(name: &str) -> &str {
    match name {
        "loc" => "Lines of Code",
        "test_count" => "Tests",
        "test_coverage" => "Test Coverage",
        "src_file_count" => "Source Files",
        "test_file_count" => "Test Files",
        _ => name,
    }
}

/// Find a metric by name in a slice of `ProjectMetric`.
fn find_metric<'a>(metrics: &'a [ProjectMetric], name: &str) -> Option<&'a ProjectMetric> {
    metrics.iter().find(|m| m.metric_name == name)
}

/// A dashboard of project metrics combining ProgressRing and MetricStat.
///
/// Fetches metrics for the given `project_id` and renders them in a
/// responsive grid. Shows a ProgressRing for test coverage and
/// MetricStat cards for LOC, test count, and source file count.
///
/// Renders nothing if no metrics are found.
#[component]
pub fn MetricsDashboard(project_id: i64) -> impl IntoView {
    let metrics = Resource::new(move || project_id, get_project_metrics);

    let skeleton_fallback = move || {
        view! {
            <div class="grid grid-cols-2 sm:grid-cols-4 gap-6 mt-8">
                <div class="flex flex-col items-center gap-2">
                    <Skeleton height="h-16" width="w-16" rounded="rounded-full"/>
                    <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <Skeleton height="h-8" width="w-24" rounded="rounded"/>
                    <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <Skeleton height="h-8" width="w-24" rounded="rounded"/>
                    <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <Skeleton height="h-8" width="w-24" rounded="rounded"/>
                    <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                </div>
            </div>
        }
    };

    view! {
        <Suspense fallback=skeleton_fallback>
            {move || {
                metrics
                    .get()
                    .map(|result| {
                        match result {
                            Ok(data) if data.is_empty() => {
                                // No metrics — render nothing
                                view! { <div></div> }.into_any()
                            }
                            Ok(data) => {
                                render_metrics_grid(&data).into_any()
                            }
                            Err(_) => {
                                // Silently degrade — metrics are supplementary, not critical
                                view! { <div></div> }.into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}

/// Render the metrics grid from a slice of `ProjectMetric`.
///
/// Extracts known metrics by name and renders them in a responsive grid:
/// - Test coverage as a ProgressRing (visual anchor)
/// - LOC, test count, source file count as MetricStat cards
fn render_metrics_grid(metrics: &[ProjectMetric]) -> impl IntoView {
    let coverage = find_metric(metrics, "test_coverage");
    let loc = find_metric(metrics, "loc");
    let tests = find_metric(metrics, "test_count");
    let src_files = find_metric(metrics, "src_file_count");

    // Only render if we have at least one metric to show
    let has_any = coverage.is_some() || loc.is_some() || tests.is_some() || src_files.is_some();
    if !has_any {
        return view! { <div></div> }.into_any();
    }

    let coverage_view = coverage.map(|m| {
        view! {
            <div class="flex justify-center">
                <div class="w-20">
                    <ProgressRing
                        value=m.metric_value
                        label=metric_label(&m.metric_name).to_string()
                    />
                </div>
            </div>
        }
    });

    let loc_view = loc.map(|m| {
        let val = format_metric_value(m.metric_value, &m.metric_name);
        view! { <MetricStat value=val label=metric_label(&m.metric_name).to_string()/> }
    });

    let tests_view = tests.map(|m| {
        let val = format_metric_value(m.metric_value, &m.metric_name);
        view! { <MetricStat value=val label=metric_label(&m.metric_name).to_string()/> }
    });

    let src_files_view = src_files.map(|m| {
        let val = format_metric_value(m.metric_value, &m.metric_name);
        view! { <MetricStat value=val label=metric_label(&m.metric_name).to_string()/> }
    });

    view! {
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-6 mt-8 py-6 px-4 rounded-xl bg-surface-raised border border-border-subtle">
            {coverage_view}
            {loc_view}
            {tests_view}
            {src_files_view}
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- format_metric_value tests ---

    #[test]
    fn format_loc_large() {
        assert_eq!(format_metric_value(90000.0, "loc"), "90K+");
    }

    #[test]
    fn format_loc_medium() {
        assert_eq!(format_metric_value(2876.0, "loc"), "3K+");
    }

    #[test]
    fn format_loc_small() {
        assert_eq!(format_metric_value(500.0, "loc"), "500");
    }

    #[test]
    fn format_loc_very_large() {
        assert_eq!(format_metric_value(150_000.0, "loc"), "150K+");
    }

    #[test]
    fn format_test_count_with_commas() {
        assert_eq!(format_metric_value(14789.0, "test_count"), "14,789");
    }

    #[test]
    fn format_src_files_with_commas() {
        assert_eq!(format_metric_value(3773.0, "src_file_count"), "3,773");
    }

    #[test]
    fn format_test_coverage() {
        assert_eq!(format_metric_value(95.9, "test_coverage"), "95.9%");
    }

    #[test]
    fn format_unknown_metric() {
        assert_eq!(format_metric_value(42.0, "some_metric"), "42");
    }

    // --- format_with_commas tests ---

    #[test]
    fn commas_for_thousands() {
        assert_eq!(format_with_commas(1234), "1,234");
    }

    #[test]
    fn commas_for_millions() {
        assert_eq!(format_with_commas(1_234_567), "1,234,567");
    }

    #[test]
    fn no_commas_for_hundreds() {
        assert_eq!(format_with_commas(999), "999");
    }

    #[test]
    fn commas_for_zero() {
        assert_eq!(format_with_commas(0), "0");
    }

    #[test]
    fn commas_for_exact_thousand() {
        assert_eq!(format_with_commas(1000), "1,000");
    }

    // --- metric_label tests ---

    #[test]
    fn label_loc() {
        assert_eq!(metric_label("loc"), "Lines of Code");
    }

    #[test]
    fn label_test_count() {
        assert_eq!(metric_label("test_count"), "Tests");
    }

    #[test]
    fn label_coverage() {
        assert_eq!(metric_label("test_coverage"), "Test Coverage");
    }

    #[test]
    fn label_unknown_returns_raw() {
        assert_eq!(metric_label("custom_metric"), "custom_metric");
    }

    // --- find_metric tests ---

    #[test]
    fn find_existing_metric() {
        let metrics = vec![
            sample_metric("loc", 90000.0),
            sample_metric("test_count", 14789.0),
        ];
        let found = find_metric(&metrics, "loc");
        assert!(found.is_some());
        assert!((found.map_or(0.0, |m| m.metric_value) - 90000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn find_missing_metric() {
        let metrics = vec![sample_metric("loc", 90000.0)];
        assert!(find_metric(&metrics, "test_coverage").is_none());
    }

    /// Helper to create a sample metric for testing.
    fn sample_metric(name: &str, value: f64) -> ProjectMetric {
        ProjectMetric {
            id: 1,
            project_id: 1,
            metric_name: name.to_string(),
            metric_value: value,
            metric_unit: None,
            source: "test".to_string(),
            measured_at: "2026-03-20T00:00:00".to_string(),
        }
    }
}
