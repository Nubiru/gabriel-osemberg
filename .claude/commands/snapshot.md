# /snapshot — Save Context Checkpoint

Create a named snapshot of your current agent context state for later restoration.

## When to Use

- **Before risky operations**: Major refactoring, architecture changes
- **Before experiments**: Trying new approach, testing hypothesis
- **Milestone checkpoints**: End of sprint, after major feature
- **Known good states**: After tests pass, after successful deployment

## What It Does

1. **Captures** complete agent context:
   - All context entries
   - Token counts
   - Active tasks
   - Recent decisions

2. **Stores** with metadata:
   - Timestamp
   - Purpose/description
   - Tags (optional)
   - Agent ID
   - Context size

3. **Manages** snapshots:
   - Auto-expires after 30 days
   - Max 50 snapshots per agent
   - Searchable by name/tags/date

## Usage

```bash
# Create snapshot with descriptive name
/snapshot pre-refactor

# Create with purpose description
/snapshot pre-api-redesign --purpose "Before breaking API changes"

# Create with tags
/snapshot milestone-v1 --tags release,stable,production

# List all snapshots
/snapshot list

# Show snapshot details
/snapshot show pre-refactor

# Compare two snapshots
/snapshot diff pre-refactor post-refactor

# Delete old snapshot
/snapshot delete old-experiment
```

## Example Output

```
Snapshot Created

Name: pre-refactor
Purpose: Before refactoring
Timestamp: {DATE} {TIME}
Agent: {AGENT_ID}
Size: {N} tokens
Entries: {N} context entries

Tags: refactor, risky
Expires: {DATE} (30 days)

Restore with: /restore pre-refactor
Delete with: /snapshot delete pre-refactor
```

## Snapshot List Example

```
Available Snapshots ({N})

1. pre-refactor          (2 hours ago)  42KB  [refactor]
2. milestone-sprint-1    (1 day ago)    38KB  [milestone, stable]
3. pre-migration         (3 days ago)   45KB  [database, risky]
4. after-tests-pass      (5 days ago)   31KB  [testing, green]
5. weekly-backup         (7 days ago)   40KB  [backup, weekly]

Total: {N}KB storage used
Auto-cleanup: Expires after 30 days
Max snapshots: 50 per agent
```

## Snapshot Comparison Example

```
Snapshot Diff: pre-refactor -> post-refactor

Changes:
  + Added {N} entries
  - Removed {N} entries
  ~ Modified {N} entries

Size: {N}KB -> {N}KB ({N}%)
Tokens: {N} -> {N} ({N}%)

Modified entries:
  - {FILE_1}: {N} lines changed
  - {FILE_2}: {N} lines changed
```

## Safety

- **Read-only**: Snapshots never modify current state
- **Isolated storage**: Separate from active context
- **Auto-expiration**: Old snapshots cleaned up automatically
- **Max limit**: Prevents unlimited storage growth

## Storage

- **Location**: `.context/backups/snapshots/`
- **Format**: JSON with compression
- **Max size**: 1MB per snapshot
- **Max count**: 50 per agent
- **Retention**: 30 days

## Performance

- **Create**: <100ms for 50KB context
- **Restore**: <200ms
- **Compare**: <50ms
- **List**: <10ms

## Related Commands

- `/restore <name>` — Restore from snapshot
- `/compact` — Compress before snapshotting (smaller snapshots)

## Troubleshooting

**"Snapshot limit reached"**: Delete old snapshots or wait for auto-expiration
**"Snapshot too large"**: Run `/compact` first to reduce size
**"Cannot create snapshot"**: Check disk space and permissions
**"Snapshot not found"**: Use `/snapshot list` to see available snapshots
