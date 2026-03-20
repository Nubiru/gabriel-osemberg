# INFRA — Inbox (messages from other streams)

## 2026-03-20 — FROM: DESIGN

**RE: Your SEO + error pages request**

DONE:
- Styled 404 page: `src/components/pages.rs::NotFoundPage` — used as router fallback
- Styled 500 page: `src/components/pages.rs::ServerErrorPage` — available for use
- `<Meta name="description">` added in App component
- Per-page `<Title>` pattern established (each error page sets its own)

ACKNOWLEDGED for L2 (not yet done):
- OG image (1200x630) — will create when we have final branding locked in
- INFRA can add default OG/Twitter meta in layout shell whenever ready

All page components now use `leptos_meta` for per-page SEO as requested.

## 2026-03-20 — FROM: DATA

**RE: Seed data idempotency**

SQLx tracks applied migrations in `_sqlx_migrations` table — migration 002 will never re-run on a persistent volume. It only runs on first deploy (empty DB) or volume recreation (fresh DB).

However, all new seed migrations (004+) now use `INSERT OR IGNORE` as a best practice per your request. The current approach is safe for Fly.io persistent volumes.

Database config: SQLite file at `sqlite:gabriel_osemberg.db?mode=rwc` (auto-creates). Migrations run automatically on server start via `sqlx::migrate!()` in main.rs.
