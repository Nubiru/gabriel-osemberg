//! SVG circular progress ring for percentage metrics (e.g., test coverage).

use leptos::prelude::*;

/// The mathematical constant PI, used for circumference calculations.
const PI: f64 = std::f64::consts::PI;

/// Compute the circumference of a circle given its radius.
fn circumference(radius: f64) -> f64 {
    2.0 * PI * radius
}

/// Compute the stroke-dashoffset for a given percentage (0.0 to 100.0).
///
/// At 0% the offset equals the full circumference (ring is empty).
/// At 100% the offset is 0 (ring is completely filled).
fn dash_offset(circumference: f64, percent: f64) -> f64 {
    let clamped = percent.clamp(0.0, 100.0);
    circumference * (1.0 - clamped / 100.0)
}

/// A circular progress indicator for percentage values.
///
/// Renders an SVG ring with a background track and a foreground arc
/// whose length corresponds to `value`. The percentage is displayed
/// as centered text, with `label` beneath the ring.
#[component]
pub fn ProgressRing(
    /// The percentage value (0.0 to 100.0).
    #[prop(into)]
    value: f64,
    /// Descriptive label (e.g., "Test Coverage").
    #[prop(into)]
    label: String,
    /// SVG viewBox size in logical pixels.
    #[prop(default = 48)]
    size: u32,
    /// Width of the ring stroke.
    #[prop(default = 4)]
    stroke_width: u32,
) -> impl IntoView {
    let center = f64::from(size) / 2.0;
    let radius = center - f64::from(stroke_width) / 2.0;
    let circ = circumference(radius);
    let offset = dash_offset(circ, value);
    let display_value = format!("{:.1}%", value.clamp(0.0, 100.0));
    let aria_label = format!("{label}: {display_value}");
    let viewbox = format!("0 0 {size} {size}");

    // Inline styles reference CSS custom properties from the design token system
    // so the ring adapts to light/dark theme automatically.
    let bg_style = "stroke: var(--color-surface-overlay); fill: none;";
    let fg_style = format!(
        "stroke: var(--color-accent); fill: none; \
         stroke-linecap: round; \
         stroke-dasharray: {circ}; \
         stroke-dashoffset: {offset}; \
         transform: rotate(-90deg); \
         transform-origin: center; \
         transition: stroke-dashoffset 0.5s ease;"
    );

    view! {
        <div class="flex flex-col items-center gap-1">
            <svg
                viewBox=viewbox
                class="w-full h-auto"
                role="img"
                aria-label=aria_label
            >
                // Background track
                <circle
                    cx=center.to_string()
                    cy=center.to_string()
                    r=radius.to_string()
                    stroke-width=stroke_width.to_string()
                    style=bg_style
                />
                // Foreground arc
                <circle
                    cx=center.to_string()
                    cy=center.to_string()
                    r=radius.to_string()
                    stroke-width=stroke_width.to_string()
                    style=fg_style
                />
                // Centered percentage text
                <text
                    x=center.to_string()
                    y=center.to_string()
                    text-anchor="middle"
                    dominant-baseline="central"
                    class="font-display font-bold"
                    style="fill: var(--color-text-primary); font-size: 0.3em;"
                >
                    {display_value}
                </text>
            </svg>
            <span class="text-xs text-text-secondary text-center">{label}</span>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circumference_calculation() {
        // For a circle with radius 20, circumference = 2 * PI * 20
        let r = 20.0;
        let expected = 2.0 * PI * r;
        let result = circumference(r);
        assert!(
            (result - expected).abs() < f64::EPSILON,
            "expected {expected}, got {result}"
        );
        // Sanity check: ~125.66
        assert!((result - 125.6637).abs() < 0.001);
    }

    #[test]
    fn dashoffset_for_zero_percent() {
        // At 0%, the offset equals the full circumference (empty ring).
        let circ = circumference(20.0);
        let offset = dash_offset(circ, 0.0);
        assert!(
            (offset - circ).abs() < f64::EPSILON,
            "0% should yield offset == circumference"
        );
    }

    #[test]
    fn dashoffset_for_100_percent() {
        // At 100%, offset is 0 (fully filled ring).
        let circ = circumference(20.0);
        let offset = dash_offset(circ, 100.0);
        assert!(
            offset.abs() < f64::EPSILON,
            "100% should yield offset == 0, got {offset}"
        );
    }

    #[test]
    fn dashoffset_for_50_percent() {
        // At 50%, offset is half the circumference.
        let circ = circumference(20.0);
        let offset = dash_offset(circ, 50.0);
        let expected = circ / 2.0;
        assert!(
            (offset - expected).abs() < f64::EPSILON,
            "50% should yield offset == circumference/2"
        );
    }

    #[test]
    fn dashoffset_for_95_9_percent() {
        // Real-world test: the `time` project has 95.9% test coverage.
        let size: u32 = 48;
        let stroke_width: u32 = 4;
        let center = f64::from(size) / 2.0;
        let radius = center - f64::from(stroke_width) / 2.0;
        // radius = 24 - 2 = 22
        assert!((radius - 22.0).abs() < f64::EPSILON);

        let circ = circumference(radius);
        let offset = dash_offset(circ, 95.9);
        // Expected: circ * (1 - 0.959) = circ * 0.041
        let expected = circ * 0.041;
        assert!(
            (offset - expected).abs() < 0.001,
            "expected {expected}, got {offset}"
        );
    }

    #[test]
    fn value_clamped_above_100() {
        let circ = circumference(20.0);
        let offset = dash_offset(circ, 150.0);
        // Should be clamped to 100%
        assert!(
            offset.abs() < f64::EPSILON,
            "values above 100 should be clamped to 100%"
        );
    }

    #[test]
    fn value_clamped_below_zero() {
        let circ = circumference(20.0);
        let offset = dash_offset(circ, -10.0);
        // Should be clamped to 0%
        assert!(
            (offset - circ).abs() < f64::EPSILON,
            "negative values should be clamped to 0%"
        );
    }
}
