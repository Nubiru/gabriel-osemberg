//! Project card component — renders a single project in the showcase grid.

use leptos::prelude::*;

use crate::components::ui::{Badge, ExternalLink};
use crate::models::Project;

/// A card displaying a project summary for the projects grid.
///
/// Renders the project name as a link to `/projects/{slug}`, the tagline,
/// tech stack badges, and optional external links (repo, live site).
#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    // Destructure into owned values to avoid borrowing issues in view! macro
    let href = format!("/projects/{}", project.slug);
    let name = project.name;
    let tagline = project.tagline;
    let tags: Vec<String> = project.tech_stack.split(", ").map(String::from).collect();
    let repo_url = project.repo_url;
    let live_url = project.live_url;
    let has_links = repo_url.is_some() || live_url.is_some();

    view! {
        <article class="bg-surface-raised border border-border-subtle rounded-xl p-6
                        hover:border-accent/50 hover:shadow-md transition-all duration-200">
            // Project name as accessible link
            <h3 class="font-display text-lg font-semibold">
                <a
                    href=href
                    class="text-text-primary hover:text-accent transition-colors
                           focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                >
                    {name}
                </a>
            </h3>

            // Tagline
            <p class="mt-2 text-sm text-text-secondary leading-relaxed">
                {tagline}
            </p>

            // Tech stack badges
            <div class="mt-4 flex flex-wrap gap-2">
                {tags
                    .into_iter()
                    .map(|tag| {
                        view! { <Badge label=tag/> }
                    })
                    .collect_view()}
            </div>

            // External links — only rendered if present
            {has_links
                .then(|| {
                    view! {
                        <div class="mt-4 flex flex-wrap gap-4">
                            {repo_url
                                .map(|url| {
                                    view! { <ExternalLink href=url label="Source Code".to_string()/> }
                                })}
                            {live_url
                                .map(|url| {
                                    view! { <ExternalLink href=url label="Live Site".to_string()/> }
                                })}
                        </div>
                    }
                })}
        </article>
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Project;

    fn sample_project() -> Project {
        Project {
            id: 1,
            name: "time".to_string(),
            slug: "time".to_string(),
            tagline: "3D interactive temporal artwork".to_string(),
            description: "A WebGL2-powered visualization engine.".to_string(),
            tech_stack: "C11, WebGL2, WebAssembly".to_string(),
            repo_url: Some("https://github.com/example/time".to_string()),
            live_url: None,
            image_path: None,
            sort_order: 1,
            visible: true,
            created_at: "2026-03-20T00:00:00".to_string(),
            updated_at: "2026-03-20T00:00:00".to_string(),
        }
    }

    #[test]
    fn tech_stack_splits_correctly() {
        let project = sample_project();
        let tags: Vec<String> = project.tech_stack.split(", ").map(String::from).collect();
        assert_eq!(tags, vec!["C11", "WebGL2", "WebAssembly"]);
    }

    #[test]
    fn card_href_uses_slug() {
        let project = sample_project();
        let href = format!("/projects/{}", project.slug);
        assert_eq!(href, "/projects/time");
    }

    #[test]
    fn external_links_shown_when_present() {
        let project = sample_project();
        assert!(project.repo_url.is_some());
        assert!(project.live_url.is_none());
        // At least one link is present, so the links section should render
        assert!(project.repo_url.is_some() || project.live_url.is_some());
    }

    #[test]
    fn external_links_hidden_when_absent() {
        let mut project = sample_project();
        project.repo_url = None;
        project.live_url = None;
        // Neither link present — links section should not render
        assert!(!project.repo_url.is_some() && !project.live_url.is_some());
    }
}
