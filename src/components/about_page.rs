use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::loading::{Skeleton, SkeletonCard};
use crate::components::scroll_reveal::ScrollReveal;
use crate::components::skills::SkillsDisplay;
use crate::components::timeline::{Timeline, TimelineEntry};
use crate::components::ui::SectionHeading;
use crate::models::{CvSection, Experience};
use crate::server_fns::{get_cv_sections, get_experiences, get_skills};

/// About page — single scrollable page with all identity sections.
///
/// Sections: About narrative, Experience timeline, Skills grid,
/// Methodology, Military, Education, Languages, Contact.
#[component]
pub fn AboutPage() -> impl IntoView {
    let cv_sections = Resource::new(|| (), |_| get_cv_sections());
    let experiences = Resource::new(|| (), |_| get_experiences());
    let skills = Resource::new(|| (), |_| get_skills());

    view! {
        <Title text="About — Gabriel Osemberg"/>

        <div class="py-12 sm:py-16 lg:py-20 space-y-16 sm:space-y-20">
            // About narrative
            <Suspense fallback=move || view! { <AboutSkeleton/> }>
                {move || {
                    cv_sections
                        .get()
                        .map(|result| {
                            match result {
                                Ok(sections) => {
                                    view! { <CvSectionsView sections=sections/> }.into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <p class="text-state-error text-sm">
                                            {format!("Failed to load content: {e}")}
                                        </p>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>

            // Experience timeline
            <Suspense fallback=move || {
                view! {
                    <section>
                        <SectionHeading title="Experience".to_string()/>
                        <div class="space-y-6">
                            <SkeletonCard/>
                            <SkeletonCard/>
                            <SkeletonCard/>
                        </div>
                    </section>
                }
            }>
                {move || {
                    experiences
                        .get()
                        .map(|result| {
                            match result {
                                Ok(exps) => {
                                    view! { <ExperienceSection experiences=exps/> }.into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <p class="text-state-error text-sm">
                                            {format!("Failed to load experiences: {e}")}
                                        </p>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>

            // Skills grid
            <Suspense fallback=move || {
                view! {
                    <section>
                        <SectionHeading title="Technical Skills".to_string()/>
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                            <Skeleton height="h-24"/>
                            <Skeleton height="h-24"/>
                            <Skeleton height="h-24"/>
                            <Skeleton height="h-24"/>
                        </div>
                    </section>
                }
            }>
                {move || {
                    skills
                        .get()
                        .map(|result| {
                            match result {
                                Ok(sk) => {
                                    view! { <SkillsSection skills=sk/> }.into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <p class="text-state-error text-sm">
                                            {format!("Failed to load skills: {e}")}
                                        </p>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>

            // Contact
            <ScrollReveal>
                <ContactSection/>
            </ScrollReveal>
        </div>
    }
}

/// Renders all CV sections in order: about, methodology, writing, philosophy, military, education, languages.
#[component]
fn CvSectionsView(sections: Vec<CvSection>) -> impl IntoView {
    let about = sections.iter().find(|s| s.section_type == "about").cloned();
    let methodology = sections
        .iter()
        .find(|s| s.section_type == "methodology")
        .cloned();
    let writing = sections
        .iter()
        .find(|s| s.section_type == "writing")
        .cloned();
    let philosophy = sections
        .iter()
        .find(|s| s.section_type == "philosophy")
        .cloned();
    let military = sections
        .iter()
        .find(|s| s.section_type == "military")
        .cloned();
    let education = sections
        .iter()
        .find(|s| s.section_type == "education")
        .cloned();
    let languages = sections
        .iter()
        .find(|s| s.section_type == "languages")
        .cloned();

    view! {
        // About section — the hook
        {about
            .map(|s| {
                view! {
                    <ScrollReveal>
                        <section aria-label="About">
                            <SectionHeading
                                title="About Me".to_string()
                                subtitle="AI-Augmented Software Engineer".to_string()
                            />
                            <div class="prose-content max-w-3xl">
                                <ContentRenderer content=s.content.clone()/>
                            </div>
                        </section>
                    </ScrollReveal>
                }
            })}

        // Methodology — the differentiator
        {methodology
            .map(|s| {
                view! {
                    <ScrollReveal delay=100>
                        <section aria-label="Engineering Methodology" class="mt-16 sm:mt-20">
                            <SectionHeading title=s.title.clone()/>
                            <div class="prose-content max-w-3xl">
                                <ContentRenderer content=s.content.clone()/>
                            </div>
                        </section>
                    </ScrollReveal>
                }
            })}

        // Technical Writing
        {writing
            .map(|s| {
                view! {
                    <ScrollReveal delay=100>
                        <section aria-label="Technical Writing" class="mt-16 sm:mt-20">
                            <SectionHeading title=s.title.clone()/>
                            <div class="prose-content max-w-3xl">
                                <ContentRenderer content=s.content.clone()/>
                            </div>
                        </section>
                    </ScrollReveal>
                }
            })}

        // Philosophy
        {philosophy
            .map(|s| {
                view! {
                    <ScrollReveal delay=150>
                        <section aria-label="Engineering Philosophy" class="mt-16 sm:mt-20">
                            <SectionHeading title=s.title.clone()/>
                            <div class="prose-content max-w-3xl">
                                <ContentRenderer content=s.content.clone()/>
                            </div>
                        </section>
                    </ScrollReveal>
                }
            })}

        // Military service
        {military
            .map(|s| {
                view! {
                    <ScrollReveal delay=100>
                        <section aria-label="Military Service" class="mt-16 sm:mt-20">
                            <SectionHeading title=s.title.clone()/>
                            <div class="prose-content max-w-3xl">
                                <ContentRenderer content=s.content.clone()/>
                            </div>
                        </section>
                    </ScrollReveal>
                }
            })}

        // Education
        {education
            .map(|s| {
                view! {
                    <ScrollReveal delay=100>
                        <section aria-label="Education" class="mt-16 sm:mt-20">
                            <SectionHeading title=s.title.clone()/>
                            <div class="prose-content max-w-3xl">
                                <ContentRenderer content=s.content.clone()/>
                            </div>
                        </section>
                    </ScrollReveal>
                }
            })}

        // Languages
        {languages
            .map(|s| {
                view! {
                    <ScrollReveal delay=100>
                        <section aria-label="Languages" class="mt-16 sm:mt-20">
                            <SectionHeading title=s.title.clone()/>
                            <div class="prose-content max-w-3xl">
                                <ContentRenderer content=s.content.clone()/>
                            </div>
                        </section>
                    </ScrollReveal>
                }
            })}
    }
}

/// Renders markdown-like content with basic formatting.
///
/// Handles **bold** markers and paragraph breaks. For full markdown
/// rendering, we would add pulldown-cmark — but for seed content
/// with known formatting, this is sufficient for L0.
#[component]
fn ContentRenderer(#[prop(into)] content: String) -> impl IntoView {
    let paragraphs: Vec<String> = content
        .split("\n\n")
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();

    view! {
        <div class="space-y-4">
            {paragraphs
                .into_iter()
                .map(|p| {
                    view! {
                        <p
                            class="text-text-secondary leading-relaxed"
                            inner_html=render_inline_markdown(&p)
                        ></p>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Minimal inline markdown: **bold** and line breaks.
fn render_inline_markdown(text: &str) -> String {
    let mut result = String::with_capacity(text.len() + 64);
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'*') {
            // Consume second *
            chars.next();
            // Find closing **
            let mut bold_content = String::new();
            let mut found_close = false;
            while let Some(inner) = chars.next() {
                if inner == '*' && chars.peek() == Some(&'*') {
                    chars.next();
                    found_close = true;
                    break;
                }
                bold_content.push(inner);
            }
            if found_close {
                result.push_str("<strong class=\"text-text-primary font-medium\">");
                result.push_str(&html_escape(&bold_content));
                result.push_str("</strong>");
            } else {
                result.push_str("**");
                result.push_str(&html_escape(&bold_content));
            }
        } else if ch == '\n' {
            result.push_str("<br/>");
        } else {
            // Escape HTML entities
            match ch {
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '&' => result.push_str("&amp;"),
                '"' => result.push_str("&quot;"),
                _ => result.push(ch),
            }
        }
    }

    result
}

/// Escape HTML special characters.
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Experience timeline section.
#[component]
fn ExperienceSection(experiences: Vec<Experience>) -> impl IntoView {
    if experiences.is_empty() {
        return view! {
            <section aria-label="Experience">
                <SectionHeading
                    title="Experience".to_string()
                    subtitle="Experience data coming soon.".to_string()
                />
            </section>
        }
        .into_any();
    }

    view! {
        <ScrollReveal>
            <section aria-label="Experience">
                <SectionHeading title="Experience".to_string()/>
                <Timeline>
                    {experiences
                        .into_iter()
                        .map(|exp| {
                            view! { <ExperienceTimelineEntry exp=exp/> }
                        })
                        .collect_view()}
                </Timeline>
            </section>
        </ScrollReveal>
    }
    .into_any()
}

/// Renders a single Experience as a TimelineEntry, handling Option props correctly.
///
/// Leptos's `#[prop(optional, into)]` on `Option<String>` expects `impl Into<String>`,
/// not `impl Into<Option<String>>`. We must conditionally provide these props by
/// rendering different view branches — not by passing empty strings, which would
/// break the "Present" display and current-position highlighting.
#[component]
fn ExperienceTimelineEntry(exp: Experience) -> impl IntoView {
    let highlights: Vec<String> = serde_json::from_str(&exp.highlights).unwrap_or_default();
    let role = exp.role;
    let company = exp.company;
    let start_date = exp.start_date;
    let description = exp.description;

    match (exp.company_url, exp.end_date) {
        (Some(url), Some(end)) => view! {
            <TimelineEntry
                role=role company=company company_url=url
                start_date=start_date end_date=end
                description=description highlights=highlights
            />
        }
        .into_any(),
        (Some(url), None) => view! {
            <TimelineEntry
                role=role company=company company_url=url
                start_date=start_date
                description=description highlights=highlights
            />
        }
        .into_any(),
        (None, Some(end)) => view! {
            <TimelineEntry
                role=role company=company
                start_date=start_date end_date=end
                description=description highlights=highlights
            />
        }
        .into_any(),
        (None, None) => view! {
            <TimelineEntry
                role=role company=company
                start_date=start_date
                description=description highlights=highlights
            />
        }
        .into_any(),
    }
}

/// Skills section — uses the shared SkillsDisplay component.
#[component]
fn SkillsSection(skills: Vec<crate::models::Skill>) -> impl IntoView {
    // Filter out spoken languages — they have their own cv_section
    let tech_skills: Vec<crate::models::Skill> = skills
        .into_iter()
        .filter(|s| s.category != "spoken_language")
        .collect();

    view! {
        <ScrollReveal>
            <section aria-label="Technical Skills">
                <SectionHeading title="Technical Skills".to_string()/>
                <SkillsDisplay skills=tech_skills/>
            </section>
        </ScrollReveal>
    }
}

/// Contact section — static info display.
#[component]
fn ContactSection() -> impl IntoView {
    view! {
        <section aria-label="Contact">
            <SectionHeading
                title="Get In Touch".to_string()
                subtitle="Open to opportunities at companies building AI responsibly."
                    .to_string()
            />
            <div class="flex flex-wrap gap-6">
                <ContactLink
                    href="mailto:osemberg.gabi@gmail.com"
                    label="osemberg.gabi@gmail.com"
                    icon="email"
                />
                <ContactLink
                    href="https://github.com/Nubiru"
                    label="GitHub"
                    icon="github"
                />
                <ContactLink
                    href="https://linkedin.com/in/gabrielosem"
                    label="LinkedIn"
                    icon="linkedin"
                />
            </div>
        </section>
    }
}

/// A single contact link with icon.
#[component]
fn ContactLink(
    #[prop(into)] href: String,
    #[prop(into)] label: String,
    #[prop(into)] icon: String,
) -> impl IntoView {
    let is_external = !href.starts_with("mailto:");

    view! {
        <a
            href=href
            target=if is_external { "_blank" } else { "" }
            rel=if is_external { "noopener noreferrer" } else { "" }
            class="inline-flex items-center gap-2 px-4 py-2.5 rounded-lg
                   text-sm font-medium text-text-primary
                   bg-surface-raised border border-border-default
                   hover:border-border-focus hover:bg-surface-overlay
                   transition-all duration-200
                   focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
        >
            <ContactIcon icon_type=icon/>
            {label}
        </a>
    }
}

/// Simple SVG icons for contact links.
#[component]
fn ContactIcon(#[prop(into)] icon_type: String) -> impl IntoView {
    match icon_type.as_str() {
        "email" => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
            </svg>
        }
            .into_any(),
        "github" => view! {
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/>
            </svg>
        }
            .into_any(),
        "linkedin" => view! {
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
            </svg>
        }
            .into_any(),
        _ => view! { <span></span> }.into_any(),
    }
}

/// Loading skeleton for the about page sections.
#[component]
fn AboutSkeleton() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <Skeleton height="h-8" width="w-48"/>
            <div class="space-y-3">
                <Skeleton height="h-4" width="w-full"/>
                <Skeleton height="h-4" width="w-5/6"/>
                <Skeleton height="h-4" width="w-4/5"/>
            </div>
        </div>
    }
}
