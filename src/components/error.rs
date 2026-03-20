use leptos::prelude::*;

/// A styled error display component.
#[component]
pub fn ErrorDisplay(
    #[prop(into)] title: String,
    #[prop(into)] message: String,
    #[prop(default = false)] retry: bool,
) -> impl IntoView {
    view! {
        <div
            class="p-6 rounded-xl bg-surface-raised border border-state-error/30 text-center"
            role="alert"
        >
            <div class="text-state-error text-3xl mb-3" aria-hidden="true">
                "!"
            </div>
            <h3 class="font-display text-lg font-semibold text-text-primary">{title}</h3>
            <p class="mt-2 text-sm text-text-secondary">{message}</p>
            {retry.then(|| {
                view! {
                    <button
                        on:click=move |_| {
                            // Reload the current page
                            #[cfg(feature = "hydrate")]
                            {
                                if let Some(window) = web_sys::window() {
                                    if let Ok(location) = window.location().href() {
                                        let _ = window.location().set_href(&location);
                                    }
                                }
                            }
                        }
                        class="mt-4 inline-flex items-center px-4 py-2 rounded-md text-sm font-medium
                               bg-accent text-text-inverse hover:bg-accent-hover
                               transition-colors duration-200
                               focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                    >
                        "Try again"
                    </button>
                }
            })}
        </div>
    }
}
