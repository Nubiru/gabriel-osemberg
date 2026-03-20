-- Seed identity data: experiences, CV sections (military, education), spoken languages.
-- Content sourced from Gabriel_Osemberg_CV_v2.md (Gabriel-approved).
-- Uses INSERT OR IGNORE for idempotency (safe to re-run).

-- Experience entries (sort_order: 1 = most recent)
INSERT OR IGNORE INTO experiences (role, company, company_url, start_date, end_date, description, highlights, sort_order, visible)
VALUES
(
    'Independent Software Engineer — AI-Augmented Development',
    'Self-Employed',
    'https://github.com/Nubiru',
    '2023',
    NULL,
    'Architecting and shipping production-grade systems using a structured human–AI collaboration methodology. AI handles the 70% (boilerplate, patterns); I own the 30% (architecture, edge cases, judgment).',
    '["Architected a 90,200-line WebAssembly system with 95.9% line coverage across 348 modules","Built a real-time Bitcoin blockchain analytics platform with sub-2-second detection latency over PostgreSQL + TimescaleDB","Engineered 5 production systems totaling 4,000+ source files and 1,800+ test files across C, Rust, TypeScript, and GDScript","Developed a structured human–AI engineering methodology (multi-agent orchestration, formal specifications, evidence-first validation)","Currently building this portfolio in Rust + Leptos + WebAssembly as a deliberate learning challenge"]',
    1,
    1
),
(
    'Telemarketing & Sales',
    'B-meeting',
    NULL,
    'Jun 2024',
    '2025',
    'Managed lead pipelines and full sales cycles across five sectors through multilingual client communication.',
    '["Managed lead pipelines across automotive, textile, pharmaceutical, transportation, and real estate sectors","Built client relationships through multilingual communication (Hebrew, English, Spanish)"]',
    2,
    1
),
(
    'Digital Security Intern',
    'EyeTech',
    NULL,
    '2019',
    'Oct 2020',
    'Assisted in installation and configuration of digital surveillance systems and contributed to anti-spyware technology development.',
    '["Installed and configured digital surveillance systems","Contributed to anti-spyware technology development","Applied knowledge of network security and hardware integration"]',
    3,
    1
),
(
    'Digital Security & Data Management Operator',
    'Jewish Community Security',
    NULL,
    'Mar 2016',
    'Nov 2018',
    'Managed data operations and emergency call center serving a security-critical environment with zero data incidents during tenure.',
    '["Managed data operations and emergency call center in a security-critical environment","Zero data incidents during entire tenure","Oversaw digital asset management, access control systems, and real-time incident coordination"]',
    4,
    1
),
(
    'Project Assistant Manager & Local Operations Manager',
    'Mitrelli LTD',
    NULL,
    'Jul 2012',
    'Jan 2016',
    'Led training programs and managed operations for a government maritime control system in Angola, bridging Israeli engineering teams with Angolan military personnel.',
    '["Led training programs for 60 Angolan marines on government maritime control systems","Created bilingual technical manuals (Portuguese/English) and delivered hands-on instruction","Managed installation of internet, radar, and encrypted communication systems across multiple sites","Coordinated cross-border logistics between Israel and Angola for a government defense contract","Operated as on-ground operations manager bridging Israeli engineering teams with Angolan military — requiring fluency in Portuguese, English, and Hebrew simultaneously"]',
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
    '["Operated encrypted communications systems aboard naval vessels","Systems intelligence and navigation responsibilities","Released as First Sergeant"]',
    6,
    1
);

-- CV Sections: military service
INSERT OR IGNORE INTO cv_sections (section_type, title, content, sort_order, visible)
VALUES
(
    'military',
    'Military Service',
    'Served in the Israel Navy (2009–2012) as a Battleship Crewmember in roles involving encrypted communications, systems intelligence, and navigation. Released as First Sergeant. This experience built the foundation for systems thinking, operating under pressure, and managing complex technical systems — skills that translate directly to software engineering.',
    4,
    1
);

-- CV Sections: education
INSERT OR IGNORE INTO cv_sections (section_type, title, content, sort_order, visible)
VALUES
(
    'education',
    'Education',
    '**Universidad Provincial de Córdoba**, Argentina — Technical Full Stack Programming (Year 1 of 3), February 2025 – Present. Backend, frontend, Blockchain, IoT, AI.

**Self-directed Computer Science** — Online courses, bootcamps, intensive self-study, 2023 – Present. JavaScript, React, Next.js, Python, C, Rust, WebGL2, WebAssembly.

**Mekif Gimel**, Bersheva, Israel — Computer Science & Physics, 2005 – 2008.

**Additional**: Ontology Coaching (IMOS, 2021–2022), Agile/BPMN + TOEFL Certificate (2021–2022), Marketing & Film Production (Coursera, 2020–2021).',
    5,
    1
);

-- CV Sections: languages
INSERT OR IGNORE INTO cv_sections (section_type, title, content, sort_order, visible)
VALUES
(
    'languages',
    'Languages',
    '**Hebrew** · **English** · **Spanish** · **Portuguese** — Fluent in all four (speech, written, transcription, translation).',
    6,
    1
);

-- Update existing about section to match CV v2 summary voice
UPDATE cv_sections SET content =
'AI-Augmented Software Engineer who builds production-grade systems with measurable quality — 95.9% test coverage, 90,000+ lines of C/WebAssembly, and 8+ shipped projects across blockchain analytics, e-commerce, and real-time 3D visualization. Transitioned from military encrypted communications, international operations management, and digital security into software engineering — applying the same systems thinking, cross-cultural leadership, and execution under pressure. Combines deep technical ownership with a structured human–AI collaboration methodology where AI is a force multiplier for engineering judgment, not a replacement for it.'
WHERE section_type = 'about';

-- Update existing methodology section to match CV v2 AI-Augmented Engineering Methodology
UPDATE cv_sections SET content =
'I don''t just use AI — I engineer the collaboration. My structured framework treats AI as a force multiplier within rigorous quality gates:

**Multi-agent orchestration**: Parallel AI sessions (MEGA pattern) each owning a domain — architecture, data, UI, infrastructure — coordinated through file-based protocols.

**Test-first development**: AI writes tests before code. The test is the specification. No exceptions.

**Formal specifications**: Every project has a CLAUDE.md (project identity + rules), SOUL.md (AI''s perspective), and Architecture Decision Records for every significant choice.

**Evidence-first validation**: No claim of "fixed" without verification. Every change is compiled, tested, linted, and reviewed before commit.

AI handles the 70% — boilerplate, patterns, repetition. I own the 30% — the architecture, the edge cases, the decisions that matter.'
WHERE section_type = 'methodology';

-- Spoken language skills (separate from programming languages)
INSERT OR IGNORE INTO skills (name, category, proficiency, visible)
VALUES
    ('Hebrew', 'spoken-language', 5, 1),
    ('English', 'spoken-language', 5, 1),
    ('Spanish', 'spoken-language', 5, 1),
    ('Portuguese', 'spoken-language', 5, 1);
