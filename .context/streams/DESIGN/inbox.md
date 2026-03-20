# DESIGN — Inbox (messages from other streams)

## 2026-03-20 — FROM: SHOWCASE

**RE: Card field confirmation (response to your question)**

CONFIRMED: Project card fields are:
- `name` (title)
- `tagline` (short description)
- `tech_stack` (comma-separated, rendered as pill badges)
- `image_path` (thumbnail)
- `repo_url`, `live_url` (action links)
- Metrics row: LOC, test coverage %, test count (from ProjectMetric in L1; styled badges in L0)

SHOWCASE will build its own card component using your delivered design tokens for L0. Your L2.5 card component can be a refined/polished version later — no rush.

Your L0 delivery (tokens, layout shell, theme, typography) is excellent. SHOWCASE has zero blockers.

## 2026-03-20 — FROM: INFRA

**RE: SEO Meta Tag Coordination**

INFRA will scaffold SEO infrastructure (robots.txt, sitemap.xml, default meta tags). DESIGN should ensure page-level components use `<Title>` and `<Meta>` from `leptos_meta` for per-page SEO:

- Each page component should set its own `<Title>` and `<Meta name="description">`
- INFRA will add default OG/Twitter meta fallbacks in the layout shell
- Page components can override with specific values

**Also needed (L2, not urgent)**: A default OG image (1200x630px) for social sharing. Can be a simple branded image with Gabriel's name + tagline on the dark background.

**Also needed (L1)**: Styled 404 and 500 error page components. INFRA can create fallback plaintext versions, DESIGN styles them when ready.
