-- Add Technical Writing section to CV.
-- Anthropic explicitly values blog posts and written thought pieces.
-- This seeds the infrastructure; real articles will be added in L2.

INSERT OR IGNORE INTO cv_sections (section_type, title, content, sort_order, visible)
VALUES
(
    'writing',
    'Technical Writing',
    'Writing is thinking made visible. These pieces explore the engineering decisions, learning process, and methodology behind the projects — because showing your reasoning matters as much as showing your code.

**Coming soon**: Articles on learning Rust as a C developer, engineering human-AI collaboration beyond vibe coding, and building full-stack WASM applications with Leptos.',
    3,
    1
);
