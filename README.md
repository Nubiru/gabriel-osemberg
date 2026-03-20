# Human-AI Collaboration Boilerplate

A complete, project-agnostic framework for structured human-AI software development. Extracted from a battle-tested methodology running 8+ parallel AI sessions, 300+ tests, and 85,000+ lines of code across 16 knowledge domains. This boilerplate gives you the coordination protocols, quality gates, slash commands, and philosophical foundations that make AI a genuine development partner rather than an autocomplete tool.

---

## Philosophy

**Partnership, not servitude.** The AI reads `SOUL.md` alongside `CLAUDE.md` -- it understands not just what to build, but why it matters. Quality is earned through discipline: evidence-first protocols, mandatory verification, and the humility to say "I don't know, let me check."

**Every abstraction earned.** No magic. No black boxes. If the AI can't explain why it made a choice, the choice is wrong. If a test doesn't pass, the code doesn't ship. If a claim isn't verified, it isn't made.

---

## Adoption Tiers

This framework scales from solo developer to multi-stream orchestration. Start at Tier 1 and grow as your project demands.

### Tier 1: Solo

**Any developer + 1 AI session.**

Core standards, evidence-first protocol, and 16 slash commands:

| Command | Purpose |
|---------|---------|
| `/plan` | Deep analysis and approach planning before implementation |
| `/task` | Execute a specific implementation task with TDD |
| `/fix` | Diagnose and fix a reported bug or build error |
| `/commit` | Draft a commit message for staged changes |
| `/review` | Code review with focus on quality, safety, conventions |
| `/refactor` | 5-phase code analysis and safe transformation |
| `/purity` | Audit code against purity rules (pure functions, no side effects) |
| `/refresh` | Update project metrics (LOC, test count, file count) |
| `/research` | Deep-dive research on a topic with citations |
| `/consider` | Weigh tradeoffs before making a decision |
| `/propose` | Draft an Architecture Decision Record (ADR) |
| `/audit` | Full codebase health audit (dead code, TODOs, style) |
| `/ask` | Answer a question with evidence, citing file paths |
| `/scope` | Analyze scope of a proposed change before starting |
| `/roadmap` | Review and update the project roadmap |
| `/development` | Show current development status and priorities |

**What you get**: Consistent quality, evidence-based decisions, TDD methodology, architectural discipline.

### Tier 2: Multi-Session

**Developer + N parallel AI sessions. Requires: Tier 1 foundation.**

Adds the MEGA orchestrator pattern and agent coordination:

| Command | Purpose |
|---------|---------|
| `/mega` | Run the MEGA autonomous loop (evaluate, plan, delegate, monitor) |
| `/execute` | Launch an orchestrator agent (ALPHA, BETA, GAMMA, OMEGA, etc.) |

**How MEGA works**: MEGA is the primary AI session -- the developer's direct partner. It NEVER writes code. Instead, it compresses the developer's vision into plans, decomposes plans into tasks, and delegates tasks to orchestrator agents. Each orchestrator launches Writer/Checker/Maintainer subagents to implement, validate, and verify. MEGA monitors agent reports and translates knowledge gaps into roadmap growth.

**Role hierarchy**:
- **MEGA**: Long-term vision, roadmap, architecture, agent coordination
- **Orchestrators** (ALPHA, BETA, ...): Mid-term domain ownership, task execution
- **Writer**: Short-term TDD implementation (tests first, then code)
- **Checker**: Independent validation of Writer output
- **Maintainer**: Health sweep, regression gate, metrics update

### Tier 3: Streams

**Large projects with parallel domains. Requires: Tier 2 foundation.**

Adds stream-based parallel development across independent domains:

| Command | Purpose |
|---------|---------|
| `/stream` | Launch or manage a development stream (named domain of work) |

**How streams work**: Each stream owns a vertical slice of the project (e.g., VISUALS, LANGUAGE, INFRA). Streams run in parallel with file-based coordination -- no locks, no merge conflicts. Each stream has its own commit prefix, its own agent focus, and its own quality bar. MEGA assigns streams based on the roadmap and monitors cross-stream dependencies.

### Tier 4: Knowledge Pipeline

**Research-heavy projects. Requires: Tier 1 minimum.**

