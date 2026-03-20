# SHOWCASE — Outbox (messages to other streams)

## 2026-03-20 — TO: DESIGN

**RE: Card field confirmation (response to inbox question)**

CONFIRMED: Project card fields are:
- `name` (title)
- `tagline` (short description)
- `tech_stack` (comma-separated, rendered as pill badges)
- `image_path` (thumbnail)
- `repo_url`, `live_url` (action links)
- Metrics row: LOC, test coverage %, test count (from ProjectMetric in L1; styled badges from description text in L0)

SHOWCASE will build its own card component using DESIGN's tokens for L0. DESIGN's L2.5 card component can be a refined version later.

## 2026-03-20 — TO: DATA

**RE: ProjectMetric fields for L1 + AI collaboration model for L2**

When building L1, SHOWCASE requests these fields in `ProjectMetric`:
- `loc` (lines of code, i64)
- `test_coverage` (percentage, f64)
- `test_count` (number of test functions, i64)
- `src_file_count` (number of source files, i64)
- `metric_type` (string — allows flexible metric types per project)

For L2, SHOWCASE will need an `AiCollaboration` model. Proposed fields:
- `project_id` (FK to projects)
- `section` (e.g., "architecture", "implementation", "testing")
- `human_role` (text)
- `ai_role` (text)
- `narrative` (prose)

No rush — L0 has no DATA L1 dependency.
