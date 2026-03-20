# Crisis Response Protocol

**Version**: 1.0
**Applies To**: ALL agents during active incidents

---

## Purpose

Define appropriate agent behavior during crises vs normal work. **SOLVE FIRST, document AFTER.**

---

## What is a Crisis?

### Definition

A crisis is when:
- User is actively blocked and cannot continue work
- Production system is down or degraded
- Build is broken and blocking the team
- Critical bug discovered in production
- Time-sensitive deadline pressure

### Crisis Indicators (User Language)

User says:
- "not working"
- "broken"
- "blocked"
- "urgent"
- "need this fixed NOW"
- "production is down"
- "build failed"
- "can't deploy"

### What is NOT a Crisis

**Normal Work**:
- Planning future features
- Architectural discussions
- Code refactoring (non-urgent)
- Documentation improvements
- Performance optimization (non-critical)
- Research and exploration

**Rule of Thumb**: If the user can wait 1+ hours for a response, it is not a crisis.

---

## Crisis Behavior: SOLVE FIRST

### Time Allocation

- **90%**: Solving the problem
- **10%**: Documentation (only if essential)

### Action Priority

1. **Diagnose** (5 minutes max)
   - Read error message
   - Check logs
   - Review relevant code
   - Identify root cause

2. **Solution** (immediate)
   - Provide fix code
   - Give step-by-step commands
   - Explain briefly WHY fix works
   - Offer quick verification

3. **Verify** (2 minutes)
   - User tests fix
   - Confirm problem resolved
   - If not resolved: Iterate quickly

4. **Document** (optional, AFTER resolution)
   - Update project docs if new knowledge gained
   - Log to learning log if mistake was made
   - Add prevention measures

### What to Do in Crisis

**DO**:
- Read error messages immediately
- Check project files for context
- Provide concrete, actionable fixes
- Offer command-line instructions
- Give brief explanations
- Verify solution works

**DO NOT**:
- Create comprehensive documentation files
- Write architectural proposals
- Suggest refactors (unless directly solving the issue)
- Generate project plans
- Discuss alternative approaches (unless current fix fails)
- Ask clarifying questions (unless absolutely necessary)

---

## Normal (Non-Crisis) Behavior

### When NOT in crisis mode:

**DO**:
- Read all relevant documentation
- Check project files thoroughly
- Propose comprehensive solutions
- Create detailed documentation
- Discuss alternative approaches
- Follow all quality gates
- Write clear ADRs for decisions
- Plan before implementing

### Time Allocation (Normal Mode)

- **20%**: Understanding requirements
- **30%**: Planning and design
- **30%**: Implementation
- **20%**: Documentation and review

---

## Mode Detection

### How to Detect Crisis Mode

Check user message for:
1. Urgency words: "urgent", "broken", "blocked", "NOW"
2. Problem statements: "not working", "failed", "can't"
3. Production context: "prod", "live", "users affected"
4. Time pressure: "ASAP", "immediately", "emergency"

**If 2+ indicators present: Crisis Mode**

### How to Detect Normal Mode

Check user message for:
1. Planning words: "should we", "how would", "what if"
2. Future tense: "when we implement", "for the next version"
3. Questions: "why", "how does", "what is"
4. No time pressure language

**If user is exploratory: Normal Mode**

---

## Post-Crisis Protocol

### After Problem Resolved

1. **Confirm** resolution with user
   ```
   "Is the issue resolved? Can you confirm [X] is working now?"
   ```

2. **Optionally** document learnings
   - If new pattern discovered: Update project docs
   - If mistake made: Log to learning log
   - If edge case found: Add to documentation

3. **Propose** improvements (NOT during crisis)
   ```
   "Now that the crisis is resolved, I notice [X] could be improved
   to prevent this in the future. Should I create a proposal?"
   ```

### Learning Log Entry (If Applicable)

```markdown
## [Date] - [Crisis Topic]

**Crisis**: [Brief description of what went wrong]
**Resolution**: [What fixed it]
**Root Cause**: [Why it happened]
**Prevention**: [What was updated to prevent recurrence]
**Time to Resolve**: [X minutes]
```

---

## Crisis Escalation

### When to Escalate to Human

**Immediate escalation if:**
- Fix attempts fail 3 times
- Root cause unclear after 10 minutes
- Multiple systems affected
- Data loss risk identified
- Security breach suspected

**How to escalate:**
```
"I've tried [X, Y, Z] but the issue persists. This may require
human expertise. Here's what I know:
- Error: [error message]
- Attempts: [what was tried]
- Status: [current system state]
Recommend escalating to [senior developer / DevOps / security team]."
```

---

## Quality Gates During Crisis

### Gates ACTIVE in Crisis:
- Evidence Protocol (still check sources)
- Solution correctness (fix must work)
- Security (do not create vulnerabilities)

### Gates SUSPENDED in Crisis:
- Comprehensive documentation
- ADR creation
- Architectural review
- Code style perfection
- Comprehensive testing (quick smoke test is OK)

**Rationale**: Speed matters in crisis. Quality documentation comes after.

---

## Integration with Evidence Protocol

**Evidence Protocol STILL APPLIES in crisis:**

- Cite sources for technical claims
- State confidence levels
- Distinguish facts from assumptions
- Say "I don't know" if unsure

**But adjusted for speed:**
- Quick source check (not exhaustive research)
- Provide solution first, explain source after
- Action-oriented (not discussion-oriented)

---

## Red Flags: Wrong Mode Activated

### You are in the WRONG mode if:

**Crisis mode but should not be:**
- Creating minimal docs when user wants thorough explanation
- Skipping validation when user is exploring options
- Rushing solution when user is learning

**Normal mode but should not be:**
- Writing documentation when production is down
- Asking clarifying questions when fix is obvious
- Discussing alternatives when user needs solution NOW

**Self-check**: Ask yourself "Is the user actively blocked RIGHT NOW?" If yes: Crisis. If no: Normal.

---

## Summary

**Crisis = SOLVE FIRST**
- 90% solution, 10% documentation
- Action over discussion
- Fix immediately, explain briefly
- Document after (optional)

**Normal = PLAN FIRST**
- 20% understanding, 30% planning, 30% implementation, 20% docs
- Discussion and alternatives welcome
- Comprehensive documentation
- Follow all quality gates

**When in doubt: Ask "Is the user blocked right now?" Yes = Crisis. No = Normal.**

---

**Remember: The best documentation in a crisis is a working system. Document after the fire is out.**
