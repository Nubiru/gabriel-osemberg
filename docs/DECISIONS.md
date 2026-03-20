# Architecture Decision Records — gabriel-osemberg

---

## How to Use This Document

An Architecture Decision Record (ADR) captures a significant technical decision. ADRs are numbered sequentially and never deleted — superseded decisions are marked as such with a reference to the replacing ADR.

### ADR Template

```markdown
## ADR-NNN: {Title}

**Date**: YYYY-MM-DD
**Status**: Proposed | Accepted | Superseded by ADR-NNN | Deprecated

### Context
What is the issue? What forces are at play?

### Decision
What is the change being proposed or decided?

### Alternatives Considered
1. **{Alternative}**: Rejected — {reason}

### Consequences
**Positive**:
- {benefit}

**Negative**:
- {tradeoff}

### Rationale
Why was this chosen over the alternatives?
```

---

## ADR-000: Use Architecture Decision Records

**Date**: {DATE}
**Status**: Accepted

### Context

Technical decisions made early in a project are often forgotten or misunderstood later. New team members (human or AI) lack the context for why things are the way they are. Decisions made verbally or in chat are lost.

### Decision

Record every significant architectural decision in this file using the ADR format. Each record is numbered sequentially (ADR-001, ADR-002, ...) and captures the context, decision, alternatives considered, and consequences.

### Consequences

**Positive**:
- Decisions are discoverable and searchable
- New contributors understand the "why" behind the architecture
- Prevents relitigating settled decisions
- Creates an audit trail of technical evolution

**Negative**:
- Requires discipline to write ADRs for significant decisions
- May slow down decision-making slightly (this is a feature, not a bug)

### Rationale

The cost of writing a short ADR is far less than the cost of rediscovering or reversing an undocumented decision months later.

---

## ADR-001: Leptos (Rust + WASM) as Primary Tech Stack

**Date**: 2026-03-20
**Status**: Accepted

### Context

Gabriel is building a Visual Virtual Digital CV website to showcase his work and position himself for employment at top AI/tech companies. He has extensive experience with NextJS, React, Vanilla JS, C, and WebGL2. The goal is to learn a new language/framework that itself becomes a portfolio talking point, while delivering a production-quality website with database, styling, user interaction, media assets, and downloadable PDF generation.

12+ frameworks were evaluated across emerging (Astro, SolidJS, Qwik, Fresh, SvelteKit, HTMX) and unconventional (Leptos, Yew, Dioxus, Elixir Phoenix LiveView, Gleam/Lustre, Elm, OCaml/Dream) categories.

### Decision

Use **Leptos** (Rust full-stack web framework) compiling to WebAssembly for the frontend, with server functions for backend logic. Supporting stack:
- **Language**: Rust
- **Framework**: Leptos (fine-grained reactivity, SSR + hydration)
- **Build**: cargo-leptos
- **Database**: SQLx (compile-time checked SQL) + SQLite or PostgreSQL
- **Styling**: Tailwind CSS (via cargo-leptos integration)
- **Testing**: cargo test (unit/integration) + Playwright (E2E)
- **PDF generation**: Server-side via headless Chromium or `printpdf` crate
- **Deployment**: Fly.io or Shuttle.rs (NOT Vercel — novelty constraint)

### Alternatives Considered

1. **SvelteKit**: Rejected — excellent DX and built-in animations, but "just another JS framework" in the eyes of some interviewers. Lower career impact signal. Would have been fastest to build (1-2 weeks). Remains a strong fallback if Leptos proves impractical.

2. **Elixir Phoenix LiveView**: Rejected — most paradigm-shifting option with superb ORM (Ecto) and testing story. However, steepest learning curve (4-8 weeks), niche job market, and if interviewer doesn't know Elixir the technical choice may not register. The BEAM VM is beautiful but overkill for a portfolio's content serving needs.

3. **Astro**: Rejected — static-first nature fights against dynamic features (database, auth, admin). Not novel enough for 2026.

4. **Qwik**: Rejected — resumability shines on large complex apps, not portfolio sites. Smaller community. The `$` convention adds friction without proportional benefit here.

5. **HTMX + Go/Rust backend**: Rejected — architecturally mature but weak on animations and visual polish. The server-rendered HTML fragment approach produces less visually impressive results by default.

### Consequences

