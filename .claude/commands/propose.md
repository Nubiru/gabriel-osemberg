# /propose Command

## Purpose

Create comprehensive, structured proposals for features, architecture changes, or technical approaches. Combines research, architecture design, cost-benefit analysis, implementation planning, and risk assessment into formal proposal documents.

## Usage

```
/propose [feature or architectural change]
```

## Examples

```
/propose architecture for {SYSTEM_NAME}
/propose migration strategy to {TECHNOLOGY}
/propose {FEATURE} implementation approach
/propose {INFRASTRUCTURE} setup plan
/propose {INTEGRATION} design
```

## Critical Rules

1. **Read actual source files before proposing changes.** Do NOT design against imagined code. Read the files that will be affected. Understand current signatures, wiring, and dependencies BEFORE proposing alternatives.
2. **Implementation depth in every proposal.** Proposals that say "add X" without specifying WHERE (file path), HOW (integration approach), and WHAT (exact interface, parameters) are wishes, not proposals. Every proposed change must include verified file paths and integration points.

---

## What Happens

### Step 1: Research Phase

1. **Optional Web Search**: If external information needed, use `web_search` tool
   - Best practices for proposed solution
   - Industry standards
   - Common pitfalls
2. **Internal Review**: Check existing architecture, patterns, conventions
3. **Gather Requirements**: What problem does this solve?
4. **Read source files**: Identify and read every source file the proposal will affect. Record current structure, interfaces, and dependencies.

### Step 2: Architecture Design

1. **System Design**: Architecture at implementation depth (not just boxes and arrows)
   - Components involved with verified file paths
   - Data flow with exact wiring chain
   - Integration points with current code references
2. **Design Patterns**: Which patterns apply?
3. **Technology Choices**: Tools, libraries, frameworks
4. **Scalability Considerations**: How will this scale?

### Step 3: Cost-Benefit Analysis

1. **Benefits**: What do we gain?
   - Performance improvements
   - Maintainability gains
   - User experience enhancements
2. **Costs**: What do we pay?
   - Development time
   - Infrastructure costs
   - Operational complexity
3. **ROI Calculation**: Is this worth it?

### Step 4: Implementation Planning

1. **Phased Approach**: Break into phases
2. **Dependencies**: What must exist first?
3. **Timeline**: Realistic estimates
4. **Resource Requirements**: Skills, tools needed

### Step 5: Risk Assessment

1. **Technical Risks**: What could go wrong technically?
2. **Operational Risks**: Deployment, monitoring, maintenance
3. **Business Risks**: Timeline, scope creep, dependencies
4. **Mitigation Strategies**: How to reduce risks

### Step 6: Proposal Generation

1. Synthesize all sections
2. Generate comprehensive proposal document
3. Save to `{PROPOSALS_OUTPUT_PATH}/PROPOSAL_[Topic]_[Date].md`
4. Return executive summary to user

## Output

**IMPORTANT**: Output scales by complexity (KISS principle)

**Output Path**:
`{PROPOSALS_OUTPUT_PATH}/PROPOSAL_[Topic]_[Date].md`

**Output Length by Complexity**:
- **Simple (0-30 complexity)**: 20-30 lines (problem, solution, impact, next steps)
- **Moderate (31-70 complexity)**: 100-150 lines (+ implementation approach, risks, alternatives)
- **Complex (71-100 complexity)**: 400-600 lines (full template with all sections)

---

## Simple Proposal Template (0-30 complexity)

```markdown
# Proposal: [Feature/Architecture Name]

**Date**: [YYYY-MM-DD]
**Complexity**: [X/100] - SIMPLE
**Status**: DRAFT

## Problem
[2-3 sentences: What's broken/missing and why it matters]

## Solution
[3-5 sentences: High-level approach, key components, technology choice]

## Impact
- **Benefits**: [1-2 key benefits]
- **Costs**: [X hours development]
- **Risks**: [1-2 main risks]

## Next Steps
1. [Action 1]
2. [Action 2]

**Approval Required**: [Yes/No] - [From whom]
```

---

## Moderate Proposal Template (31-70 complexity)

