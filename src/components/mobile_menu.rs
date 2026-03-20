use leptos::prelude::*;

/// Hamburger button that toggles the mobile menu.
#[component]
pub fn HamburgerButton(
    is_open: ReadSignal<bool>,
    on_toggle: impl Fn() + 'static + Clone,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| on_toggle()
            class="md:hidden inline-flex items-center justify-center w-10 h-10 rounded-md
                   text-text-secondary hover:text-text-primary hover:bg-surface-raised
                   transition-colors duration-200
                   focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            aria-label=move || if is_open.get() { "Close menu" } else { "Open menu" }
            aria-expanded=move || is_open.get().to_string()
            aria-controls="mobile-menu"
        >
            <svg
                class="w-6 h-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="2"
                aria-hidden="true"
            >
                {move || {
                    if is_open.get() {
                        // X icon
                        view! {
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        }
                            .into_any()
                    } else {
                        // Hamburger icon
                        view! {
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M4 6h16M4 12h16M4 18h16"
                            />
                        }
                            .into_any()
                    }
                }}
            </svg>
        </button>
    }
}

/// Mobile slide-out menu overlay.
#[component]
pub fn MobileMenu(
    is_open: ReadSignal<bool>,
    on_close: impl Fn() + 'static + Clone,
) -> impl IntoView {
    // Close on Escape key
    let on_close_esc = on_close.clone();
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close_esc();
        }
    };

    let on_close_link = on_close.clone();
    let on_close_backdrop = on_close.clone();

    view! {
        // Backdrop overlay
        <div
            class=move || {
                format!(
                    "fixed inset-0 z-40 bg-surface-sunken/60 backdrop-blur-sm transition-opacity duration-300 md:hidden {}",
                    if is_open.get() { "opacity-100" } else { "opacity-0 pointer-events-none" },
                )
            }
            on:click=move |_| on_close_backdrop()
            aria-hidden="true"
        ></div>

        // Slide-out drawer
        <nav
            id="mobile-menu"
            class=move || {
                format!(
                    "fixed top-0 right-0 z-50 h-full w-72 max-w-[80vw]
                     bg-surface-raised border-l border-border-default
                     shadow-xl transition-transform duration-300 ease-snappy md:hidden
                     flex flex-col {}",
                    if is_open.get() { "translate-x-0" } else { "translate-x-full" },
                )
            }
            on:keydown=on_keydown
            aria-label="Mobile navigation"
            role="dialog"
            aria-modal=move || is_open.get().to_string()
        >
            // Close button inside drawer
            <div class="flex items-center justify-end h-16 px-6 border-b border-border-subtle">
                <button
                    on:click={
                        let on_close = on_close.clone();
                        move |_| on_close()
                    }
                    class="inline-flex items-center justify-center w-10 h-10 rounded-md
                           text-text-secondary hover:text-text-primary hover:bg-surface-overlay
                           transition-colors duration-200
                           focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                    aria-label="Close menu"
                >
                    <svg
                        class="w-6 h-6"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="2"
                        aria-hidden="true"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M6 18L18 6M6 6l12 12"
                        />
                    </svg>
                </button>
            </div>

            // Navigation links
            <div class="flex-1 flex flex-col gap-1 px-4 py-6">
                <MobileNavLink href="/" label="Home" on_click=on_close_link.clone()/>
                <MobileNavLink href="/projects" label="Projects" on_click=on_close_link.clone()/>
                <MobileNavLink href="/about" label="About" on_click=on_close_link.clone()/>
            </div>

            // Footer in drawer
            <div class="px-6 py-4 border-t border-border-subtle">
                <p class="text-xs text-text-muted">"Gabriel Osemberg"</p>
            </div>
        </nav>
    }
}

/// A navigation link for the mobile menu.
#[component]
fn MobileNavLink(
    #[prop(into)] href: String,
    #[prop(into)] label: String,
    on_click: impl Fn() + 'static + Clone,
) -> impl IntoView {
    view! {
        <a
            href=href
            on:click=move |_| on_click()
            class="block px-4 py-3 rounded-lg text-base font-medium
                   text-text-secondary hover:text-text-primary hover:bg-surface-overlay
                   transition-colors duration-200"
        >
            {label}
        </a>
    }
}
