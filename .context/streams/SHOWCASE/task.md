# SHOWCASE Task: L1 Integration — Metrics Visualization + Project Enhancement

**Claimed by**: SHOWCASE stream
**Date**: 2026-03-20
**Status**: IN PROGRESS

## Scope

Build SVG chart components (progress ring, bar chart), metrics dashboard, and integrate metrics + normalized tech tags into project cards and detail pages.

## Tasks

- [ ] L1-001: Progress ring SVG component (`src/components/charts/progress_ring.rs`)
- [ ] L1-002: Metric stat component (`src/components/charts/metric_stat.rs`)
- [ ] L1-003: Metrics dashboard per project (`src/components/metrics_dashboard.rs`)
- [ ] L1-006: Enhance project card with metrics row
- [ ] L1-007: Enhance project detail with metrics dashboard + normalized tech tags
- [ ] Wire charts module into components/mod.rs

## Dependencies (all DELIVERED)

- `ProjectMetric` model: `src/models/project_metric.rs`
- `TechTag` model: `src/models/tech_tag.rs`
- `get_project_metrics(project_id)`: `src/server_fns.rs`
- `get_project_tags(project_id)`: `src/server_fns.rs`
- Design tokens: `style/tailwind.css`
