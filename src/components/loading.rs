use leptos::prelude::*;

/// A pulsing skeleton placeholder for loading content.
#[component]
pub fn Skeleton(
    #[prop(default = "h-4")] height: &'static str,
    #[prop(default = "w-full")] width: &'static str,
    #[prop(default = "rounded-md")] rounded: &'static str,
) -> impl IntoView {
    view! {
        <div
            class=format!(
                "{height} {width} {rounded} bg-surface-overlay animate-pulse-subtle",
            )
            role="status"
            aria-label="Loading"
        ></div>
    }
}

/// A card-shaped skeleton for loading project cards or content blocks.
#[component]
pub fn SkeletonCard() -> impl IntoView {
    view! {
        <div class="p-6 rounded-xl bg-surface-raised border border-border-subtle animate-pulse-subtle">
            <Skeleton height="h-5" width="w-3/4" rounded="rounded"/>
            <div class="mt-3 space-y-2">
                <Skeleton height="h-3" width="w-full" rounded="rounded"/>
                <Skeleton height="h-3" width="w-5/6" rounded="rounded"/>
            </div>
            <div class="mt-4 flex gap-2">
                <Skeleton height="h-6" width="w-16" rounded="rounded-full"/>
                <Skeleton height="h-6" width="w-16" rounded="rounded-full"/>
                <Skeleton height="h-6" width="w-16" rounded="rounded-full"/>
            </div>
        </div>
    }
}

/// A spinning loading indicator.
#[component]
pub fn Spinner(#[prop(default = "w-6 h-6")] size: &'static str) -> impl IntoView {
    view! {
        <svg
            class=format!("{size} animate-spin text-accent")
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            role="status"
            aria-label="Loading"
        >
            <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
            ></circle>
            <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
        </svg>
    }
}
