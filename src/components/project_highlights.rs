//! Aggregated project highlights — portfolio-wide stats displayed above the project grid.

use leptos::prelude::*;

use crate::components::charts::metric_stat::MetricStat;
use crate::components::loading::Skeleton;
use crate::server_fns::get_aggregated_stats;

/// Format a large number for display (e.g., 90000 → "90K+").
fn format_large_number(value: f64) -> String {
    if value >= 100_000.0 {
        format!("{}K+", (value / 1000.0) as i64)
    } else if value >= 1000.0 {
        format!("{:.0}K+", value / 1000.0)
    } else {
        format!("{:.0}", value)
    }
}

/// Displays aggregated statistics across all showcase projects.
///
/// Shows total LOC, total tests, project count, and language count
/// in a compact grid above the project cards. Fetches data via
/// `get_aggregated_stats()` server function.
#[component]
pub fn ProjectHighlights() -> impl IntoView {
    let stats = Resource::new(|| (), |_| get_aggregated_stats());

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-10 py-5 px-4 rounded-xl bg-surface-raised border border-border-subtle">
                    <div class="flex flex-col items-center gap-2">
                        <Skeleton height="h-8" width="w-16" rounded="rounded"/>
                        <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Skeleton height="h-8" width="w-16" rounded="rounded"/>
                        <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Skeleton height="h-8" width="w-16" rounded="rounded"/>
                        <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Skeleton height="h-8" width="w-16" rounded="rounded"/>
                        <Skeleton height="h-3" width="w-20" rounded="rounded"/>
                    </div>
                </div>
            }
        }>
            {move || {
                stats
                    .get()
                    .map(|result| {
                        match result {
                            Ok(data) => {
                                let loc = format_large_number(data.total_loc);
                                let tests = format_large_number(data.total_tests);
                                let projects = data.total_projects.to_string();
                                let languages = data.languages_count.to_string();

                                view! {
                                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-10 py-5 px-4 rounded-xl bg-surface-raised border border-border-subtle">
                                        <MetricStat value=loc label="Total Lines of Code".to_string()/>
                                        <MetricStat value=tests label="Total Tests".to_string()/>
                                        <MetricStat value=projects label="Projects".to_string()/>
                                        <MetricStat value=languages label="Languages".to_string()/>
                                    </div>
                                }
                                    .into_any()
                            }
                            Err(_) => {
                                // Silently degrade — highlights are supplementary
                                view! { <div></div> }.into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_large_number_thousands() {
        assert_eq!(format_large_number(90000.0), "90K+");
    }

    #[test]
    fn format_large_number_small() {
        assert_eq!(format_large_number(500.0), "500");
    }

    #[test]
    fn format_large_number_medium() {
        assert_eq!(format_large_number(16000.0), "16K+");
    }

    #[test]
    fn format_large_number_very_large() {
        assert_eq!(format_large_number(150_000.0), "150K+");
    }
}
