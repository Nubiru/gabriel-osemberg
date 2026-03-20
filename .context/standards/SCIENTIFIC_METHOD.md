# Scientific Method -- Development Guide

## TDD Integration with Scientific Method

The Scientific Method naturally aligns with Test-Driven Development:

| Scientific Method Stage | TDD Stage              | Description                         |
| ----------------------- | ---------------------- | ----------------------------------- |
| **Prediction**          | Write Tests            | Define expected behavior as tests   |
| **Experimentation**     | RED -> GREEN -> REFACTOR | Implement solution to pass tests  |
| **Analysis**            | Validate Tests Pass    | Confirm solution meets expectations |
| **Conclusion**          | Accept/Iterate         | Ship or try new hypothesis          |

**Key Insight**: Prediction stage = Writing tests. Experimentation stage = Making tests pass.

This approach ensures every implementation is:

- **Hypothesis-driven**: Clear reason for every change
- **Testable**: Success criteria defined upfront
- **Measurable**: Objective validation of outcomes
- **Documented**: Knowledge captured for future reference

---

## The 7 Stages

### Stage 1: Observation

Observe the current state. Gather evidence. Read error messages, logs, metrics, user reports.

```
OBSERVATION: {What you see}
EVIDENCE: {Data, error messages, metrics}
CONTEXT: {When it started, what changed}
```

### Stage 2: Question

Define the problem precisely. Scope it. Identify constraints.

```
QUESTION: {What needs to be answered or solved?}
SCOPE: {Boundaries of the investigation}
CONSTRAINTS: {Limitations, requirements, deadlines}
```

### Stage 3: Hypothesis

Propose an explanation or solution. Be specific and falsifiable.

```
HYPOTHESIS: {Your proposed explanation/solution}
PREDICTION: {What you expect to happen if hypothesis is correct}
RISK: {What could go wrong, and how to mitigate}
```

### Stage 4: Prediction (= Write Tests)

Define measurable success criteria. Write tests BEFORE implementing.

```
PREDICTION: {Specific, measurable expected outcome}
SUCCESS CRITERIA: {Concrete thresholds}
TEST: {How you will verify}
```

### Stage 5: Experimentation (= Make Tests Pass)

Implement the solution. Run the tests. Measure results.

```
EXPERIMENT: {What you built/changed}
IMPLEMENTATION: {Key details}
TEST: {Results of running tests}
```

### Stage 6: Analysis

Compare results against predictions. Was the hypothesis correct?

```
ANALYSIS: Hypothesis {CORRECT / PARTIALLY CORRECT / INCORRECT}
RESULTS: {What actually happened}
UNEXPECTED: {Anything surprising}
CONCLUSION: {What this means}
```

### Stage 7: Conclusion

Accept, iterate, or reject. Document learnings for the future.

```
CONCLUSION: {ACCEPT / ITERATE / REJECT}
DECISION: {What to do next}
LEARNINGS: {What was learned}
NEXT STEPS: {Follow-up actions}
```

---

## Integration with Agent Workflows

All agents reference this Scientific Method framework:

**Orchestrator / Lead Agent**:
- Ensures all tasks follow 7-stage process
- Validates each stage before proceeding
- Escalates if stages are skipped

**Test-Writer Agent**:
- Executes Stage 4 (Prediction): Write tests first
- Enforces TDD cycle: RED -> GREEN -> REFACTOR
- Validates test coverage meets criteria

**Code-Reviewer Agent**:
- Validates Stage 5 (Experimentation): Implementation matches tests
- Checks Stage 6 (Analysis): Tests pass, metrics met
- Ensures Stage 7 (Conclusion): Documentation complete

**Architecture Agent**:
- Applies Scientific Method to design decisions
- Creates ADRs following Stage 7 format
- Validates hypotheses with proof-of-concept

**Refactoring Agent**:
- Treats refactoring as experiment
- Hypothesis: Refactor will improve maintainability
- Prediction: Tests pass, no regressions
- Experiment: Implement refactor
- Analysis: Measure impact
- Conclusion: Accept or revert

---

## Anti-Patterns (What NOT to Do)

