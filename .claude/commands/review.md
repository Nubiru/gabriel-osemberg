# /review Command

## Purpose

Perform a systematic code quality audit with prioritized, actionable feedback.

## Usage

```
/review [file, directory, or scope]
```

## Examples

```
/review src/auth/session.py
/review src/utils/
/review recent changes
/review src/api/handlers/users.ts
```

## What Happens

### Step 1: Read Source Files

Read every file in the review scope. A review that doesn't read the code is a guess.

### Step 2: Code Analysis

Check against these categories:

- **Correctness**: Does the logic produce correct results? Are edge cases handled?
- **Error handling**: Are errors caught, propagated, and reported properly?
- **Security**: Input validation, injection risks, authentication/authorization gaps, secrets exposure
- **Performance**: Unnecessary allocations, N+1 queries, missing caching, algorithmic complexity
- **Naming**: Are names descriptive, consistent, and following project conventions?
- **Complexity**: Are functions/methods too long? Too many parameters? Deep nesting?
- **Test coverage**: Is the code testable? Are critical paths tested?
- **Architecture compliance**: Does the code respect the project's architectural boundaries?
- **{LANGUAGE_SPECIFIC_CHECKS}**: Add language/framework-specific checks as needed (e.g., memory safety for C/C++, type safety for TypeScript, concurrency issues for Go, SQL injection for Python/Django)

### Step 3: Prioritized Feedback

Rank issues by severity:

- **Critical**: Security vulnerabilities, data loss risks, incorrect results, crashes
- **Important**: Performance issues, missing error handling, convention violations, missing tests
- **Minor**: Style, naming improvements, comment quality, optional optimizations

### Step 4: Report

Output feedback with this format:

```
SEVERITY: [Critical | Important | Minor]
LOCATION: file:line
ISSUE: What's wrong
FIX: How to fix (with code example)
```

End with a summary:

```
## Summary
- Critical: N issues
- Important: N issues
- Minor: N issues
- Overall: [PASS | PASS WITH NOTES | NEEDS WORK]
```

## When to Use

- After implementing a function or module
- Before drafting a commit message
- Periodic code health checks
- Reviewing someone else's changes
- Before merging a feature branch

## When NOT to Use

- Code is a rough prototype / spike (review after stabilization)
- Already planning to rewrite the module
