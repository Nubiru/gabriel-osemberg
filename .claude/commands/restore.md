# /restore — Restore from Snapshot

Restore agent context from a previously saved snapshot.

## When to Use

- **After failed experiment**: Tried new approach that didn't work
- **After breaking changes**: Refactoring broke tests, need to revert
- **Compare approaches**: Restore old state to compare with new
- **Recover from mistakes**: Accidentally deleted important context

## What It Does

1. **Loads** snapshot from storage
2. **Creates backup** of current context (before restore)
3. **Replaces** current context with snapshot state
4. **Reports** what changed

## Usage

```bash
# Restore from named snapshot
/restore pre-refactor

# List available snapshots first
/restore --list

# Restore with confirmation prompt
/restore pre-refactor --confirm

# Restore and compare changes
/restore pre-refactor --diff
```

## Example Output

```
Restoring from Snapshot

Snapshot: pre-refactor
Created: 2 hours ago ({DATE})
Size: {N} tokens
Purpose: Before refactoring

Current context will be backed up to:
   .context/backups/pre-restore-{DATE}.json

Changes to apply:
  + Will add {N} entries (old state)
  - Will remove {N} entries (current state)
  ~ Will modify {N} entries

Continue? [y/N]: y

Restore Complete

Before: {N} tokens
After:  {N} tokens
Change: {+/-N} tokens ({N}%)

Context restored to state from {TIME_AGO}.
Backup saved for safety.

Undo with: /restore pre-restore-{DATE}
```

## Interactive Mode

```bash
/restore --list

Available Snapshots ({N})

1. pre-refactor          (2 hours ago)  42KB  [refactor]
2. milestone-sprint-1    (1 day ago)    38KB  [milestone, stable]
3. pre-migration         (3 days ago)   45KB  [database, risky]

Select snapshot to restore [1-N]: 1

Loading snapshot: pre-refactor...
[Shows changes and asks for confirmation]
```

## Safety

- **Always creates backup** before restoring
- **Confirmation required** by default (unless `--force`)
- **Shows diff** before applying changes
- **Reversible**: Backup can be restored

## Comparison Output

```
Restore Preview

Current state vs. Snapshot "pre-refactor":

Files that will change:
  {FILE_1}:
    Current:  {N} lines (new implementation)
    Snapshot: {N} lines (old implementation)

  {FILE_2}:
    Current:  {N} lines
    Snapshot: {N} lines

Context entries:
  + {N} entries will be added
  - {N} entries will be removed
  ~ {N} entries will be modified

Tokens: {N} -> {N} ({+/-N}%)
```

## Restore Workflow

```
Current State
     |
[Backup created automatically]
     |
Snapshot Loaded
     |
Changes Applied
     |
Restored State
```

If something goes wrong:
```bash
# Restore from the automatic backup
/restore pre-restore-{DATE}
```

## Performance

- **Load snapshot**: <100ms
- **Create backup**: <100ms
- **Apply changes**: <50ms
- **Total**: <250ms

## Related Commands

- `/snapshot <name>` — Create new snapshot
- `/snapshot list` — View available snapshots
- `/snapshot diff <a> <b>` — Compare snapshots
- `/compact --undo` — Undo last compaction (similar to restore)

## Flags

- `--list` — Show available snapshots before restoring
- `--confirm` — Ask for confirmation (default)
- `--force` — Skip confirmation
- `--diff` — Show detailed diff before restoring
- `--dry-run` — Show what would change without applying

## Troubleshooting

**"Snapshot not found"**: Use `/restore --list` to see available snapshots
**"Backup failed"**: Check disk space and permissions (restore aborted for safety)
**"Snapshot corrupted"**: File may be damaged, try different snapshot
**"Cannot restore"**: Check file permissions

## Best Practices

1. **Review diff** before restoring (`--diff` flag)
2. **Create snapshot** of current state before restore
3. **Test after restore** — run tests to verify state
4. **Clean up** old snapshots regularly
5. **Name snapshots well** — use descriptive names like `pre-refactor` not `snapshot1`