**Positive**:
- Rust proficiency on resume (15-20% salary premium in 2026 market)
- Second WASM project after `time` — establishes a pattern, not an experiment
- Gabriel's C experience provides head start on Rust's memory model
- "Built my portfolio in Rust + WASM" is a genuine interview talking point
- Full-stack type safety — compile-time guarantees for both frontend and backend
- Fine-grained reactivity without virtual DOM (like SolidJS but in Rust)
- Leptos Portfolio Admin project on GitHub proves the exact use case works

**Negative**:
- Slower development velocity (Rust compile times, borrow checker friction)
- Smaller ecosystem — some things will need to be built from scratch
- Hot reload exists but is slower than JS frameworks
- WASM binary sizes require optimization attention
- Fewer tutorials and community resources than React/Svelte ecosystem

### Rationale

The career impact calculation dominated: Gabriel already has JS/React/Next.js projects in his portfolio (anan-yarok, blocksight, chamana, Luzerin). Adding another JS framework adds breadth but not depth. Adding Rust adds a fundamentally new capability on his resume, aligns with his existing systems-programming strength (C + WebGL2), and positions him for the AI/systems companies where Rust is increasingly valued. The development speed tradeoff is acceptable because the timeline has zero pressure — quality and learning are explicit goals.

---

## ADR-002: Project Showcase Selection

**Date**: 2026-03-20
**Status**: Accepted

### Context

Gabriel has 10 projects in `/home/gabiota/personal/projects/code/`. Research indicates 3-5 curated projects is optimal — quality over quantity. Each should be presented as a case study with metrics, architectural decisions, and measurable outcomes.

### Decision

Showcase 4 existing projects + the CV website itself as #5:

1. **time** — 3D interactive temporal artwork (C11 + WebGL2 + WASM, 90K LOC, 95.9% test coverage)
2. **blocksight.live** — Bitcoin blockchain analytics platform (Node + React + TimescaleDB, 2,876 src files)
3. **AnanYarok** — B2B wholesale e-commerce (Next.js + Prisma + PostgreSQL, real business)
4. **Chamana** — Artisanal clothing catalog (Next.js + Payload CMS)
5. **gabriel-osemberg** (this website) — The CV itself, built in Rust + WASM

### Alternatives Considered

1. **Include grindie**: Rejected — game is still in early stages without clear direction. May be added later when it has a coherent identity.
2. **Include Luzerin**: Rejected — digital reading platform is complete but less technically impressive than the selected 4. Could be mentioned as supplementary.
3. **Include all 10**: Rejected — dilutes impact. Research shows 85% of hiring managers prioritize quality over quantity.

### Consequences

**Positive**:
- Each selected project demonstrates a different domain (art/science, fintech, e-commerce, fashion, meta/self-referential)
- Tech stack diversity: C/WASM, Node/React/TypeScript, Next.js/Prisma, Rust/WASM
- `time` is the hero piece — 95.9% coverage in C is interview gold
- The CV website as showcase #5 creates a recursive proof-of-skill

**Negative**:
- Omitting grindie means no game development representation
- Omitting Luzerin means no content/publishing platform representation

### Rationale

Four diverse, mature projects plus the CV website itself covers the essential signals: systems programming mastery, full-stack web development, real business impact, design sensibility, and Rust/WASM capability. Each project tells a different story. Together they paint the picture of a versatile AI-augmented engineer.

---

## ADR-003: SQLite as Primary Database

**Date**: 2026-03-20
**Status**: Accepted
**Requested by**: DATA stream (via outbox -> MEGA)

### Context

The CV website needs a database to store project data, experience entries, skills, and CV content sections. The choice is between SQLite (embedded, zero-config) and PostgreSQL (full-featured, client-server). DATA stream completed research and recommends SQLite. Full rationale in `.context/streams/DATA/research/004-questions.md` (Q1).

### Decision

Use **SQLite** as the primary database via SQLx with compile-time checked queries.

### Alternatives Considered

1. **PostgreSQL**: Rejected for now — overkill for a portfolio site's read-heavy, single-writer workload. Adds deployment complexity (separate DB service). SQLx migration to PostgreSQL is trivial if the project ever needs it.
2. **No database (hardcoded data)**: Rejected — violates the "content is data" rule. Hardcoded content cannot be updated without recompilation, and demonstrating database skills is a portfolio goal.
3. **JSON files**: Rejected — no query capability, no schema validation, no migration story.

### Consequences

**Positive**:
- Zero configuration for development (single file, no service to run)
- Supported by all deployment candidates (Fly.io, Shuttle.rs, Railway)
- SQLx provides compile-time query checking — same developer experience as PostgreSQL
- Single-file backup (copy the `.db` file)
- Excellent read performance for the portfolio's workload pattern
- Migration to PostgreSQL is trivial with SQLx (change connection string + minor SQL dialect adjustments)

