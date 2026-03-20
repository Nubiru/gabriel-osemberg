use leptos::prelude::*;

use crate::models::ProjectMetric;

/// Displays a grid of project metrics as styled stat cards.
#[component]
pub fn MetricsGrid(metrics: Vec<ProjectMetric>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
            {metrics
                .into_iter()
                .map(|m| {
                    let display_value = format_metric_value(m.metric_value, m.metric_unit.as_deref());
                    let label = format_metric_name(&m.metric_name);
                    view! { <MetricCard value=display_value label=label/> }
                })
                .collect_view()}
        </div>
    }
}

/// A single metric displayed as a card with a large value and label.
#[component]
pub fn MetricCard(#[prop(into)] value: String, #[prop(into)] label: String) -> impl IntoView {
    view! {
        <div class="p-4 rounded-lg bg-surface-raised border border-border-subtle text-center">
            <div class="font-display text-2xl font-bold text-text-primary">{value}</div>
            <div class="mt-1 text-xs text-text-muted uppercase tracking-wide">{label}</div>
        </div>
    }
}

/// A horizontal progress bar showing a percentage value (0-100).
#[component]
pub fn ProgressBar(
    /// The percentage value (0.0 to 100.0).
    value: f64,
    /// Label shown to the left.
    #[prop(into)]
    label: String,
    /// Optional color override (defaults to accent).
    #[prop(default = "bg-accent")]
    color: &'static str,
) -> impl IntoView {
    let clamped = value.clamp(0.0, 100.0);
    let width_pct = format!("{clamped:.0}%");

    view! {
        <div class="flex items-center gap-3">
            <span class="text-sm text-text-secondary w-24 shrink-0">{label}</span>
            <div class="flex-1 h-2 rounded-full bg-surface-overlay overflow-hidden">
                <div
                    class=format!("h-full rounded-full {color} transition-all duration-500 ease-fluid")
                    style=format!("width: {width_pct}")
                    role="progressbar"
                    aria-valuenow=clamped.to_string()
                    aria-valuemin="0"
                    aria-valuemax="100"
                ></div>
            </div>
            <span class="text-sm font-mono text-text-muted w-12 text-right">{width_pct}</span>
        </div>
    }
}

/// Aggregated stats display — large numbers in a horizontal row.
#[component]
pub fn StatsRow(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-wrap justify-center gap-8 sm:gap-12 py-8">{children()}</div>
    }
}

/// A single large stat in a StatsRow.
#[component]
pub fn StatItem(#[prop(into)] value: String, #[prop(into)] label: String) -> impl IntoView {
    view! {
        <div class="text-center">
            <div class="font-display text-3xl sm:text-4xl font-bold text-text-primary">{value}</div>
            <div class="mt-1 text-sm text-text-muted">{label}</div>
        </div>
    }
}

/// Format a metric value for display (e.g., 90000.0 → "90K+").
fn format_metric_value(value: f64, unit: Option<&str>) -> String {
    match unit {
        Some("%") => format!("{value:.1}%"),
        Some("lines") | None if value >= 1000.0 => {
            let k = value / 1000.0;
            if k >= 100.0 {
                format!("{}K+", k as i64)
            } else if k >= 10.0 {
                format!("{:.0}K", k)
            } else {
                format!("{k:.1}K")
            }
        }
        _ => {
            if value == value.floor() {
                format!("{}", value as i64)
            } else {
                format!("{value:.1}")
            }
        }
    }
}

/// Convert a metric_name like "test_coverage" to "Test Coverage".
fn format_metric_name(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_large_loc() {
        assert_eq!(format_metric_value(90200.0, Some("lines")), "90K");
    }

    #[test]
    fn format_medium_loc() {
        assert_eq!(format_metric_value(2876.0, Some("lines")), "2.9K");
    }

    #[test]
    fn format_percentage() {
        assert_eq!(format_metric_value(95.9, Some("%")), "95.9%");
    }

    #[test]
    fn format_integer() {
        assert_eq!(format_metric_value(374.0, None), "374");
    }

    #[test]
    fn format_name_snake_case() {
        assert_eq!(format_metric_name("test_coverage"), "Test Coverage");
    }

    #[test]
    fn format_name_single_word() {
        assert_eq!(format_metric_name("loc"), "Loc");
    }
}
