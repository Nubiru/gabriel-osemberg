# 006 — Dependencies: Cross-Stream Needs & Provides

**Stream**: SHOWCASE
**Date**: 2026-03-20
**Status**: COMPLETE

---

## What SHOWCASE Needs

### From DATA

| What | Layer | Status | Blocking? |
|------|-------|--------|-----------|
| `Project` model + `get_projects()` + `get_project_by_slug()` | L0 | **DELIVERED** — models in src/models/, server fns in src/server_fns.rs | No |
| `ProjectMetric` model (per-project quantitative data: LOC, coverage, test count) | L1 | NOT BUILT — DATA L1 work | No (L0 uses description text + badges) |
| `TechTag` model (normalized tech names as entities) | L1 | NOT BUILT — DATA L1 work | No (L0 parses tech_stack string) |
| GitHub stats caching (stars, commits, languages per repo) | L2 | NOT BUILT — DATA L1-L2 work | No |
| AI collaboration data model (human vs AI contribution per project) | L2 | NOT DESIGNED — needs joint design | No |

### From DESIGN

| What | Layer | Status | Blocking? |
|------|-------|--------|-----------|
| Design tokens (@theme block) | L0 | **DELIVERED** — style/tailwind.css | No |
| Layout shell (Layout, Header, Footer) | L0 | **DELIVERED** — src/components/layout.rs | No |
| Dark/light theme | L0 | **DELIVERED** — src/components/theme_toggle.rs | No |
| Typography (Space Grotesk, Inter, JetBrains Mono) | L0 | **DELIVERED** — public/fonts/ | No |
| Card component template | L2.5 | NOT BUILT — SHOWCASE will build its own with tokens | No |
| Base component library (Button, Badge, Link) | L1 | PENDING — DESIGN L1 | Non-blocking (can inline styles) |

### From IDENTITY

| What | Layer | Status | Blocking? |
|------|-------|--------|-----------|
| CV content text for about/methodology narrative | L2 | PENDING — IDENTITY not launched | No |
| Project summary data for PDF | L2 | PENDING | No |

### From INFRA

| What | Layer | Status | Blocking? |
|------|-------|--------|-----------|
| Deployment pipeline | L3 | PENDING — INFRA not launched | No |
| CI/CD for automated testing | L3 | PENDING | No |

---

## What SHOWCASE Provides

### To IDENTITY

| What | Layer | When Available |
|------|-------|---------------|
| Project card component (reusable for CV projects section) | L0 | After L0-001 |
| Metrics badge component (reusable for skills display) | L0 | After L0-006 |
| Project highlights data (top metrics for CV PDF) | L1 | After L1-003 |

### To DESIGN

| What | Layer | When Available |
|------|-------|---------------|
| Card field requirements (confirmed from inbox) | NOW | **CONFIRMED**: name, tagline, tech_stack[], image_path, repo_url, live_url. Metrics (LOC, tests, coverage) from L1. |
| SVG chart components (reusable patterns) | L1 | After L1-001, L1-002 |

### To INFRA

| What | Layer | When Available |
|------|-------|---------------|
| Route inventory for sitemap generation | L0 | After L0-004 |
| Page meta tags for SEO | L1 | After L1-007 |

### To DATA

| What | Layer | When Available |
|------|-------|---------------|
| AI collaboration data structure requirements | L2 | Before L2-001 build |

---

## Dependency Timeline

```
NOW (L0 start):
  ✓ DATA: Project model + server functions
  ✓ DESIGN: Tokens + layout + theme + fonts

BEFORE L1:
  NEED: Project screenshots (Gabriel manual task)
  NICE: DESIGN base component library (Button, Badge)

BEFORE L2:
  NEED: DATA L1 — ProjectMetric model
  NEED: DATA L1 — TechTag model
  NICE: IDENTITY CV content for methodology section

BEFORE L3:
  NEED: INFRA deployment pipeline
  NEED: Video assets (Gabriel manual task)
```

---

## Messages to Send

### → DESIGN inbox (responding to card field question)

**CONFIRMED**: Project card fields are:
- `name` (title)
- `tagline` (short description)
- `tech_stack` (comma-separated, rendered as pill badges)
- `image_path` (thumbnail)
- `repo_url`, `live_url` (action links)
- Metrics row (LOC, test coverage %, test count) — comes from L1 `ProjectMetric`, until then from styled badges in description

SHOWCASE will build its own card component using DESIGN's tokens. DESIGN's L2.5 card component can be a refined version later.

### → DATA inbox

**REQUEST**: When building L1, please include these fields in `ProjectMetric`:
- `loc` (lines of code, i64)
- `test_coverage` (percentage, f64)
- `test_count` (number of test functions, i64)
- `src_file_count` (number of source files, i64)
- `metric_type` (string — allows flexible metric types)

Also: SHOWCASE will need an `AiCollaboration` model at L2. Proposed fields:
- `project_id` (FK)
- `section` (e.g., "architecture", "implementation", "testing")
- `human_role` (text description of human contribution)
- `ai_role` (text description of AI contribution)
- `narrative` (prose explaining the collaboration)
