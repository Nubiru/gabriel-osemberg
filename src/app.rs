use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::components::about_page::AboutPage;
use crate::components::hero::Hero;
use crate::components::layout::Layout;
use crate::components::pages::NotFoundPage;
use crate::components::project_detail::ProjectDetailPage;
use crate::components::projects_page::ProjectsPage;
use crate::components::scroll_reveal::ScrollRevealInit;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                // Font preloads — critical for avoiding FOUT
                <link
                    rel="preload"
                    href="/fonts/inter-latin-variable.woff2"
                    as_="font"
                    type_="font/woff2"
                    crossorigin="anonymous"
                />
                <link
                    rel="preload"
                    href="/fonts/space-grotesk-latin-variable.woff2"
                    as_="font"
                    type_="font/woff2"
                    crossorigin="anonymous"
                />

                // Anti-FOUC: apply dark class before any paint
                <script>
                    r#"(function(){
                        var d=document.documentElement;
                        var t=localStorage.getItem('theme');
                        if(t==='light'){d.classList.remove('dark')}
                        else if(t==='dark'){d.classList.add('dark')}
                        else{
                            if(!window.matchMedia('(prefers-color-scheme: dark)').matches){
                                d.classList.remove('dark')
                            }
                        }
                    })()"#
                </script>

                // JSON-LD structured data — Person schema for rich search results
                <script type_="application/ld+json">
                    r#"{
                        "@context": "https://schema.org",
                        "@type": "Person",
                        "name": "Gabriel Osemberg",
                        "jobTitle": "AI-Augmented Engineer",
                        "url": "https://gabriel-osemberg.fly.dev",
                        "knowsAbout": ["Rust", "WebAssembly", "C", "TypeScript", "React", "AI Engineering"],
                        "sameAs": ["https://github.com/Nubiru"]
                    }"#
                </script>

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-surface-base text-text-primary font-body">
                <App/>

                // GoatCounter analytics — privacy-first, no cookies, GDPR-compliant
                // Create account at https://www.goatcounter.com then update the data-goatcounter URL
                <script
                    data-goatcounter="https://gabriel-osemberg.goatcounter.com/count"
                    async_=""
                    src="//gc.zgo.at/count.js"
                />
                <noscript>
                    <img src="https://gabriel-osemberg.goatcounter.com/count?p=/noscript" alt=""/>
                </noscript>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/gabriel-osemberg.css"/>

        <Title text="Gabriel Osemberg — AI-Augmented Engineer"/>
        <Meta name="description" content="AI-Augmented Engineer building real systems with Rust, WebAssembly, C, and 95.9% test coverage. Portfolio showcasing 5 production projects."/>

        // Open Graph defaults (pages can override with their own <Meta> tags)
        <Meta property="og:type" content="website"/>
        <Meta property="og:site_name" content="Gabriel Osemberg"/>
        <Meta property="og:title" content="Gabriel Osemberg — AI-Augmented Engineer"/>
        <Meta property="og:description" content="AI-Augmented Engineer building real systems with Rust, WebAssembly, C, and 95.9% test coverage."/>
        <Meta property="og:url" content="https://gabriel-osemberg.fly.dev"/>

        // Twitter card defaults
        <Meta name="twitter:card" content="summary"/>
        <Meta name="twitter:title" content="Gabriel Osemberg — AI-Augmented Engineer"/>
        <Meta name="twitter:description" content="AI-Augmented Engineer building real systems with Rust, WebAssembly, C, and 95.9% test coverage."/>

        <Link rel="icon" href="/favicon.ico"/>

        <Router>
            <Layout>
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("about") view=AboutPage/>
                    <Route path=StaticSegment("projects") view=ProjectsPage/>
                    <Route path=(StaticSegment("projects"), ParamSegment("slug")) view=ProjectDetailPage/>
                </Routes>
            </Layout>
        </Router>

        // Initialize scroll reveals after DOM is ready
        <ScrollRevealInit/>
    }
}

/// Home page — hero section with key highlights.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Hero/>
    }
}
