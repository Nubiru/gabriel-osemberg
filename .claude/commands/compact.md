# /compact — Context Compression Command

Compress your current agent context to approximately 20% of its original size while retaining critical information.

## When to Use

- Context approaching token limits
- Performance degrading due to large context
- Need to focus on current task without historical noise
- Before starting new major feature (clean slate)

## What It Does

1. **Analyzes** all context entries and scores them by:
   - **Recency**: Recent entries score higher
   - **Relevance**: Errors, decisions, active work score higher
   - **Position**: Current task context scores higher

2. **Compresses** by:
   - **Keeping**: High-priority entries (recent work, decisions, blockers)
   - **Summarizing**: Medium-priority entries (compress to key points)
   - **Removing**: Low-priority entries (old logs, completed tasks)

3. **Creates backup** before compacting (can undo)

4. **Reports**:
   - Tokens before/after
   - Compression ratio
   - Quality score (how much info retained)

## Usage

```bash
# Basic usage - compress current context
/compact

# Undo last compaction (restore backup)
/compact --undo

# Dry run (see what would be removed without doing it)
/compact --dry-run

# Aggressive compression (target 10% instead of 20%)
/compact --aggressive
```

## Example Output

```
Context Compression Report

Before: {N} tokens ({N}% of limit)
After:  {N} tokens ({N}% of limit)
Ratio:  {N}% reduction
Quality: {N}/100 (retention score)

Kept:
  - Last N messages (current work)
  - N architecture decisions
  - N active blockers
  - Error logs from last hour

Summarized:
  - N older messages -> N key points
  - Test results -> pass/fail summary

Removed:
  - N old debug logs
  - Completed task checklists
  - Redundant clarifications

Backup saved to: .context/backups/compact-{DATE}.json
Undo with: /compact --undo
```

## Safety

- **Always creates backup** before compacting
- **Undo capability** restores previous state
- **Quality score** warns if too much info lost
- **Dry run** lets you preview before executing

## Performance

- **Speed**: <500ms for 200KB context
- **Compression**: Typically 70-85% reduction
- **Quality**: Aims for 85%+ quality score

## Related Commands

- `/snapshot pre-compact` — Save checkpoint before compacting
- `/restore` — Restore from a previous snapshot

## Troubleshooting

**"Quality score too low"**: Use `--dry-run` first, may need manual cleanup
**"Nothing to compress"**: Context already small
**"Backup failed"**: Check disk space and permissions
**"Lost important info"**: Use `/compact --undo` immediately
