# Cross-Stream Dependency Index

**Last Updated**: 2026-03-20

## Active Dependencies

| From | To | What's Needed | Layer | Status | Notes |
|------|----|---------------|-------|--------|-------|
| SHOWCASE | DATA | Project data server functions (get_projects, get_project_by_slug) | L0 | ROUTED | DATA research complete, model proposed, message in SHOWCASE inbox |
| SHOWCASE | DESIGN | Card component, layout shell, design tokens | L0 | OPEN | SHOWCASE builds on DESIGN's foundation |
| IDENTITY | DATA | CV content server functions (get_experiences, get_skills, get_cv_sections) | L0 | ROUTED | DATA research complete, model proposed, message in IDENTITY inbox |
| IDENTITY | DESIGN | Timeline component, layout shell, design tokens | L0 | OPEN | IDENTITY builds on DESIGN's foundation |
| INFRA | DATA | Database deployment config (SQLite) | L1 | ROUTED | DATA recommends SQLite, ADR pending from MEGA |
| INFRA | ALL | Working build for deployment | L0 | OPEN | Need compilable project to deploy |
| IDENTITY | SHOWCASE | Project summary data for CV PDF | L2 | OPEN | PDF includes project highlights |
| DATA | IDENTITY | CV content text for seed data (about, experience, skills) | L0 | ROUTED | DATA needs text content, message in IDENTITY inbox |
| DATA | MEGA | ADR-003: SQLite as database | L0 | ROUTED | DATA recommends SQLite, awaiting formal ADR |

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
