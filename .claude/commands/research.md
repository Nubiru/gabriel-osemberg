# /research Command

## Purpose

Deep investigation with web access to research external information, investigate technologies, compare approaches, and synthesize findings into comprehensive research reports.

## Usage

```
/research [topic or question]
```

## Examples

```
/research best practices for {TOPIC} implementation
/research {TECHNOLOGY_A} vs {TECHNOLOGY_B} for {USE_CASE}
/research latest security vulnerabilities in {FRAMEWORK}
/research current state of {TECHNOLOGY_AREA}
/research performance optimization techniques for {DOMAIN}
/research industry standards for {PRACTICE}
```

## What Happens

### Step 1: Research Phase

1. **Web Search**: Use `web_search` tool to gather external information
   - Search for best practices, documentation, comparisons
   - Look for recent articles, official docs, community discussions
   - Prioritize authoritative sources (official docs, well-known blogs, academic papers)

2. **Multi-Source Synthesis**: Collect from 3-5 different sources
   - Ensure diversity of perspectives
   - Cross-reference recommendations
   - Identify common patterns and disagreements

### Step 2: Internal Cross-Reference

1. Check against internal standards:
   - `{CONVENTIONS_PATH}` - Coding standards
   - `{QUALITY_GATES_PATH}` - Quality requirements
   - `{ARCHITECTURE_PATH}` - Project architecture (if exists)

2. Identify conflicts or synergies with current project

### Step 3: Analysis & Synthesis

1. **Pros/Cons Analysis**: For each option or approach
2. **Trade-offs**: Performance, security, maintainability, cost
3. **Recommendation**: Based on project context
4. **References**: URLs, dates, source credibility

### Step 4: Report Generation

1. Generate comprehensive research report
2. Save to `{RESEARCH_OUTPUT_PATH}/RESEARCH_[Topic]_[Date].md`
3. Return summary to user

## Output

**Research Report File**:
`{RESEARCH_OUTPUT_PATH}/RESEARCH_[Topic]_[Date].md`

**Report Structure**:

```markdown
# Research Report: [Topic]

**Date**: [YYYY-MM-DD]
**Status**: COMPLETE

## Executive Summary
[2-3 sentence high-level findings]

## Research Question
[Original question or topic]

## Findings

### Source 1: [Source Name]
**URL**: [https://...]
**Date**: [publication date]
**Credibility**: [Official docs / Industry blog / Academic / Community]

**Key Points**:
- [Key point 1]
- [Key point 2]

### Source 2: [Source Name]
[...]

## Synthesis

### Common Recommendations
[What all sources agree on]

### Divergent Opinions
[Where sources disagree and why]

### Industry Trends
[Current direction, what's gaining traction]

## Pros & Cons

### Option A: [Approach 1]
**Pros**:
- [Benefit 1]
- [Benefit 2]

**Cons**:
- [Drawback 1]
- [Drawback 2]

### Option B: [Approach 2]
[...]

## Recommendation for This Project

**Recommended Approach**: [A / B / Hybrid]

**Rationale**:
1. [Reason 1 based on project context]
2. [Reason 2 based on findings]
3. [Reason 3 considering trade-offs]

**Alignment with Internal Standards**:
- Conventions: [alignment notes]
- Quality Gates: [compliance notes]

## Implementation Considerations
[Practical notes for if/when this is implemented]

## Next Steps
[What to do after this research]

## References
1. [Source 1 full citation]
2. [Source 2 full citation]
[...]

---

**Research Validity**: [Date], valid for ~3-6 months (tech changes fast)
**Follow-Up Research**: [Topics requiring deeper investigation]
```

## When to Use

**Use `/research` when**:
- Need external information (not in codebase/docs)
- Evaluating technology options (library, framework, tool)
- Looking for best practices (industry standards)
- Security considerations (vulnerabilities, recommendations)
- Performance optimization techniques
- Architectural pattern comparisons
- Current state of technology (what's the latest?)
- Before making major technical decisions

**Don't use `/research` when**:
- Information is in internal docs (use `/ask` instead)
- Need immediate decision (use `/consider` for analysis)
- Want formal proposal (use `/propose` after research)
- Ready to implement (research is input, not action)
- Simple internal questions (use `/ask`)

## Web Access

**Permissions**: YES - This command has web access

**Search Strategy**:
1. Use specific, technical queries
2. Include year for time-sensitive topics
3. Search multiple perspectives (official docs, blogs, discussions)
4. Verify source credibility

**Source Credibility Hierarchy**:
1. **Official Documentation** (highest credibility)
   - Language/framework official docs
   - Standards bodies (OWASP, NIST, W3C, IETF, etc.)

2. **Industry Leaders** (high credibility)
   - Recognized expert blogs and publications
   - Major tech company engineering blogs

3. **Community Consensus** (medium-high credibility)
   - Stack Overflow (high-voted answers)
   - GitHub discussions (popular repos)
   - Established tech publications

4. **General Articles** (medium credibility)
   - Tech blogs (less established)
   - Tutorial sites
   - Personal blogs (cross-reference needed)

## Integration

**Workflow**:
```
/research [topic]  ->  Research report generated
  |
/consider [should we use option A or B?]  ->  Decision analysis
  |
/propose [implementation approach]  ->  Formal proposal
  |
Implementation begins
```

## Anti-Patterns

### Anti-Pattern 1: Research Without Context
**Problem**: Researching without checking project needs first
**Fix**: Understand project context first, then research specific options

### Anti-Pattern 2: Single-Source Research
**Problem**: Relying on one source
**Fix**: Always use 3-5 diverse sources

### Anti-Pattern 3: Outdated Research
**Problem**: Using old sources (>2 years in fast-moving tech)
**Fix**: Prioritize recent sources, note publication dates

### Anti-Pattern 4: Research Without Recommendation
**Problem**: Presenting facts without conclusion
**Fix**: Always provide recommendation based on project context

## Validation Checklist

**Before Starting Research**:
- [ ] Research question is clear
- [ ] Not already documented internally (checked with `/ask`)
- [ ] Relevant to current project needs

**During Research**:
- [ ] 3-5 sources collected
- [ ] Sources are credible (official docs, industry leaders)
- [ ] Sources are recent (<2 years for most tech)
- [ ] Multiple perspectives represented

**Before Generating Report**:
- [ ] Common recommendations identified
- [ ] Divergent opinions explained
- [ ] Trade-offs clearly articulated
- [ ] Recommendation aligns with project context
- [ ] All claims have citations
- [ ] Next steps defined

## Tool Permissions

- **Read**: All project docs, codebase (for context)
- **Write**: `{RESEARCH_OUTPUT_PATH}/RESEARCH_*.md`
- **Web Access**: YES
- **Execute**: None (research only, no code changes)
- **Escalate**: To `/consider` for decision analysis, `/propose` for formal proposal

---

**Command Type**: Research with Web Access
**Integration**: Part of research -> consider -> propose -> implement workflow
