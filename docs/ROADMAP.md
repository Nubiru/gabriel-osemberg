# gabriel-osemberg — Development Roadmap

**Version**: 1.0 **Last Updated**: 2026-03-20

---

## Overview

7 phases from zero to deployed Visual Virtual Digital CV. Each phase builds on the previous. Learning Rust and Leptos is a first-class goal alongside building the website.

---

## Phase 0: Foundation Setup

**Goal**: Working Rust + Leptos toolchain with a compiling hello-world.

**Done when**:
- [ ] Rust toolchain installed (rustup, cargo, wasm32 target)
- [ ] Leptos project scaffolded with cargo-leptos
- [ ] `cargo leptos watch` compiles and serves a page at localhost
- [ ] Tailwind CSS integrated and rendering
- [ ] cargo test runs (even with zero project tests)
- [ ] Playwright installed for future E2E tests
- [ ] Git initialized with first commit
- [ ] Development environment documented in docs/ENVIRONMENT.md

**Tasks**:
1. Install Rust via rustup, add wasm32-unknown-unknown target
2. Install cargo-leptos (`cargo install cargo-leptos`)
3. Scaffold Leptos project with SSR (server-side rendering)
4. Configure Tailwind CSS integration
5. Verify hot-reload works
6. Set up test infrastructure (cargo test + Playwright)
7. Initialize git, create .gitignore, first commit
8. Document environment in ENVIRONMENT.md

**Files created**: `Cargo.toml`, `src/main.rs`, `src/app.rs`, `style/`, `tailwind.config.js`, `Cargo.lock`

---

## Phase 1: Design System & Layout Shell

**Goal**: Establish visual identity, design tokens, responsive layout, and navigation.

**Done when**:
- [ ] Design tokens defined (colors, typography, spacing, shadows)
- [ ] Dark/light theme support
- [ ] Responsive navigation (desktop + mobile)
- [ ] Page layout components (header, footer, main content area)
- [ ] CSS animations library for page transitions
- [ ] All design tokens tested (snapshot tests)
- [ ] Lighthouse accessibility score >= 90

**Tasks**:
1. Research and define color palette (OKLCH for perceptual uniformity)
2. Choose and integrate typography (font pairing)
3. Build design token system in Tailwind config
4. Create layout shell components (Header, Footer, Layout)
5. Implement responsive navigation with mobile menu
6. Add dark/light mode toggle with smooth transition
7. Create page transition animations
8. Write tests for all components

**Dependencies**: Phase 0 complete

---

## Phase 2: Database & Content Architecture

**Goal**: Structured data model for projects, experience, skills, and CV content. All content driven by database, not hardcoded.

**Done when**:
- [ ] Database schema designed and migrated (SQLx + SQLite or PostgreSQL)
- [ ] Project model: name, description, tech_stack, metrics, links, images, status
- [ ] Experience model: role, company, dates, description, highlights
- [ ] Skill model: name, category, proficiency, projects_used_in
- [ ] CV content model: sections, ordering, visibility
- [ ] Seed data for all 4 showcase projects
- [ ] Server functions for all CRUD operations
- [ ] Full test coverage on data layer

**Tasks**:
1. Design database schema (ERD)
2. Set up SQLx with compile-time checked queries
3. Write migrations
4. Create seed data from actual project metrics
5. Build server functions for data access
6. Write comprehensive tests for all queries
7. Document data architecture in ARCHITECTURE.md

**Dependencies**: Phase 0 complete

---

## Phase 3: Project Showcase Pages

**Goal**: Each project presented as an interactive case study with metrics, tech stack visualization, and live data.

**Done when**:
- [ ] Projects index page with filterable/sortable cards
- [ ] Individual project detail pages with case study format
- [ ] Metrics visualization (test coverage, LOC, file counts, quality scores)
- [ ] Tech stack visual representation (radar/spider chart or similar)
- [ ] Photo/video assets integrated per project
- [ ] GitHub integration (live stats via API where applicable)
- [ ] "AI collaboration" section per project showing human vs AI contributions
- [ ] Responsive and animated
- [ ] Full test coverage

**Tasks**:
1. Build project card component with hover animations
2. Build project detail page template
3. Implement metrics visualization (charts/graphs in Rust/WASM or SVG)
4. Build tech stack visual component
5. Integrate photo/video assets with lazy loading
6. Add GitHub API integration for live repo stats
7. Build "AI Collaboration Methodology" showcase section
8. Write tests for all components and pages

**Dependencies**: Phase 1 + Phase 2 complete

---

## Phase 4: CV & About Pages

**Goal**: Professional CV presentation alongside an engaging About page. Downloadable PDF generation.

**Done when**:
- [ ] About page with personal narrative and philosophy
- [ ] Experience timeline (interactive, scannable)
- [ ] Skills visualization
- [ ] Education section
- [ ] AI-Augmented Engineering methodology section
- [ ] PDF CV generation (server-side, downloadable)
- [ ] PDF content mirrors website content (DRY — same data source)
- [ ] PDF is ATS-friendly (clean layout, parseable text)
- [ ] Full test coverage

**Tasks**:
1. Build About page with personal narrative
2. Build interactive experience timeline
3. Build skills visualization component
4. Build AI methodology showcase (CLAUDE.md, MEGA framework explanation)
5. Implement server-side PDF generation
6. Ensure PDF and web share the same data model
7. Write tests for all pages and PDF generation

**Dependencies**: Phase 2 + Phase 3 complete

---

## Phase 5: Polish, Animation & Interaction

**Goal**: Elevate from functional to exceptional. Microinteractions, scroll animations, performance optimization.

**Done when**:
- [ ] Hero section with impactful first impression (6-second rule)
- [ ] Scroll-triggered animations throughout
- [ ] Microinteractions on all interactive elements
- [ ] Page transition animations (View Transitions API or equivalent)
- [ ] Loading states with personality
- [ ] Performance budget met: < 3s FTL, < 100KB WASM (gzipped)
- [ ] Lighthouse scores: Performance >= 90, Accessibility >= 95, SEO >= 90
- [ ] Cross-browser tested (Chrome, Firefox, Safari, mobile)
- [ ] Full test coverage including visual regression

**Tasks**:
1. Design and build hero section
2. Implement scroll-triggered reveal animations
3. Add microinteractions (hover, focus, click feedback)
4. Optimize WASM bundle size
5. Performance audit and optimization
6. Cross-browser testing
7. Visual regression test setup
8. Accessibility audit and fixes

**Dependencies**: Phase 3 + Phase 4 complete

---

## Phase 6: Deployment & Production

**Goal**: Live on the internet with monitoring, analytics, and CI/CD.

**Done when**:
- [ ] Custom domain configured (gabrielosemberg.com or similar)
- [ ] Deployed to Fly.io / Shuttle.rs / Railway
- [ ] HTTPS with proper certificates
- [ ] CI/CD pipeline (GitHub Actions): build, test, deploy
- [ ] Basic analytics (privacy-respecting — Plausible or similar)
- [ ] Error monitoring
- [ ] SEO metadata (Open Graph, Twitter cards, structured data)
- [ ] Social media preview images
- [ ] Final Lighthouse audit passing all thresholds
- [ ] Production smoke tests passing

**Tasks**:
1. Choose and configure deployment platform
2. Set up custom domain + DNS
3. Build CI/CD pipeline
4. Configure analytics
5. Add SEO metadata and social previews
6. Production smoke test suite
7. Launch

**Dependencies**: Phase 5 complete

---

## Current Status

**Active Phase**: 0
**Last Updated**: 2026-03-20
**Phases complete**: 0/7
