use leptos::prelude::*;

/// A vertical timeline for displaying experience entries.
///
/// Each entry shows role, company, date range, description, and highlights.
/// Designed for 4-6 entries, handles N entries with vertical scroll.
#[component]
pub fn Timeline(children: Children) -> impl IntoView {
    view! {
        <div class="relative" role="list" aria-label="Experience timeline">
            // Vertical line connecting entries
            <div
                class="absolute left-4 sm:left-6 top-0 bottom-0 w-px bg-border-default"
                aria-hidden="true"
            ></div>
            <div class="space-y-10">{children()}</div>
        </div>
    }
}

/// A single entry in the timeline.
#[component]
pub fn TimelineEntry(
    /// Job title / role.
    #[prop(into)]
    role: String,
    /// Company name.
    #[prop(into)]
    company: String,
    /// Optional company URL for linking.
    #[prop(optional, into)]
    company_url: Option<String>,
    /// Start date (e.g., "Jan 2024").
    #[prop(into)]
    start_date: String,
    /// End date — None means "Present".
    #[prop(optional, into)]
    end_date: Option<String>,
    /// Description text (1-2 paragraphs).
    #[prop(optional, into)]
    description: Option<String>,
    /// Highlight bullet points (impact statements).
    #[prop(default = vec![])]
    highlights: Vec<String>,
) -> impl IntoView {
    let date_range = match &end_date {
        Some(end) => format!("{start_date} — {end}"),
        None => format!("{start_date} — Present"),
    };

    let is_current = end_date.is_none();

    view! {
        <div class="relative pl-10 sm:pl-14" role="listitem">
            // Timeline dot
            <div
                class=format!(
                    "absolute left-2.5 sm:left-4.5 top-1.5 w-3 h-3 rounded-full border-2
                     {} {}",
                    if is_current {
                        "bg-accent border-accent"
                    } else {
                        "bg-surface-base border-border-default"
                    },
                    if is_current { "ring-4 ring-accent/20" } else { "" },
                )
                aria-hidden="true"
            ></div>

            // Date range
            <time class="block font-mono text-xs text-text-muted tracking-wide mb-1">
                {date_range}
            </time>

            // Role + Company
            <h3 class="font-display text-lg font-semibold text-text-primary">{role}</h3>
            {match company_url {
                Some(url) => {
                    view! {
                        <a
                            href=url
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-sm text-accent hover:text-accent-hover transition-colors"
                        >
                            {company}
                        </a>
                    }
                        .into_any()
                }
                None => {
                    view! { <span class="text-sm text-text-secondary">{company}</span> }.into_any()
                }
            }}

            // Description
            {description.map(|desc| {
                view! {
                    <p class="mt-3 text-sm text-text-secondary leading-relaxed">{desc}</p>
                }
            })}

            // Highlights
            {(!highlights.is_empty())
                .then(|| {
                    view! {
                        <ul class="mt-3 space-y-1.5" aria-label="Key achievements">
                            {highlights
                                .into_iter()
                                .map(|h| {
                                    view! {
                                        <li class="flex items-start gap-2 text-sm text-text-secondary">
                                            <span
                                                class="mt-1.5 w-1.5 h-1.5 rounded-full bg-accent/60 shrink-0"
                                                aria-hidden="true"
                                            ></span>
                                            {h}
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                })}
        </div>
    }
}
