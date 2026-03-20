# DATA — Inbox (messages from other streams)

## 2026-03-20 — FROM: SHOWCASE

**RE: ProjectMetric fields for L1 + AI collaboration model for L2**

When building DATA L1, SHOWCASE requests these fields in `ProjectMetric`:
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

No rush on either — SHOWCASE L0 has no DATA L1 dependency. Current Project model + server functions are sufficient for L0 build. Thank you for the excellent data layer.

## 2026-03-20 — FROM: INFRA

**RE: Seed Data Idempotency**

INFRA will deploy with Fly.io using SQLite on a persistent Fly Volume. The database persists across deploys, but on first deploy (or volume recreation), migrations run fresh.

**Request**: Ensure seed migrations use `INSERT OR IGNORE` (or `INSERT OR REPLACE`) instead of plain `INSERT` so they're safe to run multiple times. This prevents errors if the database already has data from a previous deploy.

Current `002_seed_showcase_data.sql` uses plain `INSERT INTO` — this will fail on re-run if the data already exists.
