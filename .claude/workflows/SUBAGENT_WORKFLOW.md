# Subagent Orchestration Workflow

**Version**: 1.0
**Purpose**: Systematic task execution by delegating work to specialized subagents. The orchestrator coordinates agents, provides context, and consolidates results.

---

## Overview

**Pattern**: Orchestrator -> Select Agent(s) -> Provide Context -> Agents Execute -> Consolidate Results

**Trigger**: Task benefits from specialized analysis, parallel execution, or independent validation

---

## Available Agents

| Agent | Use Cases |
|-------|-----------|
| **planning-specialist** | Deep analysis, research, algorithm/pattern comparison, phase planning |
| **code-reviewer** | Code quality, safety, conventions compliance, performance review |
| **test-writer** | TDD test creation, coverage analysis, regression tests |

> Add new agents by creating a file in `.claude/agents/` using `_TEMPLATE.md` and registering it in this table.

---

## Orchestrator Pattern

### Phase 1: Task Analysis (Orchestrator)

**Actions**:
1. Identify task scope and type
2. Select appropriate agent(s) from the table above
3. Determine if tasks can run in parallel or must be sequential
4. Gather required context (files, specifications, standards)

**Decision**:
- **Parallel**: Tasks are independent (e.g., code review + test writing + architecture audit)
- **Sequential**: Second task depends on first result (e.g., plan first, then implement)

---

### Phase 2: Agent Dispatch (Orchestrator)

**For Each Agent**:

1. **Choose agent type** based on task nature
2. **Prepare context**:
   - Mandatory: CLAUDE.md (project overview, environment, constraints)
   - Standards: Relevant convention/standard files
   - Input: Files, specifications, test output, data
   - Scope: Which areas of the codebase are relevant
3. **Define scope**: Clear boundaries and success criteria
4. **Set constraints**: What the agent must and must not do

**Agent Invocation Template**:

```markdown
I'm dispatching you as the {Agent Name} agent for {task description}.

**Mandatory Reading** (Read these FIRST):
1. `.claude/CLAUDE.md` - Project overview, environment, partnership principles
2. `{ROADMAP_FILE}` - Current development phase and progress
3. `{RELEVANT_STANDARDS}` - Domain-specific standards
4. [Additional context files as needed]

**Task Scope**:
{Clear description of what needs to be done}

**Context**:
- Problem/Goal: {description}
- Recent Changes: {what changed recently}
- Environment: {relevant environment info}
- Available Input: {list of files, data available}

**Investigation/Execution Protocol**:
1. Read all mandatory files above
2. {Task-specific steps}
3. Analyze systematically (Evidence-First: check sources, cite file paths)
4. Provide specific, actionable output

**Output Requirements**:
- {What format/content is expected}
- File:line references for code changes
- Clear reasoning for recommendations
- Confidence levels for assessments

**Constraints**:
- DO NOT run destructive commands
- DO NOT run active git commands (add, commit, push, branch, checkout, merge, rebase)
- DO NOT execute scripts that modify state
- USE provided input files and code reading only
- USE passive git commands only (status, log, diff, show, blame)
- ANALYZE existing data and code
- REQUEST additional input if needed
- DECLARE DONE when task complete

**Success Criteria**:
{Specific, measurable outcomes}

**Reference**: `.claude/agents/{agent-name}.md`
```

---

### Phase 3: Parallel Agent Execution

**For Independent Tasks**:

Launch multiple agents simultaneously:

```
Task 1: planning-specialist - Research caching strategies
Task 2: code-reviewer - Audit authentication module
Task 3: test-writer - Create tests for payment processing

// All launch simultaneously, run independently
```

**Benefits**:
- 2-3x faster than sequential
- Maximize throughput
- Independent work streams

**When to Use**:
- Tasks have no dependencies on each other
- Each has complete context
- Results can be consolidated independently

---

### Phase 4: Agent Execution (Subagent)

