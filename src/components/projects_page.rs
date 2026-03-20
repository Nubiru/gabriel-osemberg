//! Projects index page — displays all visible projects in a responsive grid.

use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::error::ErrorDisplay;
use crate::components::loading::SkeletonCard;
use crate::components::project_card::ProjectCard;
use crate::components::ui::SectionHeading;
use crate::server_fns::get_projects;

/// The `/projects` route. Fetches all visible projects and renders them
/// in a responsive grid of `ProjectCard` components.
#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects = Resource::new(|| (), |_| get_projects());

    view! {
        <Title text="Projects — Gabriel Osemberg"/>

        <section class="py-12">
            <SectionHeading
                title="Projects"
                subtitle="Real systems built with intention — from temporal artwork to production platforms."
            />

            <Suspense fallback=move || {
                view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <SkeletonCard/>
                        <SkeletonCard/>
                        <SkeletonCard/>
                    </div>
                }
            }>
                {move || {
                    projects
                        .get()
                        .map(|result| {
                            match result {
                                Ok(data) => {
                                    view! {
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                            {data
                                                .into_iter()
                                                .map(|project| {
                                                    view! { <ProjectCard project=project/> }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <ErrorDisplay
                                            title="Failed to load projects"
                                            message=e.to_string()
                                            retry=true
                                        />
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}
