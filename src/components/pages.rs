use leptos::prelude::*;
use leptos_meta::Title;

/// Styled 404 Not Found page.
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="404 — Page Not Found | Gabriel Osemberg"/>
        <div class="flex flex-col items-center justify-center py-24 text-center">
            <p class="font-mono text-sm text-accent tracking-wide mb-4">"ERROR 404"</p>
            <h1 class="font-display text-7xl sm:text-8xl font-bold text-text-primary">"404"</h1>
            <p class="mt-6 text-lg text-text-secondary max-w-md">
                "The page you're looking for doesn't exist or has been moved."
            </p>
            <a
                href="/"
                class="mt-10 inline-flex items-center gap-2 px-5 py-2.5 rounded-lg font-medium text-sm
                       bg-accent text-text-inverse hover:bg-accent-hover
                       transition-colors duration-200
                       focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            >
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" aria-hidden="true">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
                </svg>
                "Back to home"
            </a>
        </div>
    }
}

/// Styled 500 Server Error page.
#[component]
pub fn ServerErrorPage() -> impl IntoView {
    view! {
        <Title text="500 — Server Error | Gabriel Osemberg"/>
        <div class="flex flex-col items-center justify-center py-24 text-center">
            <p class="font-mono text-sm text-state-error tracking-wide mb-4">"ERROR 500"</p>
            <h1 class="font-display text-7xl sm:text-8xl font-bold text-text-primary">"500"</h1>
            <p class="mt-6 text-lg text-text-secondary max-w-md">
                "Something went wrong on our end. Please try again."
            </p>
            <button
                on:click=move |_| {
                    #[cfg(feature = "hydrate")]
                    {
                        if let Some(window) = web_sys::window() {
                            let _ = window.location().reload();
                        }
                    }
                }
                class="mt-10 inline-flex items-center gap-2 px-5 py-2.5 rounded-lg font-medium text-sm
                       bg-accent text-text-inverse hover:bg-accent-hover
                       transition-colors duration-200
                       focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            >
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" aria-hidden="true">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                </svg>
                "Try again"
            </button>
        </div>
    }
}
