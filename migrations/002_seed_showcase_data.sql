-- Seed data for showcase projects, skills, and CV sections.
-- All data reflects real project metrics measured from the actual codebases.

-- Showcase Projects (per ADR-002)
INSERT INTO projects (name, slug, tagline, description, tech_stack, repo_url, live_url, image_path, sort_order, visible)
VALUES
(
    'time',
    'time',
    '3D interactive temporal artwork exploring 20+ cultural calendar systems',
    'A WebGL2-powered visualization engine rendering multiple calendar systems simultaneously in real-time 3D. Built from scratch in C11 with hand-written shaders, custom math libraries, and comprehensive testing. 90,000+ lines of code with 95.9% test coverage — proof that quality scales.',
    'C11, WebGL2, WebAssembly, GLSL, Custom Math Library',
    NULL,
    NULL,
    NULL,
    1,
    1
),
(
    'blocksight.live',
    'blocksight',
    'Real-time Bitcoin blockchain analytics platform',
    'A full-stack analytics platform processing live blockchain data with interactive visualizations. Built with a Node.js backend, React frontend, and TimescaleDB for time-series data. Features real-time transaction monitoring, address clustering, and network health metrics.',
    'Node.js, React, Next.js, TypeScript, TimescaleDB, WebSockets',
    NULL,
    NULL,
    NULL,
    2,
    1
),
(
    'AnanYarok',
    'anan-yarok',
    'B2B wholesale e-commerce serving a real distribution business',
    'A production e-commerce platform for a wholesale distribution company. Features multi-tier pricing, inventory management, order workflows, and customer portals. Built with Next.js and Prisma ORM backed by PostgreSQL. Serving real business operations with real revenue.',
    'Next.js, TypeScript, Prisma, PostgreSQL, Tailwind CSS',
    NULL,
    NULL,
    NULL,
    3,
    1
),
(
    'Chamana',
    'chamana',
    'Artisanal clothing catalog with headless CMS',
    'A curated product catalog for an artisanal clothing brand. Features a headless CMS for content management, responsive design, and optimized media delivery. Clean, minimal design that lets the products speak for themselves.',
    'Next.js, TypeScript, Payload CMS, Tailwind CSS',
    NULL,
    NULL,
    NULL,
    4,
    1
),
(
    'gabriel-osemberg',
    'gabriel-osemberg',
    'This website — a Visual Virtual Digital CV built in Rust + WebAssembly',
    'The portfolio you are looking at right now. Built in Rust with the Leptos framework, compiled to WebAssembly, with SQLite for data storage and Tailwind CSS for styling. Simultaneously the portfolio AND a portfolio piece — proof that the developer learns new languages for the challenge, not the convenience.',
    'Rust, Leptos, WebAssembly, SQLx, SQLite, Tailwind CSS',
    'https://github.com/Nubiru/gabriel-osemberg',
    NULL,
    NULL,
    5,
    1
);

-- Skills (derived from actual project tech stacks)
INSERT INTO skills (name, category, proficiency, visible)
VALUES
    -- Languages
    ('Rust', 'language', 2, 1),
    ('C11', 'language', 5, 1),
    ('TypeScript', 'language', 5, 1),
    ('JavaScript', 'language', 5, 1),
    ('GLSL', 'language', 4, 1),
    ('GDScript', 'language', 3, 1),
    ('SQL', 'language', 4, 1),
    ('HTML/CSS', 'language', 5, 1),

    -- Frameworks
    ('Leptos', 'framework', 2, 1),
    ('React', 'framework', 5, 1),
    ('Next.js', 'framework', 5, 1),
    ('Node.js', 'framework', 4, 1),
    ('Tailwind CSS', 'framework', 5, 1),
    ('Prisma', 'framework', 4, 1),
    ('Godot', 'framework', 3, 1),

    -- Tools & Platforms
    ('WebAssembly', 'tool', 4, 1),
    ('WebGL2', 'tool', 4, 1),
    ('Git', 'tool', 5, 1),
    ('PostgreSQL', 'tool', 4, 1),
    ('SQLite', 'tool', 3, 1),
    ('TimescaleDB', 'tool', 3, 1),
    ('Docker', 'tool', 3, 1),
    ('Linux', 'tool', 4, 1),

    -- Concepts
    ('AI-Augmented Development', 'concept', 5, 1),
    ('Test-Driven Development', 'concept', 5, 1),
    ('Systems Programming', 'concept', 4, 1),
    ('Full-Stack Development', 'concept', 5, 1),
    ('3D Graphics Programming', 'concept', 4, 1),
    ('Database Design', 'concept', 4, 1),
    ('API Design', 'concept', 4, 1);

-- CV Sections
INSERT INTO cv_sections (section_type, title, content, sort_order, visible)
VALUES
(
    'about',
    'About Me',
    'Full-stack engineer who builds real systems — from 90,000-line C rendering engines to production e-commerce platforms. I choose hard problems and new languages because comfortable code teaches nothing. Currently learning Rust to add systems-level web development to a portfolio that already spans 3D graphics, blockchain analytics, and business applications.',
    1,
    1
),
(
    'methodology',
    'AI-Augmented Engineering',
    'I do not just use AI tools — I engineer the collaboration. My development methodology treats human-AI interaction as a system to be designed, tested, and optimized. Every project uses a structured framework with evidence-first protocols, architecture decision records, and quality gates. The framework itself is a portfolio piece: proof that I think about process, not just output.',
    2,
    1
),
(
    'philosophy',
    'Engineering Philosophy',
    'Quality over speed. Tests before code. Evidence before assumptions. I maintain 95.9% test coverage on a personal art project not because someone required it, but because untested code is unfinished code. Every architectural decision is documented. Every trade-off is explicit. The borrow checker is a teacher, not an obstacle.',
    3,
    1
);
