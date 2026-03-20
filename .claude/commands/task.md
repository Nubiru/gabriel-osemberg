# /task Command

## Purpose

Expand a brief prompt into a comprehensive task specification with implementation steps and acceptance criteria.

## Usage

```
/task [5-10 word prompt]
```

## Examples

```
/task Implement user session timeout handler
/task Add pagination to search results API
/task Create email validation utility
/task Write CSV export for reports
/task Add retry logic to HTTP client
```

## What Happens

### Step 1: Source Verification

1. Identify files likely affected by the task
2. Read key source files in the affected area
3. Record current state (what exists, relevant signatures, imports, dependencies)

### Step 2: Task Expansion

1. Break the task into:
   - **Requirements**: What the function/module must do (inputs, outputs, constraints)
   - **Implementation approach**: Algorithm, data structures, patterns to use
   - **File locations**: Which files to create or modify
   - **Test strategy**: Known reference values, edge cases, expected behavior
   - **Dependencies**: What other modules or libraries are needed

### Step 3: Task Specification

Output as a structured specification (in `.context/active/right-now/TASK_SPEC_{name}.md` if the project uses `.context/`, otherwise in chat):

- **Human intent**: Original prompt
- **Requirements**: Inputs, outputs, constraints, acceptance criteria
- **Technical approach**: Algorithm choice, design patterns, data flow
- **Subtasks**: Atomic, ordered implementation steps
- **Test cases**: With expected values and verification methods
- **Files to create/modify**: Exact paths
- **Definition of done**: How to verify the task is complete

## When to Use

- Starting a new function or module
- Need to think through an algorithm before coding
- Complex task spanning multiple files
- Want clear acceptance criteria before implementation

## When NOT to Use

- Simple, obvious operations (use `/fix` instead)
- Already know exactly what to write and just need to do it
- Architectural decisions needed first (use `/plan` instead)
