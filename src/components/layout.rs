use leptos::prelude::*;

use crate::components::mobile_menu::{HamburgerButton, MobileMenu};
use crate::components::theme_toggle::ThemeToggle;

/// Main layout shell — wraps all page content with header, main, and footer.
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-surface-base text-text-primary transition-colors duration-300">
            // Skip-to-content link (accessibility)
            <a
                href="#main-content"
                class="sr-only focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-[100]
                       focus:px-4 focus:py-2 focus:rounded-md focus:bg-accent focus:text-text-inverse
                       focus:text-sm focus:font-medium"
            >
                "Skip to main content"
            </a>
            <Header/>
            <main id="main-content" class="flex-1 w-full max-w-5xl mx-auto px-6 py-12">
                {children()}
            </main>
            <Footer/>
        </div>
    }
}

/// Site header — logo/name, desktop nav, mobile hamburger, and theme toggle.
#[component]
fn Header() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    let toggle_menu = move || set_menu_open.update(|v| *v = !*v);
    let close_menu = move || set_menu_open.set(false);

    view! {
        <header class="sticky top-0 z-50 border-b border-border-default bg-surface-base/80 backdrop-blur-sm transition-colors duration-300">
            <div class="max-w-5xl mx-auto px-6 h-16 flex items-center justify-between">
                <a
                    href="/"
                    class="font-display text-xl font-semibold text-text-primary hover:text-accent transition-colors"
                >
                    "Gabriel Osemberg"
                </a>

                // Desktop navigation
                <nav
                    class="hidden md:flex items-center gap-8"
                    aria-label="Main navigation"
                >
                    <NavLink href="/" label="Home"/>
                    <NavLink href="/projects" label="Projects"/>
                    <NavLink href="/about" label="About"/>
                </nav>

                // Right side: theme toggle + mobile hamburger
                <div class="flex items-center gap-2">
                    <ThemeToggle/>
                    <HamburgerButton is_open=menu_open on_toggle=toggle_menu/>
                </div>
            </div>
        </header>

        // Mobile slide-out menu (rendered outside header for z-index stacking)
        <MobileMenu is_open=menu_open on_close=close_menu/>
    }
}

/// A navigation link with hover styling.
#[component]
fn NavLink(#[prop(into)] href: String, #[prop(into)] label: String) -> impl IntoView {
    view! {
        <a
            href=href
            class="text-sm font-medium text-text-secondary hover:text-text-primary transition-colors"
        >
            {label}
        </a>
    }
}

/// Site footer — links, built-with, and copyright.
#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-border-default bg-surface-base transition-colors duration-300">
            <div class="max-w-5xl mx-auto px-6 py-8 flex flex-col md:flex-row items-center justify-between gap-4">
                <p class="text-sm text-text-muted">"Built with Rust + Leptos + WebAssembly"</p>
                <div class="flex items-center gap-6">
                    <a
                        href="https://github.com/Nubiru"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm text-text-secondary hover:text-accent transition-colors"
                        aria-label="GitHub profile"
                    >
                        "GitHub"
                    </a>
                    <a
                        href="https://linkedin.com/in/gabriel-osemberg"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm text-text-secondary hover:text-accent transition-colors"
                        aria-label="LinkedIn profile"
                    >
                        "LinkedIn"
                    </a>
                </div>
            </div>
        </footer>
    }
}
