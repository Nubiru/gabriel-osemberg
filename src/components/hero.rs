use leptos::prelude::*;

/// Hero section — the first thing visitors see. Must make a strong impression within 6 seconds.
///
/// Communicates: Gabriel Osemberg, AI-Augmented Engineer, builds real systems.
/// CTAs: View Projects, About Me.
#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative py-20 sm:py-28 lg:py-36">
            // Subtle background gradient
            <div
                class="absolute inset-0 -z-10 overflow-hidden"
                aria-hidden="true"
            >
                <div class="absolute -top-40 -right-40 w-80 h-80 rounded-full
                            bg-accent/5 blur-3xl"></div>
                <div class="absolute -bottom-20 -left-20 w-60 h-60 rounded-full
                            bg-accent/3 blur-3xl"></div>
            </div>

            // Intro label
            <p class="font-mono text-sm text-accent tracking-wide animate-fade-in">
                "AI-Augmented Engineer"
            </p>

            // Name — large, impactful
            <h1 class="mt-4 font-display text-5xl sm:text-6xl lg:text-7xl font-bold
                        tracking-tight text-text-primary animate-fade-up"
                style="animation-delay: 100ms"
            >
                "Gabriel Osemberg"
            </h1>

            // Value proposition
            <p class="mt-6 text-lg sm:text-xl text-text-secondary leading-relaxed max-w-2xl
                       animate-fade-up"
                style="animation-delay: 200ms"
            >
                "I build real systems — from a "
                <span class="text-text-primary font-medium">"90,000-line C/WebGL temporal artwork"</span>
                " to "
                <span class="text-text-primary font-medium">"production e-commerce platforms"</span>
                ". Now learning Rust because the easy path isn't the impressive path."
            </p>

            // Stat highlights — social proof in numbers
            <div class="mt-10 flex flex-wrap gap-x-8 gap-y-4 animate-fade-up"
                style="animation-delay: 300ms"
            >
                <HeroStat value="90K+" label="Lines of C"/>
                <HeroStat value="95.9%" label="Test Coverage"/>
                <HeroStat value="5" label="Production Projects"/>
                <HeroStat value="3" label="Languages"/>
            </div>

            // CTAs
            <div class="mt-12 flex flex-wrap gap-4 animate-fade-up"
                style="animation-delay: 400ms"
            >
                <a
                    href="/projects"
                    class="inline-flex items-center gap-2 px-6 py-3 rounded-lg font-medium text-sm
                           bg-accent text-text-inverse hover:bg-accent-hover
                           shadow-md hover:shadow-lg
                           transition-all duration-200
                           focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                >
                    "View Projects"
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3"/>
                    </svg>
                </a>
                <a
                    href="/about"
                    class="inline-flex items-center px-6 py-3 rounded-lg font-medium text-sm
                           text-text-primary border border-border-default
                           hover:bg-surface-raised hover:border-border-focus
                           transition-all duration-200
                           focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
                >
                    "About Me"
                </a>
            </div>
        </section>
    }
}

/// A single stat highlight in the hero section.
#[component]
fn HeroStat(#[prop(into)] value: &'static str, #[prop(into)] label: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <span class="font-display text-2xl sm:text-3xl font-bold text-text-primary">
                {value}
            </span>
            <span class="text-sm text-text-muted">{label}</span>
        </div>
    }
}
