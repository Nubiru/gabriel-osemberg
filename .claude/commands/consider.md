# /consider Command

## Purpose

Think through implications, evaluate options, and reason about decisions using structured analysis frameworks. Generates decision documents with recommendations based on systematic evaluation.

## Usage

```
/consider [decision or question to analyze]
```

## Examples

```
/consider {OPTION_A} vs {OPTION_B} for {USE_CASE}
/consider security implications of {APPROACH}
/consider tradeoffs of adding {FEATURE_OR_COMPONENT}
/consider should we use {TECHNOLOGY_A} or {TECHNOLOGY_B}?
/consider migration strategy for {SYSTEM}
```

## What Happens

### Step 1: Decision Framing

1. **Identify the decision**: What are we trying to decide?
2. **List options**: What are the alternatives? (A, B, C, or Do Nothing)
3. **Define success criteria**: What makes a good decision here?
4. **Identify stakeholders**: Who/what is impacted?

### Step 2: Factor Analysis

1. **Performance**: Speed, scalability, resource usage
2. **Security**: Vulnerabilities, compliance, data protection
3. **Maintainability**: Code complexity, learning curve, debugging
4. **Cost**: Development time, infrastructure, operational
5. **Developer Experience**: Tooling, documentation, community
6. **Risk**: Technical debt, vendor lock-in, future-proofing

### Step 3: Options Evaluation

1. **Evaluate each option** against factors (1-5 scale)
2. **Apply decision frameworks**:
   - Decision Matrix (weighted scoring)
   - Pros/Cons lists
   - Trade-off analysis
   - Cost-Benefit analysis
3. **Cross-reference with**:
   - Project conventions and standards
   - Quality gate requirements
   - Existing architecture decisions

### Step 4: Recommendation

1. **Score options** using decision matrix
2. **Determine recommendation** (with confidence level)
3. **Justify decision** with clear reasoning
4. **Note considerations** for future (risks, unknowns)

### Step 5: Document Generation

1. Generate decision document
2. Save to `{DECISIONS_OUTPUT_PATH}/DECISION_[Topic]_[Date].md`
3. Return summary to user

## Output

**Decision Document File**:
`{DECISIONS_OUTPUT_PATH}/DECISION_[Topic]_[Date].md`

**Document Structure**:

```markdown
# Decision Analysis: [Topic]

**Date**: [YYYY-MM-DD]
**Status**: RECOMMENDATION READY

## Decision Question

[Clear statement of what we're deciding]

## Context

[Why this decision is needed now]
[Project constraints, requirements, current state]

## Options

### Option A: [Name]

[Description]

### Option B: [Name]

[Description]

### Option C: Do Nothing (Baseline)

[What happens if we don't act]

## Evaluation Factors

Factors rated on 1-5 scale (1=Poor, 5=Excellent)

### 1. Performance

[How this factor applies to this decision]

### 2. Security

[Security implications]

### 3. Maintainability

[Long-term maintenance considerations]

### 4. Cost

[Development + operational costs]

### 5. Developer Experience

[Team productivity, learning curve]

### 6. Risk

[Technical debt, vendor lock-in, uncertainty]

## Decision Matrix

| Option     | Performance | Security | Maintainability | Cost | Dev Experience | Risk | **Total** | **Weighted Total** |
| ---------- | ----------- | -------- | --------------- | ---- | -------------- | ---- | --------- | ------------------ |
| Option A   | X/5         | X/5      | X/5             | X/5  | X/5            | X/5  | X/30      | X/100              |
| Option B   | X/5         | X/5      | X/5             | X/5  | X/5            | X/5  | X/30      | X/100              |
| Do Nothing | X/5         | X/5      | X/5             | X/5  | X/5            | X/5  | X/30      | X/100              |

**Weights Applied**: [Factor weights based on project priorities]

## Pros & Cons

### Option A

**Pros**:
- [Benefit 1]
- [Benefit 2]

**Cons**:
- [Drawback 1]
- [Drawback 2]

### Option B

[...]

## Trade-Off Analysis

### Option A vs Option B

[Key trade-offs to consider]

### Option A vs Do Nothing

[Cost of action vs cost of inaction]

## Recommendation

**Recommended Option**: [Option A / Option B / Do Nothing]

**Confidence Level**: [High / Medium / Low]

**Rationale**:
1. [Primary reason - scored highest on critical factors]
2. [Secondary reason - aligns with project goals]
3. [Tertiary reason - manageable risks]

**When to Reconsider**:
- [Condition 1 that would change recommendation]
- [Condition 2 that would change recommendation]

## Alignment with Standards

**Conventions**: [Which standards this aligns with or contradicts]
**Quality Gates**: [How this satisfies quality requirements]
**Architecture**: [How this fits current architecture]

## Implementation Considerations

**If Choosing Option A**:
- [Key consideration 1]
- [Key consideration 2]

**If Choosing Option B**:
- [Key consideration 1]
- [Key consideration 2]

## Risks & Mitigation

### Risk 1: [Risk Description]

**Probability**: [Low / Medium / High]
**Impact**: [Low / Medium / High]
**Mitigation**: [How to reduce risk]

### Risk 2: [...]

## Future Considerations

**Short-term** (0-6 months):
[What to watch for]

**Long-term** (6+ months):
[How this decision may evolve]

## Next Steps

1. [Action item 1]
2. [Action item 2]
3. Consider using `/propose` to formalize implementation plan

## References

- [Research reports if `/research` was used]
- [External sources if relevant]

---

**Decision Validity**: [Date], review if context changes significantly
**Review By**: [Date 3-6 months out for long-term decisions]
```

## When to Use

**Use `/consider` when**:
- Weighing two or more technical approaches
- Evaluating trade-offs before committing to a direction
- Need structured analysis, not just gut feeling
- Want documented reasoning for future reference
- After `/research` gathered external data, now need to decide

**Don't use `/consider` when**:
- Need external research first (use `/research`)
- Ready to formalize a plan (use `/propose`)
- Just need information (use `/ask`)
- Decision is trivial or already made

## Integration

**Workflow**:
```
/research [topic]  ->  Research report
  |
/consider [options]  ->  Decision analysis (YOU ARE HERE)
  |
/propose [solution]  ->  Formal proposal
  |
Implementation begins
```

## Tool Permissions

- **Read**: All project docs, codebase (for context)
- **Write**: `{DECISIONS_OUTPUT_PATH}/DECISION_*.md`
- **Web Access**: Optional (if additional data needed during analysis)
- **Execute**: None (analysis only, no code changes)
- **Escalate**: To `/propose` for formal implementation plan

---

**Command Type**: Decision Analysis
**Integration**: Part of research -> consider -> propose -> implement workflow
