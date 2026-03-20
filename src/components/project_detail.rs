//! Project detail page — renders a full case study for a single project.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params_map;

use crate::components::charts::framework_diagram::FrameworkDiagram;
use crate::components::error::ErrorDisplay;
use crate::components::loading::SkeletonCard;
use crate::components::metrics_dashboard::MetricsDashboard;
use crate::components::scroll_reveal::ScrollReveal;
use crate::components::ui::{Badge, ExternalLink};
use crate::models::TechTag;
use crate::server_fns::{get_project_by_slug, get_project_tags};

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
                                let is_self_referential = proj.slug == "gabriel-osemberg";
                                let title_text = format!("{} — Gabriel Osemberg", proj.name);
                                let meta_description = format!(
                                    "{} — {}",
                                    proj.name, proj.tagline
                                );
                                let name = proj.name;
                                let tagline = proj.tagline;
                                let fallback_tags: Vec<String> = proj
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

                                // Fetch normalized tech tags (falls back to comma-split)
                                let tags_resource = Resource::new(
                                    move || project_id,
                                    get_project_tags,
                                );

                                view! {
                                    <Title text=title_text/>
                                    <Meta name="description" content=meta_description/>

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
                                        <ScrollReveal>
                                            <div class="mt-8">
                                                <h1 class="font-display text-4xl sm:text-5xl font-bold tracking-tight text-text-primary">
                                                    {name}
                                                </h1>
                                                <p class="mt-3 text-lg text-text-secondary leading-relaxed">
                                                    {tagline}
                                                </p>
                                            </div>
                                        </ScrollReveal>

                                        // Metrics dashboard
                                        <ScrollReveal delay=100>
                                            <MetricsDashboard project_id=project_id/>
                                        </ScrollReveal>

                                                // Tech stack badges (normalized from TechTag, fallback to comma-split)
                                        <ScrollReveal delay=200>
                                            <div class="mt-6 flex flex-wrap gap-2">
                                                {
                                                    let tags_for_fallback = fallback_tags.clone();
                                                    let tags_for_err = fallback_tags;
                                                    view! {
                                                        <Suspense fallback=move || {
                                                            tags_for_fallback.clone().into_iter()
                                                                .map(|tag| view! { <Badge label=tag/> })
                                                                .collect_view()
                                                        }>
                                                            {move || {
                                                                tags_resource.get().map(|result| {
                                                                    match result {
                                                                        Ok(tech_tags) if !tech_tags.is_empty() => {
                                                                            render_grouped_tags(&tech_tags).into_any()
                                                                        }
                                                                        _ => {
                                                                            tags_for_err.clone().into_iter()
                                                                                .map(|tag| view! { <Badge label=tag/> })
                                                                                .collect_view()
                                                                                .into_any()
                                                                        }
                                                                    }
                                                                })
                                                            }}
                                                        </Suspense>
                                                    }
                                                }
                                            </div>
                                        </ScrollReveal>

                                        // Description paragraphs
                                        <ScrollReveal delay=300>
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
                                        </ScrollReveal>

                                        // MEGA Framework diagram (only on the self-referential project)
                                        {is_self_referential.then(|| {
                                            view! {
                                                <ScrollReveal delay=400>
                                                    <div class="mt-10">
                                                        <h2 class="font-display text-2xl font-semibold text-text-primary mb-4">
                                                            "AI-Augmented Engineering Framework"
                                                        </h2>
                                                        <p class="text-text-secondary leading-relaxed mb-6">
                                                            "This website is built using a multi-session AI collaboration framework. "
                                                            "Five parallel Claude Code sessions work simultaneously — each owning a domain "
                                                            "(data, design, infrastructure, content, showcase) — coordinated by a primary "
                                                            "session that maintains vision and architectural consistency."
                                                        </p>
                                                        <FrameworkDiagram/>
                                                    </div>
                                                </ScrollReveal>
                                            }
                                        })}

                                        // External links
                                        {has_links
                                            .then(|| {
                                                view! {
                                                    <ScrollReveal delay=400>
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
                                                    </ScrollReveal>
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

/// Render tech tags grouped by category (languages first, then frameworks, tools, concepts).
fn render_grouped_tags(tags: &[TechTag]) -> impl IntoView {
    // Category display order
    let category_order = ["language", "framework", "tool", "concept"];

    let mut sorted_tags: Vec<&TechTag> = tags.iter().collect();
    sorted_tags.sort_by(|a, b| {
        let a_pos = category_order
            .iter()
            .position(|&c| c == a.category)
            .unwrap_or(99);
        let b_pos = category_order
            .iter()
            .position(|&c| c == b.category)
            .unwrap_or(99);
        a_pos.cmp(&b_pos).then(a.name.cmp(&b.name))
    });

    sorted_tags
        .into_iter()
        .map(|tag| {
            let name = tag.name.clone();
            view! { <Badge label=name/> }
        })
        .collect_view()
}
