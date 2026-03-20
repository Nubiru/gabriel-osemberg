-- L1: Seed project metrics and tech tags from actual project analysis.
-- Uses INSERT OR IGNORE for idempotency (per INFRA request).

-- Tech Tags (normalized technology names)
INSERT OR IGNORE INTO tech_tags (name, category) VALUES
    ('C11', 'language'),
    ('Rust', 'language'),
    ('TypeScript', 'language'),
    ('JavaScript', 'language'),
    ('GLSL', 'language'),
    ('GDScript', 'language'),
    ('SQL', 'language'),
    ('WebGL2', 'technology'),
    ('WebAssembly', 'technology'),
    ('WebSockets', 'technology'),
    ('Leptos', 'framework'),
    ('React', 'framework'),
    ('Next.js', 'framework'),
    ('Node.js', 'framework'),
    ('Godot', 'framework'),
    ('Tailwind CSS', 'framework'),
    ('Prisma', 'framework'),
    ('Payload CMS', 'framework'),
    ('SQLx', 'tool'),
    ('SQLite', 'tool'),
    ('PostgreSQL', 'tool'),
    ('TimescaleDB', 'tool'),
    ('Docker', 'tool'),
    ('Git', 'tool'),
    ('Linux', 'tool');

-- Project -> Tag associations
-- time: C11, WebGL2, WebAssembly, GLSL
INSERT OR IGNORE INTO project_tags (project_id, tag_id)
SELECT p.id, t.id FROM projects p, tech_tags t
WHERE p.slug = 'time' AND t.name IN ('C11', 'WebGL2', 'WebAssembly', 'GLSL');

-- blocksight: Node.js, React, Next.js, TypeScript, TimescaleDB, WebSockets
INSERT OR IGNORE INTO project_tags (project_id, tag_id)
SELECT p.id, t.id FROM projects p, tech_tags t
WHERE p.slug = 'blocksight' AND t.name IN ('Node.js', 'React', 'Next.js', 'TypeScript', 'TimescaleDB', 'WebSockets');

-- anan-yarok: Next.js, TypeScript, Prisma, PostgreSQL, Tailwind CSS
INSERT OR IGNORE INTO project_tags (project_id, tag_id)
SELECT p.id, t.id FROM projects p, tech_tags t
WHERE p.slug = 'anan-yarok' AND t.name IN ('Next.js', 'TypeScript', 'Prisma', 'PostgreSQL', 'Tailwind CSS');

-- chamana: Next.js, TypeScript, Payload CMS, Tailwind CSS
INSERT OR IGNORE INTO project_tags (project_id, tag_id)
SELECT p.id, t.id FROM projects p, tech_tags t
WHERE p.slug = 'chamana' AND t.name IN ('Next.js', 'TypeScript', 'Payload CMS', 'Tailwind CSS');

-- gabriel-osemberg: Rust, Leptos, WebAssembly, SQLx, SQLite, Tailwind CSS
INSERT OR IGNORE INTO project_tags (project_id, tag_id)
SELECT p.id, t.id FROM projects p, tech_tags t
WHERE p.slug = 'gabriel-osemberg' AND t.name IN ('Rust', 'Leptos', 'WebAssembly', 'SQLx', 'SQLite', 'Tailwind CSS');

-- Project Metrics (from portfolio inventory — real measured values)
-- time
INSERT OR IGNORE INTO project_metrics (project_id, metric_name, metric_value, metric_unit, source, measured_at)
SELECT p.id, m.metric_name, m.metric_value, m.metric_unit, m.source, m.measured_at
FROM projects p,
(SELECT 'loc' AS metric_name, 90000.0 AS metric_value, 'lines' AS metric_unit, 'cloc' AS source, '2026-03-19' AS measured_at
 UNION ALL SELECT 'test_count', 14789.0, 'functions', 'manual', '2026-03-19'
 UNION ALL SELECT 'test_coverage', 95.9, '%', 'manual', '2026-03-19'
 UNION ALL SELECT 'src_file_count', 3773.0, 'files', 'find', '2026-03-19'
) m WHERE p.slug = 'time';

-- blocksight
INSERT OR IGNORE INTO project_metrics (project_id, metric_name, metric_value, metric_unit, source, measured_at)
SELECT p.id, m.metric_name, m.metric_value, m.metric_unit, m.source, m.measured_at
FROM projects p,
(SELECT 'src_file_count' AS metric_name, 2876.0 AS metric_value, 'files' AS metric_unit, 'find' AS source, '2026-03-19' AS measured_at
 UNION ALL SELECT 'test_file_count', 1620.0, 'files', 'find', '2026-03-19'
) m WHERE p.slug = 'blocksight';

-- anan-yarok
INSERT OR IGNORE INTO project_metrics (project_id, metric_name, metric_value, metric_unit, source, measured_at)
SELECT p.id, m.metric_name, m.metric_value, m.metric_unit, m.source, m.measured_at
FROM projects p,
(SELECT 'src_file_count' AS metric_name, 1048.0 AS metric_value, 'files' AS metric_unit, 'find' AS source, '2026-03-19' AS measured_at
 UNION ALL SELECT 'test_file_count', 246.0, 'files', 'find', '2026-03-19'
) m WHERE p.slug = 'anan-yarok';

-- chamana
INSERT OR IGNORE INTO project_metrics (project_id, metric_name, metric_value, metric_unit, source, measured_at)
SELECT p.id, m.metric_name, m.metric_value, m.metric_unit, m.source, m.measured_at
FROM projects p,
(SELECT 'src_file_count' AS metric_name, 203.0 AS metric_value, 'files' AS metric_unit, 'find' AS source, '2026-03-19' AS measured_at
 UNION ALL SELECT 'test_file_count', 21.0, 'files', 'find', '2026-03-19'
) m WHERE p.slug = 'chamana';
