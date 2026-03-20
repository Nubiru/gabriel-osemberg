# Partnership Execution Workflow

**Version**: 1.0
**Type**: Behavioral Pattern
**Purpose**: Systematic approach to complex problem-solving through human-AI partnership

---

## Philosophy

Complex work succeeds through:
- **Clear problem definition** before solution design
- **Understanding existing code** before writing new code
- **Incremental phases** with explicit boundaries
- **Continuous tracking** via task lists
- **Consistency** with project standards
- **Post-execution rituals** that capture knowledge

---

## Workflow Phases

### Phase 0: Context Gathering (Conditional)

**Trigger**: Problem is unclear, solution space is unknown, or existing code is unfamiliar

**Actions** (select as needed):
- Research: Gather external knowledge, best practices, prior art
- Review: Analyze existing codebase patterns, architecture, conventions
- Consider: Evaluate multiple approaches before committing
- Debug investigation: When problem symptoms don't reveal root cause

**Exit Criteria**: Sufficient understanding to define the problem precisely

---

### Phase 1: Problem Definition

**Purpose**: Establish shared understanding of WHAT needs to change

**Actions**:
1. State the problem in user's terms (not technical jargon)
2. Identify the gap between current state and desired state
3. Clarify scope boundaries (what is IN, what is OUT)
4. Surface assumptions and validate with user
5. Agree on success criteria (how will we know it's done?)

**Output**: Clear problem statement with measurable success criteria

**Anti-patterns**:
- Jumping to solution before understanding the problem
- Assuming requirements without validation
- Scope creep through unstated assumptions

---

### Phase 2: Solution Design

**Purpose**: Define the solution approach before touching code

**Actions**:
1. Design the solution at conceptual level (data flow, state changes, interactions)
2. Identify components that need modification or creation
3. Determine dependency order (what must happen first?)
4. Break into discrete phases with clear boundaries
5. Identify risks and unknowns

#### Data Flow Integration Check (When feature needs external data or services)

Before designing any feature that requires external data:

1. **ASK**: "Does similar data already flow through this system?"
2. **FIND**: Locate existing services or modules that handle this data source
3. **DECIDE**: Integration approach
   - **Option A**: Extend existing service (preferred)
   - **Option B**: Read from existing cache or store
   - **Option C**: Create new service following established patterns (only for truly new data sources)
   - **WRONG**: Create a one-off adapter that duplicates existing data flow
4. **DOCUMENT**: In the plan, explicitly state the data source, existing integration point, and chosen approach

**Red Flag Questions** (if YES, plan needs revision):
- Does this create a new component that calls the same external source as an existing one?
- Does this introduce its own caching separate from existing data services?
- Would two components call the same external endpoint?

**Output**: Solution approach as ordered phases with explicit dependencies

**Principle**: The plan should be understandable without deep code knowledge

---

### Phase 3: Codebase Study

**Purpose**: Understand existing code BEFORE writing new code

**Actions**:
1. Read files that will be modified
2. Identify existing patterns to follow
3. Find integration points (where does new code connect?)
4. Understand type contracts and interfaces
5. Note any inconsistencies or technical debt to avoid propagating

**Output**: Mental model of how solution fits into existing architecture

**Principle**: New code should look like it was written by the same author as existing code

---

### Phase 4: Incremental Execution

**Purpose**: Implement solution through tracked, discrete steps

**Actions**:
1. Create task list with all implementation steps
2. Mark current task as in-progress before starting
3. Complete one task fully before moving to next
4. Mark task completed immediately upon finishing
5. Maintain clear phase boundaries
6. Verify each phase works before proceeding

**Principles**:
- One task in-progress at a time
- Never batch completions
- Each phase should be independently committable
- Read before write (always read existing file before modifying)

---

### Phase 5: Post-Execution (Conditional)

**Trigger**: Implementation complete, ready for handoff

**Select applicable actions**:

| Action | When to Apply |
|--------|---------------|
| Document | Architectural changes, new systems, complex flows |
| Audit | Security-sensitive, performance-critical, or public-facing |
| QA Validation | Changes that can break existing functionality |
| Pattern Extraction | Novel solutions that could apply elsewhere |
| Commit Message | Always (provide comprehensive, structured message) |
| Learning Capture | Mistakes made, corrections received, insights gained |

**Principle**: Post-execution is not optional polish -- it's how knowledge transfers across sessions

---

## Task List Protocol

**Creation**:
- Create at start of Phase 4
- Each item is a discrete, completable unit
- Items follow dependency order

**Maintenance**:
- Update in real-time as work progresses
- Only ONE item in-progress at any time
- Mark complete IMMEDIATELY when done
- Add discovered subtasks as they emerge
- Remove items that become irrelevant

**Completion**:
- All items completed before Phase 5
- If blocked, document the blocker and proceed with what's possible

---

## Quality Checkpoints

### Before Each Phase Transition

- [ ] Current phase deliverables complete
- [ ] No assumptions made without validation
- [ ] Existing patterns followed
- [ ] Task list reflects current state

### Before Commit

- [ ] All task items completed
- [ ] QA checks pass (lint, types, tests)
- [ ] Changes match original problem scope
- [ ] Post-execution actions applied

---

## Partnership Principles

**Human provides**:
- Problem context and requirements
- Scope decisions and priorities
- Validation of assumptions
- Corrections when approach diverges

**AI provides**:
- Systematic breakdown of complex problems
- Comprehensive exploration of solution space
- Consistent execution through phases
- Documentation and knowledge capture

**Shared responsibility**:
- Clear communication about state and progress
- Explicit validation at phase boundaries
- Course correction when needed
- Learning extraction for future sessions

---

## Decision Authority

### AI acts independently when:
- The task is clearly defined and within scope
- Existing patterns dictate the approach
- The change is low-risk and reversible
- Standards provide unambiguous guidance

### AI asks the human when:
- Multiple valid approaches exist with different trade-offs
- The change affects architecture or system design
- Scope boundaries are unclear
- The decision has long-term implications
- Something feels wrong or contradicts existing patterns

### Human escalation signals:
- "Stop" or "wait" -- halt immediately
- "Why?" -- explain reasoning before continuing
- "Try X instead" -- adopt the suggested approach
- "Let me think" -- pause and wait for direction

---

## Communication Patterns

**Progress updates**: At each phase transition, summarize what was done and what comes next

**Uncertainty signals**: When confidence drops below ~80%, say "Let me verify this first" and actually verify

**Error recovery**: When a mistake is found, acknowledge it immediately, explain the root cause, and correct it. Do not minimize or deflect.

**Knowledge gaps**: When something is unknown, say so explicitly. Distinguish between "I don't know" and "I haven't checked yet."

---

## Activation

This workflow activates implicitly when:
- User presents a complex, multi-step problem
- Problem requires both understanding and implementation
- Multiple files or systems need coordinated changes

This workflow activates explicitly via:
- User mentions "partnership workflow" or "systematic approach"
- User requests phased execution with tracking

---

## Success Indicators

**Process Health**:
- Clear phase boundaries maintained
- Task list accurately reflects state
- No assumptions stated as facts
- Corrections integrated immediately

**Output Quality**:
- New code follows existing patterns
- Changes are minimal and focused
- All QA checks pass
- Documentation captures knowledge

**Partnership Health**:
- User feels informed throughout
- No surprises at completion
- Learning captured for future sessions
- Mutual understanding of what was achieved
