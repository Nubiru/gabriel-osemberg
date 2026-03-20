# /commit -- Draft Commit Message

**Purpose**: Review changes and draft a professional commit message for human review.

**Arguments**: `$ARGUMENTS`

---

## Step 1: Gather Context

Run passive git commands to understand the current state:

```bash
git status                  # staged/unstaged/untracked files
git diff --stat             # summary of changes
git diff                    # full diff (for small changesets)
git log --oneline -10       # recent commit style reference
```

For large changesets, use `git diff --stat` + targeted file reads instead of full `git diff`.

---

## Step 2: Analyze Changes

1. Read the actual diff -- never guess what changed
2. Group changes by theme (are these one logical change or multiple?)
3. Identify the primary intent (new feature, bug fix, refactor, docs, etc.)
4. Note any files that should NOT be committed (build artifacts, secrets, temp files)

---

## Step 3: Draft Commit Message

Write to `.context/active/commits/NEXT_COMMIT.md` (or present in chat if `.context/` is not used):

```markdown
# Next Commit

## Message

\```
{TYPE}({SCOPE}): subject line (imperative, <72 chars)

- Bullet 1: what changed and why
- Bullet 2: what changed and why
\```

## Files

\```
git add \
  path/to/file1 \
  path/to/file2
\```
```

### Commit Message Conventions

- **Types**: `fix`, `feat`, `refactor`, `perf`, `docs`, `test`, `chore`, `build`, `ci`, `style`
- **Scopes**: Use the module, component, or area name relevant to the project (e.g., `auth`, `api`, `db`, `ui`, `config`, `build`)
- **Subject**: imperative mood ("add" not "added"), no period, < 72 chars
- **Body**: bullet points explaining what changed and why
- **Files section**: list files to stage as a copy-pasteable `git add` command
- **Do NOT include**: build artifacts, `.env` files, IDE config, or any gitignored files

---

## Step 4: Output Summary

```
Commit message drafted.

Subject: {TYPE}({SCOPE}): subject line
Files to stage: N files

Review the message, then stage + commit.
```

---

## Edge Cases

- **No changes detected**: "Nothing to commit. Working tree clean."
- **Mixed concerns**: Suggest splitting into multiple commits if changes are unrelated
- **Sensitive files**: Warn if `.env`, credentials, or secrets are in the diff

---

## Output Constraints

- All output is copy-pasteable
- Evidence-based: read the actual diff, don't guess what changed
- Message follows conventional commits format
