# SHOWCASE Task: L0-001 through L0-004 — Core Project Showcase Pages

**Claimed by**: SHOWCASE stream
**Date**: 2026-03-20
**Status**: IN PROGRESS

## Scope

Build the core project showcase: card component, projects index page, project detail page, and route wiring. This is one cohesive delivery — the card feeds the index, the detail is the companion page, and routes wire them together.

## Tasks

- [x] L0-001: Project card component (`src/components/project_card.rs`)
- [x] L0-002: Projects index page (`src/components/projects_page.rs`)
- [x] L0-003: Project detail page (`src/components/project_detail.rs`)
- [x] L0-004: Route wiring (update `src/app.rs`)

## Dependencies

- `Project` model: `src/models/project.rs` (DELIVERED)
- `get_projects()`, `get_project_by_slug()`: `src/server_fns.rs` (DELIVERED)
- `Badge`, `ExternalLink`, `SectionHeading`: `src/components/ui.rs` (DELIVERED)
- `ErrorDisplay`: `src/components/error.rs` (DELIVERED)
- `SkeletonCard`: `src/components/loading.rs` (DELIVERED)
- Design tokens: `style/tailwind.css` (DELIVERED)
- Layout shell: `src/components/layout.rs` (DELIVERED)

## TDD Spec

### Project Card
- Renders project name, tagline
- Renders tech stack as Badge pills (split on ", ")
- Links to `/projects/{slug}`
- Shows external links (repo, live) when present

### Projects Index Page
- Fetches projects via `get_projects()` server function
- Renders grid of ProjectCard components
- Shows SkeletonCard loading state during fetch
- Shows ErrorDisplay on failure

### Project Detail Page
- Fetches project by slug via `get_project_by_slug()`
- Renders full case study layout: hero, description, tech stack, links
- Shows loading state during fetch
- Shows error on invalid slug

### Route Wiring
- `/projects` → ProjectsPage
- `/projects/:slug` → ProjectDetailPage
