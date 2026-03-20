//! Project detail page — renders a full case study for a single project.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::components::error::ErrorDisplay;
use crate::components::loading::SkeletonCard;
use crate::components::metrics_dashboard::MetricsDashboard;
use crate::components::ui::{Badge, ExternalLink};
use crate::server_fns::get_project_by_slug;

/// The `/projects/:slug` route. Fetches a single project by its URL slug
/// and renders a detailed case study page.
#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let project = Resource::new(slug, get_project_by_slug);

    view! {
        <Suspense fallback=move || {
            view! {
                <section class="py-12 space-y-8">
                    <SkeletonCard/>
                    <SkeletonCard/>
                </section>
            }
        }>
            {move || {
                project
                    .get()
                    .map(|result| {
                        match result {
                            Ok(proj) => {
                                // Destructure into owned values to avoid borrow issues in view! macro
                                let project_id = proj.id;
                                let title_text = format!("{} — Gabriel Osemberg", proj.name);
                                let name = proj.name;
                                let tagline = proj.tagline;
                                let tags: Vec<String> = proj
                                    .tech_stack
                                    .split(", ")
                                    .map(String::from)
                                    .collect();
                                let description_paragraphs: Vec<String> = proj
                                    .description
                                    .split("\n\n")
                                    .map(String::from)
                                    .collect();
                                let repo_url = proj.repo_url;
                                let live_url = proj.live_url;
                                let has_links = repo_url.is_some() || live_url.is_some();

                                view! {
                                    <Title text=title_text/>

                                    <section class="py-12">
                                        // Back link
                                        <a
                                            href="/projects"
                                            class="inline-flex items-center gap-1.5 text-sm font-medium
                                                   text-text-secondary hover:text-accent transition-colors
                                                   focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                                        >
                                            <svg
                                                class="w-4 h-4"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                aria-hidden="true"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    d="M10 19l-7-7m0 0l7-7m-7 7h18"
                                                />
                                            </svg>
                                            "Back to Projects"
                                        </a>

                                        // Hero section
                                        <div class="mt-8">
                                            <h1 class="font-display text-4xl sm:text-5xl font-bold tracking-tight text-text-primary">
                                                {name}
                                            </h1>
                                            <p class="mt-3 text-lg text-text-secondary leading-relaxed">
                                                {tagline}
                                            </p>
                                        </div>

                                        // Metrics dashboard
                                        <MetricsDashboard project_id=project_id/>

                                        // Tech stack badges
                                        <div class="mt-6 flex flex-wrap gap-2">
                                            {tags
                                                .into_iter()
                                                .map(|tag| {
                                                    view! { <Badge label=tag/> }
                                                })
                                                .collect_view()}
                                        </div>

                                        // Description paragraphs
                                        <div class="mt-8 space-y-4">
                                            {description_paragraphs
                                                .into_iter()
                                                .map(|para| {
                                                    view! {
                                                        <p class="text-text-secondary leading-relaxed">{para}</p>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>

                                        // External links
                                        {has_links
                                            .then(|| {
                                                view! {
                                                    <div class="mt-8 flex flex-wrap gap-4">
                                                        {repo_url
                                                            .map(|url| {
                                                                view! {
                                                                    <ExternalLink href=url label="Source Code".to_string()/>
                                                                }
                                                            })}
                                                        {live_url
                                                            .map(|url| {
                                                                view! {
                                                                    <ExternalLink href=url label="Live Site".to_string()/>
                                                                }
                                                            })}
                                                    </div>
                                                }
                                            })}
                                    </section>
                                }
                                    .into_any()
                            }
                            Err(e) => {
                                view! {
                                    <section class="py-12">
                                        <a
                                            href="/projects"
                                            class="inline-flex items-center gap-1.5 text-sm font-medium
                                                   text-text-secondary hover:text-accent transition-colors
                                                   focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                                        >
                                            <svg
                                                class="w-4 h-4"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                aria-hidden="true"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    d="M10 19l-7-7m0 0l7-7m-7 7h18"
                                                />
                                            </svg>
                                            "Back to Projects"
                                        </a>
                                        <div class="mt-8">
                                            <ErrorDisplay
                                                title="Project not found"
                                                message=e.to_string()
                                                retry=true
                                            />
                                        </div>
                                    </section>
                                }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}
