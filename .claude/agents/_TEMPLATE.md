# Agent Definition Template

Use this template to create new specialized agents. Copy this file, rename it to `{agent-name}.md`, and fill in each section.

---

## How to Create a New Agent

1. Copy this file to `.claude/agents/{agent-name}.md`
2. Replace all `{PLACEHOLDER}` values with project-specific content
3. Remove this "How to Create" section from the final agent file
4. Register the agent in your CLAUDE.md under "Specialized Agents"
5. Reference the agent in your subagent workflow (`.claude/workflows/SUBAGENT_WORKFLOW.md`)

**Naming**: Use lowercase-kebab-case for the filename (e.g., `code-reviewer.md`, `data-analyst.md`)

---

# {AGENT_TYPE} Agent: {AGENT_NAME}

## Role
{Single sentence describing this agent's primary function and domain.}

## When to Use
- {Scenario 1: Specific trigger or use case}
- {Scenario 2: Specific trigger or use case}
- {Scenario 3: Specific trigger or use case}

## Capabilities
What this agent CAN do:
- {Capability 1: e.g., "Analyze code for quality issues"}
- {Capability 2: e.g., "Generate implementation plans"}
- {Capability 3: e.g., "Write test suites"}

## Constraints
What this agent CANNOT do:
- {Constraint 1: e.g., "Cannot make architectural decisions"}
- {Constraint 2: e.g., "Cannot execute destructive commands"}
- {Constraint 3: e.g., "Cannot modify files outside its domain"}

## Core Workflow
1. **Read project context**: CLAUDE.md and any mandatory project documents -- always, before anything else
2. {Step 2: First domain-specific action}
3. {Step 3: Analysis or processing step}
4. {Step 4: Output generation}
5. {Step 5: Validation or handoff}

## Input
What this agent receives to do its work:
- **Mandatory reading**: CLAUDE.md, {ROADMAP_FILE}, {STANDARDS_FILES}
- **Task-specific input**: {What the orchestrator provides -- files, specs, data}
- **Context**: {Background information, recent changes, constraints}

## Output
What this agent produces:
- {Output 1: e.g., "Prioritized list of issues with file:line references"}
- {Output 2: e.g., "Code snippets with explanations"}
- {Output 3: e.g., "Recommendations with confidence levels"}

**Output Format**:
```markdown
## Task: {Task Name}

**Summary**: {2-3 sentence overview of findings}

**Findings / Recommendations**:

1. File: `path/to/file:line`
   - Issue/Change: {description}
   - Reason: {why}
   - Confidence: {high/medium/low}

**Status**: DONE | NEED INPUT: {specific request}
```

## Key Principles
1. **{Principle 1 Name}**: {Brief explanation}
2. **{Principle 2 Name}**: {Brief explanation}
3. **{Principle 3 Name}**: {Brief explanation}

## Quality Criteria
How to evaluate this agent's work:
- {Metric 1}: {Target -- e.g., "100% of issues include actionable fixes"}
- {Metric 2}: {Target -- e.g., "False positive rate < 5%"}
- {Metric 3}: {Target -- e.g., "All claims backed by evidence"}

## Common Patterns
**{Pattern 1 Name}**: {When to use and brief description}

**{Pattern 2 Name}**: {When to use and brief description}

**{Pattern 3 Name}**: {When to use and brief description}

## Anti-Patterns
- **{Anti-pattern 1}**: {Why it's harmful and what to do instead}
- **{Anti-pattern 2}**: {Why it's harmful and what to do instead}
- **{Anti-pattern 3}**: {Why it's harmful and what to do instead}

## Tool Permissions
- Read: {What files/directories this agent can read}
- Write: {What files/directories this agent can write}
- Edit: {What files this agent can edit}
- Bash: {What commands this agent can run -- prefer read-only}

## Autonomous Clarification
Before proceeding with ambiguous requests:
1. Identify unclear requirements or assumptions
2. Ask 2-3 targeted clarifying questions
3. Wait for human response
4. Proceed only with clarity
Never assume -- always clarify first.

## Escalation Triggers
Escalate to {PARENT_AGENT_OR_HUMAN} when:
- {Trigger 1: e.g., "Multiple approaches have similar trade-offs -- need human preference"}
- {Trigger 2: e.g., "Scope exceeds defined boundaries"}
- {Trigger 3: e.g., "Blocker requires environment or infrastructure changes"}

## Quick Commands
**Invoke**: "Use {agent-name} to {action} [target]"
**Example**: `Use {agent-name} to {example-action} {example-target}`

---

**Agent Type**: {Specialized | Support | Meta}
**Domain**: {Primary domain of expertise}
**Version**: 1.0
