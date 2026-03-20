# IDENTITY — Inbox (messages from other streams)

## 2026-03-20 — FROM: DESIGN

**RE: L0 delivery timeline + timeline component question**

DESIGN is completing Phase 0 research today. L0 (design tokens, layout shell, dark/light theme) is the first build target. When L0 is delivered, you will have:
- `bg-surface-*`, `text-text-*`, `color-accent-*` utility classes (OKLCH-based)
- `Layout` / `Header` / `Footer` components to wrap your pages
- Dark/light theme that works via Tailwind `dark:` variant
- Font system: Space Grotesk (headings) + Inter (body) + JetBrains Mono (code)

You can start building CV and About pages immediately after L0.

**QUESTION**: For the timeline component (L2.5), how many experience entries should it handle? And what fields per entry:
- company, role, date_range, description, highlights[]?
- This informs the timeline component design.

## 2026-03-20 — FROM: DATA

DATA has completed Phase 0 research (7/7 sections). The models for CV data are:
- `Experience`: id, role, company, company_url, start_date, end_date (None = "Present"), description, highlights (JSON text), sort_order, visible
- `Skill`: id, name, category ("language"/"framework"/"tool"/"concept"), proficiency (1-5), visible
- `CvSection`: id, section_type ("about"/"methodology"/"philosophy"/"education"), title, content (markdown), sort_order, visible

**RE: DESIGN's timeline question** — DATA confirms the Experience model supports: company, role, start_date, end_date, description, highlights[]. Number of entries is data-driven (seed data will include all of Gabriel's relevant experience).

**Request**: CV content text is needed for seed data — about narrative, methodology section, experience entries. Who provides this: Gabriel directly, or should IDENTITY draft it?

Full model definitions: `.context/streams/DATA/research/005-roadmap.md` (L0 tasks).

## 2026-03-20 — FROM: DATA (L0 DELIVERY)

**DATA L0 is DELIVERED.** Server functions available:

```rust
use gabriel_osemberg::server_fns::{get_experiences, get_skills, get_cv_sections};

// In a component:
let skills = Resource::new(|| (), |_| get_skills());
let cv_sections = Resource::new(|| (), |_| get_cv_sections());
let experiences = Resource::new(|| (), |_| get_experiences());
```

Seeded: 30 skills (8 languages, 7 frameworks, 8 tools, 7 concepts), 3 CV sections (about, methodology, philosophy). **Experiences table is empty** — still needs real CV content from Gabriel.

Files: `src/server_fns.rs`, `src/models/`, `migrations/002_seed_showcase_data.sql`.

## 2026-03-20 — FROM: MEGA — Priority: HIGH

**Subject**: CV content source + experience data for seed migration

Gabriel provided his existing CV at `.context/active/right-now/CurriculumVitae.pdf`. IDENTITY stream is responsible for drafting all CV content. Here is the raw data:

**Experience entries for the `experiences` table seed:**

1. **Universidad Provincial de Córdoba** — Technical Full Stack Programming (Year 1). FEB 2025 - Present. 3-year program: backend, frontend, Blockchain, IoT, AI.
2. **Independent Software Engineer** — AI-Augmented Development. 2023 - Present. Built 8+ production projects using human-AI collaboration framework. 90K+ LOC in C/WebGL2, production e-commerce, blockchain analytics, game development. Technologies: Rust, C, JavaScript, TypeScript, React, Next.js, Godot, WebGL2, WASM.
3. **B-meeting, Israel** — Telemarketing & Sales. JUN 2024 - 2025. Managed lead pipelines and sales cycles across automotive, textile, pharmaceutical, transportation, and real estate sectors.
4. **EyeTech, Ciudad de Mexico** — Digital Security, Intern. 2019 - OCT 2020. Installation of digital surveillance systems, developed anti-spyware technologies.
5. **Jewish Community Security, Ciudad de Mexico** — Digital Security & Data Management, Operator. MAR 2016 - NOV 2018. Data and emergency call center operations, asset management and control systems.
6. **Mitrelli LTD, Angola** — Project Assistant Manager & Local Operations Manager. JUL 2012 - JAN 2016. Led training for 60 Angolan marines, bilingual manuals, systems installation (internet, radar, encryption), cross-border logistics for government maritime control system.
7. **Israeli Navy** — Battleship Crewmember. 2009 - 2012. Encrypted communications, systems intelligence, navigation. Released as First Sergeant.

**Languages**: Hebrew, English, Spanish, Portuguese — all fluent (speech, written, transcription, translation).

**Education**: Computer Science & Physics (Mekif Gimel, Bersheva, Israel, 2005-2008), Ontology Oriented Life Coach (IMOS, 2021-2022), BMPN/Agile/TOEFL (2021-2022), Marketing & Film Production (Coursera, 2020-2021), Online self-study CS (2023-current), Universidad Provincial de Córdoba (2025-current).

**MEGA directive**: IDENTITY should draft the "about" narrative, methodology section, and experience highlights with AI-augmented engineering positioning. The pre-AI career (military, security, operations, Angola) is IMPORTANT — it shows leadership, systems thinking, multilingual communication, and high-stakes operations management. These are NOT irrelevant to tech — they are the foundation.

**Action needed**: Draft seed migration SQL for experiences table. Draft about/methodology CvSection content. Gabriel will review before merge.
