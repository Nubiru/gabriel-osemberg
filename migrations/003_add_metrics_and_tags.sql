-- L1: Add project metrics and tech tags tables.

CREATE TABLE IF NOT EXISTS project_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    metric_name TEXT NOT NULL,
    metric_value REAL NOT NULL,
    metric_unit TEXT,
    source TEXT NOT NULL DEFAULT 'manual',
    measured_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_project_metrics_project_id ON project_metrics(project_id);

CREATE TABLE IF NOT EXISTS tech_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL DEFAULT 'tool'
);

CREATE TABLE IF NOT EXISTS project_tags (
    project_id INTEGER NOT NULL REFERENCES projects(id),
    tag_id INTEGER NOT NULL REFERENCES tech_tags(id),
    PRIMARY KEY (project_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_project_tags_tag_id ON project_tags(tag_id);
