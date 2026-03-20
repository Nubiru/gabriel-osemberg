# /prompt — Cross-Agent Research Pipeline

**Purpose**: Manage research extraction from reference materials via external tools or agents. Question cards with Q&A pairs in single files.

**Arguments**: `$ARGUMENTS`

---

## Mode Detection

```
Parse $ARGUMENTS:
- "generate" or "gen"  -> MODE: GENERATE (create a question card)
- "consume" or "read"  -> MODE: CONSUME (digest a completed card)
- "status" or "list"   -> MODE: STATUS (show pipeline state)
- "chain"              -> MODE: CHAIN (use completed card to generate next)
```

---

## Directory Structure

```
.context/research/
  prompts/          Question cards (Q + A in same file)
    NNN-topic.md    Owner copies Q's to research tool, pastes A's back
  digested/         Processed into implementable specs
    NNN-topic.md    Algorithms, test cases, function signatures
```

---

## Question Card Format

Each card targets ONE topic from ONE source. Contains 3-5 focused questions that build conversationally.

```markdown
# NNN: [Topic]

**Source**: [Reference title (Author)]
**Status**: pending | done

---

## Q1
[Focused question — one specific thing]

**A1:**
[Answer pasted here after research]

---

## Q2
[Builds on Q1's context]

**A2:**

---
```

### Question Design Rules

- Each question asks ONE thing
- Under 500 characters (focused prompts work best)
- Questions build on each other (Q2 assumes Q1's answer is in context)
- Ask for: exact formulas, API call sequences, worked examples, parameter specs
- Avoid: broad conceptual overviews, multi-part compound questions
- End with concrete asks: "Show the formula", "List each step", "Provide a worked example"

---

## Mode 1: GENERATE

**When**: Need a new question card for a topic.

### Arguments

```
/prompt generate <source> --focus "topic" [--questions N]
```

### Steps

1. Scan `.context/research/prompts/` for highest NNN, increment
2. Read project roadmap for context
3. Read any existing checklist for what has been covered
4. Create card with 3-5 focused questions
5. Write to `.context/research/prompts/NNN-topic.md`
6. Update any tracking checklist
7. Tell the owner: "Card NNN ready. Open it, copy Q1 to your research tool with [source] selected."

---

## Mode 2: CONSUME

**When**: Owner has filled in all answers on a card.

### Arguments

```
/prompt consume NNN [--extract algorithms|test-cases|all]
```

### Steps

1. Read `.context/research/prompts/NNN-topic.md`
2. Verify all answers are filled
3. Extract into implementable form:
   - Function signatures
   - Algorithm steps
   - Test cases (input/output pairs)
   - Key constants
4. Write to `.context/research/digested/NNN-topic.md`
5. Mark card status as `done`
6. Report: what's usable, what needs follow-up

---

## Mode 3: STATUS

Scan prompts/ directory, report card states.

```
| # | Topic | Questions | Answered | Status |
|---|-------|-----------|----------|--------|
| 001 | {TOPIC_1} | 4 | 4/4 | done |
| 002 | {TOPIC_2} | 4 | 2/4 | in progress |
| 003 | {TOPIC_3} | 4 | 0/4 | pending |
```

---

## Mode 4: CHAIN

**When**: A completed card reveals gaps or next steps.

```
/prompt chain NNN -> "new topic"
```

Reads completed card, generates new card targeting the gaps.
