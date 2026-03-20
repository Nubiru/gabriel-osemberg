# IDENTITY — Active Task

**Task**: L0 Foundation — Seed Identity Data + About Page
**Status**: IN PROGRESS
**Started**: 2026-03-20

## Subtasks

### L0-01: Seed Migration (005_seed_identity_data.sql)
- [ ] 6 experience entries from approved CV v2
- [ ] Military cv_section (section_type = "military")
- [ ] Education cv_section (section_type = "education")
- [ ] Spoken language skills (category = "spoken-language", proficiency = 5)
- [ ] Refined about section content (UPDATE existing seed)
- [ ] Refined methodology section content (UPDATE existing seed)
- [ ] Use INSERT OR IGNORE for idempotency (per INFRA request)

### L0-02: About Page Component
- [ ] `src/components/about_page.rs` — main About page
- [ ] Fetch cv_sections, experiences, skills via server resources
- [ ] Render About narrative section
- [ ] Render Experience timeline (using DESIGN's Timeline component)
- [ ] Render Skills categorized grid
- [ ] Render Methodology section
- [ ] Render Military service section
- [ ] Render Education section
- [ ] Render Contact info section
- [ ] Register module in components/mod.rs

### L0-03: Route Wiring
- [ ] Add `/about` route in app.rs pointing to AboutPage component
- [ ] Verify compilation

## Quality Gates
- [ ] cargo build (zero warnings)
- [ ] cargo clippy -- -D warnings
- [ ] cargo fmt --check
- [ ] cargo test (all pass, no regressions)