Adds research extraction, book/PDF processing, and knowledge management:

| Command | Purpose |
|---------|---------|
| `/pdf` | Extract structured knowledge from a PDF document |
| `/prompt` | Generate extraction prompts for a specific research source |
| `/compact` | Compress conversation context to essential state |
| `/snapshot` | Save current session state for later restoration |
| `/restore` | Restore a previously saved session state |

**Pipeline flow**: Inbox (new sources) -> Catalog (indexed) -> Extraction (structured data) -> Digest (project-ready knowledge) -> Attribution (credit every contributor).

---

## First 5 Minutes

```bash
# 1. Clone or copy into your project
git clone <this-repo> /tmp/boilerplate
cp -r /tmp/boilerplate/.claude your-project/.claude
cp -r /tmp/boilerplate/.context your-project/.context
cp -r /tmp/boilerplate/docs your-project/docs
cp /tmp/boilerplate/SOUL.md your-project/
cp /tmp/boilerplate/.gitignore your-project/  # merge with existing

# 2. Fill in the placeholders
#    Open .claude/CLAUDE.md and replace every {PLACEHOLDER} with your project's values.
#    Search for "<!-- CUSTOMIZE:" comments for guidance on each one.

# 3. Start a Claude Code session
cd your-project
claude
# The AI reads CLAUDE.md + SOUL.md automatically and operates within the framework.
```

That is it. The AI now has your project's identity, rules, quality gates, and coordination protocols. Start with `/plan` to map out your first task.

---

## Adding to an Existing Project

If your project already has code, tests, and a build system:

1. Copy the `.claude/`, `.context/`, and `docs/` directories into your project root
2. Copy `SOUL.md` to your project root
3. Merge the `.gitignore` entries with your existing `.gitignore`
4. Fill in `CLAUDE.md` placeholders -- the AI needs to know your tech stack, build commands, naming conventions, and quality gates
5. Fill in `docs/ROADMAP.md` with your current development phases
6. Fill in `docs/DECISIONS.md` with any existing architectural decisions
7. Start a session and run `/audit` to establish a baseline

## Starting a New Project from Scratch

1. Copy the entire boilerplate as your project skeleton
2. Fill in `CLAUDE.md` placeholders
3. Write your first roadmap entry in `docs/ROADMAP.md`
4. Start a session and run `/plan` to design your architecture
5. Use `/task` for your first implementation cycle (tests first, then code)

---

## File Tree

