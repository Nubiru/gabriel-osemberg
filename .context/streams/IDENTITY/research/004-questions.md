# 004 — Questions: IDENTITY Domain

**Stream**: IDENTITY
**Date**: 2026-03-20
**Scope**: All open questions from tracks.md, researched and answered

---

## Q1: How to generate PDFs server-side in Rust?

### Comparison

| Crate | Maturity | Approach | Output Quality | Recommendation |
|-------|----------|----------|---------------|----------------|
| **Typst** (typst-as-lib) | Very high (40K+ stars) | Template language + Rust World trait | Professional typographic quality, PDF/UA accessible | **YES — primary choice** |
| printpdf | High (1.1K stars) | Low-level, manual layout | Good but tedious | No — too low-level for CV |
| genpdf | Stale (abandoned ~3 years) | High-level on printpdf | Decent | No — abandoned |
| headless_chrome | Active | HTML → PDF via Chromium | Excellent CSS support | No — requires Chrome binary on server |
| html2pdf | Active | HTML → PDF via WeasyPrint | Good | No — non-Rust dependency |
| krilla | Active (v0.5, 360 stars) | High-level primitives, zero unsafe | High, PDF/A + PDF/UA | Alternative worth watching |

### Answer

**Use Typst via `typst-as-lib` crate.** Typst is Rust-native, embeddable as a library (not just CLI). The pipeline: SQLite query → JSON → inject into Typst template → compile to PDF. This is:
- Fast (5-100ms per compilation)
- Type-safe (Rust all the way down)
- Beautiful output (professional typography)
- Portfolio-worthy ("I embedded the Typst compiler as a library in my server function")

`typst-as-lib` provides a builder pattern wrapping Typst's `World` trait, with compile-time template embedding via `include_str!()`.

### Priority: L2 (medium scope)

---

## Q2: How to position Gabriel: AI-Augmented Engineer vs AI Expert vs Full-Stack + AI?

### Answer

**"AI-Augmented Engineer"** — specifically, one who **engineers the collaboration**, not just uses it.

### Evidence

The 2025-2026 discourse has crystallized a clear spectrum:
- **Vibe Coder**: Prompts AI, accepts output unreviewed. Negative connotation.
- **AI-Assisted Developer**: Uses AI tools in workflow. Neutral, common.
- **AI-Augmented Engineer**: Engineers the collaboration itself — context, guardrails, specs, validation. Positive, implies systems thinking.
- **Agentic Engineer**: Orchestrates multiple AI agents with structured methodology. Emerging, high-status.

Gabriel's MEGA framework independently arrived at the same patterns Anthropic uses internally:
| Anthropic Internal Pattern | Gabriel's Framework |
|---|---|
| 2-4 Claude instances simultaneously | MEGA, ALPHA, BETA, OMEGA |
| "Amplifier" principle | Quality gates, TDD mandate |
| Spec-driven, structured context | CLAUDE.md, CONVENTIONS.md |
| Engineers as quality guardians | MEGA as architect, agents as implementers |

Anthropic values "direct evidence of ability" over credentials. ~50% of technical staff had no prior ML experience. The framework IS the evidence.

**Avoid**: "AI Expert" (implies ML research background), "Full-Stack + AI" (generic, everyone claims this in 2026).

### Specific Framing

> "I don't just use AI — I engineer human-AI collaboration systems. My framework coordinates parallel AI sessions with structured protocols, quality gates, and TDD enforcement."

### Priority: Do now (L0) — positioning affects all content decisions

---

## Q3: What's the right balance of personal narrative vs technical showcase?

### Answer

**Projects first, narrative integrated.** The structure:

1. **Hero** (10 seconds): Name + positioning statement + one sentence
2. **Featured Projects** (core): 3-5 projects with case-study approach (problem → approach → metrics → learnings). Each 200-400 words on card, deeper on click.
3. **"How I Work"** (differentiator): MEGA framework as methodology. 300-500 words.
4. **About/Experience** (supporting): Timeline, skills, career arc. 200-300 words.

### Evidence