### Anti-Pattern 1: Skip Observation

**Problem**: Jump straight to solution without understanding the problem.
**Example**: "Let's add caching" without knowing why it is slow.
**Fix**: Always start with Stage 1 (Observation).

### Anti-Pattern 2: Vague Hypothesis

**Problem**: "This should work" without specific predictions.
**Example**: "Using Redis will make it faster."
**Fix**: "Redis will reduce response time from 2s to 200ms for 95% of requests."

### Anti-Pattern 3: No Test Design

**Problem**: Implement without defining success criteria.
**Example**: Deploy and hope it works.
**Fix**: Always define measurable success criteria in Stage 4.

### Anti-Pattern 4: Ignore Results

**Problem**: Do not analyze if hypothesis was correct.
**Example**: Deploy and forget.
**Fix**: Always analyze results in Stage 6.

### Anti-Pattern 5: No Learning

**Problem**: Do not document what worked and what did not.
**Example**: Repeat same mistakes.
**Fix**: Always document learnings in Stage 7.

### Anti-Pattern 6: Skip Stages for "Simple" Tasks

**Problem**: "This is too simple for scientific method."
**Example**: Quick bug fix without process.
**Fix**: Even 5-minute tasks benefit from abbreviated process.

---

## Examples

### Example 1: Bug Fix (5 minutes)

**Stage 1**: User reports form validation rejects valid input. Error log shows `ValidationError`.
**Stage 2**: Why is validation too strict? Scope: form save only.
**Stage 3**: Hypothesis: Regex pattern is too restrictive.
**Stage 4**: Updated pattern will accept valid inputs. Write test with edge cases.
**Stage 5**: Update regex. Run tests.
**Stage 6**: Hypothesis correct. All valid inputs accepted.
**Stage 7**: Accept. Deploy fix. Add comprehensive validation tests.

### Example 2: Performance Optimization (2 days)

**Stage 1**: API response p95 = 2.5s (target < 500ms). Database CPU at 90%.
**Stage 2**: How to reduce response time? Scope: search endpoint only.
**Stage 3**: Hypothesis: Database indexes + query optimization will solve it.
**Stage 4**: Prediction: p95 < 500ms, CPU < 70%. Run load test for 2 hours.
**Stage 5**: Add composite index. Optimize query. Add result caching.
**Stage 6**: p95 = 350ms, CPU = 45%. Hypothesis correct. Caching gave bonus 20%.
**Stage 7**: Accept. Deploy. Monitor production. Consider caching other slow queries.

### Example 3: Feature Development (1 week)

**Stage 1**: Users need password reset; current flow broken. 40% failure rate.
**Stage 2**: How to implement secure, reliable password reset?
**Stage 3**: Hypothesis: Token-based reset with email verification will solve it.
**Stage 4**: Prediction: 95% success rate, no security vulnerabilities, satisfaction > 4.0/5.0.
**Stage 5**: Implement token generation, email sending, validation, password update.
**Stage 6**: 98% success, 1 minor security issue (fixed), satisfaction 4.2/5.0.
**Stage 7**: Accept. Deploy. Plan SMS option for next iteration.

---

## Summary

### Key Takeaways

1. **Always Start with Evidence**: Never guess, always observe first
2. **Make Testable Predictions**: Every hypothesis must be measurable
3. **Document Everything**: Knowledge is your most valuable asset
4. **Learn from Results**: Every experiment teaches something
5. **Iterate and Improve**: Use learnings to make better hypotheses

### Best Practices

- **Use abbreviated process for small tasks** (5-15 minutes)
- **Use full process for complex tasks** (1+ days)
- **Always document learnings** for future reference
- **Share knowledge** with the team through ADRs and documentation
- **Review and improve** your scientific method application regularly

### Integration Points

- **TDD**: Prediction stage = Write tests
- **Code Review**: Validate all stages completed
- **Architecture**: Use for design decisions
- **Debugging**: Systematic approach to problem solving
- **Learning**: Document what works and what does not

---

**Remember**: The Scientific Method is not just a process -- it is a mindset. Apply it consistently to become a more effective developer.