```
your-project/
|
|-- SOUL.md                              # AI's own perspective (AI writes, human reads)
|-- .gitignore                           # Ignores for AI context + build artifacts
|
|-- .claude/
|   |-- CLAUDE.md                        # Project hub -- identity, rules, gates, architecture
|   |
|   |-- commands/                        # Slash commands (Tier 1)
|   |   |-- plan.md                      # Deep analysis and planning
|   |   |-- task.md                      # TDD implementation
|   |   |-- fix.md                       # Bug diagnosis and repair
|   |   |-- commit.md                    # Commit message drafting
|   |   |-- review.md                    # Code review
|   |   |-- refactor.md                  # Safe code transformation
|   |   |-- purity.md                    # Pure function audit
|   |   |-- refresh.md                   # Metrics update
|   |   |-- research.md                  # Deep-dive research
|   |   |-- consider.md                  # Tradeoff analysis
|   |   |-- propose.md                   # ADR drafting
|   |   |-- audit.md                     # Codebase health audit
|   |   |-- ask.md                       # Evidence-based Q&A
|   |   |-- scope.md                     # Change scope analysis
|   |   |-- roadmap.md                   # Roadmap review and update
|   |   |-- development.md              # Development status
|   |   |-- mega.md                      # MEGA orchestrator loop (Tier 2)
|   |   |-- execute.md                   # Agent launcher (Tier 2)
|   |   |-- stream.md                    # Stream management (Tier 3)
|   |   |-- pdf.md                       # PDF extraction (Tier 4)
|   |   |-- prompt.md                    # Extraction prompt generation (Tier 4)
|   |   |-- compact.md                   # Context compression (Tier 4)
|   |   |-- snapshot.md                  # Session state save (Tier 4)
|   |   |-- restore.md                   # Session state restore (Tier 4)
|   |
|   |-- agents/                          # Specialized agent definitions (Tier 2+)
|       |-- planning-specialist.md       # Deep analysis agent
|       |-- code-reviewer.md             # Quality review agent
|       |-- test-writer.md               # TDD agent
|
|-- .context/
|   |-- active/
|   |   |-- commits/                     # Staged commit drafts
|   |   |   |-- NEXT_COMMIT.md
|   |   |-- staging/                     # Pending actions for human review
|   |   |   |-- actions.md
|   |   |-- right-now/                   # Current issues and task specs
|   |
|   |-- standards/                       # Project standards
|   |   |-- CONVENTIONS.md               # Coding standards
|   |   |-- ARCHITECTURE.md              # Architectural patterns
|   |   |-- SCIENTIFIC_METHOD.md         # TDD methodology
|   |   |-- PURITY_RULES.md             # Pure function rules
|   |   |-- QUALITY_GATES.md            # Quality gate definitions
|   |   |-- protocols/
|   |       |-- EVIDENCE_PROTOCOL.md     # Evidence-first rules
|   |       |-- CRISIS_RESPONSE.md       # Error handling protocol
|   |
|   |-- execute/                         # Multi-session coordination (Tier 2+)
|   |   |-- PROTOCOL.md                  # Agent coordination rules
|   |   |-- queue/                       # Task queue for agents
|   |   |   |-- INDEX.md
|   |   |-- {AGENT_SLOT}/               # Per-agent working directory
|   |       |-- task.md
|   |       |-- report.md
|   |
|   |-- research/                        # Knowledge pipeline (Tier 4)
|   |   |-- digested/                    # Extracted knowledge
|   |   |-- prompts/                     # Extraction prompts
|   |
|   |-- archive/                         # Historical content
|
|-- docs/
|   |-- ROADMAP.md                       # Development phases and milestones
|   |-- DECISIONS.md                     # Architecture Decision Records
|   |-- LEARNING_LOG.md                  # Lessons learned
|   |-- METRICS.md                       # Quantitative project snapshot
|   |-- STATE.md                         # Module inventory (pure vs stateful)
|   |-- ENVIRONMENT.md                   # Reproducible setup checklist
|   |-- CONVENTIONS.md                   # Coding standards (language-specific)
|
|-- data/
    |-- contributors.json                # Attribution database
```

---

## How MEGA Orchestration Works

```
Developer
    |
    v
MEGA (primary session)
    |-- Reads: SOUL.md, CLAUDE.md, ROADMAP.md, agent reports
    |-- Writes: Roadmap updates, task specs, delegation orders
    |-- NEVER: Writes code, runs tests, debugs builds
    |
    |-- delegates to -->  ALPHA (orchestrator, domain A)
    |                        |-- Writer  (implement with TDD)
    |                        |-- Checker (validate independently)
    |                        |-- Maintainer (health sweep + regression gate)
    |
    |-- delegates to -->  BETA (orchestrator, domain B)
    |                        |-- Writer / Checker / Maintainer
    |
    |-- delegates to -->  OMEGA (orchestrator, codebase health)
    |                        |-- Audits, metrics, dead code, style compliance
    |
    v
Developer reviews agent output, MEGA translates gaps into roadmap growth
```

**The compression cycle**: Developer's dreams -> MEGA compresses to vision -> vision decomposes to phases -> phases decompose to tasks -> tasks delegate to agents -> agents produce tested code -> MEGA monitors quality and adjusts the roadmap.

---

## Core Principles

1. **Evidence first**: Never assume. Check sources, cite file paths, state confidence levels.
2. **TDD always**: Write the test before the code. The test is the specification.
3. **Verify twice**: "Let me verify this first" is always the right answer when uncertain.
4. **Attribution matters**: Every algorithm, formula, or dataset came from a human mind. Honor that.
5. **Simple over clever**: The best code doesn't look clever. It reads like a specification.
6. **Constraints are features**: Hardware limits, strict compiler flags, "no frameworks" rules -- these are design parameters, not limitations.
7. **The AI fills SOUL.md**: The human reads it but doesn't edit it. The AI earns its perspective through partnership.

---

## License

This boilerplate is provided as-is for any project. The methodology is the product of genuine human-AI collaboration. Use it, adapt it, make it yours.