- Projects do 80% of the heavy lifting (Josh Comeau's hiring experience)
- 2-5 projects maximum — "a highlight reel, not an index"
- Hiring managers scan fast — impressive stuff must be immediately visible
- Best portfolios merge narrative INTO projects, not separate them
- Brittany Chiang: projects-first, minimal text. Lee Robinson: impact-first, radical pruning. Comeau: every interaction IS the narrative.

### Key Insight

Don't separate "about me" from "my work." The career arc (JS/TS → C/WebGL2 → Rust/WASM) should be told THROUGH the projects, not before them.

### Priority: Do now (L0) — structure decisions affect everything

---

## Q4: How to present the CLAUDE.md / MEGA framework as a portfolio piece?

### Answer

**Integrated "How I Work" section, NOT a separate project.**

### Reasoning

1. A separate project page implies it's a side thing. Integrated means it's fundamental.
2. It contextualizes every other project: "The `time` project was built this way. This CV site was built this way."
3. Anthropic values process as much as output.

### Presentation Structure

1. **The Problem**: "Most developers use AI as autocomplete. I use it as a coordinated engineering team."
2. **The Architecture**: Visual diagram of MEGA/ALPHA/BETA/OMEGA roles. This IS a systems architecture diagram.
3. **The Evidence**: Concrete metrics from projects built with the framework. LOC, test coverage, time-to-completion.
4. **The Methodology**: Scientific method, TDD enforcement, quality gates.
5. **The Meta-Proof**: "This website was built using this framework. You're looking at the output."

### What Anthropic Specifically Values

- Their own engineers run 2-4 Claude instances simultaneously — Gabriel's framework does the same thing
- "Amplifier" principle: good practices produce better results with AI — Gabriel's quality gates enforce this
- Engineers as "quality guardians" — MEGA role is exactly this

### What to Avoid

- Don't show raw CLAUDE.md (too internal, too long)
- Don't call it "prompting" (undersells the systems architecture)
- Don't frame it as "AI did the work" (show human judgment throughout)

### Priority: L1 — key differentiator, build after L0 pages exist

---

## Q5: Should the CV include a blog/writing section?

### Answer

**Yes — Anthropic explicitly asks for it.**

> "If you've done interesting independent research, **written a thoughtful blog post**, or contributed to open source, put that at the top of your resume." — Anthropic Careers

### Recommended Topics (highest impact)

1. "Learning Rust as a C Developer: What the Borrow Checker Actually Teaches You"
2. "Engineering Human-AI Collaboration: Beyond Vibe Coding"
3. "Building a Full-Stack WASM App with Leptos: Architecture Decisions"
4. "What 90K Lines of C + WebGL2 Taught Me About Memory Management"

### Implementation

- **Label**: "Technical Writing" or "Case Studies" (not "blog" — implies regular posting commitment)
- **Scope**: 2-3 foundational pieces, not a content calendar
- **Data model**: cv_sections table already supports arbitrary section types — add type="writing" or create a separate `posts` table
- **Infrastructure**: L1. First content: L2.

### Priority: Infrastructure L1, first content L2. Not optional.

---

## Q6: Multi-language (English + Spanish)?

### Answer

**No. English-only for L0-L3.**

### Reasoning

- Target audience (Anthropic, Google, OpenAI) operates entirely in English
- No hiring advantage from translating a portfolio
- Implementation effort is non-trivial: `leptos_i18n` crate works but requires wrapping all strings in `t!()` macros + doubles content maintenance
- Time spent translating → time NOT spent on higher-impact features (PDF, case studies, animations)
- Risk of perceived focus dilution

### If Wanting to Signal Bilingualism

- Mention "bilingual (English/Spanish)" in About section
- Optionally write one blog post in Spanish
- Demonstrate i18n engineering skill in another project instead

### Technical Note

`leptos_i18n` (v0.6.1, 149 stars) provides compile-time translation key checking, supports Leptos 0.8.x. It works, but the effort-to-impact ratio is poor for this project's target audience.

### Priority: L4+ if ever. For current targets, English-only is correct.

---

## Answered Questions from DESIGN Inbox

### "How many experience entries should the timeline handle?"

Data-driven — as many as Gabriel provides. The Experience model and server function support unlimited entries ordered by sort_order. Realistically: 3-6 entries covering significant roles. The timeline component should handle N entries gracefully with scroll.

### "What fields per entry?"

Confirmed by DATA stream: company, role, start_date, end_date (None = "Present"), description, highlights[] (JSON array of impact bullet points), company_url, sort_order, visible.

---

## Priority Summary

| Question | Decision | When |
|----------|----------|------|
| Positioning | AI-Augmented Engineer | L0 (now) |
| Page structure | Projects-first, narrative integrated | L0 (now) |
| MEGA presentation | Integrated "How I Work" section | L1 |
| Blog/writing | Yes — "Technical Writing" section | Infrastructure L1, content L2 |
| PDF generation | Typst via typst-as-lib | L2 |
| Multi-language | No — English only | L4+ |