**Negative**:
- Single-writer limitation (irrelevant for a portfolio site)
- No built-in full-text search without FTS5 extension
- Less impressive on resume than PostgreSQL (minor concern)

### Rationale

The workload is read-heavy with single-writer updates (only during content seeding or admin updates). SQLite eliminates deployment complexity while SQLx ensures the same type-safe query experience. The migration path to PostgreSQL is trivial, so this is a low-risk, high-convenience choice.

---

## ADR-004: Code as Portfolio — REST API + Architecture Showcase

**Date**: 2026-03-20
**Status**: Accepted

### Context

This codebase serves dual purpose: it IS the portfolio AND it showcases portfolio items. The architecture must demonstrate programming prowess to anyone who reads the source code on GitHub. Additionally, Leptos server functions use RPC-over-HTTP (not REST), which means the codebase doesn't demonstrate REST API design skills — a gap for job applications.

Gabriel asked: "How do I display programming prowess in the SAME CODEBASE where I wish to display my prowess visibly?"

### Decision

1. **Add a thin REST API layer** at `/api/v1/` alongside Leptos server functions. This demonstrates both patterns (RPC + REST) and makes project data reusable.

2. **Add an `/architecture` page** on the website that shows:
   - Tech stack with version details
   - Module dependency visualization
   - Design token system explanation
   - Test coverage and quality metrics
   - Link to public GitHub repo
   - The human-AI collaboration methodology

3. **Architecture principles that the code itself demonstrates:**
   - Pure/stateful separation: models are pure data, server functions are the I/O boundary
   - Design tokens: zero hardcoded visual values
   - Error handling: `Result<T, E>` everywhere, domain error enums, no `.unwrap()`
   - Semantic HTML + ARIA for accessibility
   - Module docs (`//!`) on every file
   - Files under 400 lines
   - ADRs for every significant decision

### REST API Endpoints

```
GET /api/v1/projects          → Vec<Project>
GET /api/v1/projects/:slug    → Project
GET /api/v1/skills            → Vec<Skill>
GET /api/v1/experiences       → Vec<Experience>
GET /api/v1/health            → { status: "ok", version: "..." }
```

JSON responses with proper HTTP status codes, Content-Type headers, and error responses.

### Alternatives Considered

1. **No REST API**: Rejected — misses the opportunity to demonstrate REST knowledge.
2. **Full OpenAPI spec + Swagger UI**: Rejected — over-engineering for a portfolio site.
3. **GraphQL instead of REST**: Rejected — REST is more universally expected and simpler to demonstrate.

### Consequences

**Positive**:
- Demonstrates both RPC (Leptos server fns) and REST patterns in one codebase
- `/architecture` page creates a recursive proof-of-skill
- Data becomes reusable for future integrations
- Health endpoint useful for monitoring (INFRA stream)

**Negative**:
- Slight code duplication between server fns and REST handlers (acceptable — different audiences)
- Additional surface area to test

### Rationale

The marginal effort to add 5 REST endpoints is small. The portfolio signal is significant: "I know REST, I know RPC, and I know when to use each." The `/architecture` page turns the meta-narrative into a visible feature.

---

## ADR-005: Fly.io as Deployment Platform

**Date**: 2026-03-20
**Status**: Accepted
**Requested by**: INFRA stream (via research/004-questions.md)

### Context

The project needs a deployment platform that is NOT Vercel (novelty constraint from ADR-001). INFRA stream evaluated Fly.io, Shuttle.rs, and Railway.

### Decision

Use **Fly.io** for deployment.

### Alternatives Considered

1. **Shuttle.rs**: Rejected — Rust-native but limited control over the runtime, less mature for production, uncertain future.
2. **Railway**: Rejected — simpler but less educational. Fly.io's Dockerfile-based deployment teaches more about containerization.

### Consequences

**Positive**:
- Excellent Rust/Docker support, global edge network
- Persistent Fly Volumes for SQLite database
- Free tier sufficient for a portfolio site
- Teaches Dockerfile + deployment concepts (portfolio value)
- Health checks, auto-scaling, rolling deploys built-in

**Negative**:
- Requires Fly.io account setup + flyctl CLI
- SQLite on Fly Volume means single-region write (acceptable for portfolio)

### Rationale

Fly.io offers the best combination of production quality, learning value, and Rust support. The Dockerfile-based workflow itself becomes a portfolio demonstration.
