//! Large metric stat display — prominent number with a label.

use leptos::prelude::*;

/// A prominent metric display showing a large formatted value and a label.
///
/// Use for metrics like "90K+" lines of code or "14,789" tests.
/// The value should be pre-formatted by the caller.
#[component]
pub fn MetricStat(
    /// Pre-formatted display value (e.g., "90K+", "14,789", "95.9%").
    #[prop(into)]
    value: String,
    /// Descriptive label (e.g., "Lines of Code", "Tests").
    #[prop(into)]
    label: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-center">
            <span class="font-display text-3xl font-bold text-text-primary">{value}</span>
            <span class="text-sm text-text-secondary mt-1">{label}</span>
        </div>
    }
}
