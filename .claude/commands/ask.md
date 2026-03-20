# /ask Command

## Purpose

Get information, clarification, or explanation about the codebase, documentation, architecture, or development processes. Pure conversational mode (read-only) with optional documentation.

## Usage

```
/ask [your question]
```

## Examples

```
/ask How does {MODULE} work?
/ask What's the difference between {COMMAND_A} and {COMMAND_B}?
/ask Explain the {CONCEPT} architecture
/ask Where should I put {FILE_TYPE}?
/ask What design patterns are used for {FEATURE}?
/ask What are the quality gates for {PHASE}?
```

## What Happens

### Step 1: Context Loading

1. Load relevant standards documentation
2. Load relevant project documentation if exists
3. Load relevant development context if needed
4. Search codebase if question relates to implementation

### Step 2: Information Synthesis

1. Extract relevant information from loaded contexts
2. Cross-reference multiple sources
3. Identify key concepts and relationships
4. Prepare comprehensive explanation

### Step 3: Conversational Response

1. Answer question clearly and concisely
2. Provide examples if helpful
3. Reference source documents
4. Suggest related concepts or next steps

### Step 4: Optional Documentation

1. User can request to save valuable Q&A
2. Saves to `{QA_OUTPUT_PATH}/QA_[Topic]_[Date].md`
3. For future reference and knowledge building

## Output

**Primary Output**: Conversational response (in chat)

**Optional Output**: Saved Q&A document (if explicitly requested)

### Response Format

```
**Answer**: [Clear, concise explanation]

**How It Works**:
1. [Step-by-step breakdown if relevant]
2. [...]

**Example**:
[Concrete example if helpful]

**References**:
- [relevant doc path]
- [relevant doc path]

**Related Concepts**:
- [Related topic 1]
- [Related topic 2]

**Next Steps** (optional):
- [Suggestion for user's workflow]
```

## When to Use

**Use `/ask` when**:
- Understanding how something works (system, module, pattern)
- Clarifying concepts (architecture, workflows, standards)
- Finding documentation (where is X documented?)
- Learning best practices (how should I approach Y?)
- Understanding command differences (what's the difference between X and Y?)
- Navigating the project (where do I put Z?)
- Quick reference lookups (what are the quality gates?)

**Don't use `/ask` when**:
- Want to execute a task (implement, not just learn)
- Need to fix something
- Want to plan architecture (use `/propose`)
- Need external research with web access (use `/research`)
- Need decision analysis (use `/consider`)

## Mode: Read-Only

**Important**: `/ask` is read-only (no code changes, no file creation unless explicitly requested)

**Permissions**:
- Read any project documentation
- Read any source code
- Search codebase
- Reference commands, workflows, standards
- Write only if user explicitly requests Q&A save
- No code changes
- No task execution

## Optional: Q&A Documentation

**When to Save**:
- User explicitly requests: "Save this"
- High-value explanation (core concepts)
- Frequently asked question
- Complex topic requiring detailed explanation

**When NOT to Save**:
- Simple, obvious answers
- Already documented in standards
- One-time question
- Low future reference value

**Saved Q&A Format**:

```markdown
# Q&A: [Topic]

**Date**: [YYYY-MM-DD]
**Question**: [original user question]
**Context**: [why this was asked]

## Answer

[comprehensive answer provided]

## References

- [source documents]

## Related Questions

- [related topics for further exploration]
```

## Integration

**References**:
- `{CONVENTIONS_PATH}` - Coding standards
- `{ARCHITECTURE_PATH}` - Project architecture
- `{QUALITY_GATES_PATH}` - Quality requirements
- `.claude/commands/` - Command definitions

**Context Loading Priority**:
1. Standards - Universal project standards
2. Architecture - Project-specific docs
3. Active context - Current work state
4. Codebase - Implementation details

## Anti-Patterns

### Anti-Pattern 1: Using /ask for Execution
**Problem**: `/ask` cannot execute tasks
**Fix**: Use the appropriate implementation command instead

### Anti-Pattern 2: Using /ask for External Research
**Problem**: `/ask` searches internal docs, not the web
**Fix**: Use `/research` for external information gathering

### Anti-Pattern 3: Using /ask for Decisions
**Problem**: `/ask` provides information, not structured decision analysis
**Fix**: Use `/consider` for weighted trade-off analysis

## Tool Permissions

- **Read**: All project docs, source code, development context
- **Write**: `{QA_OUTPUT_PATH}/QA_*.md` (only on explicit user request)
- **Execute**: None (read-only mode)
- **Escalate**: To appropriate command if user needs execution

---

**Command Type**: Conversational (read-only)
**Integration**: Information gathering; escalates to /research, /consider, /propose as needed