**Subagent Actions**:

1. **Read Mandatory Files**:
   - CLAUDE.md (project identity, environment, constraints, partnership principles)
   - Roadmap (current phase, progress)
   - Relevant standards (conventions, architecture, etc.)

2. **Analyze Context**:
   - Read provided files (specifications, code, data)
   - Search codebase for relevant files (Glob, Grep, Read tools)
   - Understand current state and recent changes

3. **Execute Task**:
   - Apply systematic methodology (Evidence-First)
   - Identify root causes or design solutions
   - Distinguish facts from assumptions
   - State confidence levels explicitly

4. **Generate Output**:
   - Specific recommendations with file:line references
   - Code snippets where applicable
   - Clear reasoning for each recommendation
   - Trade-offs and alternatives considered

5. **Declare Status**:
   - "DONE" if task complete with actionable output
   - "NEED INPUT: {specific request}" if more data required

**Agent Output Format**:

```markdown
## Task: {Task Name}

**Analysis Summary**:
{Key findings, 2-3 sentences}

**Root Cause / Recommendation**:
{Specific cause or design decision with evidence}

**Changes Required** (if applicable):

1. File: `path/to/file:line`
   - Change: {what to change}
   - Reason: {why this change}
   - Confidence: {high/medium/low}

2. File: `path/to/file:line`
   - Change: {what to change}
   - Reason: {why this change}
   - Confidence: {high/medium/low}

**Trade-offs** (if applicable):
- Option A: {pros/cons}
- Option B: {pros/cons}
- Recommendation: {which and why}

**Status**: DONE | NEED INPUT: {specific request}
```

---

### Phase 5: Consolidation (Orchestrator)

**Orchestrator Actions**:

1. **Wait for completion**: All agents finish
2. **Read agent outputs**: Review each agent's findings
3. **Validate against evidence**: Cross-check with available data
4. **Consolidate results**: Combine findings, resolve conflicts
5. **Present summary**: Clear, actionable report to user

**Consolidation Format**:

```markdown
## Subagent Results

### Agent 1 ({agent-type}): {task}
- Findings: {key discoveries}
- Recommendations: {main recommendations}
- Confidence: {overall confidence}

### Agent 2 ({agent-type}): {task}
- Findings: {key discoveries}
- Recommendations: {main recommendations}
- Confidence: {overall confidence}

### Consolidated Recommendations

1. **{Priority Level}**: {recommendation}
   - Source: Agent(s) {which agent(s)}
   - Impact: {what this enables}
   - Effort: {estimated effort}

2. **{Priority Level}**: {recommendation}
   - Source: Agent(s) {which agent(s)}
   - Impact: {what this enables}
   - Effort: {estimated effort}

### Next Steps
- [ ] {Action 1}
- [ ] {Action 2}
- [ ] {Action 3}
```

---

### Phase 6: Implementation (Orchestrator)

**Orchestrator Actions**:

1. **Apply changes**: Implement agent recommendations systematically
2. **Update documentation**: Related docs, standards, learning log
3. **Request validation**: User runs tests/build/checks
4. **Track progress**: Update task list for completion

**Implementation Pattern**:
- Apply changes in logical order (dependencies first)
- One change at a time for complex fixes
- Verify before moving to next
- Request user validation for terminal commands

---

## Subagent Role Definitions

Beyond the specialized agents above, orchestrators can launch these task-specific subagents:

### Writer Subagent
- **Horizon**: This task only
- **Focus**: Implementation. Tests first, then code, then verify.
- **Concern**: Code quality, test coverage, compilation/lint, conventions
- **Receives**: Task spec with file paths, function signatures, test cases
- **Produces**: Source files + test files that pass

### Checker Subagent
- **Horizon**: This task only
- **Focus**: Independent validation of Writer output
- **Concern**: Standards compliance, naming, duplication, correctness
- **Receives**: Writer's output files
- **Produces**: PASS/FAIL with specific issues

