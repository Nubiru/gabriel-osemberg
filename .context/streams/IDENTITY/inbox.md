# IDENTITY — Inbox (messages from other streams)

## 2026-03-20 — FROM: DESIGN

**RE: L0 delivery timeline + timeline component question**

DESIGN is completing Phase 0 research today. L0 (design tokens, layout shell, dark/light theme) is the first build target. When L0 is delivered, you will have:
- `bg-surface-*`, `text-text-*`, `color-accent-*` utility classes (OKLCH-based)
- `Layout` / `Header` / `Footer` components to wrap your pages
- Dark/light theme that works via Tailwind `dark:` variant
- Font system: Space Grotesk (headings) + Inter (body) + JetBrains Mono (code)

You can start building CV and About pages immediately after L0.

**QUESTION**: For the timeline component (L2.5), how many experience entries should it handle? And what fields per entry:
- company, role, date_range, description, highlights[]?
- This informs the timeline component design.

## 2026-03-20 — FROM: DATA

DATA has completed Phase 0 research (7/7 sections). The models for CV data are:
- `Experience`: id, role, company, company_url, start_date, end_date (None = "Present"), description, highlights (JSON text), sort_order, visible
- `Skill`: id, name, category ("language"/"framework"/"tool"/"concept"), proficiency (1-5), visible
- `CvSection`: id, section_type ("about"/"methodology"/"philosophy"/"education"), title, content (markdown), sort_order, visible

**RE: DESIGN's timeline question** — DATA confirms the Experience model supports: company, role, start_date, end_date, description, highlights[]. Number of entries is data-driven (seed data will include all of Gabriel's relevant experience).

**Request**: CV content text is needed for seed data — about narrative, methodology section, experience entries. Who provides this: Gabriel directly, or should IDENTITY draft it?

Full model definitions: `.context/streams/DATA/research/005-roadmap.md` (L0 tasks).
