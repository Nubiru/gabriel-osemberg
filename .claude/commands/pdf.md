# /pdf — Document Extraction Pipeline

**Purpose**: Extract data, formulas, algorithms, and knowledge from PDF reference materials directly within Claude sessions. Multi-pass pipeline for thorough extraction.

**Arguments**: `$ARGUMENTS`

---

## Mode Detection

```
Parse $ARGUMENTS:
- "<path>"                    -> MODE: SURVEY (default)
- "<path> survey"             -> MODE: SURVEY
- "<path> extract <questions>" -> MODE: EXTRACT
- "<path> full"               -> MODE: FULL
- "<path> data"               -> MODE: DATA
```

---

## Constraints

- Claude Read tool supports PDF with `pages` parameter (e.g., "1-5", "10-20")
- Maximum 20 pages per Read call
- Large PDFs (100+ pages) require multiple passes
- NEVER fabricate content — if a section is unreadable, say so
- Always cite page numbers for extracted information
- Output includes source file path for traceability

---

## Mode 1: SURVEY (default)

**When**: First encounter with a PDF. Quick assessment.

### Steps

1. Read pages 1-5 (cover, copyright, TOC, intro)
2. If TOC found, note chapter structure and page ranges
3. Read last 2 pages (appendices, bibliography, index)
4. Output structured summary

### Output

```markdown
# Survey: [filename]

**Title**: [full title]
**Author**: [author(s)]
**Pages**: ~[N] pages
**Published**: [year]

## Structure
- Ch 1: [topic] (pp. N-N)
- Ch 2: [topic] (pp. N-N)
- ...

## Key Topics
- [topic 1]
- [topic 2]

## Relevance to {PROJECT_NAME}
- [how it helps]
- [specific data/algorithms available]

## Extraction Priority
[P0/P1/P2] — [reason]

## Recommended Next Step
[survey deeper | extract specific questions | full extraction | data extraction]
```

---

## Mode 2: EXTRACT

**When**: User has specific questions for a document.

### Arguments

```
/pdf <path> extract "What formula is used for X?" "How is Y computed?"
```

### Steps

1. Run quick survey (pages 1-5) to locate relevant chapters
2. For each question:
   a. Identify likely chapter/section from TOC
   b. Read those pages (20-page chunks)
   c. Find answer with exact page citation
3. Output answers with citations

### Output

```markdown
# Extract: [filename]

## Q1: [question]
**Answer**: [answer with specifics]
**Source**: pp. [N-N]
**Confidence**: [high/medium/low]

## Q2: [question]
...
```

---

## Mode 3: FULL

**When**: Comprehensive extraction of an entire document. Use subagent to avoid polluting main context.

### Steps

1. **Survey pass**: Read pages 1-10. Map chapter structure
2. **Chapter passes**: For each chapter, read in 20-page chunks. Extract:
   - Key concepts and definitions
   - Formulas and algorithms
   - Data tables and constants
   - Test cases (worked examples)
3. **Synthesis**: Combine chapter extracts into digest
4. **Write** digest to `.context/research/digests/<filename-slug>.md`

### Output

Written to `.context/research/digests/<slug>.md`:

```markdown
# Digest: [title]

**Source**: [full path]
**Extracted**: [date]
**Pages read**: [N of M]

## Key Algorithms
- [algorithm 1]: [description] (p. N)

## Constants & Data
- [constant]: [value] (p. N)

## Formulas
- [formula name]: [description] (p. N)

## Implementable Specs
[Function signatures, test cases derived from worked examples]
```

---

## Mode 4: DATA

**When**: Extract tables, constants, formulas for direct implementation.

### Steps

1. Survey to find data-heavy sections (tables, appendices, catalogs)
2. Read those sections in 20-page chunks
3. Extract into implementation-ready format:
   - Constants as named values
   - Tables as structured data
   - Formulas as function pseudocode
   - Worked examples as test case input/output pairs

### Output

```markdown
# Data Extract: [filename]

## Constants
{LANGUAGE}-appropriate constant definitions with page citations

## Tables
{LANGUAGE}-appropriate data structures with source references

## Formulas -> Functions
Pseudocode or {LANGUAGE} function signatures with algorithm from source

## Test Cases
Input/output pairs from worked examples in the source material
```

---

## When to Use

- **survey**: First time looking at a new PDF. Quick 2-minute assessment
- **extract**: You know what you need. Targeted Q&A
- **full**: Important reference that will be used repeatedly. Worth the investment
- **data**: Catalogs, tables, reference data — anything that becomes structured data

## When NOT to Use

- For very short documents (< 10 pages) — just read them directly
- For non-PDF formats (.epub, .mobi, .djvu) — convert to PDF first
- When conversational exploration is more appropriate than structured extraction
