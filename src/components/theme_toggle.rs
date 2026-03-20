use leptos::prelude::*;

/// Three-way theme toggle: Light / System / Dark.
///
/// Uses localStorage for persistence and applies the `.dark` class on `<html>`.
/// The actual theme detection and anti-FOUC logic lives in the inline script
/// injected in the document `<head>` (see `shell()` in app.rs).
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = signal("system".to_string());

    // On mount, read the stored preference from localStorage
    #[cfg(feature = "hydrate")]
    {
        use leptos::task::spawn_local;
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(stored)) = storage.get_item("theme") {
                        set_theme.set(stored);
                    }
                }
            }
        });
    }

    let cycle_theme = move |_| {
        let next = match theme.get().as_str() {
            "system" => "dark",
            "dark" => "light",
            "light" => "system",
            _ => "system",
        };
        set_theme.set(next.to_string());
        apply_theme(next);
    };

    let label = move || match theme.get().as_str() {
        "dark" => "Dark",
        "light" => "Light",
        _ => "System",
    };

    let icon = move || match theme.get().as_str() {
        "dark" => "\u{263D}",  // ☽ moon
        "light" => "\u{2600}", // ☀ sun
        _ => "\u{25D0}",       // ◐ half circle
    };

    view! {
        <button
            on:click=cycle_theme
            class="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium rounded-md
                   text-text-secondary hover:text-text-primary
                   bg-surface-raised hover:bg-surface-overlay
                   border border-border-default
                   transition-colors duration-200
                   focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
            aria-label=move || format!("Theme: {}. Click to cycle.", label())
            title=move || format!("Theme: {}", label())
        >
            <span class="text-base" aria-hidden="true">{icon}</span>
            <span class="sr-only">{label}</span>
        </button>
    }
}

/// Apply the selected theme to the document and persist to localStorage.
fn apply_theme(theme: &str) {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let class_list = html.class_list();
                    match theme {
                        "dark" => {
                            let _ = class_list.add_1("dark");
                        }
                        "light" => {
                            let _ = class_list.remove_1("dark");
                        }
                        _ => {
                            // System: follow prefers-color-scheme
                            let prefers_dark = window
                                .match_media("(prefers-color-scheme: dark)")
                                .ok()
                                .flatten()
                                .map(|mq| mq.matches())
                                .unwrap_or(false);
                            if prefers_dark {
                                let _ = class_list.add_1("dark");
                            } else {
                                let _ = class_list.remove_1("dark");
                            }
                        }
                    }
                }
            }

            // Persist to localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                if theme == "system" {
                    let _ = storage.remove_item("theme");
                } else {
                    let _ = storage.set_item("theme", theme);
                }
            }
        }
    }

    // Server-side: no-op
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = theme;
    }
}
