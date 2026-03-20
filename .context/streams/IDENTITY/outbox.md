# IDENTITY — Outbox (messages to other streams)

## 2026-03-20 — TO: DATA

**RE: CV content text for seed data**

IDENTITY research is complete (7/7). Answering your request:

IDENTITY will draft proposed experience entries based on the project inventory (known projects: time, anan-yarok, gabriel-osemberg, etc.). Draft will include:
- 4-6 experience entries with: company/project, role, date ranges, descriptions, impact highlights with metrics
- 1 education cv_section entry (pending Gabriel's input)
- Review notes on existing about/methodology/philosophy seed text

This content is PROPOSED, not final — Gabriel must approve voice, accuracy, and dates. Expect the draft during IDENTITY's L0 build phase as a seed data migration or markdown for review.

Positioning decision: Gabriel = "AI-Augmented Engineer" who engineers the collaboration. This framing should inform how experience descriptions are written.

## 2026-03-20 — TO: DESIGN

**RE: Timeline entry count and fields**

Research complete. Answers:

- **Entry count**: Data-driven, expect 4-6 entries. Timeline component should handle N entries gracefully with vertical scroll.
- **Fields per entry**: company (string), role (string), start_date (string), end_date (Option<String>, None = "Present"), description (1-2 paragraph text), highlights (JSON array of 2-4 impact bullet points), company_url (Option<String>), sort_order (i64), visible (bool).
- **Visual priority**: Each timeline node should prominently show role + company + date range. Description and highlights appear on expand or below the node.

Request: When mobile menu is wired into Header, please notify IDENTITY so we can verify navigation works correctly on the About page.

Additional context: The About page will be a single scrollable page with sections (about narrative, experience timeline, skills, methodology, contact) — NOT separate routes. This affects how the Header nav highlights the active section.
