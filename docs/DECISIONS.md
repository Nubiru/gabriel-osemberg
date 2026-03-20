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
