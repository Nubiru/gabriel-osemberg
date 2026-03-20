//! SVG diagram of the MEGA multi-agent collaboration framework.
//!
//! Renders a visual representation of Gabriel's AI-augmented engineering
//! methodology: a central coordinator (MEGA) managing parallel streams
//! that each follow a Writer → Checker → Maintainer pipeline.

use leptos::prelude::*;

/// A visual SVG diagram showing the MEGA framework architecture.
///
/// Displays the multi-session orchestration pattern:
/// Gabriel (Owner) → MEGA (Coordinator) → 5 parallel streams,
/// each with a Writer → Checker → Maintainer subagent pipeline.
#[component]
pub fn FrameworkDiagram() -> impl IntoView {
    let streams = ["SHOWCASE", "DATA", "DESIGN", "IDENTITY", "INFRA"];

    view! {
        <div class="w-full max-w-2xl mx-auto" role="img" aria-label="MEGA multi-agent collaboration framework diagram">
            <svg viewBox="0 0 600 380" class="w-full h-auto" xmlns="http://www.w3.org/2000/svg">
                // --- Connection lines (drawn first, behind nodes) ---

                // Gabriel → MEGA
                <line x1="300" y1="40" x2="300" y2="90"
                    style="stroke: var(--color-border-default); stroke-width: 2;"/>

                // MEGA → each stream
                {streams.iter().enumerate().map(|(i, _)| {
                    let x = stream_x(i);
                    view! {
                        <line x1="300" y1="130" x2=x.to_string() y2="200"
                            style="stroke: var(--color-border-default); stroke-width: 1.5;"/>
                    }
                }).collect_view()}

                // Each stream → pipeline
                {streams.iter().enumerate().map(|(i, _)| {
                    let x = stream_x(i);
                    view! {
                        <line x1=x.to_string() y1="240" x2=x.to_string() y2="280"
                            style="stroke: var(--color-border-subtle); stroke-width: 1; stroke-dasharray: 4 3;"/>
                    }
                }).collect_view()}

                // --- Nodes ---

                // Gabriel (Owner)
                <rect x="225" y="12" width="150" height="32" rx="8"
                    style="fill: var(--color-accent); stroke: none;"/>
                <text x="300" y="33" text-anchor="middle"
                    style="fill: var(--color-text-inverse); font-size: 13px; font-weight: 600; font-family: var(--font-display);">
                    "Gabriel (Owner)"
                </text>

                // MEGA (Coordinator)
                <rect x="210" y="90" width="180" height="40" rx="10"
                    style="fill: var(--color-surface-overlay); stroke: var(--color-accent); stroke-width: 2;"/>
                <text x="300" y="115" text-anchor="middle"
                    style="fill: var(--color-accent); font-size: 14px; font-weight: 700; font-family: var(--font-display);">
                    "MEGA"
                </text>

                // Stream nodes
                {streams.iter().enumerate().map(|(i, name)| {
                    let x = stream_x(i);
                    let rect_x = x - 48;
                    let is_showcase = *name == "SHOWCASE";
                    let fill = if is_showcase {
                        "fill: var(--color-accent); fill-opacity: 0.15;"
                    } else {
                        "fill: var(--color-surface-raised);"
                    };
                    let stroke = if is_showcase {
                        "stroke: var(--color-accent); stroke-width: 1.5;"
                    } else {
                        "stroke: var(--color-border-default); stroke-width: 1;"
                    };

                    view! {
                        <rect x=rect_x.to_string() y="200" width="96" height="40" rx="8"
                            style=format!("{fill} {stroke}")/>
                        <text x=x.to_string() y="225" text-anchor="middle"
                            style="fill: var(--color-text-primary); font-size: 11px; font-weight: 600; font-family: var(--font-mono);">
                            {*name}
                        </text>
                    }
                }).collect_view()}

                // Pipeline label
                <text x="300" y="300" text-anchor="middle"
                    style="fill: var(--color-text-muted); font-size: 10px; font-family: var(--font-mono);">
                    "Writer → Checker → Maintainer"
                </text>

                // Pipeline boxes (small, under each stream)
                {streams.iter().enumerate().map(|(i, _)| {
                    let x = stream_x(i);
                    view! {
                        <rect x=(x - 36).to_string() y="280" width="72" height="20" rx="4"
                            style="fill: var(--color-surface-sunken); stroke: var(--color-border-subtle); stroke-width: 0.5;"/>
                        <text x=x.to_string() y="294" text-anchor="middle"
                            style="fill: var(--color-text-muted); font-size: 8px; font-family: var(--font-mono);">
                            "W → C → M"
                        </text>
                    }
                }).collect_view()}

                // Legend
                <text x="300" y="340" text-anchor="middle"
                    style="fill: var(--color-text-muted); font-size: 10px; font-family: var(--font-body);">
                    "5 parallel sessions · Evidence-first protocol · TDD pipeline"
                </text>

                // Phase label
                <text x="300" y="360" text-anchor="middle"
                    style="fill: var(--color-text-muted); font-size: 9px; font-style: italic; font-family: var(--font-body);">
                    "Research → Roadmap → Build → Polish"
                </text>
            </svg>
        </div>
    }
}

/// Compute the x-coordinate for a stream node given its index (0–4).
///
/// Distributes 5 streams evenly across the 600px-wide viewBox.
fn stream_x(index: usize) -> i32 {
    // 5 nodes: centered at x=72, 186, 300, 414, 528
    72 + (index as i32) * 114
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_positions_are_evenly_spaced() {
        let positions: Vec<i32> = (0..5).map(stream_x).collect();
        // Check uniform spacing
        for window in positions.windows(2) {
            assert_eq!(window[1] - window[0], 114);
        }
    }

    #[test]
    fn stream_positions_are_centered() {
        // Middle stream (index 2) should be at x=300 (center of 600px viewBox)
        assert_eq!(stream_x(2), 300);
    }

    #[test]
    fn first_and_last_within_bounds() {
        assert!(
            stream_x(0) >= 48,
            "First stream must have room for 96px rect"
        );
        assert!(
            stream_x(4) <= 552,
            "Last stream must have room for 96px rect"
        );
    }
}
