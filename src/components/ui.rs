use leptos::prelude::*;

/// A styled section heading with consistent spacing.
#[component]
pub fn SectionHeading(
    #[prop(into)] title: String,
    #[prop(optional, into)] subtitle: Option<String>,
) -> impl IntoView {
    view! {
        <div class="mb-8">
            <h2 class="font-display text-3xl font-bold tracking-tight text-text-primary">{title}</h2>
            {subtitle.map(|sub| {
                view! {
                    <p class="mt-2 text-lg text-text-secondary">{sub}</p>
                }
            })}
        </div>
    }
}

/// A tech stack badge/tag.
#[component]
pub fn Badge(#[prop(into)] label: String) -> impl IntoView {
    view! {
        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium
                     bg-accent/10 text-accent border border-accent/20">
            {label}
        </span>
    }
}

/// A styled external link with arrow icon.
#[component]
pub fn ExternalLink(#[prop(into)] href: String, #[prop(into)] label: String) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-1 text-sm font-medium
                   text-accent hover:text-accent-hover transition-colors
                   focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
        >
            {label}
            <svg
                class="w-3.5 h-3.5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="2"
                aria-hidden="true"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                />
            </svg>
        </a>
    }
}
