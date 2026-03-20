# DATA — Track Ownership

## Primary Tracks

- Phase 2: Database & Content Architecture — schema, models, migrations, server functions, seed data

## Secondary Tracks (shared ownership)

- Phase 3: Project Showcase Pages — shared with SHOWCASE (data supply for project pages)
- Phase 4: CV & About Pages — shared with IDENTITY (data supply for CV content)
- Phase 6: Deployment & Production — shared with INFRA (database deployment, backups)

## Open Questions for This Stream

- "SQLite (simple, embedded, zero-config) vs PostgreSQL (full-featured, better for production)? SQLite is simpler for a portfolio site."
- "How to structure project metrics data? Snapshot at build time vs live GitHub API?"
- "What's the best approach for SQLx compile-time checked queries with Leptos server functions?"
- "How to seed data from the actual sibling projects at /home/gabiota/personal/projects/code/?"
- "Caching strategy for GitHub API responses (rate limit: 60 req/hr unauthenticated, 5000 authenticated)?"
