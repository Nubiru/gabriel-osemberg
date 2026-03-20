use leptos::prelude::*;

/// Wraps children in a container that fades in when scrolled into view.
///
/// Uses CSS classes to trigger animation — the actual Intersection Observer
/// hookup happens client-side via a small inline script. This avoids adding
/// web-sys dependencies for a simple reveal pattern.
///
/// The element starts with `opacity-0 translate-y-4` and transitions to
/// `opacity-100 translate-y-0` when the `.revealed` class is added.
#[component]
pub fn ScrollReveal(
    children: Children,
    /// Additional CSS classes on the wrapper div.
    #[prop(default = "")]
    class: &'static str,
    /// Animation delay in milliseconds.
    #[prop(default = 0)]
    delay: u32,
) -> impl IntoView {
    let delay_style = if delay > 0 {
        format!("transition-delay: {}ms", delay)
    } else {
        String::new()
    };

    view! {
        <div
            class=format!(
                "scroll-reveal opacity-0 translate-y-4
                 transition-all duration-500 ease-fluid
                 motion-reduce:opacity-100 motion-reduce:translate-y-0 motion-reduce:transition-none
                 {class}",
            )
            style=delay_style
        >
            {children()}
        </div>
    }
}

/// Inline script that sets up Intersection Observer for all `.scroll-reveal` elements.
/// Include this once in the page (e.g., at the bottom of the Layout or App component).
#[component]
pub fn ScrollRevealInit() -> impl IntoView {
    view! {
        <script>
            r#"(function(){
                if(!('IntersectionObserver' in window)) {
                    document.querySelectorAll('.scroll-reveal').forEach(function(el){
                        el.classList.add('revealed');
                    });
                    return;
                }
                var observer = new IntersectionObserver(function(entries){
                    entries.forEach(function(entry){
                        if(entry.isIntersecting){
                            entry.target.classList.add('revealed');
                            observer.unobserve(entry.target);
                        }
                    });
                }, { threshold: 0.1, rootMargin: '0px 0px -40px 0px' });
                document.querySelectorAll('.scroll-reveal').forEach(function(el){
                    observer.observe(el);
                });
            })()"#
        </script>
    }
}