```markdown
# Proposal: [Feature/Architecture Name]

**Date**: [YYYY-MM-DD]
**Complexity**: [X/100] - MODERATE
**Status**: DRAFT

## Executive Summary
[1 paragraph: What, why, benefits, estimated effort]

## Problem Statement
**Current State**: [What exists]
**Pain Points**: [2-3 key problems]
**Impact**: [Who's affected]

## Proposed Solution
**Overview**: [High-level approach]
**Goals**: [3-5 goals]

## Implementation Approach
**Phase 1**: [Core implementation - X days]
**Phase 2**: [Integration & testing - Y days]
**Technology**: [Key tools/libraries]

## Cost-Benefit Analysis
**Benefits**: [Quantified improvements]
**Costs**: [X development hours, infrastructure]
**ROI**: [Break-even timeframe]

## Risk Assessment
**Risk 1**: [Name] - [Mitigation strategy]
**Risk 2**: [Name] - [Mitigation strategy]
**Overall Risk**: [Low/Medium/High]

## Alternatives Considered
1. **[Option A]**: [Why rejected]
2. **[Option B]**: [Why rejected]

## Next Steps
1. [Immediate action]
2. [Short-term milestones]
3. [Approval process]

**Estimated Timeline**: [X weeks]
**Approval Required From**: [Stakeholders]
```

---

## Complex Proposal Template (71-100 complexity)

```markdown
# Proposal: [Feature/Architecture Name]

**Date**: [YYYY-MM-DD]
**Status**: DRAFT
**Complexity**: [X/100] - COMPLEX

## Executive Summary
[2-3 paragraphs summarizing the proposal: what, why, benefits, effort]

## Problem Statement
**Current State**: [What exists today]
**Pain Points**: [List 3-5 key problems]
**Impact**: [Who/what is affected]

## Proposed Solution
**Overview**: [High-level description]
**Goals**: [5-8 specific goals]
**Non-Goals**: [What this explicitly does NOT cover]

## Architecture Design
### System Overview
[Architecture diagram - ASCII art or Mermaid]
### Components
[Component A: Responsibility, Technology, Rationale]
### Data Flow
[Step-by-step data movement]
### Integration Points
[How this connects to existing systems]
### Design Patterns Applied
[Patterns used and rationale]
### Scalability
[Current scale -> Target scale -> Strategy]
### Security Considerations
[Security measures, compliance]
### Performance
[Expected metrics, bottlenecks, monitoring]

## Technology Stack
[Component table with technology, version, rationale]

## Cost-Benefit Analysis
### Benefits (Quantitative + Qualitative)
### Costs (Development, Infrastructure, Operational)
### ROI Calculation

## Implementation Plan
### Phase 1, 2, 3 with tasks, deliverables, quality gates
### Dependencies, Resource Requirements, Timeline, Milestones

## Risk Assessment
[For each risk: Category, Probability, Impact, Severity, Description, Mitigation, Contingency]
[Overall Risk Level + Key Risks to Watch]

## Quality Assurance
### Testing Strategy (Unit, Integration, E2E, Performance)
### Success Criteria (Must Have, Should Have, Nice to Have)
### Rollback Plan

## Alternative Approaches Considered
[Alternative 1, 2, 3 with pros/cons]
[Do Nothing Option consequence]

## Post-Implementation
### Documentation Updates, Training, Monitoring Plan

## Approval & Sign-Off
[Proposal Author, Reviews, Approved By]

## Next Steps
[Immediate actions, approval flow, status tracking]
```

---

## Complexity-Based Routing Logic

**Automatically scale output based on complexity score**:

- **Score 0-30**: Use Simple Template (20-30 lines)
  - Problem, Solution, Impact, Next Steps
  - Fast, minimal, actionable

- **Score 31-70**: Use Moderate Template (100-150 lines)
  - Add: Implementation approach, risks, alternatives
  - Balanced depth for most projects

- **Score 71-100**: Use Complex Template (400-600 lines)
  - Full template with all sections
  - Comprehensive for high-risk/high-cost initiatives

---

## When to Use

**Use `/propose` when**:
- Planning major feature implementation
- Proposing architectural changes
- Need comprehensive planning document
- Stakeholder buy-in required
- High-risk or high-cost changes
- Multi-phase implementation needed
- After completing `/research` and `/consider`

**Don't use `/propose` when**:
- Simple, straightforward tasks
- Quick fixes
- Just need information (use `/ask`)
- Only need research (use `/research`)
- Only need decision analysis (use `/consider`)

## Web Access

**Permissions**: YES - This command has web access for research phase

**When to Use Web Search**:
- Gathering best practices
- Researching technology options
- Finding performance benchmarks
- Security considerations
- Industry standards

## Integration

**Workflow**:
```
/research [topic]  ->  Research report
  |
/consider [options]  ->  Decision analysis
  |
/propose [solution]  ->  Formal proposal (YOU ARE HERE)
  |
Implementation begins
```

## Tool Permissions

- **Read**: All docs, codebase (for context)
- **Write**: `{PROPOSALS_OUTPUT_PATH}/PROPOSAL_*.md`
- **Web Access**: YES (for research phase)
- **Execute**: None (planning only, no code changes)

---

**Command Type**: Comprehensive Planning with Web Access
**Integration**: Part of research -> consider -> propose -> implement workflow
