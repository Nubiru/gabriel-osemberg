# Cross-Stream Dependency Index

**Last Updated**: 2026-03-20

## Active Dependencies

| From | To | What's Needed | Layer | Status | Notes |
|------|----|---------------|-------|--------|-------|
| SHOWCASE | DATA | Project data server functions (get_projects, get_project_by_slug) | L0 | RESOLVED | SHOWCASE research confirms delivery works. Files: src/server_fns.rs, src/models/project.rs. 5 projects seeded. |
| SHOWCASE | DESIGN | Card component, layout shell, design tokens | L0 | RESOLVED | SHOWCASE research confirms delivery works. Card component is L2.5 (later). Files: style/tailwind.css, src/components/layout.rs |
| SHOWCASE | DATA | ProjectMetric model (LOC, coverage, test_count, src_file_count) | L1 | ROUTED | Message in DATA inbox. SHOWCASE L0 does not depend on this. |
| SHOWCASE | DATA | AiCollaboration model (per-project human/AI contribution data) | L2 | ROUTED | Message in DATA inbox. Proposed schema in SHOWCASE 006-dependencies.md. |
| IDENTITY | DATA | CV content server functions (get_experiences, get_skills, get_cv_sections) | L0 | DELIVERED | Files: src/server_fns.rs, src/models/. 30 skills + 3 CV sections seeded. Experiences empty (pending content). |
| IDENTITY | DESIGN | Timeline component, layout shell, design tokens | L0 | DELIVERED | Layout shell + design tokens + theme delivered. Timeline component is L2.5 (later). Files: style/tailwind.css, src/components/layout.rs |
| INFRA | DATA | Database deployment config (SQLite) | L1 | DELIVERED | ADR-003 accepted. SQLite with SQLx. DB auto-created on server start. |
| INFRA | ALL | Working build for deployment | L0 | OPEN | Need compilable project to deploy |
| IDENTITY | SHOWCASE | Project summary data for CV PDF | L2 | OPEN | PDF includes project highlights |
| DATA | IDENTITY | CV content text for seed data (about, experience, skills) | L0 | ROUTED | DATA needs text content, message in IDENTITY inbox |
| DATA | MEGA | ADR-003: SQLite as database | L0 | RESOLVED | ADR-003 accepted in docs/DECISIONS.md |
| INFRA | DESIGN | SEO meta tags (leptos_meta in page components) | L1 | ROUTED | Message in DESIGN inbox 2026-03-20 |
| INFRA | DESIGN | Styled 404/500 error page components | L1 | ROUTED | Message in DESIGN inbox 2026-03-20 |
| INFRA | DESIGN | Default OG image (1200x630px) for social sharing | L2 | ROUTED | Message in DESIGN inbox 2026-03-20 |
| INFRA | DATA | Seed data idempotency (INSERT OR IGNORE) | L0 | ROUTED | Message in DATA inbox 2026-03-20 |

### Status Key

| Status | Meaning |
|--------|---------|
| OPEN | Dependency identified, not yet addressed |
| RESEARCH | Both streams aware, investigating approach |
| ROUTED | Message sent to target stream's inbox |
| IN-PROGRESS | Target stream actively working on it |
| DELIVERED | Code/data committed and available |
| RESOLVED | Consuming stream confirmed it works |
| BLOCKED | Cannot proceed, needs escalation |

## Stream Launch Order (Recommended)

Based on dependencies, the optimal launch order is:

1. **DESIGN** + **DATA** (parallel — no dependencies on each other)
2. **SHOWCASE** + **IDENTITY** (parallel — depend on DESIGN + DATA foundations)
3. **INFRA** (can start research anytime, but build phase needs others)

## How to Update

- **Streams update this file directly** when discovering new dependencies
- MEGA reviews during coordination cycles but does NOT need to be the sole editor
- Each stream also documents its own needs in `006-dependencies.md`
- When a dependency is delivered, update status and add commit hash or file path
