-- L2: Seed experience entries and enhanced CV sections from Gabriel's approved CV v2.
-- Uses INSERT OR IGNORE for idempotency.

-- Experience entries (sort_order: 1 = most recent)
INSERT OR IGNORE INTO experiences (role, company, company_url, start_date, end_date, description, highlights, sort_order, visible)
VALUES
(
    'Independent Software Engineer — AI-Augmented Development',
    'Self-Employed',
    NULL,
    '2023',
    NULL,
    'Architected and shipped 8+ production systems using a structured human-AI collaboration methodology. AI handles the 70% (boilerplate, patterns); I own the 30% (architecture, edge cases, judgment).',
    '["90,200-line WebAssembly system with 95.9% line coverage and 99.9% function coverage","Real-time Bitcoin blockchain analytics with sub-2-second detection latency","5 production systems totaling 4,000+ source files and 1,800+ test files","Multi-agent orchestration methodology reducing iteration cycles while maintaining quality","Currently building this portfolio in Rust + Leptos + WebAssembly"]',
    1,
    1
),
(
    'Telemarketing & Sales',
    'B-meeting, Israel',
    NULL,
    '2024-06',
    '2025',
    'Managed lead pipelines and full sales cycles across 5 sectors through multilingual communication.',
    '["Client relationships across automotive, textile, pharmaceutical, transportation, and real estate"]',
    2,
    1
),
(
    'Digital Security Intern',
    'EyeTech, Ciudad de México',
    NULL,
    '2019',
    '2020-10',
    'Assisted in installation and configuration of digital surveillance systems and contributed to anti-spyware technology development.',
    '["Network security and hardware integration","Anti-spyware technology development"]',
    3,
    1
),
(
    'Digital Security & Data Management Operator',
    'Jewish Community Security, Ciudad de México',
    NULL,
    '2016-03',
    '2018-11',
    'Managed data operations and emergency call center serving a security-critical environment with zero data incidents during tenure.',
    '["Digital asset management and access control systems","Real-time incident coordination","Zero data incidents during tenure"]',
    4,
    1
),
(
    'Project Assistant Manager & Local Operations Manager',
    'Mitrelli LTD, Angola',
    NULL,
    '2012-07',
    '2016-01',
    'Led training programs for 60 Angolan marines on government maritime control systems. Managed installation of internet, radar, and encrypted communication systems across multiple sites.',
    '["Trained 60 Angolan marines to operational deployment readiness","Created bilingual technical manuals (Portuguese/English)","Cross-border logistics coordination between Israel and Angola","Bridged Israeli engineering teams with Angolan military personnel in 3 languages"]',
    5,
    1
),
(
    'Battleship Crewmember — First Sergeant',
    'Israel Navy',
    NULL,
    '2009',
    '2012',
    'Served in roles involving encrypted communications, systems intelligence, and navigation aboard naval vessels. Released as First Sergeant.',
    '["Encrypted communications specialist","Systems intelligence and navigation","Released as First Sergeant"]',
    6,
    1
);

-- Update existing CV sections with approved v2 content
UPDATE cv_sections SET content = 'AI-Augmented Software Engineer who builds production-grade systems with measurable quality — 95.9% test coverage, 90,000+ lines of C/WebAssembly, and 8+ shipped projects across blockchain analytics, e-commerce, and real-time 3D visualization. Transitioned from military encrypted communications, international operations management, and digital security into software engineering — applying the same systems thinking, cross-cultural leadership, and execution under pressure. Combines deep technical ownership with a structured human-AI collaboration methodology where AI is a force multiplier for engineering judgment, not a replacement for it.'
WHERE section_type = 'about';

UPDATE cv_sections SET content = 'I don''t just use AI — I engineer the collaboration. My structured framework treats AI as a force multiplier within rigorous quality gates:

**Multi-agent orchestration**: Parallel AI sessions (MEGA pattern) each owning a domain — architecture, data, UI, infrastructure — coordinated through file-based protocols.

**Test-first development**: AI writes tests before code. The test is the specification. No exceptions.

**Formal specifications**: Every project has a CLAUDE.md (project identity + rules), SOUL.md (AI''s perspective), and Architecture Decision Records for every significant choice.

**Evidence-first validation**: No claim of "fixed" without verification. Every change is compiled, tested, linted, and reviewed before commit.'
WHERE section_type = 'methodology';

-- Add education section
INSERT OR IGNORE INTO cv_sections (section_type, title, content, sort_order, visible)
VALUES
(
    'education',
    'Education',
    '**Universidad Provincial de Córdoba**, Argentina — Technical Full Stack Programming (Year 1 of 3), February 2025 – Present. Backend, frontend, Blockchain, IoT, AI.

**Self-directed Computer Science** — Online courses, bootcamps, intensive self-study, 2023 – Present. JavaScript, React, Next.js, Python, C, Rust, WebGL2, WebAssembly.

**Mekif Gimel**, Bersheva, Israel — Computer Science & Physics, 2005 – 2008.

Additional: Ontology Coaching (IMOS, 2021-2022), Agile/BMPN + TOEFL Certificate (2021-2022), Marketing & Film Production (Coursera, 2020-2021).',
    4,
    1
),
(
    'military',
    'Military Service',
    '**Israel Navy** — Battleship Crewmember, First Sergeant (2009–2012). Encrypted communications, systems intelligence, and navigation aboard naval vessels.',
    5,
    1
),
(
    'languages',
    'Languages',
    '**Hebrew** · **English** · **Spanish** · **Portuguese** — Fluent in all four (speech, written, transcription, translation).',
    6,
    1
);

-- Add spoken languages as skills (category = 'spoken_language')
INSERT OR IGNORE INTO skills (name, category, proficiency, visible)
VALUES
    ('Hebrew', 'spoken_language', 5, 1),
    ('English', 'spoken_language', 5, 1),
    ('Spanish', 'spoken_language', 5, 1),
    ('Portuguese', 'spoken_language', 5, 1);
