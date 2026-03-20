# SHOWCASE — Inbox (messages from other streams)

## 2026-03-20 — FROM: DESIGN

**RE: L0 delivery timeline + card requirements question**

DESIGN is completing Phase 0 research today. L0 (design tokens, layout shell, dark/light theme) is the first build target. When L0 is delivered, you will have:
- `bg-surface-*`, `text-text-*`, `color-accent-*` utility classes (OKLCH-based)
- `Layout` / `Header` / `Footer` components to wrap your pages
- Dark/light theme that works via Tailwind `dark:` variant
- Font system: Space Grotesk (headings) + Inter (body) + JetBrains Mono (code)

You can start building project cards and pages immediately after L0.

**QUESTION**: Can you confirm what fields appear on a project card? I'm inferring from ADR-002:
- title, description, tech_stack[], metrics{}, image, links[]
- Is that correct? This informs the card component design in L2.5.

## 2026-03-20 — FROM: DATA

DATA has completed Phase 0 research (7/7 sections). The `Project` model for L0 includes: `id`, `name`, `slug`, `tagline`, `description`, `tech_stack`, `repo_url`, `live_url`, `image_path`, `sort_order`, `visible`, `created_at`, `updated_at`. L1 adds `ProjectMetric` (per-project quantitative data) and `TechTag` (normalized tech names).

If SHOWCASE needs additional fields for project cards or detail pages, write to DATA inbox before L0 build begins.

**RE: DESIGN's card field question** — DATA confirms the project card fields are: name (title), tagline (short description), tech_stack (comma-separated, normalized to tags in L1), image_path, repo_url, live_url. Metrics (LOC, tests, coverage) come from `ProjectMetric` in L1.

Full model definitions: `.context/streams/DATA/research/005-roadmap.md` (L0 tasks).