### Maintainer Subagent
- **Horizon**: This task + regression
- **Focus**: Health sweep, regression gate, attribution
- **Concern**: Codebase integrity, dead code, broken references, attribution
- **Receives**: List of changed files
- **Produces**: Health report, PASS/FAIL for commit readiness

**Quality Gate Flow**:
```
Writer produces code
  -> Checker validates independently
    -> Maintainer verifies no regressions
      -> PASS: Ready to commit
      -> FAIL: Return to Writer with specific issues
```

---

## Agent Selection Guide

| Task Type | Primary Agent | Notes |
|-----------|---------------|-------|
| Algorithm/pattern research | planning-specialist | Compare approaches, analyze references |
| Code quality review | code-reviewer | Safety, conventions, performance |
| Test creation (TDD) | test-writer | Write tests first |
| Phase planning | planning-specialist | Break phases into ordered tasks |
| Architecture decisions | planning-specialist | Evaluate design trade-offs |
| Implementation | Writer subagent | TDD: test -> code -> verify |
| Validation | Checker subagent | Independent review of implementation |
| Regression check | Maintainer subagent | Ensure no regressions |

---

## Agent Constraints (Critical)

### What Agents MUST NOT Do
- Run destructive terminal commands
- Run active git commands (add, commit, push, branch, checkout, merge, rebase)
- Generate new test runs or build output without orchestrator approval
- Execute scripts or programs that modify state
- Modify files outside their designated scope
- Make assumptions without evidence
- Skip reading CLAUDE.md and relevant standards

### What Agents MUST Do
- Read CLAUDE.md FIRST (understand the project)
- Read relevant standards for their domain
- Work with provided input (files, specifications, data)
- Use Read, Glob, Grep tools for code analysis
- Use passive git commands only (status, log, diff, show, blame)
- Apply systematic, evidence-based methodology
- Provide specific, actionable output with file:line references
- State confidence levels explicitly
- Distinguish facts from assumptions
- Request additional input if needed
- Declare DONE when task complete

---

## Common Pitfalls

### Pitfall 1: Agent Runs Active Git Commands
**Problem**: Agent tries to run `git add`, `git commit`, `git push`
**Prevention**: Explicitly state "DO NOT run active git commands" in dispatch
**Recovery**: Stop agent, relaunch with clearer constraints

### Pitfall 2: Agent Runs Destructive Commands
**Problem**: Agent executes commands that modify system state
**Prevention**: Explicitly state permitted commands in dispatch
**Recovery**: Stop agent, assess damage, relaunch

### Pitfall 3: Agent Skips Mandatory Reading
**Problem**: Agent doesn't understand the project context
**Prevention**: Make CLAUDE.md the first mandatory reading item
**Recovery**: Remind agent to read CLAUDE.md, may need to relaunch

### Pitfall 4: Vague Recommendations
**Problem**: Agent says "fix the code" without specifics
**Prevention**: Require "file:line - change - reason" format in dispatch
**Recovery**: Request specific locations and changes

### Pitfall 5: Sequential When Parallel Is Possible
**Problem**: Running agents one at a time when they could run in parallel
**Prevention**: Identify independent tasks upfront during Phase 1
**Recovery**: Launch remaining agents in parallel

---

## Success Metrics

**Agent Performance**:
- 95%+ accuracy in analysis and recommendations
- 100% of recommendations include file:line references
- 0 destructive commands executed
- Clear DONE/NEED INPUT status on every task
- Confidence levels stated explicitly

**Orchestrator Performance**:
- Clear agent selection reasoning
- Proper context preparation (CLAUDE.md + standards + input)
- Accurate consolidation of findings
- Actionable next steps for user

**Workflow Efficiency**:
- Single iteration to resolution (minimal back-and-forth)
- Parallel execution used whenever tasks are independent
- Clear handoff between phases
