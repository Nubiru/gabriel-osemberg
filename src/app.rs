use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::layout::Layout;

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

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-surface-base text-text-primary font-body">
                <App/>
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

        <Link rel="icon" href="/favicon.ico"/>

        <Router>
            <Layout>
                <Routes fallback=|| {
                    view! {
                        <div class="flex flex-col items-center justify-center py-24">
                            <h1 class="font-display text-6xl font-bold text-text-primary">"404"</h1>
                            <p class="mt-4 text-text-secondary text-lg">"Page not found."</p>
                            <a href="/" class="mt-8 text-accent hover:text-accent-hover transition-colors">
                                "Back to home"
                            </a>
                        </div>
                    }
                }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </Layout>
        </Router>
    }
}

/// Home page — temporary content using design tokens to verify they work.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="py-16">
            <h1 class="font-display text-5xl font-bold tracking-tight text-text-primary">
                "Gabriel Osemberg"
            </h1>
            <p class="mt-4 text-xl text-text-secondary leading-relaxed max-w-2xl">
                "AI-Augmented Engineer building real systems with Rust, WebAssembly, and 95.9% test coverage."
            </p>
            <div class="mt-8 flex gap-4">
                <a
                    href="/projects"
                    class="inline-flex items-center px-5 py-2.5 rounded-lg font-medium text-sm
                           bg-accent text-text-inverse hover:bg-accent-hover
                           transition-colors duration-200
                           focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                >
                    "View Projects"
                </a>
                <a
                    href="/about"
                    class="inline-flex items-center px-5 py-2.5 rounded-lg font-medium text-sm
                           text-text-primary border border-border-default
                           hover:bg-surface-raised hover:border-border-focus
                           transition-colors duration-200
                           focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                >
                    "About Me"
                </a>
            </div>
        </section>

        // Design token verification section (temporary — will be replaced by real content)
        <section class="py-12 border-t border-border-default">
            <h2 class="font-display text-2xl font-semibold text-text-primary mb-6">
                "Design System Active"
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="p-6 rounded-xl bg-surface-raised border border-border-subtle">
                    <p class="font-mono text-sm text-accent">"font-mono"</p>
                    <p class="font-display text-lg font-semibold mt-2">"Space Grotesk Display"</p>
                    <p class="font-body text-sm text-text-secondary mt-1">"Inter body text"</p>
                </div>
                <div class="p-6 rounded-xl bg-surface-raised border border-border-subtle">
                    <p class="text-sm text-text-muted">"Surfaces"</p>
                    <div class="mt-2 flex gap-2">
                        <div class="w-8 h-8 rounded-md bg-surface-base border border-border-default" title="base"></div>
                        <div class="w-8 h-8 rounded-md bg-surface-raised border border-border-default" title="raised"></div>
                        <div class="w-8 h-8 rounded-md bg-surface-overlay border border-border-default" title="overlay"></div>
                        <div class="w-8 h-8 rounded-md bg-surface-sunken border border-border-default" title="sunken"></div>
                    </div>
                </div>
                <div class="p-6 rounded-xl bg-surface-raised border border-border-subtle">
                    <p class="text-sm text-text-muted">"Accents & States"</p>
                    <div class="mt-2 flex gap-2">
                        <div class="w-8 h-8 rounded-md bg-accent" title="accent"></div>
                        <div class="w-8 h-8 rounded-md bg-state-success" title="success"></div>
                        <div class="w-8 h-8 rounded-md bg-state-warning" title="warning"></div>
                        <div class="w-8 h-8 rounded-md bg-state-error" title="error"></div>
                    </div>
                </div>
            </div>
        </section>
    }
}
